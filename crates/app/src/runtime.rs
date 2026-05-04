use polygone_network::P2PNode;
use polygone_crypto::{generate_kem_key_pair, hash_data};
use std::sync::Arc;
use tokio::sync::RwLock;
use webui::AppState;
use crate::http_api::{LocalStorage, run_http_api};
use std::net::SocketAddr;

pub fn run_node(web: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Démarrage du nœud Polygone (mode démo)...");
    let node = P2PNode::new();
    let node_id_hex = hex::encode(node.node_id.0);
    println!("Nœud ID: {node_id_hex}");

    let (_pk, _sk) = generate_kem_key_pair();
    let data = b"Hello Polygone";
    let hash = hash_data(data);
    println!("Hash de test: {:x?}", hash);

    // Lancer le serveur HTTP interne (API P2P)
    let http_addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let storage = Arc::new(LocalStorage::new());
    let storage_clone = storage.clone();
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            if let Err(e) = run_http_api(http_addr, storage_clone).await {
                eprintln!("Erreur serveur HTTP interne : {e}");
            }
        });
    });

    if web {
        println!("Lancement du serveur Web UI sur http://127.0.0.1:9050...");

        let state = Arc::new(RwLock::new(AppState::fresh()));
        {
            let mut s = state.blocking_write();
            s.status.node_id = node_id_hex.clone();
            s.status.status = "online".to_string();
            s.status.peers_connected = 2;
            s.status.poly_balance = 42.0;
            s.update_uptime();
        }

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let state_clone = state.clone();
                tokio::spawn(async move {
                    loop {
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                        let mut s = state_clone.write().await;
                        s.update_uptime();
                    }
                });

                let _ = webui::start_webui(state).await;
            });
        });

        std::thread::sleep(std::time::Duration::from_secs(2));
        if let Err(e) = open::that("http://127.0.0.1:9050") {
            eprintln!("Impossible d'ouvrir le navigateur : {e}");
        } else {
            println!("Navigateur ouvert sur http://127.0.0.1:9050");
        }
    }

    Ok(())
}

pub fn run_tests() -> Result<(), Box<dyn std::error::Error>> {
    println!("Exécution des tests démonstratifs...");
    Ok(())
}
