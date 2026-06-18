//! Types et structures partagées entre les crates du workspace Polygone.
//!
//! Ce crate contient les types de messages, les identifiants de nœuds,
//! les clés de session, et tout ce qui est nécessaire à la communication
//! entre `crypto`, `network` et `app`.

pub use error::PolygoneError;
pub use packet::{Packet, PacketType};
pub use session::{Session, SessionKey};
pub use node::{NodeId, NodeInfo};
pub use fragment::{
 FragmentId, FragmentPayload, DispatchResult, FragmentAck,
 CollectRequest, CollectedFragments, DispatchConfig,
};

pub mod sync;
pub use sync::{
    Capabilities, SyncNode, TwoPSet, LWWRegister,
    DistributedState, ConsensusEngine,
};

pub mod error;
pub mod packet;
pub mod session;
pub mod node;
pub mod fragment;
