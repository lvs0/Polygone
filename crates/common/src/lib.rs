//! Types et structures partagées entre les crates du workspace Polygone.
//!
//! Ce crate contient les types de messages, les identifiants de nœuds,
//! les clés de session, et tout ce qui est nécessaire à la communication
//! entre `crypto`, `network` et `app`.

pub use error::PolygoneError;
pub use fragment::{
    CollectRequest, CollectedFragments, DispatchConfig, DispatchResult, FragmentAck, FragmentId,
    FragmentPayload,
};
pub use node::{NodeId, NodeInfo};
pub use packet::{Packet, PacketType};
pub use session::{Session, SessionKey};

pub mod sync;
pub use sync::{Capabilities, ConsensusEngine, DistributedState, LWWRegister, SyncNode, TwoPSet};

pub mod lc_wds;
pub use lc_wds::{
    assign_cluster, elect_global_leader, elect_tire_leader, ClusterMember, ClusterTopology,
    LatencyCluster, LatencyRtt, LcWdsRound,
};

pub mod error;
pub mod fragment;
pub mod node;
pub mod packet;
pub mod session;
