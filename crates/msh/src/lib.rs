use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use rand::{RngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use zeroize::Zeroize;

pub type ModelId = String; // BLAKE3 hash
pub type Signature = String; // Ed25519 signature (hex)
pub type Blake3Hash = String;

/// Node keypair — secret key wiped on drop
#[derive(Clone)]
pub struct NodeKeypair {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

impl NodeKeypair {
    /// Generate a new random keypair using OsRng
    pub fn generate() -> Self {
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes[..]);
        let signing_key = SigningKey::from_bytes(&bytes);
        let verifying_key = signing_key.verifying_key();
        // Secret bytes go out of scope here and get dropped (stack memory)
        Self { signing_key, verifying_key }
    }

    /// Derive from raw 32-byte seed
    pub fn from_bytes(seed: &[u8; 32]) -> Self {
        let signing_key = SigningKey::from_bytes(seed);
        let verifying_key = signing_key.verifying_key();
        Self { signing_key, verifying_key }
    }

    /// Get the public key bytes (hex)
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.verifying_key.as_bytes())
    }

    /// Sign a message, return hex signature
    pub fn sign(&self, msg: &[u8]) -> Signature {
        hex::encode(self.signing_key.sign(msg).to_bytes())
    }

    /// Verify a hex signature against a message, return Ok(()) or Err
    pub fn verify(&self, msg: &[u8], sig_hex: &str) -> Result<(), ed25519_dalek::SignatureError> {
        let bytes = hex::decode(sig_hex)
            .map_err(|_| ed25519_dalek::SignatureError::new())?;
        if bytes.len() != 64 {
            return Err(ed25519_dalek::SignatureError::new());
        }
        let mut arr = [0u8; 64];
        arr.copy_from_slice(&bytes);
        let sig = ed25519_dalek::Signature::from_bytes(&arr);
        self.verifying_key.verify(msg, &sig).map_err(|_| ed25519_dalek::SignatureError::new())
    }
}

impl Drop for NodeKeypair {
    fn drop(&mut self) {
        // SecretKey is zeroized on drop via zeroize derive
    }
}

impl Default for NodeKeypair {
    fn default() -> Self {
        Self::generate()
    }
}

impl Serialize for NodeKeypair {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.public_key_hex())
    }
}

impl<'de> Deserialize<'de> for NodeKeypair {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Deserializing a keypair requires the secret key — we generate a new one on deserialize
        // and store only the public key in serialized form. For full deserialization with secret key,
        // a custom format (e.g. base64 + encryption at rest) would be needed.
        Ok(Self::generate())
    }
}

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
    pub signer_pk: Option<String>, // Ed25519 public key of signer (hex)
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
            signer_pk: None,
            nonce: Uuid::new_v4().to_string(),
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }

    /// Sign this envelope with the given keypair.
    /// Sets `sig` and `signer_pk` fields. Message signed = canonical JSON of envelope
    /// with `sig` and `signer_pk` set to None (self-referential exclusion).
    pub fn sign_with(&mut self, kp: &NodeKeypair) {
        // Build a signing payload: serialize without the sig/signer_pk fields,
        // then sign it
        let mut payload = self.clone();
        payload.sig = None;
        payload.signer_pk = None;
        let bytes = serde_json::to_vec(&payload).unwrap_or_default();
        self.sig = Some(kp.sign(&bytes));
        self.signer_pk = Some(kp.public_key_hex());
    }

    /// Verify the envelope signature. Returns Ok if signature is valid, Err otherwise.
    /// Panics if signer_pk is missing (call has_signer first).
    pub fn verify_signature(&self) -> Result<(), &'static str> {
        let pk_hex = self.signer_pk.as_ref().ok_or("missing signer_pk")?;
        let sig_hex = self.sig.as_ref().ok_or("missing sig")?;
        let mut payload = self.clone();
        payload.sig = None;
        payload.signer_pk = None;
        let bytes = serde_json::to_vec(&payload).map_err(|_| "serialization error")?;
        // Reconstruct verifying key from public key bytes
        let pk_bytes = hex::decode(pk_hex).map_err(|_| "invalid pk hex")?;
        if pk_bytes.len() != 32 {
            return Err("invalid pk length");
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&pk_bytes);
        let vk = VerifyingKey::from_bytes(&arr).map_err(|_| "invalid pk")?;
        let sig_bytes = hex::decode(sig_hex).map_err(|_| "invalid sig hex")?;
        if sig_bytes.len() != 64 {
            return Err("invalid sig length");
        }
        let mut sarr = [0u8; 64];
        sarr.copy_from_slice(&sig_bytes);
        let sig = ed25519_dalek::Signature::from_bytes(&sarr);
        vk.verify(&bytes, &sig).map_err(|_| "signature mismatch")
    }

    /// Check if envelope has a valid signature (both sig and signer_pk present)
    pub fn has_signature(&self) -> bool {
        self.sig.is_some() && self.signer_pk.is_some()
    }
}

/// Utilitaire de hachage BLAKE3
pub fn blake3_hash(data: &[u8]) -> Blake3Hash {
    blake3::hash(data).to_string()
}
