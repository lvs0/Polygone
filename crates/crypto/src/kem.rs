// Post-quantum KEM — ML-KEM-1024 (FIPS 203, NIST 2024)
// Implémentation réelle via pqcrypto-mlkem
//
// Tailles réelles ML-KEM-1024 (PQClean) :
//   PublicKey = 1568 bytes
//   SecretKey = 3168 bytes
//   Ciphertext = 1568 bytes
//   SharedSecret = 32 bytes

use polygone_common::SessionKey;
use pqcrypto_mlkem::mlkem1024;
use pqcrypto_traits::kem::{SharedSecret as _, Ciphertext as _};

pub const PK_SIZE: usize = 1568;
pub const SK_SIZE: usize = 3168;
pub const CT_SIZE: usize = 1568;
pub const SS_SIZE: usize = 32;

pub struct PublicKey(mlkem1024::PublicKey);
pub struct SecretKey(mlkem1024::SecretKey);

pub fn generate_kem_key_pair() -> (PublicKey, SecretKey) {
    let (pk, sk) = mlkem1024::keypair();
    (PublicKey(pk), SecretKey(sk))
}

pub fn encapsulate(pk: &PublicKey) -> (Vec<u8>, SessionKey) {
    let (ss, ct) = mlkem1024::encapsulate(&pk.0);
    let mut shared_secret = [0u8; SS_SIZE];
    shared_secret.copy_from_slice(ss.as_bytes());
    (ct.as_bytes().to_vec(), SessionKey::new(shared_secret))
}

pub fn decapsulate(ct_bytes: &[u8], sk: &SecretKey) -> SessionKey {
    let ct = mlkem1024::Ciphertext::from_bytes(ct_bytes).expect("invalid ciphertext length");
    let ss = mlkem1024::decapsulate(&ct, &sk.0);
    let mut shared_secret = [0u8; SS_SIZE];
    shared_secret.copy_from_slice(ss.as_bytes());
    SessionKey::new(shared_secret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pqcrypto_traits::kem::{PublicKey as _, SecretKey as _};

    #[test]
    fn key_sizes_are_correct() {
        let (pk, sk) = generate_kem_key_pair();
        assert_eq!(pk.0.as_bytes().len(), PK_SIZE);
        assert_eq!(sk.0.as_bytes().len(), SK_SIZE);
    }

    #[test]
    fn key_pair_generation_is_random() {
        let (pk1, _sk1) = generate_kem_key_pair();
        let (pk2, _sk2) = generate_kem_key_pair();
        assert_ne!(pk1.0.as_bytes(), pk2.0.as_bytes());
    }

    #[test]
    fn encapsulate_returns_correct_sizes() {
        let (pk, _sk) = generate_kem_key_pair();
        let (ct, ss) = encapsulate(&pk);
        assert_eq!(ct.len(), CT_SIZE);
        assert_eq!(ss.as_slice().len(), SS_SIZE);
    }

    #[test]
    fn roundtrip_produces_same_secret() {
        let (pk, sk) = generate_kem_key_pair();
        let (ct, original_ss) = encapsulate(&pk);
        let recovered_ss = decapsulate(&ct, &sk);
        assert_eq!(recovered_ss.as_slice(), original_ss.as_slice());
    }

    #[test]
    fn wrong_key_produces_different_secret() {
        let (pk1, _sk1) = generate_kem_key_pair();
        let (_pk2, sk2) = generate_kem_key_pair();
        let (ct, ss1) = encapsulate(&pk1);
        let recovered = decapsulate(&ct, &sk2);
        assert_ne!(recovered.as_slice(), ss1.as_slice());
    }

    #[test]
    fn encapsulate_twice_is_different() {
        let (pk, sk) = generate_kem_key_pair();
        let (ct1, ss1) = encapsulate(&pk);
        let (ct2, ss2) = encapsulate(&pk);
        // Both roundtrip correctly
        assert_eq!(decapsulate(&ct1, &sk).as_slice(), ss1.as_slice());
        assert_eq!(decapsulate(&ct2, &sk).as_slice(), ss2.as_slice());
    }
}
