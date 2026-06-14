use polygone_common::NodeId;

#[derive(Clone, Debug)]
pub struct P2PNode {
    pub node_id: NodeId,
}

impl Default for P2PNode {
    fn default() -> Self {
        Self::new()
    }
}

impl P2PNode {
    pub fn new() -> Self {
        Self { node_id: NodeId([0u8; 32]) }
    }
}
