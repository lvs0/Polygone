// Bouchon réseau pour compilation.

use polygone_common::NodeId;

pub struct P2PNode {
    pub node_id: NodeId,
}

impl P2PNode {
    pub fn new() -> Self {
        // Bouchon ID
        Self { node_id: NodeId([0u8; 32]) }
    }
}
