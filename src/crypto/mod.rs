//! Cryptographic primitives for the POLYGONE protocol.
//!
//! Layered design:
//!
//! ```text
//!  ┌──────────────────────────────────────────┐
//!  │  KEM (ML-KEM-1024)  ←→  Key agreement    │
//!  │  DSA (ML-DSA-87)    ←→  Signatures       │
//!  │  AES-256-GCM        ←→  Payload cipher   │
//!  │  Shamir SS          ←→  Fragment secrets  │
//!  │  BLAKE3             ←→  Hashing / VDF    │
//!  └──────────────────────────────────────────┘
//! ```

use zeroize::{Zeroize, ZeroizeOnDrop};

pub mod kem;
pub mod shamir;
pub mod sign;
pub mod symmetric;
pub mod karma;

// ── KeyPair ──────────────────────────────────────────────────────────────────

/// A combined key-pair: one KEM key-pair (transport) and one DSA key-pair (auth).
///
/// Kept together so they share the same zeroize lifecycle.
pub struct KeyPair {
    /// KEM secret key — used once per session, then destroyed.
    pub kem_sk: kem::KemSecretKey,
    /// KEM public key — shared with peer out-of-band.
    pub kem_pk: kem::KemPublicKey,
    /// Signing secret key — authorises session establishment.
    pub sign_sk: sign::SignSecretKey,
    /// Signing public key — published in the network DHT.
    pub sign_pk: sign::SignPublicKey,
}

impl KeyPair {
    /// Generate a fresh, random key-pair.
    ///
    /// Both KEM and DSA keys are generated with OS entropy via `getrandom`.
    pub fn generate() -> crate::Result<Self> {
        let (kem_pk, kem_sk) = kem::generate_keypair()?;
        let (sign_pk, sign_sk) = sign::generate_keypair()?;
        Ok(Self { kem_sk, kem_pk, sign_sk, sign_pk })
    }
}

// ── SharedSecret ─────────────────────────────────────────────────────────────

/// 32 bytes of shared secret produced by KEM, zeroised on drop.
#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct SharedSecret(pub [u8; 32]);

impl SharedSecret {
    /// Derive topology seed and symmetric session key from this shared secret.
    ///
    /// Uses two **distinct** BLAKE3 domain labels so the outputs are
    /// cryptographically independent:
    ///
    /// ```text
    /// topo_seed    = BLAKE3("polygone topology v1"    || shared_secret)  → 32 bytes
    /// session_key  = BLAKE3("polygone session key v1" || shared_secret)  → 32 bytes
    /// ```
    ///
    /// `topo_seed` is passed to `Topology::derive` — it never touches the
    /// symmetric cipher. `session_key` is passed to `SessionKey::from_bytes`
    /// — it never touches topology derivation. The two are domain-separated
    /// and independent even though they share the same KEM output.
    pub fn derive(&self) -> ([u8; 32], [u8; 32]) {
        let topo_seed    = blake3::derive_key("polygone topology v1",    &self.0);
        let session_key  = blake3::derive_key("polygone session key v1", &self.0);
        (topo_seed, session_key)
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keypair_generation_is_deterministically_fresh() {
        let kp1 = KeyPair::generate().unwrap();
        let kp2 = KeyPair::generate().unwrap();
        // Public keys must differ (overwhelming probability)
        assert_ne!(kp1.kem_pk.as_bytes(), kp2.kem_pk.as_bytes());
    }

    #[test]
    fn shared_secret_derivation_is_deterministic() {
        let secret = SharedSecret([0xAB; 32]);
        let (t1, k1) = secret.derive();
        let (t2, k2) = secret.derive();
        assert_eq!(t1, t2);
        assert_eq!(k1, k2);
    }

    #[test]
    fn topology_and_session_key_are_distinct() {
        let secret = SharedSecret([0xCD; 32]);
        let (topo, key) = secret.derive();
        // The two derived values must differ
        assert_ne!(&topo[..], &key[..topo.len()]);
    }
}
