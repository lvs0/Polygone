//! Network initialization tests for Polygone P2P node.
//!
//! Tests cover:
//! - P2PNode creation and field initialization
//! - NodeId construction and basic properties
//! - FragmentDispatcher initialization (used by network layer)
//! - Fragment metadata structures

use crate::P2PNode;
use polygone_common::{NodeId, SessionKey, DispatchConfig, FragmentId, FragmentPayload, FragmentAck};

#[test]
fn test_p2p_node_initialization() {
    let node = P2PNode::new();
    // Stub: node_id is initialized as zeroed array
    assert_eq!(node.node_id.0, [0u8; 32]);
}

#[test]
fn test_node_id_construction() {
    let bytes = [0x42u8; 32];
    let node_id = NodeId(bytes);
    assert_eq!(node_id.0, bytes);
}

#[test]
fn test_node_id_clone() {
    let original = NodeId([0xAB; 32]);
    let cloned = original.clone();
    assert_eq!(original.0, cloned.0);
}

#[test]
fn test_session_key_construction() {
    let bytes = [0x11u8; 32];
    let key = SessionKey::new(bytes);
    assert_eq!(key.as_slice(), &bytes);
}

#[test]
fn test_fragment_id_new() {
    let id = FragmentId::new(1);
    assert_eq!(id.as_u8(), 1);
}

#[test]
#[should_panic(expected = "FragmentId must be 1-indexed")]
fn test_fragment_id_zero_panics() {
    let _ = FragmentId::new(0);
}

#[test]
fn test_fragment_id_via_tuple() {
    let id = FragmentId(7);
    assert_eq!(id.0, 7);
}

#[test]
fn test_fragment_ack_construction() {
    let ack = FragmentAck {
        fragment_id: FragmentId(42),
        node_id: NodeId([0x33; 32]),
        ciphertext_hash: [0u8; 32],
    };
    assert_eq!(ack.fragment_id.0, 42);
}

#[test]
fn test_fragment_payload_construction() {
    let payload = FragmentPayload {
        id: FragmentId(2),
        destination: NodeId([0x55; 32]),
        ciphertext: vec![0xAA; 16],
        nonce: [0u8; 12],
    };
    assert_eq!(payload.ciphertext.len(), 16);
    assert_eq!(payload.id.0, 2);
}

#[test]
fn test_p2p_node_clone() {
    let node = P2PNode::new();
    let cloned = node.clone();
    assert_eq!(node.node_id.0, cloned.node_id.0);
}

#[test]
fn test_dispatch_config_default() {
    let config = DispatchConfig::default();
    assert_eq!(config.max_fragment_size, 1_048_576);
    assert_eq!(config.max_retries, 3);
    assert_eq!(config.dispatch_timeout_ms, 5000);
    assert!(config.encrypt_fragments);
}
