//! # Polygone Core
//!
//! Crate d'unification qui réexporte tous les types publics
//! des crates fondamentales du workspace Polygone.
//!
//! Les crates supérieures (`polygone-brain`, `polygone-petals`, `polygone-shell`)
//! dépendent de `polygone-core` pour accéder à la surface publique complète.

pub use polygone_common::{
    error::PolygoneError,
    packet::{Packet, PacketType},
    session::{Session, SessionKey},
    node::{NodeId, NodeInfo},
    fragment::{
        FragmentId, FragmentPayload, DispatchResult, FragmentAck,
        CollectRequest, CollectedFragments, DispatchConfig,
    },
};

pub use polygone_crypto::{
    kem::{decapsulate, encapsulate, generate_kem_key_pair, PublicKey, SecretKey},
    shamir::{reconstruct_secret, split_secret, Fragment as ShamirFragment},
    symmetric::{decrypt, encrypt, SymmetricError},
    hash::hash_data,
};

pub use polygone_network::{
    node::P2PNode,
    dispatch::{FragmentDispatcher, DispatchError},
};
