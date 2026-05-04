use clap::Parser;

use std::path::PathBuf;
use std::fs;
use libp2p::{identity::Keypair, swarm::SwarmEvent, PeerId};
use libp2p::futures::StreamExt;
use anyhow::Result;
use tracing_subscriber::EnvFilter;

use polygone::network::p2p::{build_swarm, load_or_generate_identity};

#[derive(Parser)]
#[command(name = "polygone-server", about = "Polygone Persistent Node Runner")]
struct Cli {
    #[arg(short, long, default_value = "/data/identity.p2p")]
    identity: PathBuf,
    #[arg(short, long, default_value = "/ip4/0.0.0.0/tcp/4001")]
    listen: String,
    #[arg(short, long)]
    bootstrap: Option<String>,
    #[arg(long)]
    render_url: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("Polygone Server v{}", polygone::VERSION);

    if let Some(parent) = cli.identity.parent() {
        fs::create_dir_all(parent).ok();
    }

    let local_key: Keypair = load_or_generate_identity(&cli.identity)?;
    let peer_id = PeerId::from(local_key.public());
    tracing::info!("Node peer ID: {peer_id}");

    let config = polygone::network::p2p::P2pConfig::default();
    let mut swarm = build_swarm(local_key, &config).await?;

    let listen_addr: libp2p::Multiaddr = cli.listen.parse().expect("Invalid listen address");
    swarm.listen_on(listen_addr.clone())?;
    tracing::info!("Listening on {listen_addr}");

    if let Some(boot) = &cli.bootstrap {
        tracing::info!("Bootstrapping to {boot}");
    }

    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));

    loop {
        tokio::select! {
            event = swarm.next() => {
                match event {
                    Some(SwarmEvent::NewListenAddr { address, .. }) => tracing::info!("Listen: {address}"),
                    Some(SwarmEvent::ConnectionEstablished { peer_id: pid, .. }) => tracing::info!("Connected: {pid}"),
                    Some(SwarmEvent::Behaviour(e)) => tracing::debug!("Behaviour: {e:?}"),
                    None => break,
                    _ => {}
                }
            }
            _ = interval.tick() => {
                if let Some(url) = &cli.render_url {
                    let _ = reqwest::get(format!("{}/ping/{peer_id}", url)).await;
                }
            }
        }
    }
    Ok(())
}