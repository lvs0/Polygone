//! Ephemeral node lifecycle management.
//!
//! Each node lives for at most `ttl` milliseconds, then dissolves.
//! Fragment data is zeroed on dissolution.

use std::time::{Duration, Instant};
use zeroize::Zeroize;

use super::NodeId;

/// The lifecycle state of an ephemeral node.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeState {
    /// Node is alive and can receive/hold a fragment.
    Active,
    /// Node has dissolved — all data zeroed.
    Dissolved,
}

/// An ephemeral relay node.
///
/// Holds at most one fragment for at most `ttl` duration.
/// After dissolution, all fragment bytes are zeroed in memory.
pub struct EphemeralNode {
    pub id: NodeId,
    pub state: NodeState,
    /// The fragment this node is holding, if any.
    fragment: Option<Vec<u8>>,
    /// When this node was created.
    born_at: Instant,
    /// Maximum lifetime before forced dissolution.
    ttl: Duration,
}

impl EphemeralNode {
    /// Create a new active node with the given TTL.
    pub fn new(id: NodeId, ttl: Duration) -> Self {
        Self {
            id,
            state: NodeState::Active,
            fragment: None,
            born_at: Instant::now(),
            ttl,
        }
    }

    /// Deposit a fragment into this node.
    /// Returns Err if the node is already dissolved or already holds a fragment.
    pub fn deposit(&mut self, data: Vec<u8>) -> Result<(), &'static str> {
        if self.state == NodeState::Dissolved {
            return Err("node already dissolved");
        }
        if self.fragment.is_some() {
            return Err("node already holds a fragment");
        }
        self.fragment = Some(data);
        Ok(())
    }

    /// Collect the fragment from this node, consuming it.
    pub fn collect(&mut self) -> Option<Vec<u8>> {
        self.check_ttl();
        self.fragment.take()
    }

    /// Check if the TTL has expired and auto-dissolve if so.
    pub fn check_ttl(&mut self) {
        if self.born_at.elapsed() >= self.ttl {
            self.dissolve();
        }
    }

    /// Has this node's TTL expired?
    pub fn is_expired(&self) -> bool {
        self.born_at.elapsed() >= self.ttl
    }

    /// Dissolve this node: zero all held fragment data.
    pub fn dissolve(&mut self) {
        if let Some(mut data) = self.fragment.take() {
            data.zeroize();
        }
        self.state = NodeState::Dissolved;
    }

    /// Time remaining before TTL expiry.
    pub fn time_remaining(&self) -> Duration {
        self.ttl.saturating_sub(self.born_at.elapsed())
    }

    /// Short display string for TUI.
    pub fn status_char(&self) -> &'static str {
        match self.state {
            NodeState::Active => "●",
            NodeState::Dissolved => "○",
        }
    }
}

impl Drop for EphemeralNode {
    fn drop(&mut self) {
        // Ensure fragment bytes are zeroed on any drop path
        if let Some(mut data) = self.fragment.take() {
            data.zeroize();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_deposit_and_collect() {
        let id = NodeId([1u8; 8]);
        let mut node = EphemeralNode::new(id, Duration::from_secs(30));
        node.deposit(vec![1, 2, 3]).unwrap();
        let data = node.collect().unwrap();
        assert_eq!(data, vec![1, 2, 3]);
    }

    #[test]
    fn dissolved_node_rejects_deposit() {
        let id = NodeId([2u8; 8]);
        let mut node = EphemeralNode::new(id, Duration::from_secs(30));
        node.dissolve();
        assert!(node.deposit(vec![1, 2, 3]).is_err());
    }
}
