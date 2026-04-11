pub mod session;
pub use session::Session;

use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId([u8; 16]);

impl SessionId {
    pub fn from_shared_secret(ss: &crate::crypto::SharedSecret) -> Self {
        let hash = blake3::derive_key("polygone session id v1", &ss.0);
        let mut id = [0u8; 16];
        id.copy_from_slice(&hash[..16]);
        Self(id)
    }
    
    pub fn generate() -> Result<Self, crate::PolygoneError> {
        let mut id = [0u8; 16];
        getrandom::getrandom(&mut id).map_err(|e| crate::PolygoneError::RngError(e.to_string()))?;
        Ok(Self(id))
    }
    pub fn as_bytes(&self) -> &[u8; 16] { &self.0 }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransitState {
    Pending,
    Established,
    InTransit { dispatched_at: Instant },
    Completed,
    Dissolved,
}

impl std::fmt::Display for TransitState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending       => write!(f, "PENDING"),
            Self::Established   => write!(f, "ESTABLISHED"),
            Self::InTransit{..} => write!(f, "IN_TRANSIT"),
            Self::Completed     => write!(f, "COMPLETED"),
            Self::Dissolved     => write!(f, "DISSOLVED"),
        }
    }
}
