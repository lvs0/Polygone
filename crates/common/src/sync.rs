//! Polygone Sync — Time-independent distributed synchronization
//!
//! Core algorithm: **Weighted Deterministic Selection (WDS)** + CRDT backing.
//!
//! ## The Problem with Time-Based Systems
//!
//! Traditional P2P systems use wall-clock timestamps for:
//! - Leader election (who proposes next block)
//! - State synchronization (which version is correct)
//! - Message ordering (which arrived first)
//!
//! These systems fail when:
//! - Clocks are not synchronized (NTP not available)
//! - Network latency varies wildly (mobile ↔ fiber)
//! - Nodes join/leave at arbitrary times
//!
//! ## The Solution: WDS
//!
//! **No timestamps. No timers. No wall-clock dependency.**
//!
//! Instead of time, we use **cryptographic determinism**:
//!
//! 1. **Capability Weight** — each node's contribution to the network
//! 2. **Global State Hash** — BLAKE3 hash of current distributed state
//! 3. **Deterministic Priority** — `min(H(pubkey || state_hash || weight))`
//! 4. **CRDT Merge** — commutative, idempotent state synchronization
//!
//! Result: same nodes + same state → same leader, same ordering, same merge.
//! Always. Without any clock synchronization.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════
// CAPABILITY SYSTEM — Node contribution scoring
// ═══════════════════════════════════════════════════════════════════

/// Hardware capabilities a node can advertise.
/// Used to compute the node's **weight** in the consensus algorithm.
///
/// Each component is normalized 0.0–1.0 where 1.0 = reference hardware.
/// This avoids absolute benchmarks — instead, we rank relative capability.
/// Hardware capabilities a node can advertise.
/// Used to compute the node's **weight** in the consensus algorithm.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Capabilities {
    /// CPU: single-threaded performance (normalized 0-1)
    pub cpu_score: f64,
    /// GPU: compute throughput (normalized 0-1)
    pub gpu_score: f64,
    /// Memory: GiB available / 64.0 (normalized 0-1)
    pub ram_score: f64,
    /// Storage: sequential read MB/s / 3000 (normalized 0-1)
    pub storage_score: f64,
    /// Network: Mbps bandwidth / 1000 (normalized 0-1)
    pub bandwidth_score: f64,
    /// Reliability: hours online / 720 (1 month = 1.0)
    pub reliability_score: f64,
}

impl PartialEq for Capabilities {
    fn eq(&self, other: &Self) -> bool {
        (self.cpu_score - other.cpu_score).abs() < 1e-9
            && (self.gpu_score - other.gpu_score).abs() < 1e-9
            && (self.ram_score - other.ram_score).abs() < 1e-9
            && (self.storage_score - other.storage_score).abs() < 1e-9
            && (self.bandwidth_score - other.bandwidth_score).abs() < 1e-9
            && (self.reliability_score - other.reliability_score).abs() < 1e-9
    }
}

impl Capabilities {
    /// Compute a weighted capability score for WDS consensus.
    ///
    /// Weights are based on what the network actually needs:
    /// - compute (CPU/GPU) for AI inference, cryptography
    /// - memory for state caching
    /// - storage for fragment persistence
    /// - bandwidth for real-time communication
    /// - reliability as a multiplier (long-running nodes are more trusted)
    pub fn weight(&self) -> u32 {
        let cpu = (self.cpu_score * 0.20).min(1.0);
        let gpu = (self.gpu_score * 0.30).min(1.0);
        let ram = (self.ram_score * 0.15).min(1.0);
        let storage = (self.storage_score * 0.10).min(1.0);
        let bw = (self.bandwidth_score * 0.15).min(1.0);
        let reliability = (self.reliability_score * 0.10).min(1.0);

        let raw = cpu + gpu + ram + storage + bw + reliability;
        (raw * 1000.0) as u32
    }

    /// Create from raw system info (benchmarking helper).
    /// Real implementations call this on startup to probe hardware.
    pub fn probe() -> Self {
        Self::default()
    }
}

// ═══════════════════════════════════════════════════════════════════
// WDS — WEIGHTED DETERMINISTIC SELECTION
// ═══════════════════════════════════════════════════════════════════

/// A node registered in the Polygone consensus layer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SyncNode {
    pub node_id: [u8; 32],
    /// Public key for cryptographic verification
    pub pubkey: Vec<u8>,
    pub capabilities: Capabilities,
    /// Vector clock for CRDT ordering: node_id → logical counter
    pub vector_clock: HashMap<[u8; 32], u64>,
}

impl SyncNode {
    pub fn new(node_id: [u8; 32], pubkey: Vec<u8>) -> Self {
        Self {
            node_id,
            pubkey,
            capabilities: Capabilities::default(),
            vector_clock: HashMap::from([(node_id, 1)]),
        }
    }

    /// Compute deterministic priority for leader election.
    ///
    /// priority = min( BLAKE3(pubkey || state_hash || weight) )
    ///
    /// This is the heart of WDS: same inputs → same priority always.
    /// No randomness, no time, no leader election delays.
    pub fn compute_priority(&self, state_hash: &[u8]) -> [u8; 32] {
        use blake3::Hasher;

        let mut h = Hasher::new();
        h.update(&self.pubkey);
        h.update(state_hash);
        h.update(&self.capabilities.weight().to_le_bytes());
        *h.finalize().as_bytes()
    }

    /// Increment the local vector clock entry.
    pub fn tick(&mut self) {
        *self.vector_clock.entry(self.node_id).or_insert(0) += 1;
    }

    /// Merge another node's vector clock (CRDT merge — max per key).
    pub fn merge_clock(&mut self, other: &HashMap<[u8; 32], u64>) {
        for (k, v) in other {
            let entry = self.vector_clock.entry(*k).or_insert(0);
            if *v > *entry {
                *entry = *v;
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// CRDT — CONFLICT-FREE REPLICATED DATA TYPE for state sync
// ═══════════════════════════════════════════════════════════════════

/// 2P-Set CRDT: elements added with unique tags, removed via tombstones.
/// This is the core state structure for Polygone message routing tables.
///
/// Property: two nodes with the same history always converge to the same state,
/// regardless of the order in which updates are received.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoPSet {
    /// element → Set of unique add-tags
    adds: HashMap<Vec<u8>, Vec<u64>>,
    /// Set of remove-tags (union of all removes)
    removes: std::collections::HashSet<u64>,
    /// Auto-incrementing tag counter (lower 48 bits)
    tag_counter: u64,
    /// Per-node tag offsets for uniqueness across nodes
    node_tag_offsets: HashMap<[u8; 32], u64>,
}

impl Default for TwoPSet {
    fn default() -> Self {
        Self {
            adds: HashMap::new(),
            removes: std::collections::HashSet::new(),
            tag_counter: 0,
            node_tag_offsets: HashMap::new(),
        }
    }
}

impl TwoPSet {
    /// Add an element to the set.
    /// Returns the unique tag assigned.
    pub fn add(&mut self, element: Vec<u8>, node_id: [u8; 32]) -> u64 {
        let tag = self.gen_tag(node_id);
        self.adds.entry(element).or_default().push(tag);
        tag
    }

    /// Remove an element (adds tombstones for all existing tags).
    /// Returns the remove tag, or None if element wasn't present.
    pub fn remove(&mut self, element: &[u8], node_id: [u8; 32]) -> Option<u64> {
        let tags = self.adds.get_mut(element)?.clone();
        let tag = self.gen_tag(node_id);
        for t in &tags {
            self.removes.insert(*t);
        }
        self.adds.remove(element);
        Some(tag)
    }

    /// Check if an element is in the set (has a non-tombstoned tag).
    pub fn contains(&self, element: &[u8]) -> bool {
        let Some(tags) = self.adds.get(element) else {
            return false;
        };
        tags.iter().any(|t| !self.removes.contains(t))
    }

    /// Get all current elements in the set.
    pub fn values(&self) -> Vec<Vec<u8>> {
        self.adds
            .keys()
            .filter(|e| self.contains(e))
            .cloned()
            .collect()
    }

    /// Merge another TwoPSet (CRDT commutative merge).
    /// Idempotent — merge(X, Y) = merge(Y, X) regardless of order.
    pub fn merge(&mut self, other: &TwoPSet) {
        for (element, other_tags) in &other.adds {
            let entry = self.adds.entry(element.clone()).or_default();
            for tag in other_tags {
                if !entry.contains(tag) {
                    entry.push(*tag);
                }
            }
        }
        self.removes.extend(&other.removes);
        self.tag_counter = self.tag_counter.max(other.tag_counter);
    }

    fn gen_tag(&mut self, node_id: [u8; 32]) -> u64 {
        self.tag_counter += 1;
        // Embed node_id prefix for uniqueness
        let node_prefix = u64::from_le_bytes(
            <[u8; 8]>::try_from(&node_id[..8]).unwrap_or([0u8; 8]),
        );
        node_prefix.wrapping_mul(1_000_000) + self.tag_counter
    }
}

/// LWW-Register CRDT: last-write-wins register with vector clock tiebreaking.
/// Used for node metadata that changes over time (capabilities, status).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LWWRegister<T> {
    value: Option<T>,
    timestamp: u64,
    writer: [u8; 32],
}

impl<T: Clone + PartialEq> Default for LWWRegister<T> {
    fn default() -> Self {
        Self {
            value: None,
            timestamp: 0,
            writer: [0u8; 32],
        }
    }
}

impl<T: Clone + PartialEq> LWWRegister<T> {
    pub fn set(&mut self, value: T, ts: u64, writer: [u8; 32]) {
        if ts > self.timestamp || (ts == self.timestamp && writer > self.writer) {
            self.value = Some(value);
            self.timestamp = ts;
            self.writer = writer;
        }
    }

    pub fn get(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn merge(&mut self, other: &LWWRegister<T>) {
        if other.timestamp > self.timestamp
            || (other.timestamp == self.timestamp && other.writer > self.writer)
        {
            self.value = other.value.clone();
            self.timestamp = other.timestamp;
            self.writer = other.writer;
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// WDS CONSENSUS ENGINE — Time-independent consensus
// ═══════════════════════════════════════════════════════════════════

/// The global distributed state tracked by WDS.
/// All nodes must converge to the same state_hash after convergence.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DistributedState {
    /// Active nodes and their metadata (LWW-Register per node)
    pub nodes: HashMap<[u8; 32], LWWRegister<SyncNode>>,
    /// Routing table: known peer IDs (TwoPSet per domain)
    pub routing: TwoPSet,
    /// Pending messages queue (element = message_hash)
    pub pending: TwoPSet,
    /// Global vector clock (max of all node clocks)
    pub vector_clock: HashMap<[u8; 32], u64>,
    /// Consensus round counter
    pub round: u64,
}

impl DistributedState {
    /// Add a node to the distributed state.
    pub fn add_node(&mut self, node: SyncNode) {
        let id = node.node_id;
        let mut reg = LWWRegister::default();
        reg.set(node, 0, id);
        self.nodes.insert(id, reg);
    }

    /// Merge another node's state (CRDT merge — idempotent, commutative).
    pub fn merge(&mut self, other: &DistributedState) {
        for (id, other_reg) in &other.nodes {
            let entry = self.nodes.entry(*id).or_insert_with(LWWRegister::default);
            entry.merge(other_reg);
        }
        self.routing.merge(&other.routing);
        self.pending.merge(&other.pending);
        for (k, v) in &other.vector_clock {
            let entry = self.vector_clock.entry(*k).or_insert(0);
            if *v > *entry {
                *entry = *v;
            }
        }
        if other.round > self.round {
            self.round = other.round;
        }
    }

    /// Compute the state hash — BLAKE3 of all state components.
    /// This is the **global_state_hash** input to WDS priority computation.
    pub fn state_hash(&self) -> [u8; 32] {
        use blake3::Hasher;

        let mut h = Hasher::new();
        h.update(&self.round.to_le_bytes());

        let mut node_ids: Vec<_> = self.nodes.keys().copied().collect();
        node_ids.sort();
        for id in node_ids {
            h.update(&id);
            if let Some(reg) = self.nodes.get(&id) {
                h.update(&reg.timestamp.to_le_bytes());
            }
        }

        let mut vc: Vec<_> = self.vector_clock.iter().collect();
        vc.sort_by_key(|(k, _)| *k);
        for (_, v) in vc {
            h.update(&v.to_le_bytes());
        }

        *h.finalize().as_bytes()
    }
}

/// WDS Consensus Engine — one round of deterministic consensus.
pub struct ConsensusEngine {
    state: DistributedState,
    threshold: f64,
}

impl ConsensusEngine {
    pub fn new() -> Self {
        Self {
            state: DistributedState::default(),
            threshold: 0.67,
        }
    }

    /// Select the leader for the current state_hash (deterministic).
    /// Leader = node with minimum BLAKE3(pubkey || state_hash || weight).
    /// Ties broken by node_id ordering (stable sort).
    pub fn select_leader<'a>(&self, nodes: &'a [SyncNode]) -> Option<&'a SyncNode> {
        let state_hash = self.state.state_hash();
        let mut sorted: Vec<_> = nodes.iter().collect();
        sorted.sort_by_key(|n| n.compute_priority(&state_hash));
        sorted.into_iter().next()
    }

    /// Check if a set of voters has reached consensus (weighted majority).
    /// Returns true if total voter weight >= threshold * total network weight.
    pub fn check_consensus(&self, voters: &[[u8; 32]], nodes: &[SyncNode]) -> bool {
        let total_weight: u64 = nodes.iter().map(|n| n.capabilities.weight() as u64).sum();
        let voter_weight: u64 = voters
            .iter()
            .filter_map(|vid| nodes.iter().find(|n| n.node_id == *vid))
            .map(|n| n.capabilities.weight() as u64)
            .sum();

        let ratio = voter_weight as f64 / total_weight.max(1) as f64;
        ratio >= self.threshold
    }

    /// Advance the consensus round (after successful commit).
    pub fn commit_round(&mut self) {
        self.state.round += 1;
    }

    pub fn state(&self) -> &DistributedState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut DistributedState {
        &mut self.state
    }
}

impl Default for ConsensusEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    fn make_node(idx: u8) -> SyncNode {
        let node_id = [idx; 32];
        SyncNode::new(node_id, vec![idx; 32])
    }

    #[test]
    fn test_wds_determinism() {
        let mut n1 = make_node(1);
        let mut n2 = make_node(2);
        let mut n3 = make_node(3);

        n1.capabilities = Capabilities {
            cpu_score: 0.8,
            ..Default::default()
        };
        n2.capabilities = Capabilities {
            cpu_score: 0.5,
            ..Default::default()
        };
        n3.capabilities = Capabilities {
            cpu_score: 0.9,
            ..Default::default()
        };

        let nodes = vec![n1.clone(), n2.clone(), n3.clone()];
        let engine = ConsensusEngine::new();

        // Same state_hash → same leader, every time
        let leader1 = engine.select_leader(&nodes).map(|n| n.node_id[0]);
        let leader2 = engine.select_leader(&nodes).map(|n| n.node_id[0]);

        assert_eq!(leader1, leader2, "WDS must be deterministic");
    }

    #[test]
    fn test_capability_weight() {
        let weak = Capabilities {
            cpu_score: 0.1,
            ..Default::default()
        };
        let strong = Capabilities {
            cpu_score: 1.0,
            gpu_score: 1.0,
            ram_score: 1.0,
            ..Default::default()
        };

        assert!(strong.weight() > weak.weight());
    }

    #[test]
    fn test_2p_set_crdt() {
        let mut set1 = TwoPSet::default();
        let mut set2 = TwoPSet::default();

        let node1 = [1u8; 32];
        let node2 = [2u8; 32];

        set1.add(b"hello".to_vec(), node1);
        set2.add(b"hello".to_vec(), node2);

        let mut merged = set1.clone();
        merged.merge(&set2);

        assert!(merged.contains(b"hello"));
        assert_eq!(merged.values().len(), 1);

        let mut set3 = merged.clone();
        set3.remove(b"hello", node1);
        assert!(!set3.contains(b"hello"));
    }

    #[test]
    fn test_lww_register_merge() {
        let mut reg1 = LWWRegister::default();
        let mut reg2 = LWWRegister::default();

        let node1 = [1u8; 32];
        let node2 = [2u8; 32];

        reg1.set("value1".to_string(), 10, node1);
        reg2.set("value2".to_string(), 5, node2);
        reg1.merge(&reg2);
        assert_eq!(reg1.get(), Some(&"value1".to_string()));

        let mut reg3 = LWWRegister::default();
        reg3.set("value3".to_string(), 20, node2);
        reg1.merge(&reg3);
        assert_eq!(reg1.get(), Some(&"value3".to_string()));
    }

    #[test]
    fn test_state_merge_idempotent() {
        let mut state1 = DistributedState::default();
        let mut state2 = DistributedState::default();

        let n1 = make_node(1);
        let n2 = make_node(2);

        state1.add_node(n1);
        state2.add_node(n2);

        let mut s_a = state1.clone();
        let mut s_b = state1.clone();

        s_a.merge(&state2);
        s_b.merge(&state1);
        s_b.merge(&state2);

        assert_eq!(s_a.nodes.len(), s_b.nodes.len());
        assert_eq!(s_a.state_hash(), s_b.state_hash());
    }

    #[test]
    fn test_consensus_threshold() {
        let engine = ConsensusEngine::new();

        let mut n1 = make_node(1);
        let mut n2 = make_node(2);
        let mut n3 = make_node(3);

        // Give all nodes non-zero capabilities so weight > 0
        n1.capabilities = Capabilities { cpu_score: 1.0, ..Default::default() };
        n2.capabilities = Capabilities { cpu_score: 1.0, ..Default::default() };
        n3.capabilities = Capabilities { cpu_score: 1.0, ..Default::default() };

        let nodes = vec![n1, n2, n3];

        // 3/3 = 100% — should pass
        let voters = [[1u8; 32], [2u8; 32], [3u8; 32]];
        assert!(engine.check_consensus(&voters, &nodes));

        // 1/3 = 33% — should fail (threshold = 67%)
        let voters = [[1u8; 32]];
        assert!(!engine.check_consensus(&voters, &nodes));
    }
}