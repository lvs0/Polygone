//! Service Registry for Polygone Network
//!
//! Enables service discovery and marketplace:
//! - Register services offered by nodes
//! - Discover available services
//! - Service ratings and reputation
//! - Marketplace for server resources

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A registered service offering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceListing {
    pub id: String,
    pub node_id: String,
    pub name: String,
    pub description: String,
    pub category: ServiceCategory,
    pub price_per_hour: f64,       // Karma per hour
    pub min_quality: u8,           // 1-5 stars minimum
    pub capabilities: Vec<String>,
    pub region: Option<String>,
    pub available: bool,
    pub rating: f64,
    pub total_reviews: u32,
}

/// Service categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ServiceCategory {
    Compute,
    Storage,
    Network,
    AIInference,
    Database,
    Custom(String),
}

impl ServiceCategory {
    pub fn as_str(&self) -> &str {
        match self {
            ServiceCategory::Compute => "compute",
            ServiceCategory::Storage => "storage",
            ServiceCategory::Network => "network",
            ServiceCategory::AIInference => "ai_inference",
            ServiceCategory::Database => "database",
            ServiceCategory::Custom(s) => s,
        }
    }
}

/// A service order/request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceOrder {
    pub id: String,
    pub listing_id: String,
    pub buyer: String,
    pub seller: String,
    pub duration_hours: u32,
    pub total_cost: f64,
    pub status: OrderStatus,
    pub created_at: u64,
}

/// Order lifecycle
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrderStatus {
    Pending,
    Accepted,
    Active,
    Completed,
    Cancelled,
    Disputed,
}

/// Service registry state
pub struct ServiceRegistry {
    listings: Arc<RwLock<HashMap<String, ServiceListing>>>,
    orders: Arc<RwLock<HashMap<String, ServiceOrder>>>,
    node_services: Arc<RwLock<HashMap<String, Vec<String>>>>, // node_id -> listing_ids
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            listings: Arc::new(RwLock::new(HashMap::new())),
            orders: Arc::new(RwLock::new(HashMap::new())),
            node_services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a new service listing
    pub async fn register(&self, listing: ServiceListing) -> Result<(), String> {
        let mut listings = self.listings.write().await;
        let mut node_services = self.node_services.write().await;

        if listings.contains_key(&listing.id) {
            return Err("Listing already exists".to_string());
        }

        listings.insert(listing.id.clone(), listing.clone());
        node_services
            .entry(listing.node_id.clone())
            .or_default()
            .push(listing.id);

        Ok(())
    }

    /// Unregister a service listing
    pub async fn unregister(&self, listing_id: &str) -> Result<(), String> {
        let mut listings = self.listings.write().await;

        if let Some(listing) = listings.remove(listing_id) {
            let mut node_services = self.node_services.write().await;
            if let Some(ids) = node_services.get_mut(&listing.node_id) {
                ids.retain(|id| id != listing_id);
            }
            Ok(())
        } else {
            Err("Listing not found".to_string())
        }
    }

    /// Get a specific listing
    pub async fn get(&self, listing_id: &str) -> Option<ServiceListing> {
        self.listings.read().await.get(listing_id).cloned()
    }

    /// Search listings by category
    pub async fn search(
        &self,
        category: Option<ServiceCategory>,
        min_rating: Option<u8>,
        max_price: Option<f64>,
        available_only: bool,
    ) -> Vec<ServiceListing> {
        let listings = self.listings.read().await;

        listings
            .values()
            .filter(|l| {
                if available_only && !l.available {
                    return false;
                }
                if let Some(cat) = &category {
                    if &l.category != cat {
                        return false;
                    }
                }
                if let Some(rating) = min_rating {
                    if ((l.rating * 5.0) as u8) < rating {
                        return false;
                    }
                }
                if let Some(price) = max_price {
                    if l.price_per_hour > price {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect()
    }

    /// Create a service order
    pub async fn create_order(&self, order: ServiceOrder) -> Result<(), String> {
        let mut orders = self.orders.write().await;

        if orders.contains_key(&order.id) {
            return Err("Order already exists".to_string());
        }

        // Verify listing exists and is available
        let listings = self.listings.read().await;
        if let Some(listing) = listings.get(&order.listing_id) {
            if !listing.available {
                return Err("Service is not available".to_string());
            }
        } else {
            return Err("Listing not found".to_string());
        }

        orders.insert(order.id.clone(), order);
        Ok(())
    }

    /// Update order status
    pub async fn update_order_status(
        &self,
        order_id: &str,
        status: OrderStatus,
    ) -> Result<(), String> {
        let mut orders = self.orders.write().await;

        if let Some(order) = orders.get_mut(order_id) {
            order.status = status;
            Ok(())
        } else {
            Err("Order not found".to_string())
        }
    }

    /// Get orders for a node
    pub async fn get_node_orders(&self, node_id: &str) -> Vec<ServiceOrder> {
        self.orders
            .read()
            .await
            .values()
            .filter(|o| o.seller == node_id)
            .cloned()
            .collect()
    }

    /// Get listings by node
    pub async fn get_node_listings(&self, node_id: &str) -> Vec<ServiceListing> {
        let listings = self.listings.read().await;
        listings
            .values()
            .filter(|l| l.node_id == node_id)
            .cloned()
            .collect()
    }

    /// Update service rating
    pub async fn update_rating(
        &self,
        listing_id: &str,
        new_rating: f64,
        review_count: u32,
    ) -> Result<(), String> {
        let mut listings = self.listings.write().await;

        if let Some(listing) = listings.get_mut(listing_id) {
            // Weighted average
            let total = listing.total_reviews as f64 + review_count as f64;
            listing.rating = (listing.rating * listing.total_reviews as f64 + new_rating * review_count as f64) / total;
            listing.total_reviews += review_count;
            Ok(())
        } else {
            Err("Listing not found".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_service_registration() {
        let registry = ServiceRegistry::new();

        let listing = ServiceListing {
            id: "test-1".to_string(),
            node_id: "node-1".to_string(),
            name: "Test Service".to_string(),
            description: "A test service".to_string(),
            category: ServiceCategory::Compute,
            price_per_hour: 0.1,
            min_quality: 3,
            capabilities: vec!["cpu".to_string()],
            region: None,
            available: true,
            rating: 4.5,
            total_reviews: 10,
        };

        assert!(registry.register(listing).await.is_ok());
        assert!(registry.get("test-1").await.is_some());
        assert_eq!(registry.search(None, None, None, true).await.len(), 1);
    }
}
