//! DHT Index for Polygone-Drive
//!
//! Provides distributed file indexing:
//! - File name to ID mapping
//! - Owner to files mapping
//! - Keyword search indexing
//! - Content-addressable storage

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

/// File index entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileIndexEntry {
    pub file_id: String,
    pub name: String,
    pub owner: String,
    pub keywords: Vec<String>,
    pub size_bytes: u64,
    pub created_at: u64,
}

/// Search result
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub file_id: String,
    pub name: String,
    pub relevance: f64,
}

/// DHT Index for distributed file lookup
pub struct FileIndex {
    by_id: Arc<RwLock<HashMap<String, FileIndexEntry>>>,
    by_owner: Arc<RwLock<HashMap<String, HashSet<String>>>>,
    by_name: Arc<RwLock<HashMap<String, HashSet<String>>>>,
    by_keyword: Arc<RwLock<HashMap<String, HashSet<String>>>>,
}

impl FileIndex {
    pub fn new() -> Self {
        Self {
            by_id: Arc::new(RwLock::new(HashMap::new())),
            by_owner: Arc::new(RwLock::new(HashMap::new())),
            by_name: Arc::new(RwLock::new(HashMap::new())),
            by_keyword: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Index a file
    pub async fn index(&self, entry: FileIndexEntry) {
        let file_id = entry.file_id.clone();
        let owner = entry.owner.clone();
        let name = entry.name.clone();

        // Index by ID
        let mut by_id = self.by_id.write().await;
        by_id.insert(file_id.clone(), entry.clone());

        // Index by owner
        let mut by_owner = self.by_owner.write().await;
        by_owner.entry(owner).or_default().insert(file_id.clone());

        // Index by name
        let mut by_name = self.by_name.write().await;
        by_name.entry(name.to_lowercase()).or_default().insert(file_id.clone());

        // Index by keywords
        let mut by_keyword = self.by_keyword.write().await;
        for keyword in &entry.keywords {
            by_keyword
                .entry(keyword.to_lowercase())
                .or_default()
                .insert(file_id.clone());
        }
    }

    /// Remove a file from index
    pub async fn remove(&self, file_id: &str) {
        // Get entry first
        let entry = {
            let by_id = self.by_id.read().await;
            by_id.get(file_id).cloned()
        };

        if let Some(entry) = entry {
            let mut by_id = self.by_id.write().await;
            by_id.remove(file_id);

            // Remove from owner index
            let mut by_owner = self.by_owner.write().await;
            if let Some(files) = by_owner.get_mut(&entry.owner) {
                files.remove(file_id);
            }

            // Remove from name index
            let mut by_name = self.by_name.write().await;
            if let Some(files) = by_name.get_mut(&entry.name.to_lowercase()) {
                files.remove(file_id);
            }

            // Remove from keyword index
            let mut by_keyword = self.by_keyword.write().await;
            for keyword in &entry.keywords {
                if let Some(files) = by_keyword.get_mut(&keyword.to_lowercase()) {
                    files.remove(file_id);
                }
            }
        }
    }

    /// Get file by ID
    pub async fn get(&self, file_id: &str) -> Option<FileIndexEntry> {
        let by_id = self.by_id.read().await;
        by_id.get(file_id).cloned()
    }

    /// List files by owner
    pub async fn list_by_owner(&self, owner: &str) -> Vec<FileIndexEntry> {
        let file_ids = {
            let by_owner = self.by_owner.read().await;
            by_owner.get(owner).cloned().unwrap_or_default()
        };

        let by_id = self.by_id.read().await;
        file_ids
            .iter()
            .filter_map(|id| by_id.get(id).cloned())
            .collect()
    }

    /// Search by keyword
    pub async fn search(&self, query: &str) -> Vec<SearchResult> {
        let query_lower = query.to_lowercase();
        let keywords: Vec<&str> = query_lower.split_whitespace().collect();

        let mut scores: HashMap<String, f64> = HashMap::new();

        let by_keyword = self.by_keyword.read().await;
        let by_id = self.by_id.read().await;

        for keyword in &keywords {
            if let Some(file_ids) = by_keyword.get(*keyword) {
                for file_id in file_ids {
                    *scores.entry(file_id.clone()).or_insert(0.0) += 1.0;
                }
            }
        }

        scores
            .into_iter()
            .filter_map(|(file_id, score)| {
                by_id.get(&file_id).map(|entry| SearchResult {
                    file_id,
                    name: entry.name.clone(),
                    relevance: score,
                })
            })
            .collect()
    }

    /// Search by exact name
    pub async fn search_by_name(&self, name: &str) -> Vec<FileIndexEntry> {
        let file_ids = {
            let by_name = self.by_name.read().await;
            by_name.get(&name.to_lowercase()).cloned().unwrap_or_default()
        };

        let by_id = self.by_id.read().await;
        file_ids
            .iter()
            .filter_map(|id| by_id.get(id).cloned())
            .collect()
    }
}

impl Default for FileIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_indexing() {
        let index = FileIndex::new();

        let entry = FileIndexEntry {
            file_id: "file1".to_string(),
            name: "document.pdf".to_string(),
            owner: "alice".to_string(),
            keywords: vec!["document".to_string(), "pdf".to_string()],
            size_bytes: 1024,
            created_at: 0,
        };

        index.index(entry).await;

        assert!(index.get("file1").await.is_some());
        assert_eq!(index.list_by_owner("alice").await.len(), 1);
        assert_eq!(index.search_by_name("document.pdf").await.len(), 1);

        let results = index.search("document").await;
        assert!(!results.is_empty());

        index.remove("file1").await;
        assert!(index.get("file1").await.is_none());
    }
}
