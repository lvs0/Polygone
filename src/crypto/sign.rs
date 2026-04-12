//! ML-DSA-87 digital signatures — FIPS 204.

use crate::{PolygoneError, Result};
use pqcrypto_mldsa::mldsa87;
use pqcrypto_traits::sign::{PublicKey, SecretKey, SignedMessage};
use zeroize::ZeroizeOnDrop;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// ML-DSA-87 public key.
#[derive(Clone, Debug)]
pub struct SignPublicKey(mldsa87::PublicKey);

impl Serialize for SignPublicKey {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(self.0.as_bytes())
    }
}

impl<'de> Deserialize<'de> for SignPublicKey {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: Vec<u8> = Deserialize::deserialize(deserializer)?;
        let pk = mldsa87::PublicKey::from_bytes(&bytes).map_err(serde::de::Error::custom)?;
        Ok(SignPublicKey(pk))
    }
}

impl SignPublicKey {
    /// Raw bytes of the public key.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Parse from bytes.
    ///
    /// # Errors
    /// Returns error if bytes are invalid.
    pub fn from_bytes(b: &[u8]) -> Result<Self> {
        Ok(Self(mldsa87::PublicKey::from_bytes(b).map_err(|_| {
            PolygoneError::Serialization("Invalid Sign PK".into())
        })?))
    }
}

/// ML-DSA-87 secret key (sensitive).
/// Zeroized on drop for memory safety.
pub struct SignSecretKey {
    bytes: zeroize::Zeroizing<Vec<u8>>,
}

impl SignSecretKey {
    /// Create from a raw secret key.
    #[must_use]
    pub fn from_secret_key(sk: mldsa87::SecretKey) -> Self {
        Self {
            bytes: zeroize::Zeroizing::new(sk.as_bytes().to_vec()),
        }
    }

    /// Raw bytes of the secret key.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Parse from bytes.
    ///
    /// # Errors
    /// Returns error if bytes are invalid.
    pub fn from_bytes(b: &[u8]) -> Result<Self> {
        mldsa87::SecretKey::from_bytes(b)
            .map_err(|_| PolygoneError::Serialization("Invalid Sign SK".into()))?;
        Ok(Self {
            bytes: zeroize::Zeroizing::new(b.to_vec()),
        })
    }
}

impl ZeroizeOnDrop for SignSecretKey {}

/// A detached signature (4627 bytes for ML-DSA-87).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Signature(Vec<u8>);

impl Signature {
    /// Raw bytes of the signature.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

/// Generate a fresh ML-DSA-87 key pair.
#[must_use]
pub fn generate_keypair() -> Result<(SignPublicKey, SignSecretKey)> {
    let (pk, sk) = mldsa87::keypair();
    Ok((SignPublicKey(pk), SignSecretKey::from_secret_key(sk)))
}

/// Sign arbitrary bytes. Returns a detached signature.
#[must_use]
pub fn sign(sk: &SignSecretKey, message: &[u8]) -> Signature {
    let sk_inner = mldsa87::SecretKey::from_bytes(sk.as_bytes())
        .map_err(|_| PolygoneError::SignatureInvalid)
        .unwrap();
    let signed = mldsa87::sign(message, &sk_inner);
    let sig_bytes = signed.as_bytes()[..signed.as_bytes().len() - message.len()].to_vec();
    Signature(sig_bytes)
}

/// Verify a detached signature.
///
/// # Errors
/// Returns error if signature verification fails.
pub fn verify(pk: &SignPublicKey, message: &[u8], sig: &Signature) -> Result<()> {
    let mut combined = sig.0.clone();
    combined.extend_from_slice(message);
    mldsa87::open(
        &mldsa87::SignedMessage::from_bytes(&combined)
            .map_err(|_| PolygoneError::SignatureInvalid)?,
        &pk.0,
    )
    .map_err(|_| PolygoneError::SignatureInvalid)?;
    Ok(())
}
