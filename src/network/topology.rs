//! Deterministic topology derivation from shared key material.
//!
//! Given a 32-byte session key, both peers derive:
//! - How many nodes the transit network has (N).
//! - Which NodeIds to use.
//! - The connectivity graph between those nodes.
//! - The fragment-to-node assignment (for Shamir distribution).
//!
//! This derivation is **pure** (no randomness, no I/O) and
//! **deterministic** — both peers always get the same topology.

use crate::network::NodeId;

/// Parameters controlling the size and resilience of the ephemeral network.
#[derive(Debug, Clone)]
pub struct TopologyParams {
    /// Number of ephemeral nodes (must be ≥ threshold).
    pub node_count: u8,
    /// Shamir threshold — minimum nodes needed for reconstruction.
    pub threshold: u8,
    /// Maximum number of connections per node (controls graph density).
    pub max_edges_per_node: u8,
}

impl Default for TopologyParams {
    fn default() -> Self {
        Self {
            node_count: 7,
            threshold: 4,
            max_edges_per_node: 3,
        }
    }
}

/// A derived ephemeral topology: a graph of NodeIds and their connections.
#[derive(Debug)]
pub struct Topology {
    pub params: TopologyParams,
    /// All nodes in this topology, in order.
    pub nodes: Vec<NodeId>,
    /// Adjacency list: node index → list of neighbour indices.
    pub edges: Vec<Vec<usize>>,
    /// Which node index receives which Shamir fragment (1-indexed).
    pub fragment_assignment: Vec<(u8, usize)>,
}

impl Topology {
    /// Derive a topology from the session key bytes.
    ///
    /// Uses BLAKE3 in counter mode to deterministically produce all the
    /// structural randomness the topology needs.
    pub fn derive(session_key: &[u8; 32], params: TopologyParams) -> crate::Result<Self> {
        let n = params.node_count as usize;
        let t = params.threshold;

        if t as usize > n {
            return Err(crate::PolygoneError::TopologyDerivation(format!(
                "threshold ({t}) > node_count ({n})"
            )));
        }

        // 1. Derive all NodeIds
        let nodes: Vec<NodeId> = (0..n as u8)
            .map(|i| NodeId::derive(session_key, i))
            .collect();

        // 2. Derive edges deterministically
        //    For each node i, BLAKE3-hash (key || i || "edges") → pick neighbours
        let mut edges: Vec<Vec<usize>> = vec![vec![]; n];
        for i in 0..n {
            let seed = blake3::derive_key(
                "polygone edge seed v1",
                &[session_key.as_ref(), &[i as u8]].concat(),
            );
            let seed_bytes = seed;
            // Pick up to max_edges_per_node neighbours from the seed
            for slot in 0..params.max_edges_per_node as usize {
                let j = seed_bytes[slot as usize] as usize % n;
                if j != i && !edges[i].contains(&j) {
                    edges[i].push(j);
                    if !edges[j].contains(&i) {
                        edges[j].push(i); // undirected
                    }
                }
            }
        }

        // 3. Assign Shamir fragments to nodes
        //    First `n` fragments (1..=n) → nodes 0..n  (one-to-one for n <= n)
        let fragment_assignment: Vec<(u8, usize)> = (0..n).map(|i| (i as u8 + 1, i)).collect();

        Ok(Topology {
            params,
            nodes,
            edges,
            fragment_assignment,
        })
    }

    /// Return the NodeId at index `i`.
    pub fn node(&self, i: usize) -> Option<&NodeId> {
        self.nodes.get(i)
    }

    /// Return the neighbours of node `i`.
    pub fn neighbours(&self, i: usize) -> &[usize] {
        self.edges.get(i).map(|v| v.as_slice()).unwrap_or(&[])
    }
}
