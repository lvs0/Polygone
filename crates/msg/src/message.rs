//! Message types for Polygone's ephemeral messaging protocol.
//!
//! An `OutgoingMessage` holds plaintext + metadata before sending.
//! After encapsulation, it becomes a collection of `Envelope`s
//! ready for Shamir fragmentation and network dispatch.

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Unique identifier for a message (BLAKE3 of ciphertext).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageId(pub [u8; 32]);

impl MessageId {
    /// Generate a new message ID from ciphertext bytes.
    pub fn from_ciphertext(ciphertext: &[u8]) -> Self {
        use polygone_crypto::hash::hash_data;
        let hash = hash_data(ciphertext);
        let mut id = [0u8; 32];
        id.copy_from_slice(&hash[..32]);
        Self(id)
    }

    /// Null ID for errors/uninitialized state.
    pub fn null() -> Self {
        Self([0u8; 32])
    }

    /// True if this is the null ID.
    pub fn is_null(&self) -> bool {
        self.0 == [0u8; 32]
    }
}

impl std::fmt::Display for MessageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

/// Metadata attached to a message (not sent over wire — metadata only).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMeta {
    /// Unix timestamp when the message was created.
    pub created_at: u64,
    /// Optional human-readable subject / topic.
    pub topic: Option<String>,
    /// Number of fragments the message was split into.
    pub fragment_count: usize,
    /// Threshold required to reconstruct.
    pub threshold: usize,
}

impl MessageMeta {
    /// Create metadata for a new outgoing message.
    pub fn new(topic: Option<String>, fragment_count: usize, threshold: usize) -> Self {
        Self {
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            topic,
            fragment_count,
            threshold,
        }
    }
}

/// A plaintext message before encapsulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutgoingMessage {
    /// Raw plaintext payload.
    pub payload: Vec<u8>,
    /// Message metadata.
    pub meta: MessageMeta,
}

impl OutgoingMessage {
    /// Create a new outgoing message.
    ///
    /// - `plaintext` — the bytes to send (no size limit enforced here;
    ///   large messages are fragmented transparently by the transport).
    /// - `topic` — optional human-readable label (used for routing hints;
    ///   not encrypted).
    pub fn new(plaintext: impl Into<Vec<u8>>, topic: Option<String>) -> Self {
        Self {
            payload: plaintext.into(),
            meta: MessageMeta::new(topic, 7, 4),
        }
    }

    /// Create with default metadata (7 fragments, threshold 4, no topic).
    pub fn plaintext(data: impl Into<Vec<u8>>) -> Self {
        Self {
            payload: data.into(),
            meta: MessageMeta::new(None, 7, 4),
        }
    }
}

/// An encrypted envelope: ready for fragment dispatch.
///
/// One `Envelope` is produced per Shamir fragment. The envelope holds
/// everything needed for the recipient to reconstruct the original message:
/// the encapsulated key, the nonce, and the encrypted fragment bytes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope {
    /// Unique message identifier (BLAKE3 of full ciphertext).
    pub message_id: MessageId,
    /// Index of this fragment in the Shamir split (0..N).
    pub fragment_index: u8,
    /// Total number of fragments.
    pub total_fragments: u8,
    /// Threshold required to reconstruct.
    pub threshold: u8,
    /// ML-KEM-1024 encapsulated key (sent with every fragment).
    /// The recipient uses this to derive the session key for AES-GCM.
    pub encapsulated_key: Vec<u8>,
    /// AES-256-GCM nonce used for fragment encryption (per-fragment random nonce).
    pub nonce: Vec<u8>,
    /// Encrypted fragment bytes (Shamir share under AES-GCM session key).
    pub ciphertext_fragment: Vec<u8>,
    /// SHA-256 digest of the full plaintext (for integrity check after reassembly).
    pub plaintext_hash: Vec<u8>,
    /// The AES-GCM ciphertext of the full message (stored for reassembly).
    pub inner_ciphertext: Vec<u8>,
    /// The AES-GCM nonce used for the full message encryption.
    pub inner_nonce: Vec<u8>,
}

impl Envelope {
    /// True if this envelope is the "last" one in sequence (index == total - 1).
    /// Useful for marking the final fragment in a stream.
    pub fn is_last(&self) -> bool {
        self.fragment_index >= self.total_fragments - 1
    }

    /// Serialise the envelope to bytes for wire transmission.
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).expect("envelope serialisation must not fail")
    }

    /// Deserialize from bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(bytes)
    }
}

/// A received message after full decryption and verification.
#[derive(Debug, Clone)]
pub struct Message {
    /// The decrypted plaintext.
    pub plaintext: Vec<u8>,
    /// Metadata from the original message.
    pub meta: MessageMeta,
    /// The sender's node public key (for reply routing).
    pub sender_node_id: Vec<u8>,
    /// Unix timestamp of receipt.
    pub received_at: u64,
}

impl Message {
    /// Construct a received message.
    pub fn new(plaintext: Vec<u8>, meta: MessageMeta, sender_node_id: Vec<u8>) -> Self {
        Self {
            plaintext,
            meta,
            sender_node_id,
            received_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}