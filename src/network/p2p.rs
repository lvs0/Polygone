//! libp2p v0.56 P2P networking layer for Polygone.
//!
//! Build with: cargo build --features server
//! Requires: RUST_LOG=info for logging output.

use libp2p::{
    identify::{Behaviour as Identify, Config as IdentifyConfig},
    identity::Keypair,
    kad::{
        store::MemoryStore, Behaviour as Kademlia, Config as KademliaConfig, Mode,
    },
    noise, tcp, yamux, PeerId, StreamProtocol, Swarm, SwarmBuilder, dns,
};
use std::time::Duration;
use std::path::Path;
use std::fs;
use libp2p::swarm::NetworkBehaviour;

// ─── Identity ────────────────────────────────────────────────────────────────

/// Load a libp2p identity keypair from a file, or generate and persist a new one.
pub fn load_or_generate_identity(path: &Path) -> anyhow::Result<Keypair> {
    if path.exists() {
        let bytes = fs::read(path)?;
        let keypair = Keypair::from_protobuf_encoding(&bytes)
            .map_err(|e| anyhow::anyhow!("Failed to decode identity key: {}", e))?;
        println!("  ✓ Loaded persistent identity from {}", path.display());
        Ok(keypair)
    } else {
        let keypair = Keypair::generate_ed25519();
        let bytes = keypair.to_protobuf_encoding()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, bytes)?;
        println!("  ✓ Generated and saved new identity to {}", path.display());
        Ok(keypair)
    }
}

// ─── Behaviour ───────────────────────────────────────────────────────────────

#[derive(NetworkBehaviour)]
pub struct PolygoneBehaviour {
    pub kademlia: Kademlia<MemoryStore>,
    pub identify: Identify,
}

// ─── Swarm builder ────────────────────────────────────────────────────────────

pub async fn build_swarm(
    keypair: Keypair,
) -> anyhow::Result<Swarm<PolygoneBehaviour>> {
    // TCP transport with async I/O
    let tcp_transport = tcp::Transport::new(tcp::Config::default());

    // DNS-backed transport
    let transport = dns::Transport::new(tcp_transport, dns::Config::system());

    let local_peer_id = PeerId::from(keypair.public());

    // ─── Kademlia DHT ───────────────────────────────────────────────────────
    let store = MemoryStore::new(local_peer_id);
    let mut kad_config = KademliaConfig::default();
    kad_config.set_protocol_names(vec![StreamProtocol::new("/polygone/kad/1.1.0")]);
    kad_config.set_record_ttl(Some(Duration::from_secs(30)));
    kad_config.set_provider_record_ttl(Some(Duration::from_secs(30)));
    let mut kademlia = Kademlia::with_config(local_peer_id, store, kad_config);
    kademlia.set_mode(Some(Mode::Server));

    // ─── Identify protocol ────────────────────────────────────────────────────
    let identify_config = IdentifyConfig::new("/polygone/1.1.0".to_string(), keypair.public())
        .with_push_listen_addr_updates(true);
    let identify = Identify::new(identify_config);

    let behaviour = PolygoneBehaviour {
        kademlia,
        identify,
    };

    // ─── Build swarm ─────────────────────────────────────────────────────────
    let swarm = SwarmBuilder::new(transport, behaviour, local_peer_id)
        .idle_timeout(std::time::Duration::from_secs(60))
        .build();

    Ok(swarm)
}
