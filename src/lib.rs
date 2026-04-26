//! # polygone
//!
//! Post-quantum ephemeral privacy network.
//!
//! ## The idea
//!
//! Classical cryptography hides **content**. It cannot hide that a
//! communication happened. POLYGONE hides the communication itself.
//!
//! A message becomes distributed computational state across 7 ephemeral
//! nodes. Any 4 can reconstruct it. No observer sees more than a fragment.
//! The network dissolves. Keys are zeroed. The exchange did not happen.
//!
//! ```text
//! Alice ──[ML-KEM-1024, one-shot, out-of-band]──► Bob
//!                        │
//!          Ephemeral topology derived from shared secret
//!                        │
//!    ┌───────────────────┴───────────────────┐
//!    │  Shamir 4-of-7 secret fragments       │
//!    │  AES-256-GCM encrypted payload        │
//!    │  BLAKE3 hash commitment              │
//!    └──────────────────────────────────────┘
//! ```

#![allow(missing_docs)]

pub mod crypto;
pub mod error;
pub mod keys;
pub mod network;
pub mod protocol;
pub mod tui;

pub use crypto::{SharedSecret, KeyPair};
pub use protocol::Session;
pub use error::{PolygoneError, PolyResult, Result};

// Re-export key P2P types for convenience
pub use network::{
    P2pNode, P2pConfig, NetworkEvent, 
    PolygoneRequest, PolygoneResponse, GossipMessage, Capability,
    NodeId, Topology, TopologyParams,
};

/// Crate version.
pub const VERSION: &str = "1.0.0";