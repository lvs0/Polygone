//! # polygone-core
//!
//! Post-quantum ephemeral network protocol.
//!
//! ## Core idea
//!
//! Classical encryption hides *content*. It cannot hide that a
//! communication happened. POLYGONE hides the communication itself by
//! turning a message into a distributed, transient computational state —
//! a wave, not a particle.
//!
//! ```text
//!  Alice ──[ML-KEM key exchange, one-shot, out-of-band]──► Bob
//!               │
//!               ▼
//!  Ephemeral topology derived from shared key material
//!               │
//!               ▼
//!  Message → distributed computation state (N nodes, T ms)
//!               │
//!               ▼
//!  Network dissolves. Key destroyed. No artifact remains.
//! ```
//!
//! ## Modules
//!
//! - [`crypto`]    — Key generation, KEM, signing, symmetric encryption,
//!                   Shamir fragmentation.
//! - [`network`]   — Ephemeral topology derivation, node lifecycle, DHT.
//! - [`protocol`]  — Session negotiation, message transit, dissolution.
//! - [`error`]     — Unified error type.

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]

pub mod crypto;
pub mod error;
pub mod network;
pub mod protocol;

// Re-export the most important types at crate root for ergonomics
pub use crypto::{
    KeyPair, SharedSecret,
    kem::{KemPublicKey, KemSecretKey, KemCiphertext},
    sign::{SignPublicKey, SignSecretKey, Signature},
    symmetric::EncryptedPayload,
    shamir::{Fragment, FragmentId},
    karma::WorkVoucher,
};
pub use error::PolygoneError;
pub use network::{
    EphemeralNode, NodeId, Topology,
    topology::TopologyParams,
};
pub use protocol::{Session, SessionId, TransitState};

// Re-export major dependencies for ecosystem synchronization
pub use libp2p;
pub use anyhow;
pub use base64;

/// Convenience alias for results throughout the crate.
pub type Result<T> = std::result::Result<T, PolygoneError>;

/// Library version constant, mirrored from `Cargo.toml`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
