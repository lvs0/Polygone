//! Types et structures partagées entre les crates du workspace Polygone.
//!
//! Ce crate contient les types de messages, les identifiants de nœuds,
//! les clés de session, et tout ce qui est nécessaire à la communication
//! entre `crypto`, `network` et `app`.

use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

pub use error::PolygoneError;
pub use packet::{Packet, PacketType};
pub use session::{Session, SessionKey};
pub use node::{NodeId, NodeInfo};

pub mod error;
pub mod packet;
pub mod session;
pub mod node;
