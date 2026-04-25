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
    dns, tcp, PeerId, Swarm, SwarmBuilder,
};
use std::path::Path;
use std::time::Duration;

// ─── Identity ────────────────────────────────────────────────────────────────

/// Load a libp2p identity keypair from a file, or generate and persist a new one.
pub async fn load_or_generate_identity(path: &Path) -> anyhow::Result<Keypair> {
    if path.exists() {
        let bytes = tokio::fs::read(path).await?;
        let keypair = Keypair::from_protobuf_encoding(&bytes)
            .map_err(|e| anyhow::anyhow!("Failed to decode identity key: {}", e))?;
        println!("  ✓ Loaded persistent identity from {}", path.display());
        Ok(keypair)
    } else {
        let keypair = Keypair::generate_ed25519();
        let bytes = keypair.to_protobuf_encoding()?;
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::write(path, bytes).await?;
        println!("  ✓ Generated and saved new identity to {}", path.display());
        Ok(keypair)
    }
}

// ─── Behaviour ───────────────────────────────────────────────────────────────

// Use the NetworkBehaviour derive macro from libp2p-swarm-derive
use libp2p_swarm_derive::NetworkBehaviour;

#[derive(NetworkBehaviour)]
#[behaviour(prelude = "libp2p::swarm::derive_prelude")]
pub struct PolygoneBehaviour {
    #[behaviour(ignore)]
    pub kademlia: Kademlia<MemoryStore>,
    #[behaviour(ignore)]
    pub identify: Identify,
}

impl PolygoneBehaviour {
    pub fn new(kademlia: Kademlia<MemoryStore>, identify: Identify) -> Self {
        Self { kademlia, identify }
    }
}

// ─── Swarm builder ────────────────────────────────────────────────────────────

pub async fn build_swarm(
    keypair: Keypair,
) -> anyhow::Result<Swarm<PolygoneBehaviour>> {
    let local_peer_id = PeerId::from(keypair.public());

    // ─── Kademlia DHT ───────────────────────────────────────────────────────
    let store = MemoryStore::new(local_peer_id);
    let kad_config = KademliaConfig::default();
    let mut kademlia = Kademlia::with_config(local_peer_id, store, kad_config);
    kademlia.set_mode(Some(Mode::Server));

    // ─── Identify protocol ────────────────────────────────────────────────────
    let identify_config = IdentifyConfig::new("/polygone/1.1.0".to_string(), keypair.public())
        .with_push_listen_addr_updates(true);
    let identify = Identify::new(identify_config);

    let behaviour = PolygoneBehaviour::new(kademlia, identify);

    // ─── Build swarm with TCP + TLS + Noise + Yamux + DNS ──────────────────
    let swarm = SwarmBuilder::with_existing_identity(keypair)
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            (libp2p::tls::Config::new, libp2p::noise::Config::new),
            libp2p::yamux::Config::default,
        )?
        .with_dns_config(
            dns::ResolverConfig::cloudflare(),
            dns::ResolverOpts::default(),
        )
        .with_behaviour(|_| Ok(behaviour))?
        .with_swarm_config(|cfg| {
            cfg.with_idle_connection_timeout(Duration::from_secs(60))
        })
        .build();

    Ok(swarm)
}
