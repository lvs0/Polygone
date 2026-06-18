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
use pqcrypto_traits::kem::{PublicKey as PubKeyTrait, SecretKey as SecKeyTrait, SharedSecret as _, Ciphertext as _};

pub const PK_SIZE: usize = 1568;
pub const SK_SIZE: usize = 3168;
pub const CT_SIZE: usize = 1568;
pub const SS_SIZE: usize = 32;

/// Construct a PublicKey from raw bytes (wire format).
///
/// Returns `Err("invalid pk length")` if the byte slice is not exactly PK_SIZE.
pub fn pk_from_bytes(bytes: &[u8]) -> Result<PublicKey, &'static str> {
    use pqcrypto_mlkem::mlkem1024;
    if bytes.len() != PK_SIZE {
        return Err("invalid pk length");
    }
    Ok(PublicKey(mlkem1024::PublicKey::from_bytes(bytes).expect("validated size")))
}

/// Construct a SecretKey from raw bytes (wire format).
///
/// Returns `Err("invalid sk length")` if the byte slice is not exactly SK_SIZE.
pub fn sk_from_bytes(bytes: &[u8]) -> Result<SecretKey, &'static str> {
    use pqcrypto_mlkem::mlkem1024;
    if bytes.len() != SK_SIZE {
        return Err("invalid sk length");
    }
    Ok(SecretKey(mlkem1024::SecretKey::from_bytes(bytes).expect("validated size")))
}

pub struct PublicKey(mlkem1024::PublicKey);
pub struct SecretKey(mlkem1024::SecretKey);

pub fn generate_kem_key_pair() -> (PublicKey, SecretKey) {
    let (pk, sk) = mlkem1024::keypair();
    (PublicKey(pk), SecretKey(sk))
}

impl PublicKey {
    /// Return the raw public key bytes (wire format, PK_SIZE bytes).
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
}

impl SecretKey {
    /// Return the raw secret key bytes (wire format, SK_SIZE bytes).
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
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
    use pqcrypto_traits::kem::{PublicKey as PubKeyTrait, SecretKey as SecKeyTrait};

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
    fn key_to_bytes_has_correct_size() {
        let (pk, sk) = generate_kem_key_pair();
        assert_eq!(pk.to_bytes().len(), PK_SIZE);
        assert_eq!(sk.to_bytes().len(), SK_SIZE);
    }

    #[test]
    fn key_to_bytes_roundtrips() {
        let (pk, sk) = generate_kem_key_pair();
        let pk_bytes = pk.to_bytes();
        let sk_bytes = sk.to_bytes();
        let pk2 = pk_from_bytes(&pk_bytes).expect("roundtrip pk");
        let sk2 = ***(&sk_bytes).expect("roundtrip sk");
        assert_eq!(pk.to_bytes(), pk2.to_bytes());
        assert_eq!(sk.to_bytes(), sk2.to_bytes());
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
