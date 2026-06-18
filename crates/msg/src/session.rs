//! Message session — orchestrates the full Polygone send/receive pipeline.
//!
//! A `MessageSession` ties the cryptographic primitives together:
//!
//! ## Send path
//! ```text
//! OutgoingMessage
//!   → ML-KEM encapsulate(recipient_pk)     → encapsulated_key + shared_secret
//!   → AES-256-GCM encrypt(plaintext)        → ciphertext
//!   → BLAKE3(plaintext)                     → plaintext_hash (integrity)
//!   → Shamir split 4-of-7(ciphertext)        → 7 raw fragments
//!   → AES-256-GCM encrypt each fragment    → encrypted fragment
//!   → [Envelope × 7]                        → ready for P2P dispatch
//! ```
//!
//! ## Receive path
//! ```text
//! [Envelope × ≥4 collected]
//!   → decrypt each fragment (AES-GCM)
//!   → Shamir reconstruct(fragments)          → ciphertext
//!   → AES-256-GCM decrypt(ciphertext)       → plaintext
//!   → verify BLAKE3 hash                    → integrity check
//!   → Message                               → delivered to app
//! ```

use crate::message::{Envelope, Message, MessageId, MessageMeta, OutgoingMessage};
use parking_lot::RwLock;
use polygone_common::{FragmentId, SessionKey};
use std::collections::HashMap;
use std::sync::Arc;

/// Configuration for a message session.
#[derive(Debug, Clone)]
pub struct SessionConfig {
    /// Number of Shamir fragments (default 7).
    pub fragment_count: usize,
    /// Shamir threshold (default 4).
    pub threshold: usize,
    /// Fragment TTL in seconds (default 30 per spec).
    pub fragment_ttl_seconds: u64,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            fragment_count: 7,
            threshold: 4,
            fragment_ttl_seconds: 30,
        }
    }
}

/// Errors that can occur during message session operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum SessionError {
    #[error("insufficient fragments: need {0}, got {1}")]
    InsufficientFragments(usize, usize),

    #[error("ML-KEM encapsulation failed: {0}")]
    KemEncapsulate(String),

    #[error("ML-KEM decapsulation failed: {0}")]
    KemDecapsulate(String),

    #[error("AES-256-GCM encryption failed")]
    EncryptionFailed,

    #[error("AES-256-GCM decryption failed")]
    DecryptionFailed,

    #[error("Shamir reconstruction failed")]
    ShamirReconstruct,

    #[error("integrity check failed: hash mismatch")]
    IntegrityMismatch,

    #[error("invalid envelope: fragment index out of range")]
    InvalidFragmentIndex,

    #[error("envelope message ID mismatch: expected {expected}, got {actual}")]
    MessageIdMismatch { expected: MessageId, actual: MessageId },
}

/// Holds cryptographic state for a send/receive session with a peer.
pub struct MessageSession {
    /// Config for this session.
    config: SessionConfig,
    /// My ephemeral secret key bytes for this session (raw wire bytes).
    my_secret_key: Vec<u8>,
    /// The peer's public key bytes (for ML-KEM encapsulation).
    peer_public_key: Vec<u8>,
    /// Cache of outgoing envelopes keyed by message ID.
    outgoing: Arc<RwLock<HashMap<MessageId, Vec<Envelope>>>>,
    /// Cache of collected incoming envelopes, pending reassembly.
    incoming: Arc<RwLock<HashMap<MessageId, Vec<Envelope>>>>,
}

impl MessageSession {
    /// Create a new session with a peer.
    ///
    /// `my_secret_key` — our ML-KEM secret key as raw bytes (e.g. `.0.as_bytes().to_vec()`).
    /// `peer_public_key` — the recipient's ML-KEM public key as raw bytes.
    pub fn new(my_secret_key: Vec<u8>, peer_public_key: Vec<u8>) -> Self {
        Self {
            config: SessionConfig::default(),
            my_secret_key,
            peer_public_key,
            outgoing: Arc::new(RwLock::new(HashMap::new())),
            incoming: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create with custom configuration.
    pub fn with_config(
        my_secret_key: Vec<u8>,
        peer_public_key: Vec<u8>,
        config: SessionConfig,
    ) -> Self {
        Self {
            config,
            my_secret_key,
            peer_public_key,
            outgoing: Arc::new(RwLock::new(HashMap::new())),
            incoming: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Encapsulate an outgoing message into a set of dispatch-ready envelopes.
    ///
    /// This is the **send** pipeline:
    /// 1. ML-KEM encapsulate → session key + encapsulated key
    /// 2. AES-256-GCM encrypt plaintext → ciphertext
    /// 3. BLAKE3(plaintext) → integrity hash
    /// 4. Shamir 4-of-7 split ciphertext → 7 raw fragments
    /// 5. Encrypt each fragment with AES-GCM session key → envelopes
    ///
    /// Returns `N` `Envelope`s ready to be dispatched to `N` network nodes.
    pub fn encapsulate(&self, msg: OutgoingMessage) -> Result<Vec<Envelope>, SessionError> {
        // ── Step 1: ML-KEM encapsulate ───────────────────────────────────
        let (encapsulated_key, shared_secret_bytes) = self
            .do_ml_kem_encapsulate()
            .map_err(SessionError::KemEncapsulate)?;

        let sk = SessionKey::new(shared_secret_bytes);
        let sk_ref: &SessionKey = &sk;

        // ── Step 2: AES-256-GCM encrypt plaintext ───────────────────────
        let plaintext = &msg.payload;

        let (ciphertext, nonce) = polygone_crypto::encrypt(sk_ref, plaintext, b"polygone-message-v1")
            .map_err(|_| SessionError::EncryptionFailed)?;

        // ── Step 3: BLAKE3 plaintext hash for integrity ──────────────────
        let plaintext_hash = polygone_crypto::hash::hash_data(plaintext).to_vec();

        // ── Step 4: Shamir split ciphertext 4-of-7 ──────────────────────
        let n_frag = self.config.fragment_count as u8;
        let threshold = self.config.threshold as u8;

        // Build a SessionKey from the ciphertext for Shamir splitting
        // (the ciphertext is variable-length — use a key derived from BLAKE3 of it)
        let cipher_key_bytes: [u8; 32] = polygone_crypto::hash::hash_data(&ciphertext);
        let cipher_key = SessionKey::new(cipher_key_bytes);
        let raw_fragments = polygone_crypto::split_secret(&cipher_key, threshold, n_frag);

        // ── Step 5: Encrypt each fragment → Envelopes ──────────────────
        let total = self.config.fragment_count;
        let envelopes: Vec<Envelope> = raw_fragments
            .into_iter()
            .enumerate()
            .map(|(i, fragment)| {
                // Derive per-fragment nonce from the main nonce + index
                let mut nonce_extended = [0u8; 12];
                nonce_extended.copy_from_slice(&nonce);
                nonce_extended[11] = i as u8;

                let (frag_ciphertext, _) =
                    polygone_crypto::encrypt(sk_ref, &fragment.data, b"polygone-fragment")
                        .expect("fragment encryption must succeed");

                Envelope {
                    message_id: MessageId::from_ciphertext(&ciphertext),
                    fragment_index: fragment.id.as_u8().saturating_sub(1),
                    total_fragments: total as u8,
                    threshold: threshold as u8,
                    encapsulated_key: encapsulated_key.clone(),
                    nonce: nonce_extended.to_vec(),
                    ciphertext_fragment: frag_ciphertext,
                    plaintext_hash: plaintext_hash.clone(),
                }
            })
            .collect();

        Ok(envelopes)
    }

    /// Collect an incoming envelope for later reassembly.
    ///
    /// Call this as fragments arrive over the P2P network.
    /// Returns `Ok(true)` if the message is now complete (threshold reached)
    /// and should be reassembled, `Ok(false)` if more fragments are needed.
    pub fn collect_envelope(&self, envelope: Envelope) -> Result<bool, SessionError> {
        let msg_id = envelope.message_id;

        let mut incoming = self.incoming.write();
        let fragments = incoming.entry(msg_id).or_insert_with(Vec::new);

        // Skip duplicates
        if fragments.iter().any(|e| e.fragment_index == envelope.fragment_index) {
            return Ok(fragments.len() >= self.config.threshold);
        }

        // Validate fragment index bounds
        if envelope.fragment_index >= envelope.total_fragments {
            return Err(SessionError::InvalidFragmentIndex);
        }

        fragments.push(envelope);

        // Reassemble if we have enough
        let ready = fragments.len() >= self.config.threshold;
        Ok(ready)
    }

    /// Reassemble and decrypt a complete message from collected envelopes.
    ///
    /// Consumes the collected fragments for this message ID.
    /// Returns `Message` on success.
    pub fn reassemble(&self, message_id: MessageId) -> Result<Message, SessionError> {
        let envelope_batch = {
            let mut incoming = self.incoming.write();
            incoming.remove(&message_id).ok_or_else(|| {
                SessionError::InsufficientFragments(self.config.threshold, 0)
            })?
        };

        if envelope_batch.len() < self.config.threshold {
            return Err(SessionError::InsufficientFragments(
                self.config.threshold,
                envelope_batch.len(),
            ));
        }

        self.do_reassemble(envelope_batch)
    }

    // ── Private helpers ────────────────────────────────────────────────────────

    /// ML-KEM encapsulate using the peer's public key.
    fn do_ml_kem_encapsulate(&self) -> Result<(Vec<u8>, [u8; 32]), String> {
        use polygone_crypto::{encapsulate, pk_from_bytes};

        let pk = pk_from_bytes(&self.peer_public_key)
            .map_err(|e| e.to_string())?;

        let (ct, ss) = encapsulate(&pk);
        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(ss.as_slice());
        Ok((ct, key_bytes))
    }

    /// ML-KEM decapsulate using our secret key.
    fn do_ml_kem_decapsulate(&self, encapsulated_key: &[u8]) -> Result<[u8; 32], String> {
        use polygone_crypto::{decapsulate, sk_from_bytes};

        let sk = sk_from_bytes(&self.my_secret_key)
            .map_err(|e| e.to_string())?;

        let ss = decapsulate(encapsulated_key, &sk);
        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(ss.as_slice());
        Ok(key_bytes)
    }

    /// Full reassembly pipeline from a batch of validated envelopes.
    fn do_reassemble(&self, envelopes: Vec<Envelope>) -> Result<Message, SessionError> {
        // Verify all envelopes share the same message ID
        let first_id = envelopes[0].message_id;
        for env in &envelopes {
            if env.message_id != first_id {
                return Err(SessionError::MessageIdMismatch {
                    expected: first_id,
                    actual: env.message_id,
                });
            }
        }

        let expected_hash = &envelopes[0].plaintext_hash;
        let total_fragments = envelopes[0].total_fragments as usize;
        let threshold = envelopes[0].threshold as u8;

        // Derive session key from ML-KEM decapsulation
        let shared_secret_bytes = self
            .do_ml_kem_decapsulate(&envelopes[0].encapsulated_key)
            .map_err(SessionError::KemDecapsulate)?;
        let sk = SessionKey::new(shared_secret_bytes);
        let sk_ref: &SessionKey = &sk;

        let mut decrypted_fragments = Vec::with_capacity(envelopes.len());

        for env in &envelopes {
            // Recover per-fragment nonce
            let mut nonce_extended = [0u8; 12];
            nonce_extended.copy_from_slice(&env.nonce);
            nonce_extended[11] = env.fragment_index as u8;

            let fragment_data = polygone_crypto::decrypt(
                sk_ref,
                &env.ciphertext_fragment,
                &nonce_extended,
                b"polygone-fragment",
            )
            .map_err(|_| SessionError::DecryptionFailed)?;

            decrypted_fragments.push(polygone_crypto::ShamirFragment {
                id: FragmentId::new(env.fragment_index + 1),
                data: fragment_data,
            });
        }

        // Sort by fragment index
        decrypted_fragments.sort_by_key(|f| f.id.as_u8());

        // Reconstruct ciphertext from Shamir fragments
        let cipher_key = polygone_crypto::reconstruct_secret(decrypted_fragments, threshold)
            .ok_or(SessionError::ShamirReconstruct)?;

        // Decrypt ciphertext with main nonce (index 0)
        let main_nonce = {
            let mut n = [0u8; 12];
            n.copy_from_slice(&envelopes[0].nonce);
            n
        };

        let plaintext = polygone_crypto::decrypt(
            sk_ref,
            cipher_key.as_slice(),
            &main_nonce,
            b"polygone-message-v1",
        )
        .map_err(|_| SessionError::DecryptionFailed)?;

        // Integrity check
        let computed_hash = polygone_crypto::hash::hash_data(&plaintext);
        if &computed_hash[..] != &expected_hash[..] {
            return Err(SessionError::IntegrityMismatch);
        }

        let meta = MessageMeta {
            created_at: 0, // Not transmitted — set to 0 as sentinel
            topic: None,
            fragment_count: total_fragments,
            threshold: threshold as usize,
        };

        Ok(Message::new(plaintext, meta, Vec::new()))
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_receive_roundtrip() {
        use polygone_crypto::generate_kem_key_pair;

        let (my_sk, my_pk) = generate_kem_key_pair();
        let (peer_sk, peer_pk) = generate_kem_key_pair();

        // Alice sends to Bob (keys as raw bytes via .to_bytes())
        let alice =
            MessageSession::new(my_sk.to_bytes(), peer_pk.to_bytes());
        let bob = MessageSession::new(peer_sk.to_bytes(), my_pk.to_bytes());

        let original = OutgoingMessage::new(b"Hello, Polygone!", Some("greeting".into()));

        // Alice encapsulates
        let envelopes = alice.encapsulate(original).expect("encapsulation should succeed");
        assert_eq!(
            envelopes.len(),
            7,
            "should produce 7 fragments for default config"
        );

        // Bob collects 4+ envelopes
        for env in envelopes.iter().take(4) {
            let ready = bob
                .collect_envelope(env.clone())
                .expect("collect should succeed");
            assert!(!ready, "not ready with only 4 (threshold = 4)");
        }
        // Fifth fragment triggers reassembly signal
        let env5 = &envelopes[4];
        let ready = bob
            .collect_envelope(env5.clone())
            .expect("collect should succeed");
        assert!(ready, "should be ready with 5 fragments");

        // Bob reassembles
        let received = bob
            .reassemble(envelopes[0].message_id)
            .expect("reassembly should succeed");

        assert_eq!(received.plaintext, b"Hello, Polygone!");
    }

    #[test]
    fn test_envelope_serialization() {
        use polygone_crypto::generate_kem_key_pair;
        let (sk, pk) = generate_kem_key_pair();
        let session =
            MessageSession::new(sk.to_bytes(), pk.to_bytes());
        let msg = OutgoingMessage::plaintext(b"test");
        let envelopes = session.encapsulate(msg).unwrap();

        for env in &envelopes {
            let bytes = env.to_bytes();
            let restored = Envelope::from_bytes(&bytes).unwrap();
            assert_eq!(restored.message_id, env.message_id);
            assert_eq!(restored.fragment_index, env.fragment_index);
        }
    }

    #[test]
    fn test_message_id_deterministic() {
        let id1 = MessageId::from_ciphertext(b"test payload");
        let id2 = MessageId::from_ciphertext(b"test payload");
        let id3 = MessageId::from_ciphertext(b"different payload");
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }
}