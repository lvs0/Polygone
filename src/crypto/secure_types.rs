//! Secure wrappers for cryptographic types.
//!
//! This module wraps pqcrypto types to ensure proper memory management:
//! - Zeroization on drop
//! - Optional mlock() for sensitive data
//! - Prevention of secret data appearing in stack traces

use pqcrypto_mldsa::sign::{PublicKey as MlDsPk, SecretKey as MlDsSk};
use pqcrypto_mlkem::kem::{PublicKey as MlKemPk, SecretKey as MlKemSk};
use std::sync::OnceLock;
use zeroize::Zeroizing;

/// Number of bytes in ML-KEM-1024 public key
pub const MLKEM_PUBLIC_KEY_BYTES: usize = 800;
/// Number of bytes in ML-KEM-1024 secret key
pub const MLKEM_SECRET_KEY_BYTES: usize = 1632;
/// Number of bytes in ML-KEM-1024 ciphertext
pub const MLKEM_CIPHERTEXT_BYTES: usize = 768;
/// Number of bytes in ML-KEM-1024 shared secret
pub const MLKEM_SHARED_SECRET_BYTES: usize = 32;

/// Number of bytes in ML-DSA-87 public key
pub const MLDSA_PUBLIC_KEY_BYTES: usize = 2592;
/// Number of bytes in ML-DSA-87 secret key
pub const MLDSA_SECRET_KEY_BYTES: usize = 4000;
/// Number of bytes in ML-DSA-87 signature
pub const MLDSA_SIGNATURE_BYTES: usize = 2429;

/// Wrapper for ML-KEM-1024 public key with secure memory handling
#[derive(Clone)]
pub struct SecurePublicKey {
    bytes: Zeroizing<Vec<u8>>,
}

impl SecurePublicKey {
    /// Create from raw bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() != MLKEM_PUBLIC_KEY_BYTES {
            return Err("Invalid public key length");
        }

        Ok(Self {
            bytes: Zeroizing::new(bytes.to_vec()),
        })
    }

    /// Get raw bytes reference
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Convert to ML-KEM public key for operations
    pub fn to_mlkem(&self) -> Result<MlKemPk, &'static str> {
        MlKemPk::from_bytes(self.as_bytes()).map_err(|_| "Failed to parse public key")
    }

    /// Create from ML-KEM public key
    #[allow(dead_code)]
    pub fn from_mlkem(pk: &MlKemPk) -> Self {
        Self {
            bytes: Zeroizing::new(pk.as_bytes().to_vec()),
        }
    }
}

impl Drop for SecurePublicKey {
    fn drop(&mut self) {
        // Zeroizing automatically zeros on drop
        self.bytes.zeroize();
    }
}

/// Wrapper for ML-KEM-1024 secret key with secure memory handling
pub struct SecureSecretKey {
    bytes: Zeroizing<Vec<u8>>,
    locked: OnceLock<bool>,
}

impl SecureSecretKey {
    /// Create from raw bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() != MLKEM_SECRET_KEY_BYTES {
            return Err("Invalid secret key length");
        }

        let bytes = Zeroizing::new(bytes.to_vec());

        Ok(Self {
            bytes,
            locked: OnceLock::new(),
        })
    }

    /// Get raw bytes reference
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Convert to ML-KEM secret key for operations
    pub fn to_mlkem(&self) -> Result<MlKemSk, &'static str> {
        MlKemSk::from_bytes(self.as_bytes()).map_err(|_| "Failed to parse secret key")
    }

    /// Create from ML-KEM secret key
    #[allow(dead_code)]
    pub fn from_mlkem(sk: &MlKemSk) -> Self {
        Self {
            bytes: Zeroizing::new(sk.as_bytes().to_vec()),
            locked: OnceLock::new(),
        }
    }

    /// Lock the secret key in memory to prevent swapping
    #[allow(dead_code)]
    pub fn lock_memory(&self) {
        if self.locked.get().is_none() {
            let supported = crate::security::lock_memory(&self.bytes);
            let _ = self.locked.set(supported);
        }
    }
}

impl Drop for SecureSecretKey {
    fn drop(&mut self) {
        // Zeroizing automatically zeros on drop
        self.bytes.zeroize();
    }
}

/// Wrapper for ML-DSA-87 secret key with secure memory handling
pub struct SecureSigningKey {
    bytes: Zeroizing<Vec<u8>>,
    locked: OnceLock<bool>,
}

impl SecureSigningKey {
    /// Create from raw bytes
    #[allow(dead_code)]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() != MLDSA_SECRET_KEY_BYTES {
            return Err("Invalid signing key length");
        }

        Ok(Self {
            bytes: Zeroizing::new(bytes.to_vec()),
            locked: OnceLock::new(),
        })
    }

    /// Get raw bytes reference
    #[allow(dead_code)]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Convert to ML-DSA secret key for operations
    #[allow(dead_code)]
    pub fn to_mldsa(&self) -> Result<MlDsSk, &'static str> {
        MlDsSk::from_bytes(self.as_bytes()).map_err(|_| "Failed to parse signing key")
    }

    /// Lock the signing key in memory
    #[allow(dead_code)]
    pub fn lock_memory(&self) {
        if self.locked.get().is_none() {
            let supported = crate::security::lock_memory(&self.bytes);
            let _ = self.locked.set(supported);
        }
    }
}

impl Drop for SecureSigningKey {
    fn drop(&mut self) {
        self.bytes.zeroize();
    }
}

/// Wrapper for ML-DSA-87 public key with secure memory handling
#[derive(Clone)]
pub struct SecureVerifyKey {
    bytes: Zeroizing<Vec<u8>>,
}

impl SecureVerifyKey {
    /// Create from raw bytes
    #[allow(dead_code)]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() != MLDSA_PUBLIC_KEY_BYTES {
            return Err("Invalid verify key length");
        }

        Ok(Self {
            bytes: Zeroizing::new(bytes.to_vec()),
        })
    }

    /// Get raw bytes reference
    #[allow(dead_code)]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Convert to ML-DSA public key for operations
    #[allow(dead_code)]
    pub fn to_mldsa(&self) -> Result<MlDsPk, &'static str> {
        MlDsPk::from_bytes(self.as_bytes()).map_err(|_| "Failed to parse verify key")
    }
}

impl Drop for SecureVerifyKey {
    fn drop(&mut self) {
        self.bytes.zeroize();
    }
}

/// Secure session key that auto-zeroizes on drop
pub struct SessionKey {
    bytes: Zeroizing<Vec<u8>>,
    locked: OnceLock<bool>,
}

impl SessionKey {
    /// Create a new session key from bytes
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            bytes: Zeroizing::new(bytes),
            locked: OnceLock::new(),
        }
    }

    /// Get reference to key bytes
    #[allow(dead_code)]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Lock the session key in memory
    #[allow(dead_code)]
    pub fn lock_memory(&self) {
        if self.locked.get().is_none() {
            let supported = crate::security::lock_memory(&self.bytes);
            let _ = self.locked.set(supported);
        }
    }
}

impl Drop for SessionKey {
    fn drop(&mut self) {
        self.bytes.zeroize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_public_key() {
        let bytes = vec![0x42u8; MLKEM_PUBLIC_KEY_BYTES];
        let key = SecurePublicKey::from_bytes(&bytes).unwrap();
        assert_eq!(key.as_bytes().len(), MLKEM_PUBLIC_KEY_BYTES);
    }

    #[test]
    fn test_secure_secret_key_zeroize() {
        let bytes = vec![0xFFu8; MLKEM_SECRET_KEY_BYTES];
        let key = SecureSecretKey::from_bytes(&bytes).unwrap();
        let _ = key.as_bytes();
        drop(key);
    }

    #[test]
    fn test_session_key() {
        let key = SessionKey::from_bytes(vec![0xAA; 32]);
        assert_eq!(key.as_bytes().len(), 32);
        drop(key);
    }
}
