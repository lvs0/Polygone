//! Storage layer for Polygone-Drive
//!
//! Manages chunk storage across nodes:
//! - Local storage management
//! - Remote storage coordination
//! - Chunk replication
//! - Garbage collection

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A stored chunk with metadata
#[derive(Debug, Clone)]
pub struct StoredChunk {
    pub chunk_id: String,
    pub data: Vec<u8>,
    pub node_id: String,
    pub size_bytes: u64,
    pub created_at: u64,
    pub access_count: u64,
}

/// Local storage manager
pub struct LocalStorage {
    chunks: Arc<RwLock<HashMap<String, StoredChunk>>>,
    max_size_mb: u64,
}

impl LocalStorage {
    pub fn new(max_size_mb: u64) -> Self {
        Self {
            chunks: Arc::new(RwLock::new(HashMap::new())),
            max_size_mb,
        }
    }

    /// Store a chunk
    pub async fn store(&self, chunk_id: &str, data: Vec<u8>, node_id: &str) -> Result<(), String> {
        let size = data.len() as u64;
        let max_bytes = self.max_size_mb * 1024 * 1024;

        let current_size = self.get_total_size().await;
        if current_size + size > max_bytes {
            return Err("Storage quota exceeded".to_string());
        }

        let chunk = StoredChunk {
            chunk_id: chunk_id.to_string(),
            data,
            node_id: node_id.to_string(),
            size_bytes: size,
            created_at: current_timestamp(),
            access_count: 0,
        };

        let mut chunks = self.chunks.write().await;
        chunks.insert(chunk_id.to_string(), chunk);

        Ok(())
    }

    /// Retrieve a chunk
    pub async fn retrieve(&self, chunk_id: &str) -> Option<Vec<u8>> {
        let mut chunks = self.chunks.write().await;
        
        if let Some(chunk) = chunks.get_mut(chunk_id) {
            chunk.access_count += 1;
            return Some(chunk.data.clone());
        }
        
        None
    }

    /// Delete a chunk
    pub async fn delete(&self, chunk_id: &str) -> Option<Vec<u8>> {
        let mut chunks = self.chunks.write().await;
        chunks.remove(chunk_id).map(|c| c.data)
    }

    /// Get total stored size
    pub async fn get_total_size(&self) -> u64 {
        let chunks = self.chunks.read().await;
        chunks.values().map(|c| c.size_bytes).sum()
    }

    /// List all chunks
    pub async fn list_chunks(&self) -> Vec<String> {
        let chunks = self.chunks.read().await;
        chunks.keys().cloned().collect()
    }

    /// Garbage collect old chunks
    pub async fn gc(&self, max_age_secs: u64) -> usize {
        let now = current_timestamp();
        let mut chunks = self.chunks.write().await;
        let mut removed = 0;

        chunks.retain(|_, chunk| {
            let age = now - chunk.created_at;
            if age > max_age_secs && chunk.access_count == 0 {
                removed += 1;
                false
            } else {
                true
            }
        });

        removed
    }
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_storage() {
        let storage = LocalStorage::new(100);

        storage
            .store("chunk1", vec![1, 2, 3], "node1")
            .await
            .unwrap();

        assert_eq!(storage.retrieve("chunk1").await, Some(vec![1, 2, 3]));
        assert_eq!(storage.get_total_size().await, 3);

        storage.delete("chunk1").await;
        assert_eq!(storage.retrieve("chunk1").await, None);
    }
}
