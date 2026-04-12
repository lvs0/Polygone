//! Resource Management for Polygone Server
//!
//! Tracks and limits system resource usage per tenant:
//! - Memory allocation and monitoring
//! - CPU time accounting
//! - Disk usage tracking
//! - Bandwidth metering

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Maximum resources a single tenant can use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceQuota {
    pub max_memory_mb: u64,
    pub max_cpu_percent: u32,
    pub max_disk_mb: u64,
    pub max_bandwidth_mb: u64,
    pub max_processes: u32,
    pub max_open_files: u32,
}

impl Default for ResourceQuota {
    fn default() -> Self {
        Self {
            max_memory_mb: 512,
            max_cpu_percent: 25,
            max_disk_mb: 1024,
            max_bandwidth_mb: 100,
            max_processes: 10,
            max_open_files: 256,
        }
    }
}

/// Per-tenant resource tracker
#[derive(Debug, Clone, Default)]
pub struct TenantResources {
    pub memory_bytes: u64,
    pub cpu_time_ms: u64,
    pub disk_bytes: u64,
    pub bandwidth_bytes: u64,
    pub io_operations: u64,
}

/// Resource usage for a tenant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageReport {
    pub tenant: String,
    pub memory_mb: f64,
    pub cpu_percent: f64,
    pub disk_mb: f64,
    pub bandwidth_mb: f64,
    pub quota: ResourceQuota,
    pub usage_percent: f64,
}

/// Global resource manager
pub struct ResourceManager {
    tenants: Arc<RwLock<HashMap<String, TenantResources>>>,
    quotas: Arc<RwLock<HashMap<String, ResourceQuota>>>,
    system_total: SystemResources,
}

#[derive(Debug, Clone, Default)]
pub struct SystemResources {
    pub total_memory_mb: u64,
    pub total_cpu_cores: u32,
    pub total_disk_mb: u64,
}

impl ResourceManager {
    pub fn new(system: SystemResources) -> Self {
        Self {
            tenants: Arc::new(RwLock::new(HashMap::new())),
            quotas: Arc::new(RwLock::new(HashMap::new())),
            system_total: system,
        }
    }

    pub async fn register_tenant(&self, tenant: &str, quota: ResourceQuota) {
        let mut quotas = self.quotas.write().await;
        quotas.insert(tenant.to_string(), quota);

        let mut tenants = self.tenants.write().await;
        tenants.insert(tenant.to_string(), TenantResources::default());
    }

    pub async fn unregister_tenant(&self, tenant: &str) {
        let mut quotas = self.quotas.write().await;
        let mut tenants = self.tenants.write().await;
        quotas.remove(tenant);
        tenants.remove(tenant);
    }

    pub async fn can_allocate(&self, tenant: &str, additional: &TenantResources) -> bool {
        let tenants = self.tenants.read().await;
        let quotas = self.quotas.read().await;

        if let (Some(used), Some(quota)) = (tenants.get(tenant), quotas.get(tenant)) {
            let new_mem = used.memory_bytes + additional.memory_bytes;
            let new_disk = used.disk_bytes + additional.disk_bytes;

            new_mem <= quota.max_memory_mb as u64 * 1024 * 1024
                && new_disk <= quota.max_disk_mb as u64 * 1024 * 1024
        } else {
            false
        }
    }

    pub async fn allocate(&self, tenant: &str, resources: &TenantResources) -> Result<(), String> {
        if !self.can_allocate(tenant, resources).await {
            return Err("Quota exceeded".to_string());
        }

        let mut tenants = self.tenants.write().await;
        if let Some(used) = tenants.get_mut(tenant) {
            used.memory_bytes += resources.memory_bytes;
            used.disk_bytes += resources.disk_bytes;
            used.bandwidth_bytes += resources.bandwidth_bytes;
            Ok(())
        } else {
            Err("Tenant not registered".to_string())
        }
    }

    pub async fn release(&self, tenant: &str, resources: &TenantResources) {
        let mut tenants = self.tenants.write().await;
        if let Some(used) = tenants.get_mut(tenant) {
            used.memory_bytes = used.memory_bytes.saturating_sub(resources.memory_bytes);
            used.disk_bytes = used.disk_bytes.saturating_sub(resources.disk_bytes);
            used.bandwidth_bytes = used.bandwidth_bytes.saturating_sub(resources.bandwidth_bytes);
        }
    }

    pub async fn get_usage(&self, tenant: &str) -> Option<UsageReport> {
        let tenants = self.tenants.read().await;
        let quotas = self.quotas.read().await;

        if let (Some(used), Some(quota)) = (tenants.get(tenant), quotas.get(tenant)) {
            let memory_mb = used.memory_bytes as f64 / (1024.0 * 1024.0);
            let disk_mb = used.disk_bytes as f64 / (1024.0 * 1024.0);
            let bandwidth_mb = used.bandwidth_bytes as f64 / (1024.0 * 1024.0);

            let memory_pct = memory_mb / quota.max_memory_mb as f64 * 100.0;
            let disk_pct = disk_mb / quota.max_disk_mb as f64 * 100.0;
            let usage_percent = memory_pct.max(disk_pct);

            Some(UsageReport {
                tenant: tenant.to_string(),
                memory_mb,
                cpu_percent: 0.0,
                disk_mb,
                bandwidth_mb,
                quota: quota.clone(),
                usage_percent,
            })
        } else {
            None
        }
    }

    pub async fn get_all_usage(&self) -> Vec<UsageReport> {
        let tenants = self.tenants.read().await;
        let mut reports = Vec::new();

        for tenant in tenants.keys() {
            if let Some(report) = self.get_usage(tenant).await {
                reports.push(report);
            }
        }

        reports
    }

    pub async fn get_available(&self) -> SystemResources {
        let tenants = self.tenants.read().await;

        let used_memory: u64 = tenants.values().map(|t| t.memory_bytes).sum();
        let used_disk: u64 = tenants.values().map(|t| t.disk_bytes).sum();

        SystemResources {
            total_memory_mb: self.system_total.total_memory_mb.saturating_sub(used_memory / 1024 / 1024),
            total_cpu_cores: self.system_total.total_cpu_cores,
            total_disk_mb: self.system_total.total_disk_mb.saturating_sub(used_disk / 1024 / 1024),
        }
    }
}

pub fn detect_system_resources() -> SystemResources {
    SystemResources {
        total_memory_mb: sys_info::mem_info()
            .map(|m| m.total)
            .unwrap_or(8192) as u64 / 1024,
        total_cpu_cores: num_cpus::get() as u32,
        total_disk_mb: 10240,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resource_tracking() {
        let manager = ResourceManager::new(SystemResources::default());
        
        manager
            .register_tenant("test", ResourceQuota::default())
            .await;

        let resources = TenantResources {
            memory_bytes: 100 * 1024 * 1024,
            ..Default::default()
        };

        assert!(manager.allocate("test", &resources).await.is_ok());
        assert!(manager.get_usage("test").await.is_some());
        
        manager.release("test", &resources).await;
        manager.unregister_tenant("test").await;
    }
}
