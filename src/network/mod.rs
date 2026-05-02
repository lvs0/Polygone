//! Ephemeral network topology and node lifecycle.
//!
//! This module provides the P2P networking infrastructure for Polygone,
//! including the unified P2P layer (`p2p`), ephemeral node management,
//! and topology derivation.

pub mod discovery;
pub mod node;
pub mod p2p;
pub mod topology;

pub use node::EphemeralNode;
pub use p2p::{
    P2pNode, P2pConfig, NetworkEvent, PolygoneRequest, PolygoneResponse,
    GossipMessage, Capability, run_bootstrap_node, Multiaddr, PeerId,
    DEFAULT_BOOTSTRAP_NODES, TOPOLOGY_TOPIC, POLYGONE_PROTOCOL_VERSION,
    REQUEST_RESPONSE_PROTOCOL, KADEMLIA_PROTOCOL_NAME,
};
pub use topology::{Topology, TopologyParams};

use serde::{Deserialize, Serialize};
use std::fmt;

/// A unique ephemeral node identifier (derived from the shared secret).
///
/// NodeIds are deterministic: both Alice and Bob independently derive
/// identical node IDs from the shared secret, without any extra communication.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub [u8; 8]);

impl NodeId {
    /// Get the bytes of this NodeId
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    
    /// Create a NodeId from a byte slice (must be exactly 8 bytes)
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 8 {
            return None;
        }
        let mut arr = [0u8; 8];
        arr.copy_from_slice(bytes);
        Some(Self(arr))
    }
    
    /// Derive a NodeId from a seed and index
    pub fn derive(seed: &[u8; 32], index: u8) -> Self {
        use blake3::Hasher;
        let mut hasher = Hasher::new_derive_key("polygone-nodeid-v1");
        hasher.update(seed);
        hasher.update(&[index]);
        let mut id = [0u8; 8];
        id.copy_from_slice(&hasher.finalize().as_bytes()[..8]);
        Self(id)
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.0[..4]))
    }
}
