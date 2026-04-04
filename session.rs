//! Full session lifecycle implementation.

use std::time::{Duration, Instant};
use crate::{
    crypto::{KeyPair, SharedSecret, kem, shamir, symmetric::SessionKey},
    network::{Topology, TopologyParams, EphemeralNode, NodeId},
    protocol::{SessionId, TransitState},
    PolygoneError, Result,
};

/// Default time a session is allowed to live before forced dissolution.
const DEFAULT_SESSION_TTL: Duration = Duration::from_secs(30);

/// Default time each ephemeral node is allowed to live.
const DEFAULT_NODE_TTL: Duration = Duration::from_millis(500);

/// A complete POLYGONE session.
///
/// # Lifecycle
///
/// ```text
/// Session::new_initiator()  ← Alice creates, gets ciphertext to send Bob
/// Session::new_responder()  ← Bob creates from ciphertext
/// session.establish()       ← Derive topology + session key from shared secret
/// session.send(plaintext)   ← Encrypt, fragment, dispatch
/// session.dissolve()        ← Destroy all keying material
/// ```
pub struct Session {
    pub id: SessionId,
    pub state: TransitState,

    /// Our own key pair for this session.
    keypair: KeyPair,

    /// Shared secret — present only after key exchange, before dissolution.
    shared_secret: Option<SharedSecret>,

    /// Symmetric session key — derived from shared secret.
    session_key: Option<SessionKey>,

    /// Ephemeral network topology for this session.
    topology: Option<Topology>,

    /// Live ephemeral nodes for this session.
    nodes: Vec<EphemeralNode>,

    /// Session creation timestamp for TTL enforcement.
    born_at: Instant,

    /// Maximum session lifetime.
    ttl: Duration,
}

impl Session {
    // ── Constructors ─────────────────────────────────────────────────────────

    /// Create a new session as the **initiator** (Alice).
    ///
    /// Returns the session and the KEM ciphertext that must be sent to Bob
    /// out-of-band. The ciphertext is the *only* thing that crosses the wire
    /// before the ephemeral network exists.
    pub fn new_initiator(peer_pk: &kem::KemPublicKey) -> Result<(Self, kem::KemCiphertext)> {
        let keypair = KeyPair::generate()?;
        let (ciphertext, shared_secret) = kem::encapsulate(peer_pk)?;
        let session = Self {
            id: SessionId::generate(),
            state: TransitState::Pending,
            keypair,
            shared_secret: Some(shared_secret),
            session_key: None,
            topology: None,
            nodes: vec![],
            born_at: Instant::now(),
            ttl: DEFAULT_SESSION_TTL,
        };
        Ok((session, ciphertext))
    }

    /// Create a new session as the **responder** (Bob).
    ///
    /// `ciphertext` is what Alice sent. Bob decapsulates it to recover
    /// the same shared secret.
    pub fn new_responder(
        keypair: KeyPair,
        ciphertext: &kem::KemCiphertext,
    ) -> Result<Self> {
        let shared_secret = kem::decapsulate(&keypair.kem_sk, ciphertext)?;
        Ok(Self {
            id: SessionId::generate(),
            state: TransitState::Pending,
            keypair,
            shared_secret: Some(shared_secret),
            session_key: None,
            topology: None,
            nodes: vec![],
            born_at: Instant::now(),
            ttl: DEFAULT_SESSION_TTL,
        })
    }

    // ── Establish ─────────────────────────────────────────────────────────────

    /// Derive topology and session key from the shared secret.
    ///
    /// After this call, the raw shared secret is consumed and zeroed.
    /// The session moves to `Established`.
    pub fn establish(&mut self, params: Option<TopologyParams>) -> Result<()> {
        self.assert_state_is(TransitState::Pending, "Established")?;

        let ss = self.shared_secret.take()
            .ok_or_else(|| PolygoneError::InvalidTransition {
                from: "no shared secret".into(),
                to: "Established".into(),
            })?;

        // Derive topology seed and session key — two independent BLAKE3 outputs.
        // topo_seed → topology structure (node IDs, edges)  — never touches the cipher.
        // key_bytes → AES-256-GCM session key               — never touches topology.
        // Domain separation is enforced in SharedSecret::derive() via distinct labels.
        let (topo_seed, key_bytes) = ss.derive();
        // ss is moved here; ZeroizeOnDrop zeroes the 32 raw bytes on drop.

        // Topology is derived from topo_seed — NOT key_bytes.
        let topology = Topology::derive(&topo_seed, params.unwrap_or_default())?;

        // Build ephemeral nodes
        let nodes = topology.nodes.iter()
            .map(|&id| EphemeralNode::new(id, DEFAULT_NODE_TTL))
            .collect();

        self.topology    = Some(topology);
        self.nodes       = nodes;
        self.session_key = Some(SessionKey::from_bytes(key_bytes));
        self.state       = TransitState::Established;

        Ok(())
    }

    // ── Send ──────────────────────────────────────────────────────────────────

    /// Encrypt `plaintext` and distribute it as Shamir fragments across
    /// the ephemeral network nodes.
    ///
    /// Returns the per-node fragment assignments so the caller can
    /// dispatch them over the actual network transport.
    pub fn send(&mut self, plaintext: &[u8]) -> Result<Vec<(NodeId, Vec<u8>)>> {
        self.assert_established("send")?;

        let key = self.session_key.as_ref()
            .ok_or_else(|| PolygoneError::AeadError("no session key".into()))?;

        // 1. Encrypt the plaintext
        let encrypted = key.encrypt(plaintext)?;
        let payload_bytes = bincode::serialize(&encrypted)
            .map_err(|e| PolygoneError::Serialization(e.to_string()))?;

        // 2. Fragment the encrypted payload via Shamir
        let topo = self.topology.as_ref()
            .ok_or_else(|| PolygoneError::TopologyDerivation("no topology".into()))?;

        let n = topo.params.node_count;
        let t = topo.params.threshold;
        let fragments = shamir::split(&payload_bytes, t, n)?;

        // 3. Assign fragments to nodes
        let mut assignments = vec![];
        for (frag_id_num, node_idx) in &topo.fragment_assignment {
            if let Some(frag) = fragments.iter().find(|f| f.id.0 == *frag_id_num) {
                let node_id = topo.nodes[*node_idx];
                let frag_bytes = bincode::serialize(frag)
                    .map_err(|e| PolygoneError::Serialization(e.to_string()))?;
                assignments.push((node_id, frag_bytes));
            }
        }

        self.state = TransitState::InTransit { dispatched_at: Instant::now() };
        Ok(assignments)
    }

    // ── Receive ───────────────────────────────────────────────────────────────

    /// Reconstruct the original plaintext from collected fragment bytes.
    ///
    /// `fragment_payloads` is the raw bytes received from each node.
    /// At least `threshold` must be present.
    pub fn receive(&mut self, fragment_payloads: Vec<Vec<u8>>) -> Result<Vec<u8>> {
        self.assert_established("receive")?;

        let key = self.session_key.as_ref()
            .ok_or_else(|| PolygoneError::AeadError("no session key".into()))?;

        let topo = self.topology.as_ref()
            .ok_or_else(|| PolygoneError::TopologyDerivation("no topology".into()))?;

        let threshold = topo.params.threshold;

        // 1. Deserialize fragments
        let fragments: Vec<crate::crypto::shamir::Fragment> = fragment_payloads
            .iter()
            .map(|b| bincode::deserialize(b)
                .map_err(|e| PolygoneError::Serialization(e.to_string())))
            .collect::<Result<Vec<_>>>()?;

        // 2. Reconstruct encrypted payload
        let payload_bytes = shamir::reconstruct(&fragments, threshold)
            .map_err(|_| PolygoneError::ReassemblyFailed {
                missing: threshold as usize - fragments.len().min(threshold as usize),
            })?;

        // 3. Deserialize and decrypt
        let encrypted: crate::crypto::symmetric::EncryptedPayload =
            bincode::deserialize(&payload_bytes)
                .map_err(|e| PolygoneError::Serialization(e.to_string()))?;

        let plaintext = key.decrypt(&encrypted)?;
        self.state = TransitState::Completed;
        Ok(plaintext)
    }

    // ── Dissolve ──────────────────────────────────────────────────────────────

    /// Dissolve the session: zero all nodes, drop all keying material.
    ///
    /// After this call, the session is unusable.
    /// This MUST be called after `Completed` (or on error paths).
    pub fn dissolve(&mut self) {
        // Dissolve all ephemeral nodes (fragment data zeroed in Drop impl)
        for node in &mut self.nodes { node.dissolve(); }
        self.nodes.clear();

        // session_key and shared_secret are ZeroizeOnDrop —
        // dropping them zeroes the memory automatically.
        self.session_key   = None;
        self.shared_secret = None;
        self.topology      = None;
        self.state         = TransitState::Dissolved;
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    fn assert_state_is(&self, expected: TransitState, target: &str) -> Result<()> {
        if self.state != expected {
            return Err(PolygoneError::InvalidTransition {
                from: self.state.to_string(),
                to: target.into(),
            });
        }
        Ok(())
    }

    fn assert_established(&self, op: &str) -> Result<()> {
        if !matches!(self.state, TransitState::Established | TransitState::InTransit{..}) {
            return Err(PolygoneError::InvalidTransition {
                from: self.state.to_string(),
                to: format!("allow '{op}'"),
            });
        }
        Ok(())
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        // Ensure dissolution on any drop path
        if self.state != TransitState::Dissolved {
            self.dissolve();
        }
    }
}

// ── Integration tests ─────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn alice_bob_session() -> (Session, Session) {
        // 1. Bob generates his long-term keypair and publishes kem_pk
        let bob_kp = KeyPair::generate().unwrap();
        let bob_pk = bob_kp.kem_pk.clone();

        // 2. Alice initiates
        let (mut alice, ciphertext) = Session::new_initiator(&bob_pk).unwrap();

        // 3. Bob responds
        let mut bob = Session::new_responder(bob_kp, &ciphertext).unwrap();

        // 4. Both establish (with default topology)
        alice.establish(None).unwrap();
        bob.establish(None).unwrap();

        (alice, bob)
    }

    #[test]
    fn full_round_trip() {
        let (mut alice, mut bob) = alice_bob_session();
        let message = b"L'information n'existe pas. Elle traverse.";

        // Alice sends
        let assignments = alice.send(message).unwrap();

        // Extract just the fragment bytes (in a real network these go over the wire)
        let fragment_payloads: Vec<Vec<u8>> = assignments.into_iter().map(|(_, b)| b).collect();

        // Bob receives
        let recovered = bob.receive(fragment_payloads).unwrap();
        assert_eq!(recovered, message);
    }

    #[test]
    fn session_dissolves_after_completion() {
        let (mut alice, mut bob) = alice_bob_session();
        let assignments = alice.send(b"test").unwrap();
        let frags: Vec<_> = assignments.into_iter().map(|(_, b)| b).collect();
        bob.receive(frags).unwrap();

        alice.dissolve();
        bob.dissolve();

        assert_eq!(alice.state, TransitState::Dissolved);
        assert_eq!(bob.state,   TransitState::Dissolved);
    }

    #[test]
    fn insufficient_fragments_fails_gracefully() {
        let (mut alice, mut bob) = alice_bob_session();
        let assignments = alice.send(b"secret message").unwrap();
        // Only send 2 out of 7 fragments (threshold is 4)
        let frags: Vec<_> = assignments.into_iter().take(2).map(|(_, b)| b).collect();
        assert!(bob.receive(frags).is_err());
    }
}
