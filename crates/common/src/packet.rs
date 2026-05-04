use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PacketType {
    Handshake,
    Data,
    Ack,
    Close,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packet {
    pub packet_type: PacketType,
    pub source: crate::NodeId,
    pub destination: crate::NodeId,
    pub payload: Vec<u8>,
    pub nonce: [u8; 12], // pour AES-GCM
    pub signature: Vec<u8>,
}
