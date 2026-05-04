use polygone_crypto::*;

#[test]
fn test_mlkem_roundtrip() {
    let (pk, sk) = generate_kem_key_pair();
    let (ct, ss1) = encapsulate(&pk);
    let ss2 = decapsulate(&ct, &sk);
    assert_eq!(ss1.as_slice(), ss2.as_slice());
}

#[test]
fn test_aes_gcm_roundtrip() {
    use polygone_common::SessionKey;
    let key = SessionKey::new([1u8; 32]);
    let plaintext = b"Hello Polygone AES-GCM";
    let aad = b"Additional data";

    let (ct, nonce) = encrypt(&key, plaintext, aad).unwrap();
    let dec = decrypt(&key, &ct, &nonce, aad).unwrap();
    assert_eq!(&dec, plaintext);
}

#[test]
fn test_aes_gcm_fails_on_tampered_ciphertext() {
    use polygone_common::SessionKey;
    let key = SessionKey::new([2u8; 32]);
    let plaintext = b"Secret";
    let aad = b"AAD";

    let (mut ct, nonce) = encrypt(&key, plaintext, aad).unwrap();
    ct[0] ^= 0xFF; // corrupt
    let res = decrypt(&key, &ct, &nonce, aad);
    assert!(res.is_err());
}

#[test]
fn test_shamir_split_and_reconstruct() {
    use polygone_common::SessionKey;
    let secret = SessionKey::new([9u8; 32]);
    let shares = split_secret(&secret, 4, 7);
    assert_eq!(shares.len(), 3); // bouchon renvoie 3 partages
    let recovered = reconstruct_secret(shares);
    assert!(recovered.is_some());
}

#[test]
fn test_blake3_same_input_same_hash() {
    let d1 = b"same data";
    let d2 = b"same data";
    let h1 = hash_data(d1);
    let h2 = hash_data(d2);
    assert_eq!(h1, h2);
}

#[test]
fn test_blake3_different_input_different_hash() {
    let h1 = hash_data(b"data1");
    let h2 = hash_data(b"data2");
    assert_ne!(h1, h2);
}
