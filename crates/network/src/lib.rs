//! Couche réseau P2P (libp2p) pour Polygone.
//!
//! Fournit la découverte Kademlia, le routage et la messagerie
//! via gossipsub/request-response, en s'appuyant sur `polygone-crypto`
//! pour le chiffrement de bout en bout.
//!
//! # Modules
//!
//! - `node` — P2P node stub (minimal for compilation).
//! - `behaviour` — libp2p NetworkBehaviour composition.
//! - `dispatch` — Fragment dispatch orchestrator (Shamir → encrypt → route → collect).

pub mod node;
pub mod behaviour;
pub mod dispatch;

pub use node::P2PNode;
pub use dispatch::{FragmentDispatcher, DispatchError};

#[cfg(test)]
mod tests;
