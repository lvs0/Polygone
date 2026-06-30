use msh::{Envelope, NodeKeypair};

#[test]
fn test_keypair_generate_and_sign() {
    let kp = NodeKeypair::generate();
    let pk = kp.public_key_hex();
    assert_eq!(pk.len(), 64); // 32 bytes -> hex

    let msg = b"hello world";
    let sig = kp.sign(msg);
    assert_eq!(sig.len(), 128); // 64-byte sig -> hex

    // Verify with same keypair
    assert!(kp.verify(msg, &sig).is_ok());

    // Wrong message should fail
    let wrong_msg = b"hello worlx";
    assert!(kp.verify(wrong_msg, &sig).is_err());
}

#[test]
fn test_keypair_from_seed() {
    let seed = [42u8; 32];
    let kp1 = NodeKeypair::from_bytes(&seed);
    let kp2 = NodeKeypair::from_bytes(&seed);
    // Same seed => same keypair
    assert_eq!(kp1.public_key_hex(), kp2.public_key_hex());

    let msg = b"test";
    let sig1 = kp1.sign(msg);
    let sig2 = kp2.sign(msg);
    assert_eq!(sig1, sig2);
    // Both verify with each other's public key
    assert!(kp1.verify(msg, &sig2).is_ok());
    assert!(kp2.verify(msg, &sig1).is_ok());
}

#[test]
fn test_envelope_sign_and_verify() {
    let mut env = Envelope::new("request");
    env.model_id = Some("model-123".to_string());
    env.payload = Some(b"payload".to_vec());

    let kp = NodeKeypair::generate();
    env.sign_with(&kp);

    assert!(env.has_signature());
    assert_eq!(env.signer_pk.as_deref(), Some(kp.public_key_hex().as_str()));
    assert!(env.verify_signature().is_ok());

    // Tampering should break verification
    if let Some(ref mut p) = env.payload {
        p.push(0);
    }
    assert!(env.verify_signature().is_err());
}

#[test]
fn test_envelope_missing_fields() {
    let mut env = Envelope::new("request");
    // No signature
    assert!(!env.has_signature());
    assert!(env.verify_signature().is_err());

    // Only sig, no pk
    env.sig = Some("deadbeef".repeat(32)); // 64 bytes hex
    assert!(!env.has_signature());
    assert!(env.verify_signature().is_err());

    // Only pk, no sig
    env.sig = None;
    env.signer_pk = Some("0123456789abcdef".repeat(8)); // 32 bytes hex
    assert!(!env.has_signature());
    assert!(env.verify_signature().is_err());
}