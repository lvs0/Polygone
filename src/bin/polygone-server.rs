use clap::Parser;
use tracing_subscriber::{fmt, EnvFilter};
use std::path::PathBuf;
use std::fs;
use polygone::base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use polygone::libp2p::{self, futures::StreamExt, swarm::SwarmEvent};
use polygone::network::p2p::{build_swarm, load_or_generate_identity};

#[derive(Parser)]
#[command(name = "polygone-server", about = "Polygone Persistent Node Runner")]
struct Cli {
    /// Path to the identity key file (libp2p Ed25519)
    #[arg(short, long, default_value = "/data/identity.p2p")]
    identity: PathBuf,

    /// Listening address
    #[arg(short, long, default_value = "/ip4/0.0.0.0/tcp/4001")]
    listen: String,

    /// Bootstrap node (optional)
    #[arg(short, long)]
    bootstrap: Option<String>,
}

#[tokio::main]
async fn main() -> polygone::anyhow::Result<()> {
    fmt().with_env_filter(EnvFilter::new("info")).init();
    let cli = Cli::parse();

    println!(" ⬡ POLYGONE SERVER MODE");
    println!(" 🚀 Initializing node...");

    // 1. Load P2P Identity
    // Priority: POLY_P2P_IDENTITY_B64 (protobuf) > POLY_P2P_SEED (32 bytes) > File
    let p2p_keypair = if let Ok(b64) = std::env::var("POLY_P2P_IDENTITY_B64") {
        let bytes = BASE64.decode(b64.trim())?;
        libp2p::identity::Keypair::from_protobuf_encoding(&bytes)?
    } else if let Ok(seed_b64) = std::env::var("POLY_P2P_SEED") {
        let seed = BASE64.decode(seed_b64.trim())?;
        if seed.len() != 32 {
            anyhow::bail!("POLY_P2P_SEED must be 32 bytes Base64-encoded (found {} bytes)", seed.len());
        }
        let mut seed_arr = [0u8; 32];
        seed_arr.copy_from_slice(&seed);
        libp2p::identity::Keypair::ed25519_from_bytes(seed_arr)?
    } else {
        load_or_generate_identity(&cli.identity)?
    };

    let peer_id = libp2p::PeerId::from(p2p_keypair.public());
    println!("  ✓ Local PeerID: {}", peer_id);

    // 2. Build Swarm
    let mut swarm = build_swarm(p2p_keypair)?;

    // 3. Listen
    swarm.listen_on(cli.listen.parse::<libp2p::Multiaddr>()?)?;

    // 4. Dial Bootstrap if provided
    if let Some(addr) = cli.bootstrap {
        println!("  ▸ Connecting to bootstrap: {}", addr);
        swarm.dial(addr.parse::<libp2p::Multiaddr>()?)?;
    }

    println!(" ⬢ Node is live and relaying traffic.");

    // 5. Event Loop
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("  📡 Listening on: {}", address);
            }
            SwarmEvent::Behaviour(ev) => {
                // Handling Kademlia or Identify events if needed
                tracing::debug!("Behaviour event: {:?}", ev);
            }
            _ => {}
        }
    }
}
