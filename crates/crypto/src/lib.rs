//! Cryptographie post-quantique et symétrique pour Polygone.
//!
//! Fournit l'encapsulation/décapsulation ML-KEM,
//! le chiffrement AES-256-GCM, le secret-sharing Shamir
//! et le hachage BLAKE3.

pub use kem::{decapsulate, encapsulate, generate_kem_key_pair, pk_from_bytes, sk_from_bytes, PublicKey, SecretKey};
pub use shamir::{reconstruct_secret, split_secret, Fragment as ShamirFragment};
pub use symmetric::{decrypt, encrypt, SymmetricError};
pub use hash::hash_data;
pub use cascade::{Cascade, CascadeConfig, CascadeMetrics, OpComplexity, SecurityLevel, KemKeyCache, RoutingDecision};

pub mod kem;
pub mod symmetric;
pub mod shamir;
pub mod hash;
pub mod cascade;
