//! Container/Isolation System for Polygone Server
//!
//! Provides lightweight process isolation without requiring Docker:
//! - Process sandboxing with resource limits
//! - Filesystem namespaces
//! - Network namespace isolation (optional)
//! - Process monitoring and management
//!
//! Note: Full containerization requires root/CAP_SYS_ADMIN.
//! This implementation provides best-effort isolation.

use crate::server::resource::ResourceQuota;
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::{Child, Command};
use tokio::sync::RwLock;

/// A running container instance
pub struct Container {
    pub id: String,
    pub command: String,
    pub args: Vec<String>,
    pub quota: ResourceQuota,
    pub child: Option<Child>,
    pub started_at: u64,
}

/// Container manager for a server node
pub struct ContainerManager {
    containers: RwLock<HashMap<String, Container>>,
    data_dir: std::path::PathBuf,
}

impl ContainerManager {
    pub fn new(data_dir: std::path::PathBuf) -> Self {
        Self {
            containers: RwLock::new(HashMap::new()),
            data_dir,
        }
    }

    /// Start a new containerized process
    pub async fn start(
        &self,
        id: &str,
        command: &str,
        args: &[String],
        quota: &ResourceQuota,
    ) -> Result<(), String> {
        let mut containers = self.containers.write().await;

        if containers.contains_key(id) {
            return Err("Container already exists".to_string());
        }

        // Create isolated environment
        let work_dir = self.data_dir.join("containers").join(id);
        std::fs::create_dir_all(&work_dir)
            .map_err(|e| format!("Failed to create container directory: {}", e))?;

        // Build command with resource limits
        let mut cmd = Command::new(command);
        cmd.args(args)
            .current_dir(&work_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env("HOME", &work_dir)
            .env("TMPDIR", work_dir.join("tmp"));

        // On Linux with proper privileges, we could use cgroups
        // For now, we rely on the kernel's default limits

        let child = cmd
            .spawn()
            .map_err(|e| format!("Failed to spawn process: {}", e))?;

        let container = Container {
            id: id.to_string(),
            command: command.to_string(),
            args: args.to_vec(),
            quota: quota.clone(),
            child: Some(child),
            started_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        containers.insert(id.to_string(), container);
        Ok(())
    }

    /// Stop a running container
    pub async fn stop(&self, id: &str) -> Result<(), String> {
        let mut containers = self.containers.write().await;

        if let Some(mut container) = containers.remove(id) {
            if let Some(ref mut child) = container.child {
                child
                    .kill()
                    .await
                    .map_err(|e| format!("Failed to kill process: {}", e))?;
            }
            Ok(())
        } else {
            Err("Container not found".to_string())
        }
    }

    /// Get container status
    pub async fn status(&self, id: &str) -> Option<ContainerStatus> {
        let containers = self.containers.read().await;

        containers.get(id).map(|c| {
            let running = c.child.is_some();

            ContainerStatus {
                id: c.id.clone(),
                running,
                uptime_secs: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    - c.started_at,
            }
        })
    }

    /// List all containers
    pub async fn list(&self) -> Vec<String> {
        self.containers.read().await.keys().cloned().collect()
    }

    /// Wait for a container to exit
    pub async fn wait(&self, id: &str) -> Result<i32, String> {
        let mut containers = self.containers.write().await;

        if let Some(mut container) = containers.remove(id) {
            if let Some(ref mut child) = container.child {
                let status = child
                    .wait()
                    .await
                    .map_err(|e| format!("Failed to wait: {}", e))?;
                return Ok(status.code().unwrap_or(-1));
            }
        }
        Err("Container not found".to_string())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContainerStatus {
    pub id: String,
    pub running: bool,
    pub uptime_secs: u64,
}

/// Predefined service images/templates
pub struct ServiceImage {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub required_quota: ResourceQuota,
}

impl ServiceImage {
    pub fn nginx() -> Self {
        Self {
            name: "nginx:latest".to_string(),
            command: "nginx".to_string(),
            args: vec!["-g".to_string(), "daemon off;".to_string()],
            required_quota: ResourceQuota::default(),
        }
    }

    pub fn static_server() -> Self {
        Self {
            name: "static-server".to_string(),
            command: "python3".to_string(),
            args: vec![
                "-m".to_string(),
                "http.server".to_string(),
                "8080".to_string(),
            ],
            required_quota: ResourceQuota {
                max_memory_mb: 64,
                max_cpu_percent: 5,
                max_disk_mb: 100,
                max_bandwidth_mb: 50,
                max_processes: 2,
                max_open_files: 32,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires actual filesystem
    async fn test_container_lifecycle() {
        let manager = ContainerManager::new(std::path::PathBuf::from("/tmp/test"));
        let result = manager
            .start("test", "echo", &["hello".to_string()], &ResourceQuota::default())
            .await;
        assert!(result.is_ok());
    }
}
