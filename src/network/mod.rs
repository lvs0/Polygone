pub mod node;
pub mod p2p;
pub mod topology;
pub use node::EphemeralNode;
pub use topology::{Topology, TopologyParams};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId([u8; 16]);

impl NodeId {
    pub fn derive(session_key: &[u8; 32], index: u8) -> Self {
        let mut input = [0u8; 33];
        input[..32].copy_from_slice(session_key);
        input[32] = index;
        let hash = blake3::hash(&input);
        let mut id = [0u8; 16];
        id.copy_from_slice(&hash.as_bytes()[..16]);
        NodeId(id)
    }
    pub fn as_bytes(&self) -> &[u8; 16] { &self.0 }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(&self.0[..6]))
    }
}
