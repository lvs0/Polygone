//! Unified error type for `polygone-core`.

use thiserror::Error;

/// All errors that can be produced by the polygone protocol stack.
#[derive(Debug, Error)]
pub enum PolygoneError {
    // ── Cryptographic errors ──────────────────────────────────────
    /// Key encapsulation failed (ML-KEM).
    #[error("KEM encapsulation failed: {0}")]
    KemEncapsulate(String),

    /// Key decapsulation failed — ciphertext may have been tampered.
    #[error("KEM decapsulation failed — ciphertext integrity violation")]
    KemDecapsulate,

    /// Signature verification failed.
    #[error("Signature verification failed")]
    SignatureInvalid,

    /// AEAD encryption/decryption error — wrong key or corrupted data.
    #[error("Symmetric cipher error: {0}")]
    AeadError(String),

    /// Shamir fragmentation or reconstruction failed.
    #[error("Secret sharing error: {0}")]
    ShamirError(String),

    /// The random number generator failed — catastrophic.
    #[error("RNG failure: {0}")]
    RngError(String),

    // ── Network errors ────────────────────────────────────────────
    /// Could not derive a valid topology from the given key material.
    #[error("Topology derivation failed: {0}")]
    TopologyDerivation(String),

    /// A required node was not found in the current topology.
    #[error("Node {0:?} not found in topology")]
    NodeNotFound(crate::network::NodeId),

    /// The network quorum was not reached before the session expired.
    #[error("Quorum timeout — {reached}/{required} nodes responded")]
    QuorumTimeout { reached: usize, required: usize },

    // ── Protocol errors ───────────────────────────────────────────
    /// Session has already been dissolved and can no longer be used.
    #[error("Session {0:?} has been dissolved")]
    SessionDissolved(crate::protocol::SessionId),

    /// Unexpected protocol state transition.
    #[error("Invalid state transition: {from} → {to}")]
    InvalidTransition { from: String, to: String },

    /// Message reassembly failed — one or more fragments are missing.
    #[error("Fragment reassembly failed: {missing} fragments missing")]
    ReassemblyFailed { missing: usize },

    // ── I/O errors ────────────────────────────────────────────────
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),
}
