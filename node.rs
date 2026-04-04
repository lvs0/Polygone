//! Ephemeral node — born for one computation, dissolved after.

use std::time::{Duration, Instant};
use crate::{crypto::shamir::Fragment, network::NodeId, Result};

/// State of a single ephemeral node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeState {
    /// Node has been assigned but has not yet received its fragment.
    Initialised,
    /// Node holds its fragment and is ready to participate.
    Ready,
    /// Node has completed its part of the computation.
    Computed,
    /// Node has dissolved — its fragment is gone, its state is cleared.
    Dissolved,
}

/// A single ephemeral node in a transit network.
///
/// - Exists for the duration of exactly one session transit.
/// - Holds at most one Shamir fragment.
/// - Transitions: Initialised → Ready → Computed → Dissolved.
/// - On dissolution: fragment is zeroised, state cleared.
pub struct EphemeralNode {
    pub id: NodeId,
    state: NodeState,
    fragment: Option<Fragment>,
    born_at: Instant,
    /// Maximum time this node is allowed to live.
    pub ttl: Duration,
}

impl EphemeralNode {
    /// Create a new ephemeral node with the given ID and TTL.
    pub fn new(id: NodeId, ttl: Duration) -> Self {
        Self {
            id,
            state: NodeState::Initialised,
            fragment: None,
            born_at: Instant::now(),
            ttl,
        }
    }

    /// Assign a Shamir fragment to this node.
    ///
    /// # Errors
    /// Returns an error if the node is not in `Initialised` state.
    pub fn assign_fragment(&mut self, fragment: Fragment) -> Result<()> {
        if self.state != NodeState::Initialised {
            return Err(crate::PolygoneError::InvalidTransition {
                from: format!("{:?}", self.state),
                to: "Ready".into(),
            });
        }
        self.fragment = Some(fragment);
        self.state = NodeState::Ready;
        Ok(())
    }

    /// Extract the fragment (consuming it from this node) and mark as Computed.
    ///
    /// # Errors
    /// Returns an error if the node is not `Ready`.
    pub fn extract_fragment(&mut self) -> Result<Fragment> {
        if self.state != NodeState::Ready {
            return Err(crate::PolygoneError::InvalidTransition {
                from: format!("{:?}", self.state),
                to: "Computed".into(),
            });
        }
        let fragment = self.fragment.take().expect("fragment present when Ready");
        self.state = NodeState::Computed;
        Ok(fragment)
    }

    /// Dissolve this node, clearing all state.
    pub fn dissolve(&mut self) {
        // Zeroize fragment data in-place if present
        if let Some(ref mut f) = self.fragment {
            for byte in &mut f.data { *byte = 0; }
        }
        self.fragment = None;
        self.state = NodeState::Dissolved;
    }

    /// Whether this node's TTL has expired.
    pub fn is_expired(&self) -> bool {
        self.born_at.elapsed() > self.ttl
    }

    /// Current state.
    pub fn state(&self) -> &NodeState { &self.state }
}

impl Drop for EphemeralNode {
    fn drop(&mut self) {
        // Ensure dissolution on drop regardless of state
        self.dissolve();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::shamir::Fragment;
    use crate::network::NodeId;

    fn test_node() -> EphemeralNode {
        let id = NodeId::derive(&[0u8; 32], 0);
        EphemeralNode::new(id, Duration::from_secs(10))
    }

    #[test]
    fn node_lifecycle() {
        let mut node = test_node();
        assert_eq!(node.state(), &NodeState::Initialised);

        let frag = Fragment { id: crate::crypto::shamir::FragmentId(1), data: vec![1, 2, 3] };
        node.assign_fragment(frag).unwrap();
        assert_eq!(node.state(), &NodeState::Ready);

        let _extracted = node.extract_fragment().unwrap();
        assert_eq!(node.state(), &NodeState::Computed);

        node.dissolve();
        assert_eq!(node.state(), &NodeState::Dissolved);
    }

    #[test]
    fn double_extraction_fails() {
        let mut node = test_node();
        let frag = Fragment { id: crate::crypto::shamir::FragmentId(1), data: vec![0] };
        node.assign_fragment(frag).unwrap();
        node.extract_fragment().unwrap();
        assert!(node.extract_fragment().is_err());
    }
}
