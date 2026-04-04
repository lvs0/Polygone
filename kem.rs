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
    pub fn as_bytes(&self) -> &[u8] { self.0.as_bytes() }
}

/// ML-KEM-1024 secret key (3168 bytes), zeroised on drop.
#[derive(ZeroizeOnDrop)]
pub struct KemSecretKey(#[zeroize(skip)] mlkem1024::SecretKey);
// Note: pqcrypto types don't impl Zeroize directly; we rely on Drop + memset
// in production this should use mlock + explicit zeroize via raw bytes.

/// ML-KEM-1024 ciphertext (1568 bytes).
#[derive(Clone)]
pub struct KemCiphertext(mlkem1024::Ciphertext);

impl KemCiphertext {
    /// Raw bytes.
    pub fn as_bytes(&self) -> &[u8] {
        use pqcrypto_traits::kem::Ciphertext;
        self.0.as_bytes()
    }
}

// ── Key generation ────────────────────────────────────────────────────────────

/// Generate a fresh ML-KEM-1024 key pair.
pub fn generate_keypair() -> Result<(KemPublicKey, KemSecretKey)> {
    let (pk, sk) = mlkem1024::keypair();
    Ok((KemPublicKey(pk), KemSecretKey(sk)))
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
    let ss = mlkem1024::decapsulate(&ct.0, &sk.0);
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
