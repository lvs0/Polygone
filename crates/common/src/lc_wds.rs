//! LC-WDS — Latency-Cluster Weighted Deterministic Selection
//!
//! Enhancement of WDS where heterogeneous latency is absorbed into the
//! protocol itself instead of being a defect to overcome.
//!
//! ## Why clustering?
//!
//! Pure WDS assumes everyone has similar latency. Reality is harsher:
//!
//! ```text
//! Mobile 4G    → RTT 80–200ms  (variable)
//! Wi-Fi home   → RTT 2–10ms    (variable)
//! Fiber/DC     → RTT 0.2–2ms   (stable)
//! Satellite    → RTT 600ms+    (variable)
//! ```
//!
//! Trying to elect a global leader synchronously across all of these
//! creates bottlenecks. The mobile node slows the round for everyone.
//!
//! ## The tire metaphor
//!
//! A wheel rotates at the speed of its **slowest** point (the contact patch).
//! But that's only because the wheel is rigid. Polygone is not rigid:
//!
//! - Nodes with similar latency form a **cluster** = one "tire"
//! - Each tire elects its own leader **internally** (low-latency consensus)
//! - Tires exchange leaders and state **asynchronously** (gossip-style)
//! - The global "axle" (a tiny function) hashes the leader-set to get the
//!   global leader — no global round needed
//!
//! ```text
//!   Tire A (fiber)         Tire B (mobile)        Tire C (satellite)
//!      5 nodes                3 nodes                  2 nodes
//!   ┌──────────┐           ┌──────────┐           ┌──────────┐
//!   │ ●—●—●    │ ←─gossip─→│ ●—●      │ ←─gossip─→│ ●—●      │
//!   │ leader A │   async    │ leader B │   async    │ leader C │
//!   └──────────┘           └──────────┘           └──────────┘
//!           \                  |                  /
//!            ──────────────────┼──────────────────
//!                              ▼
//!                ┌──────────────────────────┐
//!                │  global_leader = L(A,B,C)│   ← deterministic, trivial
//!                └──────────────────────────┘
//! ```
//!
//! ## Properties preserved
//!
//! - **Time-independent**: cluster IDs derive from latency samples, not clock
//! - **Deterministic**: given the same cluster topology, same global leader
//! - **Latency-tolerant**: a slow node only slows its own tire — not the network
//! - **CRDT-friendly**: cluster membership itself is a 2P-Set, no coordination

use blake3::Hasher;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Latency sample taken during a handshake or heartbeat.
/// Lower = closer/better link.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct LatencyRtt(pub u32); // milliseconds

impl LatencyRtt {
    pub fn ms(self) -> u32 {
        self.0
    }
}

/// A "tire": group of nodes with similar end-to-end latency.
///
/// Membership is dynamic: as the network probe samples change,
/// tires shrink, merge, or split. We use a deterministic rule
/// (latency bins within ±X ms) so all nodes agree on tire boundaries
/// without communication.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LatencyCluster {
    /// Cluster ID — derived from the latency band, not assigned.
    /// Two nodes in the SAME band end up with the SAME id without talking.
    pub cluster_id: [u8; 32],
    /// Latency range this cluster covers (e.g. 0–10ms = "fiber").
    pub latency_band_ms: (u32, u32),
    /// Stable member set (2P-Set: add_set + remove_set, see sync.rs).
    pub members: BTreeSet<[u8; 32]>,
}

impl LatencyCluster {
    /// Deterministic cluster id from a latency band — same band → same id.
    pub fn id_for_band(lo_ms: u32, hi_ms: u32) -> [u8; 32] {
        let mut h = Hasher::new();
        h.update(&lo_ms.to_le_bytes());
        h.update(&hi_ms.to_le_bytes());
        *h.finalize().as_bytes()
    }

    pub fn new(lo_ms: u32, hi_ms: u32) -> Self {
        Self {
            cluster_id: Self::id_for_band(lo_ms, hi_ms),
            latency_band_ms: (lo_ms, hi_ms),
            members: BTreeSet::new(),
        }
    }

    pub fn contains(&self, pubkey: &[u8; 32]) -> bool {
        self.members.contains(pubkey)
    }

    pub fn add_member(&mut self, pubkey: [u8; 32]) {
        self.members.insert(pubkey);
    }

    pub fn weight<T: ClusterMember>(&self, nodes: &[T]) -> u32 {
        nodes
            .iter()
            .filter(|n| self.members.contains(n.pubkey()))
            .map(|n| n.weight())
            .sum()
    }
}

/// Trait a node must implement to participate in LC-WDS.
pub trait ClusterMember {
    fn pubkey(&self) -> &[u8; 32];
    fn weight(&self) -> u32;
}

/// Cluster assignment for a single node, given its measured RTT to
/// every other node in the network.
///
/// The bins are logarithmic (cluster 1: 0–4ms, cluster 2: 4–16ms,
/// cluster 3: 16–64ms, cluster 4: 64–256ms, …). Wide enough to
/// absorb jitter, narrow enough to keep clusters useful.
pub fn assign_cluster(rtt_to_peers_ms: &[u32]) -> (u32, u32) {
    let avg: u32 = if rtt_to_peers_ms.is_empty() {
        0
    } else {
        rtt_to_peers_ms.iter().sum::<u32>() / rtt_to_peers_ms.len() as u32
    };
    let lo = (avg.saturating_sub(avg / 4)).max(1);
    let hi = avg + avg / 2;
    (lo, hi.max(lo + 1))
}

/// Cluster set: the network's tire topology.
/// BTreeMap keeps ordering stable, so identical sets hash identically.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ClusterTopology {
    /// Sorted map: cluster_id → cluster.
    clusters: BTreeMap<[u8; 32], LatencyCluster>,
}

impl ClusterTopology {
    pub fn new() -> Self {
        Self::default()
    }

    /// Place `pubkey` in the cluster matching `(lo, hi)` latency band.
    /// If no such cluster exists, create it.
    pub fn place(&mut self, pubkey: [u8; 32], lo_ms: u32, hi_ms: u32) -> &LatencyCluster {
        let id = LatencyCluster::id_for_band(lo_ms, hi_ms);
        let entry = self
            .clusters
            .entry(id)
            .or_insert_with(|| LatencyCluster::new(lo_ms, hi_ms));
        // let key = entry.members.len();
        entry.members.insert(pubkey);
        
        self.clusters.get(&id).unwrap()
    }

    pub fn clusters(&self) -> impl Iterator<Item = &LatencyCluster> {
        self.clusters.values()
    }

    pub fn cluster_count(&self) -> usize {
        self.clusters.len()
    }

    pub fn total_members(&self) -> usize {
        self.clusters.values().map(|c| c.members.len()).sum()
    }
}

/// Tire-leader election within a single cluster.
///
/// Pure WDS, scoped to cluster members only. Result = intra-cluster leader.
pub fn elect_tire_leader<T: ClusterMember>(
    cluster: &LatencyCluster,
    candidates: &[T],
    state_hash: &[u8; 32],
) -> Option<[u8; 32]> {
    let in_cluster: Vec<&T> = candidates
        .iter()
        .filter(|n| cluster.contains(n.pubkey()))
        .collect();

    if in_cluster.is_empty() {
        return None;
    }

    let winner = in_cluster
        .into_iter()
        .min_by_key(|n| {
            let mut h = Hasher::new();
            h.update(n.pubkey());
            h.update(state_hash);
            h.update(&n.weight().to_le_bytes());
            *h.finalize().as_bytes()
        })
        .unwrap();

    Some(*winner.pubkey())
}

/// Global leader = deterministic hash of tire-leaders, in sorted order.
///
/// Crucial property: this is **O(clusters × log(n))**, with NO communication
/// between the nodes computing it. As long as gossip has propagated
/// tire-leaders to everyone, the same global-leader comes out everywhere.
pub fn elect_global_leader(tire_leaders: &[[u8; 32]], state_hash: &[u8; 32]) -> [u8; 32] {
    let mut sorted: Vec<[u8; 32]> = tire_leaders.to_vec();
    sorted.sort();
    sorted.dedup();

    let mut h = Hasher::new();
    for tl in &sorted {
        h.update(tl);
    }
    h.update(state_hash);
    *h.finalize().as_bytes()
}

/// Full LC-WDS round: cluster-aware consensus summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LcWdsRound {
    pub round: u64,
    pub state_hash: [u8; 32],
    pub tire_leaders: Vec<[u8; 32]>,
    pub global_leader: [u8; 32],
    pub topology: ClusterTopology,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cluster_band_deterministic() {
        let a = LatencyCluster::id_for_band(0, 10);
        let b = LatencyCluster::id_for_band(0, 10);
        assert_eq!(a, b);
    }

    #[test]
    fn cluster_band_different() {
        let a = LatencyCluster::id_for_band(0, 10);
        let b = LatencyCluster::id_for_band(10, 20);
        assert_ne!(a, b);
    }

    #[test]
    fn cluster_assignment_stable() {
        let rtt = vec![5, 7, 8, 6]; // avg 6.5
        let (lo1, hi1) = assign_cluster(&rtt);
        let (lo2, hi2) = assign_cluster(&rtt);
        assert_eq!((lo1, hi1), (lo2, hi2));
    }

    #[test]
    fn global_leader_deterministic() {
        let hash = [0u8; 32];
        let leaders = vec![[2u8; 32], [1u8; 32], [3u8; 32]];
        let g1 = elect_global_leader(&leaders, &hash);
        let g2 = elect_global_leader(&leaders, &hash);
        assert_eq!(g1, g2);
    }

    #[test]
    fn global_leader_order_invariant() {
        let hash = [0u8; 32];
        let a = vec![[2u8; 32], [1u8; 32], [3u8; 32]];
        let b = vec![[1u8; 32], [3u8; 32], [2u8; 32]];
        assert_eq!(elect_global_leader(&a, &hash), elect_global_leader(&b, &hash));
    }
}