//! ML-KEM-1024 key encapsulation — FIPS 203.
//!
//! Wraps `pqcrypto-mlkem` with a typed, zeroize-safe API.

use pqcrypto_mlkem::mlkem1024;
use pqcrypto_traits::kem::{PublicKey, SecretKey, Ciphertext as PqcCiphertext, SharedSecret as PqcSharedSecret};

use crate::{PolygoneError, Result, SharedSecret};

// ── Byte sizes ────────────────────────────────────────────────────────────────
pub const EK_SIZE: usize = 1568;
pub const DK_SIZE: usize = 3168;
pub const CT_SIZE: usize = 1568;
pub const SS_SIZE: usize = 32;

// ── Key types ─────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct KemPublicKey(mlkem1024::PublicKey);

impl KemPublicKey {
    pub fn as_bytes(&self) -> &[u8] { PublicKey::as_bytes(&self.0) }
    pub fn to_hex(&self) -> String { hex::encode(self.as_bytes()) }
    pub fn from_hex(s: &str) -> Result<Self> {
        let bytes = hex::decode(s.trim())
            .map_err(|e| PolygoneError::KeyFile(format!("hex decode: {e}")))?;
        Self::from_bytes(&bytes)
    }
    pub fn from_bytes(b: &[u8]) -> Result<Self> {
        Ok(Self(PublicKey::from_bytes(b).map_err(|_| {
            PolygoneError::KeyFile("Invalid ML-KEM-1024 public key".into())
        })?))
    }
}

// NOTE: We cannot derive ZeroizeOnDrop here because mlkem1024::SecretKey
// does not implement Zeroize (pqcrypto types use fixed-size arrays internally).
// The SecretKey bytes ARE copied to our heap-allocated buffer during from_bytes(),
// and the original pqcrypto SecretKey is dropped immediately. So the sensitive
// data lives in our [u8; DK_SIZE] buffer which IS zeroized on Drop via the
// manual Drop impl below.
pub struct KemSecretKey {
    bytes: zeroize::Zeroizing<[u8; DK_SIZE]>,
}

impl Drop for KemSecretKey {
    fn drop(&mut self) {
        // Zeroizing auto-zeroizes on drop
    }
}

impl KemSecretKey {
    pub fn as_bytes(&self) -> &[u8] { &*self.bytes }
    pub fn to_hex(&self) -> String { hex::encode(self.as_bytes()) }
    pub fn from_hex(s: &str) -> Result<Self> {
        let bytes = hex::decode(s.trim())
            .map_err(|e| PolygoneError::KeyFile(format!("hex decode: {e}")))?;
        Self::from_bytes(&bytes)
    }
    pub fn from_bytes(b: &[u8]) -> Result<Self> {
        let mut bytes = [0u8; DK_SIZE];
        bytes.copy_from_slice(b);
        Ok(Self { bytes: zeroize::Zeroizing::new(bytes) })
    }
}


#[derive(Clone)]
pub struct KemCiphertext(mlkem1024::Ciphertext);

impl KemCiphertext {
    pub fn as_bytes(&self) -> &[u8] { PqcCiphertext::as_bytes(&self.0) }
    pub fn to_hex(&self) -> String { hex::encode(self.as_bytes()) }
    pub fn from_hex(s: &str) -> Result<Self> {
        let bytes = hex::decode(s.trim())
            .map_err(|e| PolygoneError::KeyFile(format!("hex decode: {e}")))?;
        Self::from_bytes(&bytes)
    }
    pub fn from_bytes(b: &[u8]) -> Result<Self> {
        Ok(Self(mlkem1024::Ciphertext::from_bytes(b).map_err(|_| {
            PolygoneError::KeyFile("Invalid ML-KEM-1024 ciphertext".into())
        })?))
    }
}

// ── Operations ────────────────────────────────────────────────────────────────

pub fn generate_keypair() -> Result<(KemPublicKey, KemSecretKey)> {
    let (pk, sk) = mlkem1024::keypair();
    Ok((KemPublicKey(pk), KemSecretKey {
        bytes: zeroize::Zeroizing::new({
            let mut b = [0u8; DK_SIZE];
            b.copy_from_slice(SecretKey::as_bytes(&sk));
            b
        })
    }))
}

pub fn encapsulate(pk: &KemPublicKey) -> Result<(KemCiphertext, SharedSecret)> {
    let (ss, ct) = mlkem1024::encapsulate(&pk.0);
    // Use as_bytes via the trait in scope
    let raw = PqcSharedSecret::as_bytes(&ss);
    let mut bytes = [0u8; SS_SIZE];
    bytes.copy_from_slice(&raw[..SS_SIZE]);
    Ok((KemCiphertext(ct), SharedSecret(bytes)))
}

pub fn decapsulate(sk: &KemSecretKey, ct: &KemCiphertext) -> Result<SharedSecret> {
    // Reconstruct raw SecretKey from our stored bytes
    let raw_sk = SecretKey::from_bytes(&*sk.bytes).map_err(|_| {
        PolygoneError::KemDecapsulate
    })?;
    let ss = mlkem1024::decapsulate(&ct.0, &raw_sk);
    let raw = PqcSharedSecret::as_bytes(&ss);
    if raw.len() < SS_SIZE {
        return Err(PolygoneError::KemDecapsulate);
    }
    let mut bytes = [0u8; SS_SIZE];
    bytes.copy_from_slice(&raw[..SS_SIZE]);
    Ok(SharedSecret(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ml_kem_1024_round_trip() {
        let (pk, sk) = generate_keypair().unwrap();
        let (ct, ss1) = encapsulate(&pk).unwrap();
        let ss2 = decapsulate(&sk, &ct).unwrap();
        assert_eq!(ss1, ss2);
    }

    #[test]
    fn ml_kem_1024_keygen_consistent() {
        let (pk1, sk1) = generate_keypair().unwrap();
        let (pk2, sk2) = generate_keypair().unwrap();
        // Different keys each time
        assert_ne!(pk1.as_bytes(), pk2.as_bytes());
        assert_ne!(sk1.as_bytes(), sk2.as_bytes());
    }

    #[test]
    fn ml_kem_1024_hex_roundtrip() {
        let (pk, sk) = generate_keypair().unwrap();
        let pk_hex = pk.to_hex();
        let sk_hex = sk.to_hex();
        let pk2 = KemPublicKey::from_hex(&pk_hex).unwrap();
        let sk2 = KemSecretKey::from_hex(&sk_hex).unwrap();
        assert_eq!(pk.as_bytes(), pk2.as_bytes());
        assert_eq!(sk.as_bytes(), sk2.as_bytes());
    }
}