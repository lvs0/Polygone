import http.server
import socketserver
import os
import threading
import time
import urllib.request
import json

PORT = int(os.environ.get("PORT", "8080"))
URL = os.environ.get("RENDER_URL")
STATUS_FILE = "/tmp/polygone_status.json"

HTML_TEMPLATE = """
<!DOCTYPE html>
<html lang="fr">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Polygone Node Dashboard</title>
    <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@300;400;600&family=JetBrains+Mono&display=swap" rel="stylesheet">
    <style>
        :root {
            --bg: #0a0a0c;
            --accent: #00f2ff;
            --accent-dim: rgba(0, 242, 255, 0.1);
            --text: #e0e0e6;
            --glass: rgba(255, 255, 255, 0.03);
        }
        body {
            background-color: var(--bg);
            color: var(--text);
            font-family: 'Outfit', sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            height: 100vh;
            margin: 0;
            overflow: hidden;
        }
        .container {
            background: var(--glass);
            backdrop-filter: blur(20px);
            border: 1px solid rgba(255, 255, 255, 0.1);
            border-radius: 24px;
            padding: 40px;
            width: 90%;
            max-width: 500px;
            box-shadow: 0 20px 50px rgba(0,0,0,0.5);
            text-align: center;
            position: relative;
        }
        .container::before {
            content: '⬡';
            position: absolute;
            top: -30px;
            left: 50%;
            transform: translateX(-50%);
            font-size: 60px;
            color: var(--accent);
            text-shadow: 0 0 20px var(--accent);
        }
        h1 { margin: 0; font-weight: 600; letter-spacing: 2px; color: var(--accent); }
        .peer-id {
            font-family: 'JetBrains Mono', monospace;
            background: #000;
            padding: 10px;
            border-radius: 8px;
            margin: 20px 0;
            font-size: 13px;
            word-break: break-all;
            color: #888;
        }
        .status-badge {
            display: inline-block;
            padding: 8px 16px;
            border-radius: 100px;
            background: var(--accent-dim);
            color: var(--accent);
            border: 1px solid var(--accent);
            font-weight: 600;
            font-size: 14px;
            animation: pulse 2s infinite;
        }
        .stats {
            margin-top: 30px;
            display: flex;
            justify-content: space-around;
            border-top: 1px solid rgba(255, 255, 255, 0.1);
            padding-top: 20px;
        }
        .stat-item span { display: block; font-size: 12px; color: #666; text-transform: uppercase; }
        .stat-item b { font-size: 18px; color: #fff; }
        @keyframes pulse {
            0% { box-shadow: 0 0 0 0 rgba(0, 242, 255, 0.4); }
            70% { box-shadow: 0 0 0 10px rgba(0, 242, 255, 0); }
            100% { box-shadow: 0 0 0 0 rgba(0, 242, 255, 0); }
        }
    </style>
    <script>
        setTimeout(() => location.reload(), 30000);
    </script>
</head>
<body>
    <div class="container">
        <h1>POLYGONE</h1>
        <p style="opacity: 0.6; font-weight: 300;">L'information n'existe pas. Elle traverse.</p>
        
        <div class="status-badge">NŒUD ACTIF</div>
        
        <div class="peer-id">{{PEER_ID}}</div>
        
        <div class="stats">
            <div class="stat-item">
                <span>Uptime</span>
                <b>{{UPTIME}}</b>
            </div>
            <div class="stat-item">
                <span>Réseau</span>
                <b>P2P / DHT</b>
            </div>
        </div>
        
        <p style="margin-top: 30px; font-size: 10px; color: #444; font-family: 'JetBrains Mono';">
            IP: {{VERSION}} | MODE: PERSISTENT
        </p>
    </div>
</body>
</html>
"""

class Handler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header("Content-type", "text/html; charset=utf-8")
        self.end_headers()
        
        status_data = {"peer_id": "Initializing...", "uptime_secs": 0}
        if os.path.exists(STATUS_FILE):
            try:
                with open(STATUS_FILE, "r") as f:
                    status_data = json.load(f)
            except:
                pass

        uptime_min = status_data.get("uptime_secs", 0) // 60
        uptime_str = f"{uptime_min}m" if uptime_min > 0 else f"{status_data.get('uptime_secs', 0)}s"

        content = HTML_TEMPLATE.replace("{{PEER_ID}}", status_data.get("peer_id"))
        content = content.replace("{{UPTIME}}", uptime_str)
        content = content.replace("{{VERSION}}", "v0.1.0-ghcr")
        
        self.wfile.write(content.encode("utf-8"))

def pinger():
    if not URL:
        return
    while True:
        try:
            time.sleep(600) # 10 minutes
            urllib.request.urlopen(URL).read()
        except:
            pass

if __name__ == "__main__":
    threading.Thread(target=pinger, daemon=True).start()
    with socketserver.TCPServer(("", PORT), Handler) as httpd:
        httpd.serve_forever()
