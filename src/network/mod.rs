//! Ephemeral network topology and node lifecycle.

pub mod node;
pub mod topology;

#[cfg(feature = "server")]
pub mod p2p;
pub mod discovery;

pub use node::EphemeralNode;
pub use topology::{Topology, TopologyParams};

use serde::{Deserialize, Serialize};
use std::fmt;

/// A unique ephemeral node identifier (derived from the shared secret).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub [u8; 8]);

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.0[..4]))
    }
}