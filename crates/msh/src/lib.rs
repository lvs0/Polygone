use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub type ModelId = String; // BLAKE3 hash
pub type Signature = String; // Ed25519 signature (repr hex)
pub type Blake3Hash = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MshMessage {
    Announce(Announce),
    Request(Request),
    Transfer(Transfer),
    Ack(Ack),
    Error(Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope {
    pub t: String, // "announce"|"request"|"transfer"|"ack"|"error"
    pub id: String,
    pub model_id: Option<ModelId>,
    pub chunk: Option<ChunkInfo>,
    pub payload: Option<Vec<u8>>,
    pub sig: Option<Signature>,
    pub nonce: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announce {
    pub model_id: ModelId,
    pub size: u64,
    pub chunks: u32,
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub model_id: ModelId,
    pub offset: u64,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfer {
    pub model_id: ModelId,
    pub chunk: ChunkInfo,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ack {
    pub model_id: ModelId,
    pub offset: u64,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub model_id: Option<ModelId>,
    pub code: u16,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkInfo {
    pub offset: u64,
    pub size: u64,
    pub total: u64,
    pub hash: String, // BLAKE3 du chunk
}

#[derive(Debug, Clone, Serialize)]
pub struct NodeStatus {
    pub node_id: String,
    pub status: String,
    pub peers_connected: usize,
    pub uptime_seconds: u64,
    pub poly_balance: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PeerInfo {
    pub id: String,
    pub address: String,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ServiceInfo {
    pub name: String,
    pub active: bool,
}

#[derive(Clone)]
pub struct AppState {
    pub status: NodeStatus,
    pub peers: Vec<PeerInfo>,
    pub services: Vec<ServiceInfo>,
    pub started_at: u64,
}

impl AppState {
    pub fn fresh() -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Self {
            status: NodeStatus {
                node_id: "unknown".to_string(),
                status: "starting".to_string(),
                peers_connected: 0,
                uptime_seconds: 0,
                poly_balance: 0.0,
            },
            peers: vec![],
            services: vec![
                ServiceInfo { name: "Drive".to_string(), active: true },
                ServiceInfo { name: "Msg".to_string(), active: true },
                ServiceInfo { name: "Mesh".to_string(), active: false },
            ],
            started_at: now,
        }
    }

    pub fn update_uptime(&mut self) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        self.status.uptime_seconds = now.saturating_sub(self.started_at);
    }
}

impl Envelope {
    pub fn new(t: &str) -> Self {
        Self {
            t: t.to_string(),
            id: Uuid::new_v4().to_string(),
            model_id: None,
            chunk: None,
            payload: None,
            sig: None,
            nonce: Uuid::new_v4().to_string(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }

    pub fn sign(&mut self, _secret_key: &[u8]) {
        // TODO: implémenter signature Ed25519 réelle
        self.sig = Some("sig_placeholder".to_string());
    }

    pub fn verify(&self) -> bool {
        // TODO: vérifier signature et anti-rejeu
        true
    }
}

/// Utilitaire de hachage BLAKE3
pub fn blake3_hash(data: &[u8]) -> Blake3Hash {
    blake3::hash(data).to_string()
}
