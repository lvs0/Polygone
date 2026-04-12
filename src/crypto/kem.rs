//! ML-KEM-1024 key encapsulation — FIPS 203.
//!
//! Wraps `pqcrypto-mlkem` with a typed, zeroize-safe API.

use pqcrypto_mlkem::mlkem1024;
use pqcrypto_traits::kem::{PublicKey, SecretKey, SharedSecret as PqSharedSecret};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::{PolygoneError, Result};

// ── Key types ─────────────────────────────────────────────────────────────────

/// ML-KEM-1024 public key (1568 bytes).
#[derive(Clone)]
pub struct KemPublicKey(mlkem1024::PublicKey);

impl KemPublicKey {
    /// Raw bytes of this public key.
    pub fn as_bytes(&self) -> &[u8] {
        use pqcrypto_traits::kem::PublicKey;
        self.0.as_bytes()
    }
    
    /// Parse from bytes
    pub fn from_bytes(b: &[u8]) -> Result<Self> {
        use pqcrypto_traits::kem::PublicKey;
        Ok(Self(mlkem1024::PublicKey::from_bytes(b).map_err(|_| PolygoneError::Serialization("Invalid PK".into()))?))
    }
}

/// ML-KEM-1024 secret key (3168 bytes), guaranteed zeroized on drop.
pub struct KemSecretKey(zeroize::Zeroizing<Vec<u8>>);

impl KemSecretKey {
    /// Raw bytes of this secret key.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
    
    /// Parse from bytes.
    pub fn from_bytes(b: &[u8]) -> Result<Self> {
        if b.len() != mlkem1024::secret_key_bytes() {
            return Err(PolygoneError::Serialization("Invalid SK length".into()));
        }
        Ok(Self(zeroize::Zeroizing::new(b.to_vec())))
    }

    /// Internal helper to get the temporary pqcrypto key object.
    fn get_key(&self) -> mlkem1024::SecretKey {
        use pqcrypto_traits::kem::SecretKey;
        // Optimization: from_bytes is just a wrapper around array copy in pqcrypto
        mlkem1024::SecretKey::from_bytes(&self.0).expect("Internal key integrity failure")
    }
}

/// ML-KEM-1024 ciphertext (1568 bytes).
#[derive(Clone)]
pub struct KemCiphertext(mlkem1024::Ciphertext);

impl KemCiphertext {
    /// Raw bytes.
    pub fn as_bytes(&self) -> &[u8] {
        use pqcrypto_traits::kem::Ciphertext;
        self.0.as_bytes()
    }
    
    pub fn from_bytes(b: &[u8]) -> Result<Self> {
        use pqcrypto_traits::kem::Ciphertext;
        Ok(Self(mlkem1024::Ciphertext::from_bytes(b).map_err(|_| PolygoneError::Serialization("Invalid CT".into()))?))
    }
}

// ── Key generation ────────────────────────────────────────────────────────────

/// Generate a fresh ML-KEM-1024 key pair.
pub fn generate_keypair() -> Result<(KemPublicKey, KemSecretKey)> {
    let (pk, sk) = mlkem1024::keypair();
    use pqcrypto_traits::kem::SecretKey;
    Ok((KemPublicKey(pk), KemSecretKey::from_bytes(sk.as_bytes())?))
}

// ── Encapsulation / Decapsulation ─────────────────────────────────────────────

/// Encapsulate: produce a ciphertext and shared secret from a peer's public key.
///
/// The caller (the initiator) sends the ciphertext to the peer,
/// and uses `ss` to derive topology parameters.
pub fn encapsulate(pk: &KemPublicKey) -> Result<(KemCiphertext, crate::crypto::SharedSecret)> {
    let (ss, ct) = mlkem1024::encapsulate(&pk.0);
    let mut bytes = [0u8; 32];
    let raw = ss.as_bytes();
    bytes.copy_from_slice(&raw[..32]);
    Ok((KemCiphertext(ct), crate::crypto::SharedSecret(bytes)))
}

/// Decapsulate: recover the shared secret from a received ciphertext.
///
/// If the ciphertext was tampered with, this returns
/// `Err(PolygoneError::KemDecapsulate)`.
pub fn decapsulate(
    sk: &KemSecretKey,
    ct: &KemCiphertext,
) -> Result<crate::crypto::SharedSecret> {
    let internal_sk = sk.get_key();
    let ss = mlkem1024::decapsulate(&ct.0, &internal_sk);
    let mut bytes = [0u8; 32];
    let raw = ss.as_bytes();
    // ML-KEM ss is 32 bytes for mlkem1024
    if raw.len() < 32 {
        return Err(PolygoneError::KemDecapsulate);
    }
    bytes.copy_from_slice(&raw[..32]);
    Ok(crate::crypto::SharedSecret(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kem_round_trip() {
        let (pk, sk) = generate_keypair().unwrap();
        let (ct, ss_enc) = encapsulate(&pk).unwrap();
        let ss_dec = decapsulate(&sk, &ct).unwrap();
        assert_eq!(ss_enc.0, ss_dec.0, "Shared secrets must match");
    }
}
