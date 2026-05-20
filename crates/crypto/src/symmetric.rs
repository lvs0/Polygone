use aes_gcm::{
    Aes256Gcm,
    aead::{Aead, KeyInit, Nonce, Payload},
};
use polygone_common::SessionKey;
use rand;
use thiserror::Error;

/// AES-GCM 128-bit authentication tag length.
pub const GCM_TAG_SIZE: usize = 16;

#[derive(Error, Debug)]
pub enum SymmetricError {
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Decryption failed")]
    DecryptionFailed,
}

/// Encrypt `plaintext` with AES-256-GCM using the given 256-bit `key`.
///
/// `aad` (Associated Authenticated Data) is bound into the authentication tag but
/// is **not** encrypted.  Supplying a different AAD at decrypt time will cause
/// the tag verification to fail, returning `SymmetricError::DecryptionFailed`.
///
/// Returns `(ciphertext || 16-byte_tag, 96-bit_nonce)`.
pub fn encrypt(
    key: &SessionKey,
    plaintext: &[u8],
    aad: &[u8],
) -> Result<(Vec<u8>, [u8; 12]), SymmetricError> {
    let cipher =
        Aes256Gcm::new_from_slice(key.as_slice()).map_err(|_| SymmetricError::EncryptionFailed)?;

    // 96-bit random nonce (standard for AES-GCM)
    let nonce_bytes: [u8; 12] = rand::random();
    let nonce = Nonce::<Aes256Gcm>::from_slice(&nonce_bytes);

    let ciphertext = cipher
        // AadIsPassed: Payload { msg, aad } carries the AAD through encrypt_in_place
        .encrypt(nonce, Payload { msg: plaintext, aad })
        .map_err(|_| SymmetricError::EncryptionFailed)?;

    Ok((ciphertext, nonce_bytes))
}

/// Decrypt AES-256-GCM ciphertext.
///
/// Returns `Err(SymmetricError::DecryptionFailed)` if:
/// - The authentication tag does not verify (wrong key, wrong nonce, tampered
///   ciphertext, or **wrong AAD**).
/// - The ciphertext is too short to contain the 16-byte authentication tag.
pub fn decrypt(
    key: &SessionKey,
    ciphertext: &[u8],
    nonce: &[u8; 12],
    aad: &[u8],
) -> Result<Vec<u8>, SymmetricError> {
    if ciphertext.len() < GCM_TAG_SIZE {
        return Err(SymmetricError::DecryptionFailed);
    }

    let cipher =
        Aes256Gcm::new_from_slice(key.as_slice()).map_err(|_| SymmetricError::DecryptionFailed)?;

    let nonce = Nonce::<Aes256Gcm>::from_slice(nonce);

    // AadIsUsed: Pass aad explicitly via Payload struct so the tag check covers it
    let payload = Payload { msg: ciphertext, aad };

    cipher
        .decrypt(nonce, payload)
        .map_err(|_| SymmetricError::DecryptionFailed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> SessionKey {
        SessionKey::new([0xABu8; 32])
    }

    #[test]
    fn roundtrip_small_plaintext() {
        let key = test_key();
        let plaintext = b"hello world";

        let (ciphertext, nonce) = encrypt(&key, plaintext, &[]).expect("encrypt failed");
        let recovered = decrypt(&key, &ciphertext, &nonce, &[]).expect("decrypt failed");

        assert_eq!(recovered, plaintext);
    }

    #[test]
    fn roundtrip_empty_plaintext() {
        let key = test_key();
        let plaintext = b"";

        let (ciphertext, nonce) = encrypt(&key, plaintext, &[]).expect("encrypt failed");
        let recovered = decrypt(&key, &ciphertext, &nonce, &[]).expect("decrypt failed");

        assert_eq!(recovered, plaintext);
    }

    #[test]
    fn roundtrip_large_plaintext_1mb() {
        let key = test_key();
        let plaintext: Vec<u8> = (0..1_048_576).map(|i| (i % 256) as u8).collect();

        let (ciphertext, nonce) = encrypt(&key, &plaintext, &[]).expect("encrypt failed");
        let recovered = decrypt(&key, &ciphertext, &nonce, &[]).expect("decrypt failed");

        assert_eq!(recovered, plaintext);
    }

    #[test]
    fn roundtrip_random_plaintext() {
        let key = test_key();
        let mut rng = rand::thread_rng();
        let plaintext: Vec<u8> = (0..4096).map(|_| rand::Rng::gen(&mut rng)).collect();

        let (ciphertext, nonce) = encrypt(&key, &plaintext, &[]).expect("encrypt failed");
        let recovered = decrypt(&key, &ciphertext, &nonce, &[]).expect("decrypt failed");

        assert_eq!(recovered, plaintext);
    }

    #[test]
    fn different_plaintexts_produce_different_ciphertexts() {
        let key = test_key();
        let p1 = b"message A";
        let p2 = b"message B";

        let (ct1, n1) = encrypt(&key, p1, &[]).unwrap();
        let (ct2, n2) = encrypt(&key, p2, &[]).unwrap();

        assert_ne!(
            ct1, ct2,
            "two different messages encrypted with the same key must yield different ciphertexts"
        );

        assert_eq!(decrypt(&key, &ct1, &n1, &[]).unwrap(), p1);
        assert_eq!(decrypt(&key, &ct2, &n2, &[]).unwrap(), p2);
    }

    #[test]
    fn same_plaintext_key_different_nonces_different_ciphertexts() {
        let key = test_key();
        let plaintext = b"persistent message";

        let (ct1, n1) = encrypt(&key, plaintext, &[]).unwrap();
        let (ct2, n2) = encrypt(&key, plaintext, &[]).unwrap();

        assert_ne!(n1, n2, "nonces must be random and unique");
        assert_ne!(
            ct1, ct2,
            "same plaintext + same key with random nonce must yield different ciphertexts"
        );
        assert_eq!(decrypt(&key, &ct1, &n1, &[]).unwrap(), plaintext);
        assert_eq!(decrypt(&key, &ct2, &n2, &[]).unwrap(), plaintext);
    }

    #[test]
    fn decryption_fails_with_wrong_nonce() {
        let key = test_key();
        let plaintext = b"secret data";

        let (ciphertext, _nonce) = encrypt(&key, plaintext, &[]).unwrap();
        let wrong_nonce: [u8; 12] = [0xFF; 12];

        let result = decrypt(&key, &ciphertext, &wrong_nonce, &[]);
        assert!(result.is_err(), "decrypting with wrong nonce must fail");

        if let Err(SymmetricError::DecryptionFailed) = result {
            // expected
        } else {
            panic!("expected SymmetricError::DecryptionFailed");
        }
    }

    #[test]
    fn decryption_fails_with_wrong_key() {
        let right_key = test_key();
        let wrong_key = SessionKey::new([0xCDu8; 32]);
        let plaintext = b"secret data";

        let (ciphertext, nonce) = encrypt(&right_key, plaintext, &[]).unwrap();
        let result = decrypt(&wrong_key, &ciphertext, &nonce, &[]);
        assert!(result.is_err(), "decrypting with wrong key must fail");
    }

    #[test]
    fn decryption_fails_with_tampered_ciphertext() {
        let key = test_key();
        let plaintext = b"do not tamper";

        let (mut ciphertext, nonce) = encrypt(&key, plaintext, &[]).unwrap();

        if !ciphertext.is_empty() {
            let mid = ciphertext.len() / 2;
            ciphertext[mid] ^= 0xFF;
        }

        let result = decrypt(&key, &ciphertext, &nonce, &[]);
        assert!(
            result.is_err(),
            "decrypting tampered ciphertext must fail (GCM auth tag check)"
        );
    }

    #[test]
    fn decryption_fails_with_truncated_ciphertext() {
        let key = test_key();
        let plaintext = b"need full data";

        let (ciphertext, nonce) = encrypt(&key, plaintext, &[]).unwrap();
        // Drop the 16-byte tag; pass half the ciphertext body
        let truncated = &ciphertext[..ciphertext.len() / 2];

        let result = decrypt(&key, truncated, &nonce, &[]);
        assert!(result.is_err(), "decrypting truncated ciphertext must fail");
    }

    #[test]
    fn ciphertext_length_is_plaintext_plus_tag() {
        // AES-GCM appends a 16-byte authentication tag
        let key = test_key();
        let plaintext = b"payload";
        let (ciphertext, _nonce) = encrypt(&key, plaintext, &[]).unwrap();

        assert_eq!(
            ciphertext.len(),
            plaintext.len() + GCM_TAG_SIZE,
            "AES-GCM ciphertext should be plaintext_len + 16-byte tag"
        );
    }

    #[test]
    fn roundtrip_with_all_byte_values() {
        let key = test_key();
        let plaintext: Vec<u8> = (0..=255).collect();

        let (ciphertext, nonce) = encrypt(&key, &plaintext, &[]).unwrap();
        let recovered = decrypt(&key, &ciphertext, &nonce, &[]).unwrap();

        assert_eq!(recovered, plaintext);
    }

    #[test]
    fn multiple_roundtrips_same_key_100x() {
        let key = test_key();
        let plaintext = b"repeated roundtrip test";

        for i in 0..100 {
            let (ct, n) = encrypt(&key, plaintext, &[]).unwrap();
            let recovered = decrypt(&key, &ct, &n, &[]).unwrap();
            assert_eq!(recovered, plaintext, "iteration {} failed", i);
        }
    }

    #[test]
    fn aad_correct_roundtrips() {
        let key = test_key();
        let plaintext = b"with associated data";
        let aad = b"authenticated-header-info";

        let (ciphertext, nonce) = encrypt(&key, plaintext, aad).unwrap();
        let recovered = decrypt(&key, &ciphertext, &nonce, aad).unwrap();
        assert_eq!(recovered, plaintext);
    }

    #[test]
    fn aad_wrong_auth_tag_fails() {
        // GCM binds AAD into the authentication tag. A wrong AAD must cause
        // the tag check to fail, returning SymmetricError::DecryptionFailed —
        // NOT the original plaintext.
        let key = test_key();
        let plaintext = b"message protected by aad";
        let correct_aad = b"correct-aad";
        let wrong_aad   = b"wrong-aad";

        let (ciphertext, nonce) = encrypt(&key, plaintext, correct_aad).unwrap();
        let result = decrypt(&key, &ciphertext, &nonce, wrong_aad);

        assert!(
            result.is_err(),
            "decrypting with wrong AAD must fail: GCM authentication tag will not verify"
        );
        match result {
            Err(SymmetricError::DecryptionFailed) => { /* correct */ }
            _ => panic!("expected SymmetricError::DecryptionFailed for wrong AAD"),
        }
    }

    #[test]
    fn aad_empty_vs_nonempty_must_differ() {
        let key = test_key();
        let plaintext = b"aad-comparison-text";

        let (ct_empty, n_empty) = encrypt(&key, plaintext, b"").unwrap();
        let (ct_aad,   n_aad)   = encrypt(&key, plaintext, b"some-aad").unwrap();

        // Each decrypts correctly with its own AAD
        assert_eq!(decrypt(&key, &ct_empty, &n_empty, b"").unwrap(), plaintext);
        assert_eq!(decrypt(&key, &ct_aad, &n_aad, b"some-aad").unwrap(), plaintext);

        // Wrong-AAD must fail in both directions
        assert!(
            decrypt(&key, &ct_aad, &n_aad, b"").is_err(),
            "empty AAD with non-empty-AAD ciphertext must fail"
        );
        assert!(
            decrypt(&key, &ct_empty, &n_empty, b"some-aad").is_err(),
            "non-empty AAD with empty-AAD ciphertext must fail"
        );
    }

    #[test]
    fn roundtrip_128_byte_message() {
        let key = test_key();
        let plaintext = vec![0xAAu8; 128];

        let (ciphertext, nonce) = encrypt(&key, &plaintext, &[]).unwrap();
        let recovered = decrypt(&key, &ciphertext, &nonce, &[]).unwrap();

        assert_eq!(recovered, plaintext);
    }
}
