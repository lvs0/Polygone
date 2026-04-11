use libp2p::{
    identify::{self, Behaviour as Identify, Config as IdentifyConfig},
    identity::Keypair,
    kad::{
        store::MemoryStore, Behaviour as Kademlia, Config as KademliaConfig, Mode,
    },
    noise, tcp, yamux, PeerId, StreamProtocol, Swarm, SwarmBuilder,
};
use std::time::Duration;
use std::path::Path;
use std::fs;
use libp2p::swarm::NetworkBehaviour;

/// Load a libp2p identity keypair from a file or generate and save a new one.
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

#[derive(NetworkBehaviour)]
pub struct PolygoneBehaviour {
    pub kademlia: Kademlia<MemoryStore>,
    pub identify: Identify,
}

pub fn build_swarm(
    keypair: Keypair,
) -> anyhow::Result<Swarm<PolygoneBehaviour>> {
    let peer_id = PeerId::from(keypair.public());

    let swarm = SwarmBuilder::with_existing_identity(keypair.clone())
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|key| {
            let local_peer_id = PeerId::from(key.public());

            // Kademlia
            let store = MemoryStore::new(local_peer_id);
            let mut kad_config = KademliaConfig::default();
            kad_config.set_protocol_names(vec![StreamProtocol::new("/polygone/kad/1.1.0")]);
            kad_config.set_record_ttl(Some(Duration::from_secs(30)));
            kad_config.set_provider_record_ttl(Some(Duration::from_secs(30)));
            let mut kademlia = Kademlia::with_config(local_peer_id, store, kad_config);
            // By default, only act as a node if configured or explicitly
            kademlia.set_mode(Some(Mode::Server));

            // Identify
            let identify_config = IdentifyConfig::new("/polygone/1.1.0".to_string(), key.public())
                .with_push_listen_addr_updates(true);
            let identify = Identify::new(identify_config);

            PolygoneBehaviour {
                kademlia,
                identify,
            }
        })?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    Ok(swarm)
}
