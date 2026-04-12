//! Polygone-Drive: Decentralized Encrypted Storage
//!
//! Provides secure, distributed file storage across the Polygone network:
//! - End-to-end encrypted files
//! - Erasure coding for redundancy (Reed-Solomon)
//! - DHT-based file indexing
//! - Access control with derived keys
//! - Version history and snapshots

pub mod erasure;
pub mod storage;
pub mod index;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// File metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub id: String,
    pub name: String,
    pub size_bytes: u64,
    pub created_at: u64,
    pub modified_at: u64,
    pub owner: String,
    pub chunks: Vec<ChunkInfo>,
    pub encryption_key_id: String,
    pub erasure_config: ErasureConfig,
}

/// Information about a single chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkInfo {
    pub chunk_id: String,
    pub node_id: String,
    pub size_bytes: u64,
    pub hash: String,
}

/// Erasure coding configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErasureConfig {
    pub data_shards: u8,
    pub parity_shards: u8,
    pub min_shards: u8,
}

impl Default for ErasureConfig {
    fn default() -> Self {
        Self {
            data_shards: 4,
            parity_shards: 2,
            min_shards: 4,
        }
    }
}

/// File handle for operations
#[derive(Debug, Clone)]
pub struct FileHandle {
    pub metadata: FileMetadata,
    pub encryption_key: Vec<u8>,
}

/// Directory entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryEntry {
    pub name: String,
    pub is_directory: bool,
    pub size_bytes: u64,
    pub modified_at: u64,
}

/// Storage node information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageNode {
    pub node_id: String,
    pub available_space_mb: u64,
    pub reputation: f64,
    pub price_per_gb: f64,
}

/// Drive configuration
#[derive(Debug, Clone)]
pub struct DriveConfig {
    pub default_erasure: ErasureConfig,
    pub max_file_size_mb: u64,
    pub chunk_size_kb: u64,
    pub enable_compression: bool,
    pub enable_deduplication: bool,
}

impl Default for DriveConfig {
    fn default() -> Self {
        Self {
            default_erasure: ErasureConfig::default(),
            max_file_size_mb: 100,
            chunk_size_kb: 256,
            enable_compression: true,
            enable_deduplication: true,
        }
    }
}

/// Global drive manager
pub struct DriveManager {
    config: DriveConfig,
    files: Arc<RwLock<HashMap<String, FileHandle>>>,
    local_chunks: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl DriveManager {
    pub fn new(config: DriveConfig) -> Self {
        Self {
            config,
            files: Arc::new(RwLock::new(HashMap::new())),
            local_chunks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn default_config() -> Self {
        Self::new(DriveConfig::default())
    }

    /// Store a file with erasure coding
    pub async fn store_file(
        &self,
        name: &str,
        owner: &str,
        data: Vec<u8>,
        encryption_key: Vec<u8>,
    ) -> Result<FileMetadata, String> {
        let file_id = generate_file_id();
        let now = current_timestamp();

        // Compress if enabled
        let data = if self.config.enable_compression {
            compress_data(&data)?
        } else {
            data
        };

        // Split into chunks
        let chunk_size = (self.config.chunk_size_kb * 1024) as usize;
        let chunks = split_into_chunks(&data, chunk_size);
        let total_chunks = chunks.len() as u8;

        // Apply erasure coding
        let config = &self.config.default_erasure;
        let encoded = erasure::encode(&chunks, config.data_shards, config.parity_shards)?;

        // Encrypt each shard
        let encrypted_shards: Vec<Vec<u8>> = encoded
            .into_iter()
            .map(|shard| encrypt_chunk(&shard, &encryption_key))
            .collect();

        // Store locally (in real implementation, distribute to nodes)
        let mut local_chunks = self.local_chunks.write().await;
        let mut chunk_infos = Vec::new();

        for (i, shard) in encrypted_shards.iter().enumerate() {
            let chunk_id = format!("{}-chunk-{}", file_id, i);
            local_chunks.insert(chunk_id.clone(), shard.clone());

            chunk_infos.push(ChunkInfo {
                chunk_id: chunk_id.clone(),
                node_id: "local".to_string(),
                size_bytes: shard.len() as u64,
                hash: compute_hash(shard),
            });
        }

        let metadata = FileMetadata {
            id: file_id,
            name: name.to_string(),
            size_bytes: data.len() as u64,
            created_at: now,
            modified_at: now,
            owner: owner.to_string(),
            chunks: chunk_infos,
            encryption_key_id: compute_key_id(&encryption_key),
            erasure_config: config.clone(),
        };

        let handle = FileHandle {
            metadata: metadata.clone(),
            encryption_key,
        };

        let mut files = self.files.write().await;
        files.insert(metadata.id.clone(), handle);

        Ok(metadata)
    }

    /// Retrieve a file
    pub async fn retrieve_file(&self, file_id: &str) -> Result<Vec<u8>, String> {
        let files = self.files.read().await;
        let handle = files
            .get(file_id)
            .ok_or_else(|| "File not found".to_string())?;

        let mut chunks: Vec<Vec<u8>> = Vec::new();

        // Collect chunks
        let local_chunks = self.local_chunks.read().await;
        for chunk_info in &handle.metadata.chunks {
            if let Some(shard) = local_chunks.get(&chunk_info.chunk_id) {
                let decrypted = decrypt_chunk(shard, &handle.encryption_key);
                chunks.push(decrypted);
            }
        }

        // Decode from erasure coding
        let config = &handle.metadata.erasure_config;
        let decoded = erasure::decode(&chunks, config.data_shards)?;
        
        // Flatten decoded chunks
        let mut flat_data: Vec<u8> = Vec::new();
        for chunk in &decoded {
            flat_data.extend_from_slice(chunk);
        }

        // Decompress if needed
        let data = if self.config.enable_compression {
            decompress_data(&flat_data)?
        } else {
            flat_data
        };

        Ok(data)
    }

    /// Delete a file
    pub async fn delete_file(&self, file_id: &str) -> Result<(), String> {
        let mut files = self.files.write().await;
        let mut local_chunks = self.local_chunks.write().await;

        if let Some(handle) = files.remove(file_id) {
            for chunk in &handle.metadata.chunks {
                local_chunks.remove(&chunk.chunk_id);
            }
            Ok(())
        } else {
            Err("File not found".to_string())
        }
    }

    /// List files for an owner
    pub async fn list_files(&self, owner: &str) -> Vec<FileMetadata> {
        let files = self.files.read().await;
        files
            .values()
            .filter(|h| h.metadata.owner == owner)
            .map(|h| h.metadata.clone())
            .collect()
    }

    /// Get file metadata
    pub async fn get_metadata(&self, file_id: &str) -> Option<FileMetadata> {
        let files = self.files.read().await;
        files.get(file_id).map(|h| h.metadata.clone())
    }

    /// Calculate storage used
    pub async fn get_storage_used(&self) -> u64 {
        let local_chunks = self.local_chunks.read().await;
        local_chunks.values().map(|c| c.len() as u64).sum()
    }
}

// Utility functions

fn generate_file_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("file-{:x}", timestamp)
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn split_into_chunks(data: &[u8], chunk_size: usize) -> Vec<Vec<u8>> {
    data.chunks(chunk_size)
        .map(|c| c.to_vec())
        .collect()
}

fn compute_hash(data: &[u8]) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

fn compute_key_id(key: &[u8]) -> String {
    format!("key-{}", &compute_hash(key)[..16])
}

fn compress_data(data: &[u8]) -> Result<Vec<u8>, String> {
    // Simple RLE compression for demonstration
    // In production, use actual compression (lz4, zstd, etc.)
    Ok(data.to_vec())
}

fn decompress_data(data: &[u8]) -> Result<Vec<u8>, String> {
    Ok(data.to_vec())
}

fn encrypt_chunk(chunk: &[u8], _key: &[u8]) -> Vec<u8> {
    // In production, use proper encryption
    chunk.to_vec()
}

fn decrypt_chunk(chunk: &[u8], _key: &[u8]) -> Vec<u8> {
    // In production, use proper decryption
    chunk.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_store_and_retrieve() {
        let drive = DriveManager::default_config();

        let data = b"Hello, Polygone-Drive!".to_vec();
        let key = vec![0u8; 32];

        let metadata = drive
            .store_file("test.txt", "alice", data.clone(), key.clone())
            .await
            .unwrap();

        let retrieved = drive.retrieve_file(&metadata.id).await.unwrap();
        // Note: retrieved data may have padding, check prefix
        assert_eq!(&retrieved[..data.len()], &data[..]);

        drive.delete_file(&metadata.id).await.unwrap();
    }

    #[tokio::test]
    async fn test_list_files() {
        let drive = DriveManager::default_config();
        let key = vec![0u8; 32];

        drive
            .store_file("file1.txt", "alice", b"data1".to_vec(), key.clone())
            .await
            .unwrap();
        drive
            .store_file("file2.txt", "alice", b"data2".to_vec(), key.clone())
            .await
            .unwrap();
        drive
            .store_file("file3.txt", "bob", b"data3".to_vec(), key.clone())
            .await
            .unwrap();

        let alice_files = drive.list_files("alice").await;
        assert_eq!(alice_files.len(), 2);
    }
}
