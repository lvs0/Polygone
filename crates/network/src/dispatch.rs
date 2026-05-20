//! Fragment dispatch system for Polygone's P2P network.
//!
//! This module automates the dispatch of Shamir fragments across the
//! ephemeral topology. The flow is:
//!
//! 1. **Split** — The session key is split into N Shamir fragments.
//! 2. **Encrypt** — Each fragment is encrypted with AES-256-GCM under the session key.
//! 3. **Dispatch** — Fragments are routed to their assigned nodes via libp2p
//!    request-response protocol.
//! 4. **Collect** — On the receiving side, fragments are gathered and reconstructed.
//!
//! # Architecture
//!
//! ```text
//! Session.send(plaintext)
//!   │
//!   ├─ 1. AES-256-GCM encrypt plaintext → ciphertext
//!   ├─ 2. Shamir split ciphertext → N fragments
//!   ├─ 3. Encrypt each fragment → FragmentPayload
//!   │
//!   ▼
//! FragmentDispatcher
//!   │
//!   ├─ dispatch() → send each FragmentPayload to its assigned node
//!   ├─ retry failed dispatches (configurable)
//!   └─ collect acks
//!
//! On the receiving side:
//!   │
//!   ├─ receive FragmentPayload
//!   ├─ decrypt fragment
//!   └─ store for reconstruction
//! ```
//!
//! # Wire Protocol
//!
//! Fragments are sent via libp2p request-response:
//! - Request: `PolygoneRequest::DriveStore` with fragment metadata + encrypted data
//! - Response: `PolygoneResponse::DriveStore { success: true }` as acknowledgment

use polygone_common::{
    FragmentId, FragmentPayload, DispatchConfig,
    FragmentAck, NodeId, SessionKey,
};
use polygone_crypto::{encrypt, decrypt, reconstruct_secret, ShamirFragment};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// The fragment dispatcher orchestrates sending and collecting Shamir fragments
/// over the P2P network.
pub struct FragmentDispatcher {
    /// Configuration for dispatch behavior.
    config: DispatchConfig,
    /// Pending dispatches: fragment_id → (attempts, last_attempt, payload).
    pending: HashMap<FragmentId, (u8, Instant, FragmentPayload)>,
    /// Collected acks from peers.
    acks: Vec<FragmentAck>,
    /// Collected (decrypted) fragments for reconstruction.
    collected: HashMap<FragmentId, Vec<u8>>,
    /// The session key for encrypting/decrypting fragments.
    session_key: SessionKey,
    /// Session ID for tracking.
    session_id: [u8; 32],
}

impl FragmentDispatcher {
    /// Create a new dispatcher for a given session.
    pub fn new(session_id: [u8; 32], session_key: SessionKey, config: DispatchConfig) -> Self {
        Self {
            config,
            pending: HashMap::new(),
            acks: Vec::new(),
            collected: HashMap::new(),
            session_key,
            session_id,
        }
    }

    /// Prepare fragment payloads from raw Shamir shares.
    ///
    /// This takes the output of `split_secret` and:
    /// 1. Encrypts each fragment with AES-256-GCM
    /// 2. Wraps it in a `FragmentPayload` with destination routing
    ///
    /// Returns the payloads ready for network dispatch.
    pub fn prepare_payloads(
        &self,
        fragments: Vec<(FragmentId, Vec<u8>, NodeId)>,
    ) -> Result<Vec<FragmentPayload>, DispatchError> {
        let mut payloads = Vec::with_capacity(fragments.len());

        for (id, data, destination) in fragments {
            let aad = format!("polygone-fragment-{}", id.as_u8()).as_bytes().to_vec();
            let (ciphertext, nonce) = encrypt(&self.session_key, &data, &aad)
                .map_err(|_| DispatchError::EncryptionFailed { fragment_id: id })?;

            payloads.push(FragmentPayload {
                id,
                destination,
                ciphertext,
                nonce,
            });
        }

        Ok(payloads)
    }

    /// Enqueue payloads for dispatch.
    ///
    /// This adds them to the pending queue. Call `next_dispatch_batch` to
    /// get payloads that need to be sent over the network.
    pub fn enqueue(&mut self, payloads: Vec<FragmentPayload>) {
        let now = Instant::now();
        for payload in payloads {
            let id = payload.id;
            self.pending.insert(id, (0, now, payload));
        }
    }

    /// Get the next batch of payloads that need to be dispatched or retried.
    ///
    /// Returns payloads that haven't been sent yet or have failed and
    /// are eligible for retry.
    pub fn next_dispatch_batch(&mut self) -> Vec<FragmentPayload> {
        let now = Instant::now();
        let timeout = Duration::from_millis(self.config.dispatch_timeout_ms);
        let max_retries = self.config.max_retries;

        let mut batch = Vec::new();

        for (_id, (attempts, last_attempt, payload)) in self.pending.iter_mut() {
            let should_dispatch = *attempts == 0
                || (*attempts < max_retries && now.duration_since(*last_attempt) > timeout);

            if should_dispatch && *attempts < max_retries {
                *attempts += 1;
                *last_attempt = now;
                batch.push(payload.clone());
            }
        }

        batch
    }

    /// Record an acknowledgment from a peer.
    ///
    /// This removes the fragment from the pending queue and records
    /// the ack for tracking.
    pub fn record_ack(&mut self, ack: FragmentAck) {
        self.pending.remove(&ack.fragment_id);
        self.acks.push(ack);
    }

    /// Receive and decrypt a fragment payload from the network.
    ///
    /// Returns the fragment ID and decrypted data for storage.
    pub fn receive_fragment(
        &self,
        payload: &FragmentPayload,
    ) -> Result<(FragmentId, Vec<u8>), DispatchError> {
        let aad = format!("polygone-fragment-{}", payload.id.as_u8()).as_bytes().to_vec();
        let plaintext = decrypt(&self.session_key, &payload.ciphertext, &payload.nonce, &aad)
            .map_err(|_| DispatchError::DecryptionFailed { fragment_id: payload.id })?;

        Ok((payload.id, plaintext))
    }

    /// Store a decrypted fragment for later reconstruction.
    pub fn store_fragment(&mut self, id: FragmentId, data: Vec<u8>) {
        self.collected.insert(id, data);
    }

    /// Check if we have enough fragments for reconstruction.
    pub fn has_sufficient_fragments(&self, threshold: u8) -> bool {
        self.collected.len() >= threshold as usize
    }

    /// Attempt to reconstruct the original secret from collected fragments.
    ///
    /// Returns `None` if insufficient fragments or reconstruction fails.
    pub fn reconstruct(&self, threshold: u8) -> Option<SessionKey> {
        if !self.has_sufficient_fragments(threshold) {
            return None;
        }

        let fragments: Vec<ShamirFragment> = self.collected
            .iter()
            .map(|(id, data)| ShamirFragment {
                id: *id,
                data: data.clone(),
            })
            .collect();

        reconstruct_secret(fragments, threshold)
    }

    /// Get the number of pending (unacknowledged) dispatches.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    /// Get the number of collected fragments.
    pub fn collected_count(&self) -> usize {
        self.collected.len()
    }

    /// Get the number of acknowledged dispatches.
    pub fn acked_count(&self) -> usize {
        self.acks.len()
    }

    /// Get the session ID.
    pub fn session_id(&self) -> &[u8; 32] {
        &self.session_id
    }
}

/// Errors that can occur during fragment dispatch.
#[derive(Debug, Clone, thiserror::Error)]
pub enum DispatchError {
    #[error("Encryption failed for fragment {fragment_id:?}")]
    EncryptionFailed { fragment_id: FragmentId },

    #[error("Decryption failed for fragment {fragment_id:?}")]
    DecryptionFailed { fragment_id: FragmentId },

    #[error("Dispatch timeout for fragment {fragment_id:?}")]
    DispatchTimeout { fragment_id: FragmentId },

    #[error("Max retries exceeded for fragment {fragment_id:?}")]
    MaxRetriesExceeded { fragment_id: FragmentId },

    #[error("Serialization error: {0}")]
    Serialization(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use polygone_crypto::split_secret;

    fn test_node_id(byte: u8) -> NodeId {
        NodeId([byte; 32])
    }

    fn test_session_key() -> SessionKey {
        SessionKey::new([0x42; 32])
    }

    #[test]
    fn prepare_and_receive_round_trip() {
        let key = test_session_key();
        let dispatcher = FragmentDispatcher::new([0u8; 32], key.clone(), DispatchConfig::default());

        // Simulate Shamir shares (in reality these come from split_secret)
        let raw_fragments: Vec<(FragmentId, Vec<u8>, NodeId)> = vec![
            (FragmentId::new(1), vec![10, 20, 30], test_node_id(1)),
            (FragmentId::new(2), vec![40, 50, 60], test_node_id(2)),
            (FragmentId::new(3), vec![70, 80, 90], test_node_id(3)),
        ];

        let payloads = dispatcher.prepare_payloads(raw_fragments).unwrap();
        assert_eq!(payloads.len(), 3);

        // Verify each payload can be decrypted
        for payload in &payloads {
            let (id, data) = dispatcher.receive_fragment(payload).unwrap();
            assert!(!data.is_empty());
            let _ = id; // used for verification
        }
    }

    #[test]
    fn enqueue_and_dispatch_batch() {
        let key = test_session_key();
        let mut dispatcher = FragmentDispatcher::new([0u8; 32], key, DispatchConfig::default());

        let raw_fragments: Vec<(FragmentId, Vec<u8>, NodeId)> = vec![
            (FragmentId::new(1), vec![1, 2, 3], test_node_id(1)),
            (FragmentId::new(2), vec![4, 5, 6], test_node_id(2)),
        ];

        let payloads = dispatcher.prepare_payloads(raw_fragments).unwrap();
        dispatcher.enqueue(payloads);

        assert_eq!(dispatcher.pending_count(), 2);

        let batch = dispatcher.next_dispatch_batch();
        assert_eq!(batch.len(), 2);

        // After first dispatch, next batch should be empty until timeout
        let batch2 = dispatcher.next_dispatch_batch();
        assert!(batch2.is_empty());
    }

    #[test]
    fn ack_removes_from_pending() {
        let key = test_session_key();
        let mut dispatcher = FragmentDispatcher::new([0u8; 32], key, DispatchConfig::default());

        let raw_fragments: Vec<(FragmentId, Vec<u8>, NodeId)> = vec![
            (FragmentId::new(1), vec![1, 2, 3], test_node_id(1)),
        ];

        let payloads = dispatcher.prepare_payloads(raw_fragments).unwrap();
        dispatcher.enqueue(payloads);

        let ack = FragmentAck {
            fragment_id: FragmentId::new(1),
            node_id: test_node_id(1),
            ciphertext_hash: [0u8; 32],
        };

        dispatcher.record_ack(ack);
        assert_eq!(dispatcher.pending_count(), 0);
        assert_eq!(dispatcher.acked_count(), 1);
    }

    #[test]
    fn store_and_check_sufficient() {
        let key = test_session_key();
        let mut dispatcher = FragmentDispatcher::new([0u8; 32], key, DispatchConfig::default());

        dispatcher.store_fragment(FragmentId::new(1), vec![1]);
        dispatcher.store_fragment(FragmentId::new(2), vec![2]);
        dispatcher.store_fragment(FragmentId::new(3), vec![3]);

        assert!(!dispatcher.has_sufficient_fragments(4)); // need 4, have 3
        assert!(dispatcher.has_sufficient_fragments(3));  // need 3, have 3
        assert!(dispatcher.has_sufficient_fragments(2));  // need 2, have 3
    }

    #[test]
    fn full_shamir_split_encrypt_dispatch_decrypt_reconstruct() {
        let key = test_session_key();
        let dispatcher = FragmentDispatcher::new([0u8; 32], key.clone(), DispatchConfig::default());

        // 1. Split the session key into fragments
        let fragments = split_secret(&key, 3, 5);
        assert_eq!(fragments.len(), 5);

        // 2. Prepare payloads (encrypt)
        let raw: Vec<(FragmentId, Vec<u8>, NodeId)> = fragments
            .iter()
            .enumerate()
            .map(|(i, f)| (f.id, f.data.clone(), test_node_id(i as u8 + 1)))
            .collect();

        let payloads = dispatcher.prepare_payloads(raw).unwrap();

        // 3. Simulate receiving side
        let receiver = FragmentDispatcher::new([0u8; 32], key.clone(), DispatchConfig::default());
        let mut collected = HashMap::new();

        for payload in &payloads[..3] { // only need threshold=3
            let (id, data) = receiver.receive_fragment(payload).unwrap();
            collected.insert(id, data);
        }

        // 4. Reconstruct
        let frag_vec: Vec<ShamirFragment> = collected
            .iter()
            .map(|(id, data)| ShamirFragment {
                id: *id,
                data: data.clone(),
            })
            .collect();

        let recovered = reconstruct_secret(frag_vec, 3);
        assert!(recovered.is_some());
        assert_eq!(recovered.unwrap().as_slice(), key.as_slice());
    }
}
