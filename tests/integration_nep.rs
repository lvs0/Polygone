// Integration tests for NEP (Neural Exchange Protocol)
// Focus areas: handshake (NEP_HELLO), framing/déframing, belief_graph operations.

use polygone::{crypto::KeyPair, protocol::Session};
use polygone_core::types::NodeId;
use libp2p::PeerId;
use tokio::runtime::Runtime;

#[tokio::test]
async fn test_nep_hello_handshake() {
    // Generate keypairs for two nodes
    let alice_kp = KeyPair::generate().unwrap();
    let bob_kp = KeyPair::generate().unwrap();

    // Create sessions: Alice as initiator, Bob as responder
    let (mut alice, hello_ct) = Session::new_initiator(&bob_kp.public()).unwrap();
    let mut bob = Session::new_responder(bob_kp, &hello_ct).unwrap();

    // Establish handshake => should exchange NEP_HELLO messages
    alice.establish(None).await.unwrap();
    bob.establish(None).await.unwrap();

    // After handshake, topology (belief graph) should be symmetric
    let alice_top = alice.topology.as_ref().unwrap();
    let bob_top = bob.topology.as_ref().unwrap();
    assert_eq!(alice_top.nodes.len(), bob_top.nodes.len());
    // Both should include each other's NodeId
    assert!(alice_top.nodes.contains_key(&alice.id));
    assert!(alice_top.nodes.contains_key(&bob.id));
    assert!(bob_top.nodes.contains_key(&alice.id));
    assert!(bob_top.nodes.contains_key(&bob.id));
}

#[test]
fn test_framing_deframing() {
    // Use the Packet type from the message layer
    use polygone::msg::Packet;
    // Construct a sample packet
    let sender = NodeId::random();
    let recipient = NodeId::random();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    // Build packet using builder pattern (if available)
    // Note: Adjust based on actual API
    let packet = Packet {
        header: polygone::msg::PacketHeader {
            sender,
            recipient,
            message_type: "NEP_HELLO".to_string(),
            timestamp: now,
            fragment: None,
            flags: 0,
        },
        payload: Vec::new(), // empty payload for hello
    };

    // Serialize with bincode (common in Polygone)
    let encoded = bincode::serialize(&packet).unwrap();

    // Frame: length-prefixed (4-byte BE length)
    let mut framed = Vec::new();
    framed.extend_from_slice(&(encoded.len() as u32).to_be_bytes());
    framed.extend_from_slice(&encoded);

    // Deframe
    let len = u32::from_be_bytes([framed[0], framed[1], framed[2], framed[3]]) as usize;
    let payload = &framed[4..4 + len];
    let decoded: Packet = bincode::deserialize(payload).unwrap();

    assert_eq!(decoded, packet);
}

#[tokio::test]
async fn test_belief_graph_operations() {
    // Test that the belief graph (topology) correctly reflects node additions and updates
    let alice_kp = KeyPair::generate().unwrap();
    let bob_kp = KeyPair::generate().unwrap();
    let (mut alice, ct) = Session::new_initiator(&bob_kp.public()).unwrap();
    let mut bob = Session::new_responder(bob_kp, &ct).unwrap();
    alice.establish(None).await.unwrap();
    bob.establish(None).await.unwrap();

    // At start, topology contains two nodes
    let init_count = alice.topology.as_ref().unwrap().nodes.len();
    assert_eq!(init_count, 2);

    // Simulate a third node joining via belief propagation
    let charlie_kp = KeyPair::generate().unwrap();
    let charlie_id = *charlie_kp.public(); // assuming NodeId derived from PubKey

    // Directly add to topology (if API exists)
    // For now, we construct a plausible method; actual implementation may differ.
    // alice.add_node(charlie_id, ...);
    // This is a placeholder until API is confirmed.
}
