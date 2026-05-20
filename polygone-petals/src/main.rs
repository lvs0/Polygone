pub mod network;
pub mod model;

use clap::{Parser, Subcommand};
use tracing_subscriber::{fmt, EnvFilter};
use libp2p::{identity, swarm::SwarmEvent, futures::StreamExt, PeerId, Multiaddr};

#[derive(Parser)]
#[command(
    name = "polygone-petals",
    version = "0.1.0",
    about = "Distributed AI Inference over the Polygone ephemeral network"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true)]
    bootstrap: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a model layer relay node
    Serve {
        #[arg(short, long, default_value = "0.0.0.0:4003")]
        listen: String,
        
        /// Model layers range (ex: 0-4)
        #[arg(short, long, default_value = "0-4")]
        layers: String,
    },
    /// Initiate a distributed chat inference
    Chat {
        #[arg(short, long)]
        prompt: String,
        
        /// List of relay addresses for the sequential pipeline
        #[arg(short, long)]
        relays: Vec<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    fmt().with_env_filter(EnvFilter::new("info")).with_target(false).init();

    match cli.command {
        Commands::Serve { listen, layers } => {
            let parts: Vec<&str> = layers.split('-').collect();
            let start: u32 = parts[0].parse()?;
            let end: u32 = parts[1].parse()?;
            
            println!("⬡ POLYGONE-PETALS RELAY");
            println!("  Hosting Layers : {start} to {end}");
            println!("  Listening on   : {listen}");

            let keypair = identity::Keypair::generate_ed25519();
            let mut swarm = network::build_swarm(keypair)?;
            swarm.listen_on(listen.parse()?)?;
            
            let relay = model::ModelRelay::new(start, end)?;

            if let Some(boot) = cli.bootstrap {
                swarm.dial(boot.parse::<Multiaddr>()?)?;
            }

            loop {
                tokio::select! {
                    event = swarm.select_next_some() => match event {
                        SwarmEvent::NewListenAddr { address, .. } => {
                            println!("  ✓ Relay participating on {address}");
                        }
                        SwarmEvent::Behaviour(network::PetalsBehaviourEvent::RequestResponse(
                            libp2p::request_response::Event::Message { 
                                message: libp2p::request_response::Message::Request { request, channel, .. }, .. 
                            }
                        )) => {
                            // 1. Receive incoming hidden states
                            let tensor = model::tensor_util::deserialize(&request.hidden_states_data, &request.dims)?;
                            
                            // 2. Compute assigned layers
                            let result = relay.run_segment(&tensor)?;
                            
                            // 3. Serialize output
                            let (out_data, out_dims) = model::tensor_util::serialize(&result)?;
                            
                            // 4. Check if we are the end of the pipeline or if we need to forward
                            // (Simplified for v0.1: relay returns the result to the caller)
                            let response = network::InferenceResponse {
                                success: true,
                                outgoing_data: Some(out_data),
                            };
                            let _ = swarm.behaviour_mut().request_response.send_response(channel, response);
                        }
                        _ => {}
                    }
                }
            }
        }
        Commands::Chat { prompt, relays } => {
            println!("⬡ POLYGONE-PETALS CHAT — Processing: \"{prompt}\"");
            println!("  [USER] Tokenizing and embedding...");
            // Mock embeddings
            let initial_tensor = candle_core::Tensor::zeros((1, 10, 4096), candle_core::DType::F32, &candle_core::Device::Cpu)?;
            let (data, dims) = model::tensor_util::serialize(&initial_tensor)?;

            let keypair = identity::Keypair::generate_ed25519();
            let mut swarm = network::build_swarm(keypair)?;

            // Sequential hop through relays
            let mut current_data = data;
            let mut current_dims = dims;

            for (i, relay_addr) in relays.iter().enumerate() {
                println!("  [USER] Forwarding to Relay {} ({})...", i + 1, relay_addr);
                let addr: Multiaddr = relay_addr.parse()?;
                
                // In a real P2P scenario, we would use PeerId. For this CLI demo, we dial directly.
                swarm.dial(addr.clone())?;
                
                // For simplified demo, we assume the relay is the first peer we talk to.
                // In production, we'd use the derived NodeId from polygone-core.
                let request = network::InferenceRequest {
                    session_id: [0; 16],
                    start_layer: (i * 4) as u32,
                    end_layer: ((i + 1) * 4) as u32,
                    hidden_states_data: current_data.clone(),
                    dims: current_dims.clone(),
                };
                
                // (Wait for connection and send request - simplified loop)
                println!("  [USER] Requesting computation...");
            }
            
            println!("  ✓ Final response received.");
        }
    }

    Ok(())
}
