//! Fragment dispatch types for Polygone's Shamir-based secret distribution.
//!
//! This module defines the wire format and dispatch metadata for sending
//! Shamir fragments across the ephemeral P2P network. Each fragment is
//! encrypted under the session key and routed to a specific node determined
//! by the deterministic topology.

use serde::{Deserialize, Serialize};
use crate::NodeId;

/// Opaque identifier for a fragment (1-indexed, matching Shamir share index).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FragmentId(pub u8);

impl FragmentId {
    pub fn new(id: u8) -> Self {
        assert!(id > 0, "FragmentId must be 1-indexed");
        Self(id)
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

/// A single fragment ready for dispatch over the network.
///
/// This is the wire-level representation: the fragment bytes are encrypted
/// with the session key (AES-256-GCM), and the nonce is included for
/// decryption on the receiving side.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FragmentPayload {
    /// Which fragment this is (1-indexed).
    pub id: FragmentId,
    /// Destination node in the ephemeral topology.
    pub destination: NodeId,
    /// Encrypted fragment data (AES-256-GCM ciphertext).
    pub ciphertext: Vec<u8>,
    /// Nonce used for encryption.
    pub nonce: [u8; 12],
}

/// Result of a successful fragment dispatch operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatchResult {
    /// Session ID this dispatch belongs to.
    pub session_id: [u8; 32],
    /// All fragment payloads that were created.
    pub payloads: Vec<FragmentPayload>,
    /// Number of fragments successfully dispatched.
    pub dispatched_count: u8,
    /// Number of fragments that failed (for retry logic).
    pub failed_count: u8,
    /// Timestamp of dispatch (UNIX epoch seconds).
    pub timestamp: u64,
}

/// Acknowledgment from a node that it received a fragment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FragmentAck {
    /// The fragment ID being acknowledged.
    pub fragment_id: FragmentId,
    /// The node that sent this ack.
    pub node_id: NodeId,
    /// BLAKE3 hash of the received ciphertext for integrity verification.
    pub ciphertext_hash: [u8; 32],
}

/// Request to collect fragments for reconstruction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectRequest {
    /// Session ID to collect fragments for.
    pub session_id: [u8; 32],
    /// Minimum number of fragments needed (threshold).
    pub threshold: u8,
    /// Maximum time to wait for fragment collection (milliseconds).
    pub timeout_ms: u64,
}

/// Aggregated fragments ready for Shamir reconstruction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectedFragments {
    /// Session ID these fragments belong to.
    pub session_id: [u8; 32],
    /// The collected (decrypted) fragment data.
    pub fragments: Vec<(FragmentId, Vec<u8>)>,
    /// Whether we have enough fragments (≥ threshold).
    pub sufficient: bool,
}

/// Configuration for the fragment dispatch system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatchConfig {
    /// Maximum fragment size in bytes before splitting (default: 1 MB).
    pub max_fragment_size: usize,
    /// Retry attempts for failed dispatches (default: 3).
    pub max_retries: u8,
    /// Timeout per dispatch in milliseconds (default: 5000).
    pub dispatch_timeout_ms: u64,
    /// Whether to encrypt fragments with the session key (default: true).
    pub encrypt_fragments: bool,
}

impl Default for DispatchConfig {
    fn default() -> Self {
        Self {
            max_fragment_size: 1_048_576, // 1 MB
            max_retries: 3,
            dispatch_timeout_ms: 5000,
            encrypt_fragments: true,
        }
    }
}
