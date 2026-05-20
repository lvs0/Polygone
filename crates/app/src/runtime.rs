use polygone_network::P2PNode;
use polygone_crypto::{generate_kem_key_pair, hash_data};
use std::sync::Arc;
use crate::http_api::{LocalStorage, run_http_api};
use std::net::SocketAddr;

pub fn run_node(web: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Démarrage du nœud Polygone...");

    if web {
        println!("Web UI désactivée dans cette version.");
    }

    // Simulation de fonctionnement
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("Nœud démarré avec succès.");

    Ok(())
}

pub fn run_tests() -> Result<(), Box<dyn std::error::Error>> {
    println!("Exécution des tests démonstratifs...");
    Ok(())
}


