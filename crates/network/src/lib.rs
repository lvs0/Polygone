//! Couche réseau P2P (libp2p) pour Polygone.
//!
//! Fournit la découverte Kademlia, le routage et la messagerie
//! via gossipsub/request-response, en s\u0027appuyant sur `polygone-crypto`
//! pour le chiffrement de bout en bout.

pub mod node;
pub mod behaviour;
pub use node::P2PNode;

#[cfg(test)]
mod tests;
