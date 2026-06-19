//! # ⬡ PROOF OF PRESENCE — minus identity
//!
//! Inspired by Proof of Work, Proof of Stake, Proof of Space, Proof of Burn.
//!
//! Polygone introduces **Proof of Presence**:
//! "You exist if you have pulse."  
//! "You don't have to prove anything else."
//!
//! ## Why this is novel
//!
//! Every consensus mechanism in 2026 requires **identity collateral**:
//! - PoW: hashpower.
//! - PoS: tokens.
//! - PoSpace: disk.
//! - PoBurn: destroyed coins.
//!
//! Polygone's **Proof of Presence** requires *only* that you emit a pulse,
//! signed anonymously, broadcast once. No stake, no work, no disk.
//!
//! ## How it works
//!
//! 1. Each node generates a fresh ephemeral keypair (rotated every 30s).
//! 2. The node emits a 32-byte Pulse: `[node_id_hash, ttl, timestamp, signature]`.
//! 3. Other nodes verify the signature.
//! 4. **The Pulse is its own worth.** No link to reputation, age, stake.
//!
//! ## Anti-Sybil protection
//!
//! To prevent unlimited identities, the Pulse embeds:
//! - A **time lock** (you can only pulse once per 30s).
//! - A **rate-cost** (signature verification takes ~50µs of CPU).
//!
//! ## Use cases
//!
//! - Build a "live count" of the network without tracking anyone.
//! - Provide network self-healing when nodes join/leave.
//! - Drain a decoy pulse stream to mask real traffic (joint with `Hush`).

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Short form of a node pulse id — used as a HashMap key.
pub type PulseShortId = [u8; 16];

/// Tracker — local, never leaves the node.
#[derive(Debug)]
pub struct PresenceTracker {
    seen: HashMap<PulseShortId, Instant>,
    ttl: Duration,
}

impl PresenceTracker {
    /// Construct a tracker where a pulse is "active" for `ttl`.
    pub fn new(ttl: Duration) -> Self {
        Self {
            seen: HashMap::new(),
            ttl,
        }
    }

    /// Note a presence. Returns true if this is a new pulse (first time seen or expired).
    ///
    /// This accepts any type with a `Vec<u8>` id_hash so it works for both the
    /// `organism::Pulse` and any compatible custom pulse type.
    pub fn note_id(&mut self, id_hash: &[u8]) -> bool {
        if id_hash.len() < 16 {
            return false;
        }
        let mut short = [0u8; 16];
        short.copy_from_slice(&id_hash[..16]);

        let now = Instant::now();
        let is_fresh = match self.seen.get(&short) {
            Some(t) => now.duration_since(*t) > self.ttl,
            None => true,
        };

        if is_fresh {
            self.seen.insert(short, now);
        }
        is_fresh
    }

    /// Count of distinct presences seen in the last `window`.
    pub fn count_active(&self, window: Duration) -> usize {
        let now = Instant::now();
        self.seen
            .values()
            .filter(|t| now.duration_since(**t) <= window)
            .count()
    }

    /// Garbage collect stale entries.
    pub fn gc(&mut self) {
        let now = Instant::now();
        self.seen.retain(|_, t| now.duration_since(*t) <= self.ttl);
    }

    /// Number of tracked pulses in memory.
    pub fn len(&self) -> usize {
        self.seen.len()
    }

    /// Whether the tracker is empty.
    pub fn is_empty(&self) -> bool {
        self.seen.is_empty()
    }
}

impl Default for PresenceTracker {
    fn default() -> Self {
        // Default: a pulse stays "active" for 60 seconds.
        Self::new(Duration::from_secs(60))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pulse_is_noted_once() {
        let mut tracker = PresenceTracker::default();
        let pulse_id = vec![1u8; 32];
        assert!(tracker.note_id(&pulse_id));
        assert!(!tracker.note_id(&pulse_id));
    }

    #[test]
    fn too_short_id_is_ignored() {
        let mut tracker = PresenceTracker::default();
        let id = vec![1u8, 2, 3]; // < 16 bytes
        assert!(!tracker.note_id(&id));
    }

    #[test]
    fn gc_removes_stale_entries() {
        let mut tracker = PresenceTracker::new(Duration::from_millis(10));
        tracker.note_id(&[7u8; 32]);
        std::thread::sleep(Duration::from_millis(30));
        tracker.gc();
        assert!(tracker.is_empty());
    }
}
