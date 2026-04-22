import os
import json
import sqlite3
import uuid
import asyncio
import logging
from datetime import datetime, timedelta
from pathlib import Path
from typing import Optional, List, Dict, Any
from fastapi import FastAPI, HTTPException, BackgroundTasks
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
import ollama
import psutil
from contextlib import asynccontextmanager

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# Global state for connection management
class AppState:
    def __init__(self):
        self.ollama_available = True
        self.last_health_check = None
        self.connection_pool = {}
        
app_state = AppState()

app = FastAPI(
    title="Max API", 
    version="1.1.0",
    description="Local AI Assistant with enhanced reliability"
)

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

DB_PATH = Path.home() / ".max" / "memory.db"
DB_PATH.parent.mkdir(parents=True, exist_ok=True)

# Database connection pool
class DatabasePool:
    def __init__(self, max_connections=10):
        self.max_connections = max_connections
        self.connections = []
        self.lock = asyncio.Lock()
    
    async def get_connection(self):
        async with self.lock:
            if self.connections:
                return self.connections.pop()
            else:
                return sqlite3.connect(str(DB_PATH), check_same_thread=False)
    
    async def return_connection(self, conn):
        async with self.lock:
            if len(self.connections) < self.max_connections:
                self.connections.append(conn)
            else:
                conn.close()

db_pool = DatabasePool()


def init_db():
    conn = sqlite3.connect(str(DB_PATH))
    c = conn.cursor()
    c.execute("""
        CREATE TABLE IF NOT EXISTS conversations (
            id TEXT PRIMARY KEY,
            role TEXT NOT NULL,
            content TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            session_id TEXT DEFAULT 'default'
        )
    """)
    c.execute("""
        CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            due TEXT,
            completed INTEGER DEFAULT 0,
            priority TEXT,
            tag TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT DEFAULT CURRENT_TIMESTAMP
        )
    """)
    c.execute("""
        CREATE TABLE IF NOT EXISTS system (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
    """)
    # Add indexes for better performance
    c.execute("CREATE INDEX IF NOT EXISTS idx_conversations_timestamp ON conversations(timestamp)")
    c.execute("CREATE INDEX IF NOT EXISTS idx_conversations_session ON conversations(session_id)")
    c.execute("CREATE INDEX IF NOT EXISTS idx_tasks_created_at ON tasks(created_at)")
    c.execute("CREATE INDEX IF NOT EXISTS idx_tasks_completed ON tasks(completed)")
    conn.commit()
    conn.close()

async def check_ollama_health():
    """Check if Ollama is available and responsive"""
    try:
        models = ollama.list()
        app_state.ollama_available = True
        app_state.last_health_check = datetime.now()
        logger.info("Ollama health check passed")
        return True
    except Exception as e:
        app_state.ollama_available = False
        logger.warning(f"Ollama health check failed: {e}")
        return False

async def get_fallback_response(prompt: str) -> str:
    """Fallback response when Ollama is unavailable"""
    fallback_responses = [
        "I'm currently experiencing connectivity issues with my AI backend. Please check if Ollama is running.",
        "My AI services are temporarily unavailable. Please ensure Ollama is installed and running.",
        "I cannot connect to my AI model right now. Please verify Ollama is accessible."
    ]
    return fallback_responses[hash(prompt) % len(fallback_responses)]


init_db()


class ChatRequest(BaseModel):
    message: str
    model: str = "qwen3:8b"


class ChatResponse(BaseModel):
    response: str
    conversation_id: str


class Task(BaseModel):
    id: Optional[str] = None
    title: str
    due: Optional[str] = None
    completed: bool = False
    priority: Optional[str] = None
    tag: Optional[str] = None


class SystemStats(BaseModel):
    cpu: float
    memory: float
    disk: float
    network: float


@app.get("/")
async def root():
    return {"status": "ok", "name": "Max", "version": "1.0.0"}


@app.post("/chat", response_model=ChatResponse)
async def chat(req: ChatRequest, background_tasks: BackgroundTasks):
    conversation_id = str(uuid.uuid4())
    timestamp = datetime.now().isoformat()

    # Use connection pool
    conn = await db_pool.get_connection()
    try:
        c = conn.cursor()
        c.execute("INSERT INTO conversations (id, role, content, timestamp, session_id) VALUES (?, ?, ?, ?, ?)", 
                  (conversation_id, "user", req.message, timestamp, "default"))
        conn.commit()

        # Get conversation history with better query
        c.execute("""
            SELECT role, content FROM conversations 
            WHERE session_id = 'default' 
            ORDER BY timestamp DESC LIMIT 10
        """)
        history = []
        for role, content in c.fetchall():
            history.insert(0, {"role": role, "content": content})

        system_prompt = """You are Max, a local AI assistant. Be concise, helpful, and friendly. 
You have access to system information. Keep responses short unless asked for detail."""

        messages = [{"role": "system", "content": system_prompt}] + history

        # Try to get AI response with fallback
        assistant_msg = ""
        try:
            # Check Ollama health first
            if not app_state.ollama_available:
                await check_ollama_health()
            
            if app_state.ollama_available:
                response = ollama.chat(model=req.model, messages=messages)
                assistant_msg = response["message"]["content"]
            else:
                assistant_msg = await get_fallback_response(req.message)
                
        except Exception as e:
            logger.error(f"Chat error: {e}")
            assistant_msg = await get_fallback_response(req.message)

        # Store assistant response
        c.execute("INSERT INTO conversations (id, role, content, timestamp, session_id) VALUES (?, ?, ?, ?, ?)",
                  (str(uuid.uuid4()), "assistant", assistant_msg, timestamp, "default"))
        conn.commit()
        
        # Schedule cleanup in background
        background_tasks.add_task(cleanup_old_conversations, conn)
        
    except Exception as e:
        logger.error(f"Database error: {e}")
        await db_pool.return_connection(conn)
        raise HTTPException(status_code=500, detail="Database error")
    
    await db_pool.return_connection(conn)
    return ChatResponse(response=assistant_msg, conversation_id=conversation_id)

async def cleanup_old_conversations(conn, max_age_days=30):
    """Clean up old conversations to prevent database bloat"""
    try:
        c = conn.cursor()
        cutoff_date = (datetime.now() - timedelta(days=max_age_days)).isoformat()
        c.execute("DELETE FROM conversations WHERE timestamp < ?", (cutoff_date,))
        conn.commit()
        logger.info(f"Cleaned up conversations older than {max_age_days} days")
    except Exception as e:
        logger.error(f"Cleanup error: {e}")


@app.get("/memory")
async def get_memory():
    conn = await db_pool.get_connection()
    try:
        c = conn.cursor()
        c.execute("SELECT role, content, timestamp FROM conversations ORDER BY timestamp DESC LIMIT 20")
        messages = [{"role": r, "content": c, "timestamp": t} for r, c, t in c.fetchall()]
    finally:
        await db_pool.return_connection(conn)
    return {"messages": messages}


@app.delete("/memory")
async def clear_memory():
    conn = await db_pool.get_connection()
    try:
        c = conn.cursor()
        c.execute("DELETE FROM conversations")
        conn.commit()
        logger.info("Memory cleared by user request")
    finally:
        await db_pool.return_connection(conn)
    return {"status": "cleared"}


@app.get("/tasks")
async def get_tasks():
    conn = sqlite3.connect(str(DB_PATH))
    c = conn.cursor()
    c.execute("SELECT id, title, due, completed, priority, tag, created_at FROM tasks ORDER BY created_at DESC")
    tasks = [{"id": i, "title": t, "due": d, "completed": bool(c), "priority": p, "tag": g, "created_at": ca} 
            for i, t, d, c, p, g, ca in c.fetchall()]
    conn.close()
    return {"tasks": tasks}


@app.post("/tasks")
async def add_task(task: Task):
    task_id = task.id or str(uuid.uuid4())
    created_at = datetime.now().isoformat()
    
    conn = sqlite3.connect(str(DB_PATH))
    c = conn.cursor()
    c.execute("INSERT INTO tasks VALUES (?, ?, ?, ?, ?, ?, ?)",
              (task_id, task.title, task.due, int(task.completed), task.priority, task.tag, created_at))
    conn.commit()
    conn.close()
    
    return {"id": task_id, "status": "created"}


@app.put("/tasks/{task_id}")
async def update_task(task_id: str, task: Task):
    conn = sqlite3.connect(str(DB_PATH))
    c = conn.cursor()
    c.execute("UPDATE tasks SET title=?, due=?, completed=?, priority=?, tag=? WHERE id=?",
              (task.title, task.due, int(task.completed), task.priority, task.tag, task_id))
    conn.commit()
    conn.close()
    return {"id": task_id, "status": "updated"}


@app.delete("/tasks/{task_id}")
async def delete_task(task_id: str):
    conn = sqlite3.connect(str(DB_PATH))
    c = conn.cursor()
    c.execute("DELETE FROM tasks WHERE id=?", (task_id,))
    conn.commit()
    conn.close()
    return {"id": task_id, "status": "deleted"}


@app.get("/stats")
async def get_stats():
    import psutil
    return {
        "cpu": psutil.cpu_percent(),
        "memory": psutil.virtual_memory().percent,
        "disk": psutil.disk_usage('/').percent,
        "network": 0
    }


@app.get("/models")
async def list_models():
    try:
        models = ollama.list()
        return {"models": [m["name"] for m in models["models"]]}
    except:
        return {"models": []}


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="127.0.0.1", port=8000)