//! Rate Limiting for Polygone Server
//!
//! Token bucket rate limiting per tenant:
//! - Request rate limiting
//! - Bandwidth quotas
//! - Burst allowance
//! - Cost-based limiting

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Token bucket for rate limiting
#[derive(Debug, Clone)]
struct TokenBucket {
    tokens: f64,
    max_tokens: f64,
    refill_rate: f64,
    last_refill: Instant,
}

impl TokenBucket {
    fn new(capacity: f64, refill_per_second: f64) -> Self {
        Self {
            tokens: capacity,
            max_tokens: capacity,
            refill_rate: refill_per_second,
            last_refill: Instant::now(),
        }
    }

    fn try_consume(&mut self, tokens: f64) -> bool {
        self.refill();

        if self.tokens >= tokens {
            self.tokens -= tokens;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let elapsed = self.last_refill.elapsed().as_secs_f64();
        let new_tokens = elapsed * self.refill_rate;
        self.tokens = (self.tokens + new_tokens).min(self.max_tokens);
        self.last_refill = Instant::now();
    }
}

/// Rate limit configuration per tenant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_second: f64,
    pub burst_size: u32,
    pub max_requests_per_hour: u64,
    pub max_bandwidth_mb_per_hour: u64,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_second: 10.0,
            burst_size: 20,
            max_requests_per_hour: 10000,
            max_bandwidth_mb_per_hour: 500,
        }
    }
}

/// Rate limit decision
#[derive(Debug, Clone, PartialEq)]
pub enum RateLimitDecision {
    Allowed,
    RateLimited,
    QuotaExceeded,
}

/// Per-tenant rate limiter state
#[derive(Clone)]
struct TenantRateLimit {
    requests: TokenBucket,
    bandwidth: TokenBucket,
    requests_this_hour: u64,
    bandwidth_this_hour: u64,
    hour_start: Instant,
}

impl TenantRateLimit {
    fn new(config: &RateLimitConfig) -> Self {
        Self {
            requests: TokenBucket::new(
                config.burst_size as f64,
                config.requests_per_second,
            ),
            bandwidth: TokenBucket::new(
                config.max_bandwidth_mb_per_hour as f64,
                config.max_bandwidth_mb_per_hour as f64 / 3600.0,
            ),
            requests_this_hour: 0,
            bandwidth_this_hour: 0,
            hour_start: Instant::now(),
        }
    }

    fn try_request(&mut self, bandwidth_mb: f64, config: &RateLimitConfig) -> RateLimitDecision {
        if self.hour_start.elapsed() > Duration::from_secs(3600) {
            self.requests_this_hour = 0;
            self.bandwidth_this_hour = 0;
            self.hour_start = Instant::now();
        }

        if self.requests_this_hour >= config.max_requests_per_hour {
            return RateLimitDecision::QuotaExceeded;
        }

        if self.bandwidth_this_hour + (bandwidth_mb as u64) > config.max_bandwidth_mb_per_hour {
            return RateLimitDecision::QuotaExceeded;
        }

        if !self.requests.try_consume(1.0) {
            return RateLimitDecision::RateLimited;
        }

        let _ = self.bandwidth.try_consume(bandwidth_mb);
        self.requests_this_hour += 1;
        self.bandwidth_this_hour += bandwidth_mb as u64;

        RateLimitDecision::Allowed
    }
}

/// Global rate limiter
pub struct RateLimiter {
    limits: Arc<RwLock<HashMap<String, TenantRateLimit>>>,
    configs: Arc<RwLock<HashMap<String, RateLimitConfig>>>,
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            limits: Arc::new(RwLock::new(HashMap::new())),
            configs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register(&self, tenant: &str, config: RateLimitConfig) {
        let mut configs = self.configs.write().await;
        configs.insert(tenant.to_string(), config.clone());

        let mut limits = self.limits.write().await;
        limits.insert(tenant.to_string(), TenantRateLimit::new(&config));
    }

    pub async fn unregister(&self, tenant: &str) {
        let mut configs = self.configs.write().await;
        let mut limits = self.limits.write().await;
        configs.remove(tenant);
        limits.remove(tenant);
    }

    pub async fn check(&self, tenant: &str, bandwidth_mb: f64) -> RateLimitDecision {
        let config = {
            let configs = self.configs.read().await;
            configs.get(tenant).cloned()
        };

        let mut state = {
            let limits = self.limits.read().await;
            limits.get(tenant).cloned()
        };

        match (config, state) {
            (Some(cfg), Some(ref mut st)) => {
                let decision = st.try_request(bandwidth_mb, &cfg);
                
                let mut limits = self.limits.write().await;
                if let Some(state) = limits.get_mut(tenant) {
                    state.requests_this_hour = st.requests_this_hour;
                    state.bandwidth_this_hour = st.bandwidth_this_hour;
                }
                
                decision
            }
            _ => RateLimitDecision::Allowed,
        }
    }

    pub async fn get_remaining(&self, tenant: &str) -> Option<(u64, u64)> {
        let limits = self.limits.read().await;
        let configs = self.configs.read().await;

        if let (Some(limit), Some(config)) = (limits.get(tenant), configs.get(tenant)) {
            let requests_left = config.max_requests_per_hour.saturating_sub(limit.requests_this_hour);
            let bandwidth_left = config.max_bandwidth_mb_per_hour.saturating_sub(limit.bandwidth_this_hour);
            Some((requests_left, bandwidth_left))
        } else {
            None
        }
    }

    pub async fn update_config(&self, tenant: &str, config: RateLimitConfig) -> Result<(), String> {
        let mut configs = self.configs.write().await;
        let mut limits = self.limits.write().await;

        if configs.contains_key(tenant) {
            configs.insert(tenant.to_string(), config.clone());
            limits.insert(tenant.to_string(), TenantRateLimit::new(&config));
            Ok(())
        } else {
            Err("Tenant not registered".to_string())
        }
    }

    pub async fn reset_all(&self) {
        let mut limits = self.limits.write().await;
        let configs = self.configs.read().await;

        for (tenant, config) in configs.iter() {
            limits.insert(tenant.clone(), TenantRateLimit::new(config));
        }
    }
}

pub async fn apply_rate_limit(
    limiter: &RateLimiter,
    tenant: &str,
    request_size_mb: f64,
) -> Result<(), String> {
    match limiter.check(tenant, request_size_mb).await {
        RateLimitDecision::Allowed => Ok(()),
        RateLimitDecision::RateLimited => {
            Err("Rate limit exceeded. Try again later.".to_string())
        }
        RateLimitDecision::QuotaExceeded => {
            Err("Hourly quota exceeded. Please try again in the next hour.".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiting() {
        let limiter = RateLimiter::new();
        limiter.register("test", RateLimitConfig::default()).await;

        assert_eq!(
            limiter.check("test", 0.001).await,
            RateLimitDecision::Allowed
        );

        for _ in 0..19 {
            assert_eq!(
                limiter.check("test", 0.001).await,
                RateLimitDecision::Allowed
            );
        }
    }
}
