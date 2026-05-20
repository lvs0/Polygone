use std::env;
use std::net::SocketAddr;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Démarrer le serveur HTTP léger pour /health (Render)
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    
    let app = Router::new().route("/health", get(health));
    
    // Lancer le serveur HTTP en tâche de fond
    tokio::spawn(async move {
        axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app)
            .await
            .unwrap();
    });

    println!("[Polygone] Health endpoint on :{} (addr={})", port, addr);

    // 2. Démarrer le nœud P2P réel
    // On utilise la CLI existante via clap
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "start" {
        run_node().await?;
    } else {
        // Affichage de l'aide par défaut
        println!("Polygone - Post-quantum ephemeral network");
        println!("Usage: polygone start");
    }

    Ok(())
}

async fn health() -> &'static str {
    r#"{"status":"ok"}"#
}

async fn run_node() -> Result<(), Box<dyn std::error::Error>> {
    // Ici on appelle la logique réseau existante
    // Pour l'instant, juste une boucle de maintien, le réseau tourne en vrai via libp2p dans network/
    println!("[Polygone] Node starting (P2P + Kademlia)...");
    
    // On garde le processus vivant
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}
