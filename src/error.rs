//! Unified error type for POLYGONE.

use thiserror::Error;
use libp2p::{gossipsub, kad, swarm::ConnectionDenied};

pub type Result<T = ()> = std::result::Result<T, PolygoneError>;
pub type PolyResult<T = ()> = std::result::Result<T, PolygoneError>;

#[derive(Debug, Error)]
pub enum PolygoneError {
    // ── Crypto ────────────────────────────────────────────────────────────────
    #[error("KEM key generation failed")]
    KemKeyGen,

    #[error("KEM encapsulation failed")]
    KemEncapsulate,

    #[error("KEM decapsulation failed: ciphertext may be tampered or key mismatch")]
    KemDecapsulate,

    #[error("AEAD error: {0}")]
    AeadError(String),

    #[error("Shamir split failed: {0}")]
    ShamirSplit(String),

    #[error("Shamir reconstruct failed: {0}")]
    ShamirReconstruct(String),

    #[error("Signature error: {0}")]
    SignatureError(String),

    #[error("Signature verification failed")]
    SignatureInvalid,

    // ── Protocol ─────────────────────────────────────────────────────────────
    #[error("Invalid state transition: {from} → {to}")]
    InvalidTransition { from: String, to: String },

    #[error("Topology derivation failed: {0}")]
    TopologyDerivation(String),

    #[error("Reassembly failed: missing {missing} fragment(s) (threshold not met)")]
    ReassemblyFailed { missing: usize },

    #[error("Session TTL expired")]
    SessionExpired,

    // ── I/O ──────────────────────────────────────────────────────────────────
    #[error("Key file error: {0}")]
    KeyFile(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    // ── Network ──────────────────────────────────────────────────────────────
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("P2P error: {0}")]
    NetworkError(String),
    
    #[error("Bootstrap failed: {0}")]
    BootstrapError(String),
    
    #[error("DHT lookup failed: {0}")]
    DhtLookupError(String),
    
    #[error("Peer not found: {0}")]
    PeerNotFound(String),
    
    #[error("Connection timeout")]
    ConnectionTimeout,

    #[error("Shamir error: {0}")]
    ShamirError(String),

    // ── Generic ──────────────────────────────────────────────────────────────
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Not yet implemented: {0}")]
    NotImplemented(String),
}

// Required by libp2p NetworkBehaviour derive macro
impl From<ConnectionDenied> for PolygoneError {
    fn from(e: ConnectionDenied) -> Self { Self::Network(e.to_string()) }
}
impl From<std::fmt::Error> for PolygoneError {
    fn from(_: std::fmt::Error) -> Self { Self::Network("fmt error".into()) }
}
impl From<anyhow::Error> for PolygoneError {
    fn from(e: anyhow::Error) -> Self { Self::Network(e.to_string()) }
}
impl From<kad::QueryId> for PolygoneError {
    fn from(e: kad::QueryId) -> Self { Self::DhtLookupError(format!("Query error: {:?}", e)) }
}
impl From<kad::NoKnownPeers> for PolygoneError {
    fn from(_: kad::NoKnownPeers) -> Self { Self::BootstrapError("No known peers".into()) }
}
impl From<gossipsub::SubscriptionError> for PolygoneError {
    fn from(e: gossipsub::SubscriptionError) -> Self { Self::Network(format!("Subscription error: {:?}", e)) }
}
impl From<gossipsub::PublishError> for PolygoneError {
    fn from(e: gossipsub::PublishError) -> Self { Self::Network(format!("Publish error: {:?}", e)) }
}
impl From<kad::store::Error> for PolygoneError {
    fn from(e: kad::store::Error) -> Self { Self::DhtLookupError(format!("{:?}", e)) }
}

impl PolygoneError {
    /// Human-readable hint for the TUI.
    pub fn hint(&self) -> &'static str {
        match self {
            Self::KemDecapsulate => "Check that the ciphertext and secret key correspond to the same session.",
            Self::ReassemblyFailed { .. } => "At least 4 of 7 fragments are required to reconstruct.",
            Self::AeadError(_) => "Decryption failed — key mismatch or data corruption.",
            Self::KeyFile(_) => "Run `polygone keygen` first to generate your keypair.",
            Self::SessionExpired => "Sessions expire after 30 seconds. Start a new one.",
            _ => "See `polygone --help` for usage.",
        }
    }
}