//! Cryptographie post-quantique et symétrique pour Polygone.
//!
//! Fournit l’encapsulation/décapsulation ML-KEM,
//! le chiffrement AES-256-GCM, le secret-sharing Shamir
//! et le hachage BLAKE3.

pub use kem::{decapsulate, encapsulate, generate_kem_key_pair, PublicKey, SecretKey};
pub use shamir::{reconstruct_secret, split_secret};
pub use symmetric::{decrypt, encrypt, SymmetricError};
pub use hash::hash_data;

mod kem;
mod symmetric;
mod shamir;
mod hash;
