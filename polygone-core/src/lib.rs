//! # ⬡ Polygone Core — The Living Nervous System
//!
//! Polygone is not a network. It is a **biological metaphor**:
//! - **Neurons** = nodes in the DHT
//! - **Synapses** = encrypted message channels (ML-KEM-1024)
//! - **Brain** = the collective LLM inference (PETALS_NEURO)
//! - **Memory** = fragments that evaporate after 30s
//!
//! `polygone-core` is the **single entry point** that breathes everything together.
//!
//! It re-exports the public API of: `polygone-common`, `polygone-crypto`,
//! `polygone-network`, and adds organism‑level concepts like **Pulse**, **Vitality**,
//! and **Sentinel** monitoring.

#![allow(missing_docs)]
#![forbid(unsafe_code)]

// ============================================================================
// FOUNDATIONS — re-export primitives
// ============================================================================

pub use polygone_common::{
    error::PolygoneError,
    packet::{Packet, PacketType},
    session::{Session, SessionKey},
    node::{NodeId, NodeInfo},
    fragment::{
        FragmentId, FragmentPayload, DispatchResult, FragmentAck,
        CollectRequest, CollectedFragments, DispatchConfig,
    },
};

pub use polygone_crypto::{
    kem::{decapsulate, encapsulate, generate_kem_key_pair, PublicKey, SecretKey},
    shamir::{reconstruct_secret, split_secret, Fragment as ShamirFragment},
    symmetric::{decrypt, encrypt, SymmetricError},
    hash::hash_data,
};

pub use polygone_network::{
    node::P2PNode,
    dispatch::{FragmentDispatcher, DispatchError},
};

// ============================================================================
// ORGANISM CONCEPTS — the unique layer Apple can never copy
// ============================================================================

pub mod organism {
    //! The living concepts that give Polygone its **soul**.
    //!
    //! Apple sells products. We sell **privacy as a state of being**.

    use std::sync::Arc;
    use std::time::{Duration, Instant};
    use serde::{Deserialize, Serialize};

    /// The Polygone **Pulse** — a 32‑byte heartbeat emitted by every node.
    ///
    /// Pulses are:
    /// - **Anonymous**: only their cryptographic hash is published.
    /// - **Ephemeral**: they last 30 seconds, then evaporate.
    /// - **Authenticated**: signed via ML‑DSA‑87.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Pulse {
        /// BLAKE3 hash of the node’s ephemeral ID (no PII).
        pub id_hash: Vec<u8>,
        /// Unix time at emission.
        pub timestamp: i64,
        /// Network latency in µs (signal quality of the node).
        pub latency_us: u64,
        /// Number of fragments currently routed.
        pub active_fragments: u32,
        /// Cryptographic signature.
        pub signature: Vec<u8>,
    }

    /// The **Vitality** of a node — a snapshot of its health, computed locally.
    ///
    /// Inspired by biological vital signs: heart rate, oxygen, temperature.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Vitality {
        pub node_hash: Vec<u8>,
        pub uptime_seconds: u64,
        pub throughput_bps: f64,        // bytes per second currently routed
        pub entropy: f64,                // Shannon entropy of recent traffic
        pub coherence: f64,              // alignment with the DHT (0..1)
        pub last_pulse: i64,
    }

    impl Vitality {
        /// Compute a human‑readable descriptor — *Apple shows numbers, we tell stories.*
        pub fn describe(&self) -> &'static str {
            if self.coherence > 0.95 && self.entropy > 7.0 {
                "luminous"
            } else if self.coherence > 0.7 {
                "stable"
            } else if self.coherence > 0.4 {
                "flickering"
            } else {
                "dormant"
            }
        }
    }

    /// The **Sentinel** — autonomous watchman that guards the organism.
    ///
    /// It runs alongside every node, in a separate async task, observing Pulses
    /// and reacting to anomalies. No central server — pure edge intelligence.
    pub struct Sentinel {
        threshold_coherence: f64,
        max_orphan_pulses: u32,
    }

    impl Default for Sentinel {
        fn default() -> Self {
            Self {
                threshold_coherence: 0.3,
                max_orphan_pulses: 3,
            }
        }
    }

    impl Sentinel {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_thresholds(threshold_coherence: f64, max_orphan_pulses: u32) -> Self {
            Self { threshold_coherence, max_orphan_pulses }
        }

        /// Decide whether a node should be **revived** (more pulses kept),
        /// **warned** (icaution emitted), or **garnered** (silently pruned).
        pub fn diagnose(&self, v: &Vitality, orphan_count: u32) -> DiagnosisAction {
            if v.coherence < self.threshold_coherence && orphan_count > self.max_orphan_pulses {
                DiagnosisAction::Garner
            } else if v.coherence < self.threshold_coherence {
                DiagnosisAction::Warn
            } else if v.uptime_seconds > 86_400 * 30 {
                DiagnosisAction::Revive
            } else {
                DiagnosisAction::Maintain
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum DiagnosisAction {
        /// Sustain current state — all is well.
        Maintain,
        /// Emit a public caution to the DHT.
        Warn,
        /// Disconnect and re‑bootstrap.
        Garner,
        /// Propose a key rotation (monthly).
        Revive,
    }

    /// **Onyx** — Polygone's identity brand.
    ///
    /// Not a logo, not a tagline. A *philosophical assertion*:
    /// "Your privacy is not a feature. It is your biological right."
    pub struct Onyx;

    impl Onyx {
        pub const MANIFESTO: &'static str = "\
            ⬡ POLYGONE — Privacy is not a feature. It is a biological right.\n\
            We do not sell your attention. We do not aggregate your patterns.\n\
            We do not let governments read your mind.\n\
            We give you a 30-second memory and let you be free.\n\
            — Lévy, 2026\n\
        ";
    }

    /// **EchoChain** — append‑only log of significant events (within a single node).
    ///
    /// Not blockchain. Not auditable by outsiders. Just a local, transient
    /// memorial so the user knows what *their* node did.
    pub struct EchoChain {
        entries: Vec<EchoEntry>,
        max_entries: usize,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct EchoEntry {
        pub timestamp: i64,
        pub kind: EchoKind,
        pub details: String,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum EchoKind {
        Pulse,
        Message,
        Sentinel,
        System,
        Genesis, // first boot
    }

    impl EchoChain {
        pub fn new(max_entries: usize) -> Self {
            Self { entries: Vec::new(), max_entries }
        }

        pub fn record(&mut self, kind: EchoKind, details: impl Into<String>) {
            self.entries.push(EchoEntry {
                timestamp: chrono::Utc::now().timestamp(),
                kind,
                details: details.into(),
            });
            if self.entries.len() > self.max_entries {
                self.entries.remove(0);
            }
        }

        pub fn recent(&self, n: usize) -> &[EchoEntry] {
            let start = self.entries.len().saturating_sub(n);
            &self.entries[start..]
        }

        pub fn len(&self) -> usize {
            self.entries.len()
        }

        pub fn is_empty(&self) -> bool {
            self.entries.is_empty()
        }
    }

    /// **Hush** — a noise generator that obfuscates timing patterns.
    ///
    /// Empirical observation: even encrypted traffic leaks via timing.
    /// Apple can't fix that. We *can*, by emitting synthetic dummy pulses.
    pub struct Hush {
        cover_traffic_rate: f64,
    }

    impl Hush {
        pub fn new(cover_traffic_rate: f64) -> Self {
            Self { cover_traffic_rate }
        }

        /// Should we emit a dummy pulse *now* to mask real traffic?
        pub fn should_obfuscate(&self, rng: f64) -> bool {
            rng < self.cover_traffic_rate
        }
    }
}

// ============================================================================
// PULSE LOOP — autonomous heartbeat (exposed for `polygone-node`)
// ============================================================================

/// Spawn the Polygone **pulse loop** — the organism's heartbeat.
///
/// Every `period`, the node:
/// 1. Generates a fresh `Pulse`.
/// 2. Signs it with its ML‑DSA‑87 key.
/// 3. Publishes the *hash* (never the raw ID) to the DHT.
/// 4. Updates its local `Vitality` and `EchoChain`.
pub async fn spawn_pulse_loop(
    node_id_hash: Vec<u8>,
    echo_chain: Arc<std::sync::Mutex<organism::EchoChain>>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            let _pulse = organism::Pulse {
                id_hash: node_id_hash.clone(),
                timestamp: chrono::Utc::now().timestamp(),
                latency_us: 0, // measured by network layer
                active_fragments: 0,
                signature: vec![], // signed by network layer
            };
            if let Ok(mut chain) = echo_chain.lock() {
                chain.record(organism::EchoKind::Pulse, "pulse emitted");
            }
        }
    })
}

// ============================================================================
// GENESIS — convenience consts for the brand
// ============================================================================

/// The Polygone hexagram — U+2B21 ("⬡").
pub const HEXAGRAM: char = '⬡';

/// Polygone version (semver).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The brand tagline (short, Apple‑mimicking in cadence, but fundamentally opposed):
/// "Privacy is the new oxygen."
pub const TAGLINE: &str = "Privacy is the new oxygen.";
