//! Polygone Server Module
//!
//! Provides decentralized server capabilities for the Polygone network:
//! - Resource management (CPU, RAM, disk, bandwidth)
//! - Process isolation and sandboxing
//! - Service registry and marketplace
//! - Rate limiting and quotas
//! - Karma-based billing

pub mod container;
pub mod resource;
pub mod registry;
pub mod ratelimit;

pub use resource::{ResourceQuota, ResourceManager, TenantResources, UsageReport, SystemResources};
pub use ratelimit::{RateLimiter, RateLimitConfig, RateLimitDecision};
pub use registry::{ServiceRegistry, ServiceListing, ServiceCategory, ServiceOrder, OrderStatus};
pub use container::{ContainerManager, ContainerStatus, ServiceImage};

