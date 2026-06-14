use clap::{Parser, Subcommand};
use std::net::SocketAddr;
use axum::{routing::get, Router};

#[derive(Parser)]
#[command(name = "polygone")]
#[command(about = "Hexa P2P — Réseau post-quantique éphémère")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Lance le nœud P2P + dashboard TUI
    Start {
        /// Port HTTP pour /health (défaut: 8080)
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
    /// Génère un couple de clés nœud
    Keypair,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Start { port }) => {
            start_node(port).await?;
        }
        Some(Commands::Keypair) => {
            println!("🔑 Génération d'un couple de clés nœud...");
            // Placeholder — sera connecté au module crypto
            println!("   Clé publique: (à venir)");
            println!("   Clé privée:  (à venir)");
        }
        None => {
            print_banner();
        }
    }

    Ok(())
}

fn print_banner() {
    println!("⬡ Polygone — Post-quantum ephemeral network");
    println!("   Version: {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("Commandes:");
    println!("  polygone start [--port N]  Lance le nœud P2P + TUI");
    println!("  polygone keypair           Génère un couple de clés");
    println!("  polygone help              Affiche cette aide");
    println!();
    println!("Variables d'environnement:");
    println!("  PORT              Port HTTP /health (défaut: 8080)");
    println!("  POLYGONE_PEER_ID  ID du nœud (auto-généré si absent)");
}

async fn start_node(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Health endpoint (utile pour Render / monitoring)
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let app = Router::new().route("/health", get(health));

    // Binding avec fallback sur port +1 si déjà pris
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(l) => {
            println!("[Polygone] Health endpoint sur :{}", port);
            l
        }
        Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
            let fallback = port + 1;
            println!("[Polygone] ⚠ Port {} occupé, essai port {}", port, fallback);
            let fallback_addr = SocketAddr::from(([0, 0, 0, 0], fallback));
            tokio::net::TcpListener::bind(&fallback_addr).await?
        }
        Err(e) => return Err(e.into()),
    };

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // 2. Node P2P
    println!("[Polygone] Node starting (P2P + Kademlia)...");
    println!("[Polygone] Appuyez sur Ctrl+C pour arrêter.");

    // Boucle de maintien — le vrai réseau sera dans network/
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}

async fn health() -> &'static str {
    r#"{"status":"ok","version":"0.1.0"}"#
}
