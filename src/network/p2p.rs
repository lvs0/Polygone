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

pub async fn build_swarm(
    keypair: Keypair,
) -> anyhow::Result<Swarm<PolygoneBehaviour>> {
    // Build transport: TCP + DNS + noise + yamux
    let transport = tcp::async_io::Transport::new(tcp::Config::default())
        .upgrade(libp2p::core::upgrade::Version::V1Lazy)
        .authenticate(noise::Config::new(&keypair)?)
        .multiplex(yamux::Config::default())
        .unwrap()
        .into();

    // Wrap with DNS for domain name support
    let dns_resolver = dns::ResolverConfig::cloudflare();
    let dns_opts = dns::ResolverOpts::default();
    let transport = dns::Transport::custom(transport)
        .resolvable(dns::Transport::resolvable(dns_resolver, dns_opts)?)
        .boxed();

    let local_peer_id = PeerId::from(keypair.public());

    // Build behaviour
    let store = MemoryStore::new(local_peer_id);
    let mut kad_config = KademliaConfig::new();
    kad_config.set_protocol_names(vec![StreamProtocol::new("/polygone/kad/1.1.0")]);
    kad_config.set_record_ttl(Some(Duration::from_secs(30)));
    kad_config.set_provider_record_ttl(Some(Duration::from_secs(30)));
    let mut kademlia = Kademlia::with_config(local_peer_id, store, kad_config);
    kademlia.set_mode(Some(Mode::Server));

    let identify_config = IdentifyConfig::new("/polygone/1.1.0".to_string(), keypair.public())
        .with_push_listen_addr_updates(true);
    let identify = Identify::new(identify_config);

    let behaviour = PolygoneBehaviour {
        kademlia,
        identify,
    };

    // Create swarm
    let swarm = SwarmBuilder::new(transport, behaviour, local_peer_id)
        .idle_timeout(std::time::Duration::from_secs(60))
        .build();

    Ok(swarm)
}