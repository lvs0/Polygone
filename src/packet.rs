//! Wire packet format for the POLYGONE protocol.
//!
//! Every packet carries a `protocol_version` field so future versions
//! of the protocol can coexist on the same network and negotiate gracefully.
//! A v0.1 node that receives a v0.2 packet rejects it with a clear error
//! rather than silently misinterpreting the bytes.
//!
//! Packet types:
//!
//! ```text
//! HandshakeInit   — Alice → relay → Bob (KEM ciphertext, Alice's signing PK)
//! HandshakeReply  — Bob → relay → Alice (acknowledgment)
//! Fragment        — relay → Bob (one Shamir fragment)
//! Dissolve        — both sides (signal session end)
//! ```

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::protocol::SessionId;

/// Current protocol version. Bumped on any breaking wire-format change.
pub const PROTOCOL_VERSION: u8 = 1;

/// Packet type discriminant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum PacketType {
    HandshakeInit  = 0x01,
    HandshakeReply = 0x02,
    Fragment       = 0x03,
    Dissolve       = 0xFF,
}

/// Common header present in every POLYGONE packet.
///
/// Receivers MUST check `version == PROTOCOL_VERSION` and reject mismatches.
/// This prevents silent misinterpretation across protocol versions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacketHeader {
    /// Must equal `PROTOCOL_VERSION`. Reject if different.
    pub version:    u8,
    /// Identifies the packet type for deserialization.
    pub packet_type: PacketType,
    /// Links this packet to a specific session.
    /// Not secret — relay nodes need it for routing.
    pub session_id: SessionId,
}

impl PacketHeader {
    /// Create a header with the current protocol version.
    pub fn new(packet_type: PacketType, session_id: SessionId) -> Self {
        Self { version: PROTOCOL_VERSION, packet_type, session_id }
    }

    /// Validate that this header's version matches the current protocol.
    pub fn validate_version(&self) -> crate::Result<()> {
        if self.version != PROTOCOL_VERSION {
            return Err(crate::PolygoneError::InvalidTransition {
                from: format!("protocol v{}", self.version),
                to:   format!("protocol v{PROTOCOL_VERSION}"),
            });
        }
        Ok(())
    }
}

/// Handshake initiation: Alice → network → Bob.
///
/// Contains the ML-KEM ciphertext that Bob decapsulates to recover
/// the shared secret, plus Alice's signing public key so Bob can
/// verify future authenticated packets from Alice (v0.2).
///
/// # Replay protection
///
/// `timestamp_secs` is the Unix timestamp (seconds) at packet creation.
/// Receivers MUST reject packets where `|now - timestamp_secs| > REPLAY_WINDOW_SECS`.
/// This prevents an attacker from recording and re-replaying the KEM ciphertext
/// to initiate a new session (even though the replayed ciphertext produces the same
/// shared secret, the timestamp check breaks the replay).
///
/// `nonce` is a random 16-byte value that makes each HandshakeInit unique
/// even within the replay window. Both fields together provide defence-in-depth.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeInit {
    pub header: PacketHeader,
    /// ML-KEM-1024 ciphertext (1568 bytes).
    pub kem_ciphertext: Vec<u8>,
    /// Alice's ML-DSA-87 signing public key.
    /// Used in v0.2 for authenticated handshake; carried but not yet verified.
    pub alice_sign_pk: Vec<u8>,
    /// Unix timestamp (seconds) when this packet was created.
    /// Receivers reject packets outside REPLAY_WINDOW_SECS of their local clock.
    pub timestamp_secs: u64,
    /// Random 16-byte nonce making each handshake unique within the replay window.
    pub nonce: [u8; 16],
}

/// Maximum clock skew tolerated between peers (seconds).
/// Packets older than this are rejected as potential replays.
pub const REPLAY_WINDOW_SECS: u64 = 30;

impl HandshakeInit {
    /// Create a new handshake with current timestamp and fresh random nonce.
    pub fn new(
        session_id: SessionId,
        kem_ciphertext: Vec<u8>,
        alice_sign_pk: Vec<u8>,
    ) -> crate::Result<Self> {
        let timestamp_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| crate::PolygoneError::RngError(e.to_string()))?
            .as_secs();

        let mut nonce = [0u8; 16];
        getrandom::getrandom(&mut nonce)
            .map_err(|e| crate::PolygoneError::RngError(e.to_string()))?;

        Ok(Self {
            header: PacketHeader::new(PacketType::HandshakeInit, session_id),
            kem_ciphertext,
            alice_sign_pk,
            timestamp_secs,
            nonce,
        })
    }

    /// Validate this packet against replay attacks.
    ///
    /// Returns `Ok(())` if the timestamp is within `REPLAY_WINDOW_SECS` of now.
    /// Returns an error if the packet is too old or from the future (> 5s).
    pub fn validate_replay(&self) -> crate::Result<()> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| crate::PolygoneError::RngError(e.to_string()))?
            .as_secs();

        let age = now.saturating_sub(self.timestamp_secs);
        let future_drift = self.timestamp_secs.saturating_sub(now);

        if age > REPLAY_WINDOW_SECS {
            return Err(crate::PolygoneError::InvalidTransition {
                from: format!("handshake timestamp {}s ago", age),
                to: "rejected — outside replay window".into(),
            });
        }
        if future_drift > 5 {
            return Err(crate::PolygoneError::InvalidTransition {
                from: format!("handshake timestamp {}s in future", future_drift),
                to: "rejected — clock skew too large".into(),
            });
        }
        Ok(())
    }
}

/// Handshake reply: Bob → network → Alice.
/// Acknowledges receipt of the ciphertext; session is now Established on both sides.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeReply {
    pub header: PacketHeader,
    /// Bob's signing public key (for v0.2 mutual authentication).
    pub bob_sign_pk: Vec<u8>,
}

/// A single Shamir fragment in transit from Alice's side to Bob.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FragmentPacket {
    pub header: PacketHeader,
    /// Index of this fragment (1-based, matches `FragmentId`).
    pub fragment_index: u8,
    /// Raw fragment bytes (variable length, padded to uniform size at a higher layer).
    pub data: Vec<u8>,
}

/// Session dissolution signal. Sent by either party after `Completed` or on error.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DissolvePacket {
    pub header: PacketHeader,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::SharedSecret;

    #[test]
    fn header_version_check_accepts_current() {
        let ss = SharedSecret([0u8; 32]);
        let sid = SessionId::from_shared_secret(&ss);
        let h = PacketHeader::new(PacketType::Fragment, sid);
        assert!(h.validate_version().is_ok());
    }

    #[test]
    fn header_version_check_rejects_future() {
        let ss = SharedSecret([0u8; 32]);
        let sid = SessionId::from_shared_secret(&ss);
        let mut h = PacketHeader::new(PacketType::Fragment, sid);
        h.version = 99;
        assert!(h.validate_version().is_err());
    }

    #[test]
    fn packet_roundtrip_bincode() {
        let ss = SharedSecret([0xAA; 32]);
        let sid = SessionId::from_shared_secret(&ss);
        let pkt = FragmentPacket {
            header: PacketHeader::new(PacketType::Fragment, sid),
            fragment_index: 3,
            data: vec![1, 2, 3, 4, 5],
        };
        let bytes = bincode::serialize(&pkt).unwrap();
        let decoded: FragmentPacket = bincode::deserialize(&bytes).unwrap();
        assert_eq!(decoded.fragment_index, 3);
        assert_eq!(decoded.header.version, PROTOCOL_VERSION);
    }

    #[test]
    fn handshake_init_replay_check_accepts_fresh() {
        let ss = SharedSecret([0xBB; 32]);
        let sid = SessionId::from_shared_secret(&ss);
        let h = HandshakeInit::new(sid, vec![0u8; 8], vec![0u8; 8]).unwrap();
        assert!(h.validate_replay().is_ok(), "Fresh handshake must be accepted");
    }

    #[test]
    fn handshake_init_replay_check_rejects_old() {
        let ss = SharedSecret([0xCC; 32]);
        let sid = SessionId::from_shared_secret(&ss);
        let mut h = HandshakeInit::new(sid, vec![], vec![]).unwrap();
        // Wind timestamp back beyond replay window
        h.timestamp_secs = h.timestamp_secs.saturating_sub(REPLAY_WINDOW_SECS + 1);
        assert!(h.validate_replay().is_err(), "Old handshake must be rejected");
    }

    #[test]
    fn handshake_nonces_differ_per_packet() {
        let ss = SharedSecret([0xDD; 32]);
        let sid = SessionId::from_shared_secret(&ss);
        let h1 = HandshakeInit::new(sid, vec![], vec![]).unwrap();
        let h2 = HandshakeInit::new(sid, vec![], vec![]).unwrap();
        assert_ne!(h1.nonce, h2.nonce, "Each handshake must have a unique nonce");
    }
}
