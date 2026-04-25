//! Integration tests for Polygone crypto primitives.

use polygone::{
    crypto::{kem, shamir, symmetric::SessionKey, KeyPair},
    protocol::Session,
};

#[test]
fn test_ml_kem_round_trip() {
    let (pk, sk) = kem::generate_keypair().unwrap();
    let (ct, ss1) = kem::encapsulate(&pk).unwrap();
    let ss2 = kem::decapsulate(&sk, &ct).unwrap();
    assert_eq!(ss1.0, ss2.0, "shared secrets mismatch");
}

#[test]
fn test_aes_256_gcm_encrypt_decrypt() {
    let key = SessionKey::from_bytes([0xABu8; 32]);
    let msg = b"post-quantum privacy";
    let ct = key.encrypt(msg).unwrap();
    let pt = key.decrypt(&ct).unwrap();
    assert_eq!(pt, msg, "decrypted message mismatch");
}

#[test]
fn test_shamir_4_of_7() {
    let secret = b"polygone-shamir-test-secret-bytes";
    let frags = shamir::split(secret, 4, 7).unwrap();
    for i in 0..7 { for j in (i+1)..7 { for k in (j+1)..7 { for l in (k+1)..7 {
        let subset = vec![frags[i].clone(), frags[j].clone(),
                          frags[k].clone(), frags[l].clone()];
        let r = shamir::reconstruct(&subset, 4).unwrap();
        assert_eq!(r, secret, "C({i},{j},{k},{l}) failed");
    }}}}
}

#[test]
fn test_full_session_round_trip() {
    let bob_kp = KeyPair::generate().unwrap();
    let (mut alice, ct) = Session::new_initiator(&bob_kp.kem_pk).unwrap();
    let mut bob = Session::new_responder(bob_kp, &ct).unwrap();
    alice.establish(None).unwrap();
    bob.establish(None).unwrap();

    // Verify both sides derive identical topology
    let a_nodes: Vec<_> = alice.topology.as_ref().unwrap().nodes.iter().map(|n| n.0).collect();
    let b_nodes: Vec<_> = bob.topology.as_ref().unwrap().nodes.iter().map(|n| n.0).collect();
    assert_eq!(a_nodes, b_nodes, "topology mismatch between Alice and Bob");

    let msg = b"L'information n'existe pas. Elle traverse.";
    let assignments = alice.send(msg).unwrap();
    let frags: Vec<_> = assignments.into_iter().map(|(_, b)| b).collect();
    let recovered = bob.receive(frags).unwrap();
    assert_eq!(recovered, msg, "message mismatch");
    alice.dissolve();
    bob.dissolve();
}

#[test]
fn test_insufficient_fragments() {
    let bob_kp = KeyPair::generate().unwrap();
    let (mut alice, ct) = Session::new_initiator(&bob_kp.kem_pk).unwrap();
    let mut bob = Session::new_responder(bob_kp, &ct).unwrap();
    alice.establish(None).unwrap();
    bob.establish(None).unwrap();
    let assignments = alice.send(b"secret").unwrap();
    let frags: Vec<_> = assignments.into_iter().take(3).map(|(_, b)| b).collect();
    assert!(bob.receive(frags).is_err(), "should have failed with 3/7 fragments");
}