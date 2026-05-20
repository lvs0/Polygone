use libp2p::{
    identify, kad, request_response, swarm::NetworkBehaviour, StreamProtocol
};
use serde::{Deserialize, Serialize};

/// Sequential inference request.
/// Contains the transient hidden states (tensors) moving through the model's layers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceRequest {
    pub session_id: [u8; 16],
    /// The index of the first layer this node should compute.
    pub start_layer: u32,
    /// The index of the last layer this node should compute.
    pub end_layer: u32,
    /// Serialized hidden states tensor (the activation "wave").
    pub hidden_states_data: Vec<u8>,
    /// Tensor dimensions for reconstruction.
    pub dims: Vec<usize>,
}

/// Response signaling the successful computation of a model segment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResponse {
    pub success: bool,
    /// Logits or processed hidden states (returned only at the end of the pipeline).
    pub outgoing_data: Option<Vec<u8>>,
}

#[derive(NetworkBehaviour)]
pub struct PetalsBehaviour {
    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,
    pub identify: identify::Behaviour,
    pub request_response: request_response::cbor::Behaviour<InferenceRequest, InferenceResponse>,
}

pub fn build_swarm(keypair: libp2p::identity::Keypair) -> anyhow::Result<libp2p::Swarm<PetalsBehaviour>> {
    let local_peer_id = libp2p::PeerId::from(keypair.public());
    
    let kad_store = kad::store::MemoryStore::new(local_peer_id);
    let mut kad_config = kad::Config::default();
    kad_config.set_protocol_names(vec![StreamProtocol::new("/pg-petals/kad/1.0.0")]);
    let kademlia = kad::Behaviour::with_config(local_peer_id, kad_store, kad_config);

    let identify = identify::Behaviour::new(identify::Config::new(
        "/pg-petals/id/1.0.0".into(),
        keypair.public(),
    ));

    let protocols = [(StreamProtocol::new("/pg-petals/infer/1.0.0"), request_response::ProtocolSupport::Full)];
    let cfg = request_response::Config::default();
    let request_response = request_response::cbor::Behaviour::new(protocols, cfg);

    let behaviour = PetalsBehaviour {
        kademlia,
        identify,
        request_response,
    };

    let swarm = libp2p::SwarmBuilder::with_existing_identity(keypair)
        .with_tokio()
        .with_tcp(
            libp2p::tcp::Config::default(),
            libp2p::noise::Config::new,
            libp2p::yamux::Config::default,
        )?
        .with_behaviour(|_| behaviour)?
        .with_swarm_config(|c| c.with_idle_connection_timeout(std::time::Duration::from_secs(60)))
        .build();

    Ok(swarm)
}
