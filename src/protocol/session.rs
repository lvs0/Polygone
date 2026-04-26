//! Full session lifecycle: initiation, establishment, transit, dissolution.

use std::time::{Duration, Instant};

use crate::{
    crypto::{KeyPair, SharedSecret, kem, shamir, symmetric::SessionKey},
    network::{EphemeralNode, NodeId, Topology, TopologyParams},
    protocol::{SessionId, TransitState},
    PolygoneError, Result,
};

/// Default maximum session lifetime.
const DEFAULT_SESSION_TTL: Duration = Duration::from_secs(30);
/// Default node TTL (nodes dissolve after the session is done).
const DEFAULT_NODE_TTL: Duration = Duration::from_secs(30);

// ── Session ───────────────────────────────────────────────────────────────────

/// A complete POLYGONE session.
///
/// ## Lifecycle
///
/// ```text
/// Session::new_initiator(peer_pk)  →  (session, ciphertext)  ← Alice
/// Session::new_responder(kp, ct)   →  session                ← Bob
/// session.establish(params)        →  topology + session key derived
/// session.send(plaintext)          →  fragment assignments
/// session.receive(fragment_bytes)  →  plaintext
/// session.dissolve()               →  all keying material zeroed
/// ```
pub struct Session {
    pub id: SessionId,
    pub state: TransitState,

    /// Our long-term keypair (used for this session only).
    keypair: KeyPair,

    /// Raw shared secret — consumed during `establish()`.
    shared_secret: Option<SharedSecret>,

    /// AES-256-GCM session key — derived from shared secret.
    session_key: Option<SessionKey>,

    /// Ephemeral topology for this session.
    pub topology: Option<Topology>,

    /// Live ephemeral relay nodes.
    nodes: Vec<EphemeralNode>,

    /// Session creation time.
    born_at: Instant,

    /// Maximum session lifetime.
    ttl: Duration,
}

impl Session {
    // ── Constructors ──────────────────────────────────────────────────────────

    /// Create a session as the **initiator** (Alice).
    ///
    /// Encapsulates toward `peer_pk`. Returns the session and the
    /// KEM ciphertext that must be sent to the responder out-of-band.
    pub fn new_initiator(peer_pk: &kem::KemPublicKey) -> Result<(Self, kem::KemCiphertext)> {
        let keypair = KeyPair::generate()?;
        let (ciphertext, shared_secret) = kem::encapsulate(peer_pk)?;

        let session = Self::build(keypair, shared_secret);
        Ok((session, ciphertext))
    }

    /// Create a session as the **responder** (Bob).
    ///
    /// Decapsulates `ciphertext` with our secret key to recover the
    /// shared secret Alice derived. From this point on, both sides
    /// are symmetric — Alice and Bob call `establish()` independently.
    pub fn new_responder(
        keypair: KeyPair,
        ciphertext: &kem::KemCiphertext,
    ) -> Result<Self> {
        let shared_secret = kem::decapsulate(&keypair.kem_sk, ciphertext)?;
        Ok(Self::build(keypair, shared_secret))
    }

    fn build(keypair: KeyPair, shared_secret: SharedSecret) -> Self {
        Self {
            id: SessionId::generate(),
            state: TransitState::Pending,
            keypair,
            shared_secret: Some(shared_secret),
            session_key: None,
            topology: None,
            nodes: vec![],
            born_at: Instant::now(),
            ttl: DEFAULT_SESSION_TTL,
        }
    }

    // ── Establish ─────────────────────────────────────────────────────────────

    /// Derive the ephemeral topology and session key from the shared secret.
    ///
    /// After this call:
    /// - The raw shared secret is consumed and zeroed (ZeroizeOnDrop).
    /// - Both topology and session key are ready.
    /// - Session moves to `Established`.
    ///
    /// Both Alice and Bob call this independently and arrive at the same
    /// topology + session key — no further communication needed.
    pub fn establish(&mut self, params: Option<TopologyParams>) -> Result<()> {
        self.assert_state(TransitState::Pending, "establish")?;
        self.check_ttl()?;

        let ss = self.shared_secret.take()
            .ok_or_else(|| PolygoneError::InvalidTransition {
                from: "no shared secret".into(),
                to: "Established".into(),
            })?;

        // BLAKE3 KDF: two independent 32-byte outputs.
        // topo_seed → topology (node IDs, edge structure)
        // key_bytes → AES-256-GCM session key
        // Domain separation ensures zero correlation between them.
        let (topo_seed, key_bytes) = ss.derive();
        // `ss` is moved here — ZeroizeOnDrop zeros the 32 raw bytes.

        let params = params.unwrap_or_default();
        let topology = Topology::derive(&topo_seed, params)?;

        let nodes = topology.nodes.iter()
            .map(|&id| EphemeralNode::new(id, DEFAULT_NODE_TTL))
            .collect();

        self.topology = Some(topology);
        self.nodes = nodes;
        self.session_key = Some(SessionKey::from_bytes(key_bytes));
        self.state = TransitState::Established;

        Ok(())
    }

    // ── Send ──────────────────────────────────────────────────────────────────

    /// Encrypt `plaintext` and distribute it as Shamir fragments.
    ///
    /// Returns `Vec<(NodeId, bytes)>` — the caller dispatches each
    /// fragment to the corresponding node over the transport layer.
    ///
    /// In v1.0 (local mode): fragments are returned in-memory.
    /// In v2.0 (P2P mode): fragments will be dispatched via libp2p/DHT.
    pub fn send(&mut self, plaintext: &[u8]) -> Result<Vec<(NodeId, Vec<u8>)>> {
        self.assert_sendable("send")?;
        self.check_ttl()?;

        let key = self.session_key.as_ref()
            .ok_or_else(|| PolygoneError::AeadError("no session key".into()))?;

        // 1. AES-256-GCM encryption with fresh nonce
        let encrypted = key.encrypt(plaintext)?;
        let payload_bytes = bincode::serialize(&encrypted)
            .map_err(|e| PolygoneError::Serialization(e.to_string()))?;

        // 2. Shamir 4-of-7 fragmentation
        let topo = self.topology.as_ref()
            .ok_or_else(|| PolygoneError::TopologyDerivation("no topology".into()))?;
        let fragments = shamir::split(&payload_bytes, topo.params.threshold, topo.params.node_count)?;

        // 3. Assign fragments to nodes
        let mut assignments = Vec::new();
        for (frag_idx, node_idx) in &topo.fragment_assignment {
            if let Some(frag) = fragments.get(*frag_idx as usize) {
                let frag_bytes = bincode::serialize(frag)
                    .map_err(|e| PolygoneError::Serialization(e.to_string()))?;
                let node_id = topo.nodes[*node_idx];
                assignments.push((node_id, frag_bytes));
            }
        }

        self.state = TransitState::InTransit { dispatched_at: Instant::now() };
        Ok(assignments)
    }

    // ── Receive ───────────────────────────────────────────────────────────────

    /// Reconstruct and decrypt from collected fragment bytes.
    ///
    /// `fragment_payloads`: raw bytes from each node. At least `threshold` required.
    pub fn receive(&mut self, fragment_payloads: Vec<Vec<u8>>) -> Result<Vec<u8>> {
        self.assert_sendable("receive")?;
        self.check_ttl()?;

        let key = self.session_key.as_ref()
            .ok_or_else(|| PolygoneError::AeadError("no session key".into()))?;

        let topo = self.topology.as_ref()
            .ok_or_else(|| PolygoneError::TopologyDerivation("no topology".into()))?;

        let threshold = topo.params.threshold;

        if fragment_payloads.len() < threshold as usize {
            return Err(PolygoneError::ReassemblyFailed {
                missing: threshold as usize - fragment_payloads.len(),
            });
        }

        // 1. Deserialize fragments
        let fragments: Vec<crate::crypto::shamir::Fragment> = fragment_payloads
            .iter()
            .map(|b| bincode::deserialize(b)
                .map_err(|e| PolygoneError::Serialization(e.to_string())))
            .collect::<Result<_>>()?;

        // 2. Shamir reconstruction
        let payload_bytes = shamir::reconstruct(&fragments, threshold)?;

        // 3. Deserialize encrypted payload
        let encrypted: crate::crypto::symmetric::EncryptedPayload =
            bincode::deserialize(&payload_bytes)
                .map_err(|e| PolygoneError::Serialization(e.to_string()))?;

        // 4. AES-256-GCM decryption
        let plaintext = key.decrypt(&encrypted)?;

        self.state = TransitState::Completed;
        Ok(plaintext)
    }

    // ── Dissolve ──────────────────────────────────────────────────────────────

    /// Dissolve the session: zero all nodes and keying material.
    ///
    /// After this call the session is permanently unusable.
    /// Call this on all exit paths (success, error, timeout).
    pub fn dissolve(&mut self) {
        for node in &mut self.nodes {
            node.dissolve();
        }
        self.nodes.clear();
        // ZeroizeOnDrop handles the actual memory zeroing
        self.session_key = None;
        self.shared_secret = None;
        self.topology = None;
        self.state = TransitState::Dissolved;
    }

    // ── Introspection ─────────────────────────────────────────────────────────

    /// Elapsed time since session creation.
    pub fn age(&self) -> Duration { self.born_at.elapsed() }

    /// Time remaining before the session expires.
    pub fn time_remaining(&self) -> Duration {
        self.ttl.saturating_sub(self.born_at.elapsed())
    }

    /// Number of active (non-dissolved) nodes.
    pub fn active_node_count(&self) -> usize {
        self.nodes.iter().filter(|n| !n.is_expired()).count()
    }

    pub fn kem_pk(&self) -> &kem::KemPublicKey {
        &self.keypair.kem_pk
    }

    // ── Private helpers ───────────────────────────────────────────────────────

    fn check_ttl(&self) -> Result<()> {
        if self.born_at.elapsed() >= self.ttl {
            Err(PolygoneError::SessionExpired)
        } else {
            Ok(())
        }
    }

    fn assert_state(&self, expected: TransitState, op: &str) -> Result<()> {
        if self.state != expected {
            return Err(PolygoneError::InvalidTransition {
                from: self.state.to_string(),
                to: op.into(),
            });
        }
        Ok(())
    }

    fn assert_sendable(&self, op: &str) -> Result<()> {
        if !matches!(
            self.state,
            TransitState::Established | TransitState::InTransit { .. }
        ) {
            return Err(PolygoneError::InvalidTransition {
                from: self.state.to_string(),
                to: op.into(),
            });
        }
        Ok(())
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        if self.state != TransitState::Dissolved {
            self.dissolve();
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::KeyPair;

    fn alice_bob() -> (Session, Session) {
        let bob_kp = KeyPair::generate().unwrap();
        let bob_pk = bob_kp.kem_pk.clone();
        let (mut alice, ct) = Session::new_initiator(&bob_pk).unwrap();
        let mut bob = Session::new_responder(bob_kp, &ct).unwrap();
        alice.establish(None).unwrap();
        bob.establish(None).unwrap();
        (alice, bob)
    }

    #[test]
    fn full_round_trip() {
        let (mut alice, mut bob) = alice_bob();
        let msg = b"L'information n'existe pas. Elle traverse.";
        let assignments = alice.send(msg).unwrap();
        let frags: Vec<_> = assignments.into_iter().map(|(_, b)| b).collect();
        let recovered = bob.receive(frags).unwrap();
        assert_eq!(recovered.as_slice(), msg.as_slice());
    }

    #[test]
    fn insufficient_fragments_fail() {
        let (mut alice, mut bob) = alice_bob();
        let assignments = alice.send(b"secret").unwrap();
        let frags: Vec<_> = assignments.into_iter().take(2).map(|(_, b)| b).collect();
        assert!(bob.receive(frags).is_err());
    }

    #[test]
    fn session_dissolves_on_drop() {
        let bob_kp = KeyPair::generate().unwrap();
        let bob_pk = bob_kp.kem_pk.clone();
        let (mut alice, ct) = Session::new_initiator(&bob_pk).unwrap();
        let mut bob = Session::new_responder(bob_kp, &ct).unwrap();
        alice.establish(None).unwrap();
        bob.establish(None).unwrap();

        let assignments = alice.send(b"test").unwrap();
        let frags: Vec<_> = assignments.into_iter().map(|(_, b)| b).collect();
        bob.receive(frags).unwrap();

        alice.dissolve();
        bob.dissolve();
        assert_eq!(alice.state, TransitState::Dissolved);
        assert_eq!(bob.state, TransitState::Dissolved);
    }

    #[test]
    fn both_sides_derive_same_topology() {
        let bob_kp = KeyPair::generate().unwrap();
        let bob_pk = bob_kp.kem_pk.clone();
        let (mut alice, ct) = Session::new_initiator(&bob_pk).unwrap();
        let mut bob = Session::new_responder(bob_kp, &ct).unwrap();
        alice.establish(None).unwrap();
        bob.establish(None).unwrap();

        let alice_nodes: Vec<_> = alice.topology.as_ref().unwrap().nodes.iter().map(|n| n.0).collect();
        let bob_nodes: Vec<_> = bob.topology.as_ref().unwrap().nodes.iter().map(|n| n.0).collect();
        assert_eq!(alice_nodes, bob_nodes, "Both sides must derive identical topology");
    }
}
