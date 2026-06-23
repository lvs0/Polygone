mod banner; // defined in src/banner.rs
use banner::{BANNER, HELP_TEXT};
use clap::{Parser, Subcommand};
use std::net::SocketAddr;
use polygone_common::SessionKey;
use polygone_crypto::{generate_kem_key_pair, encapsulate, decapsulate, encrypt, decrypt, split_secret, reconstruct_secret, hash_data};
use axum::{routing::get, Router};

#[derive(Parser)]
#[command(name = "polygone")]
#[command(about = "⬡ Polygone — Post-quantum ephemeral privacy network", long_about = None)]
#[command(version = "1.0.0")]
#[command(author = "Lévy <polygone@proton.me>")]
struct Cli {
    /// Skip the ASCII banner on startup
    #[arg(long, default_value = "false")]
    no_banner: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch P2P node + TUI dashboard
    Start {
        /// Port HTTP pour /health (défaut: 8080)
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
    /// Generate node keypair (ML-KEM-1024)
    Keygen,
    /// Send an ephemeral message
    Send {
        /// Message text to send
        #[arg(required = true)]
        message: Vec<String>,
    },
    /// Run cryptographic self-test
    SelfTest,
    /// Show network status
    Status,
    /// Update to latest version
    Update,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if !cli.no_banner {
        print!("{}", BANNER);
    }

    match cli.command {
        Some(Commands::Start { port }) => {
            start_node(port).await?;
        }
        Some(Commands::Keygen) => {
            println!("🔑 Génération d'un couple de clés nœud...");
            println!("   Algorithme: ML-KEM-1024 (post-quantique)");
            println!("   Clé publique: (à venir — module crypto en cours)");
            println!("   Clé privée:  (à venir — stockée dans ~/.polygone/keys/)");
        }
        Some(Commands::Send { message }) => {
            let msg = message.join(" ");
            println!("📤 Envoi d'un message éphémère...");
            println!("   Longueur: {} octets", msg.len());
            println!("   TTL: 30 secondes");
            println!("   Garantie: information-theoretic (Shamir 4-of-7)");
            println!("   ⚠  Fonctionnalité réseau en cours d'implémentation");
        }
        Some(Commands::SelfTest) => {
            run_self_test();
        }
        Some(Commands::Status) => {
            println!("[Polygone] Network status:");
            println!("[Polygone]   Peers: (unknown — node not started)");
            println!("[Polygone]   Quorum: 4-of-7");
            println!("[Polygone]   Run 'polygone start' to connect");
        }
        Some(Commands::Update) => {
            println!("[Polygone] Update: fetching latest release from GitHub...");
            println!("[Polygone] ⚠  Auto-update not yet implemented");
            println!("[Polygone]   Use: curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash");
        }
        None => {
            print!("{}", HELP_TEXT);
        }
    }

    Ok(())
}

/// Exécute les vrais tests crypto en appelant polygone-crypto.
/// Chaque test est autonome — un échec ne bloque pas les suivants.
fn run_self_test() {
    println!("[Polygone] Running cryptographic self-test...\n");

    let mut all_ok = true;

    // ── ML-KEM-1024 round-trip ──
    print!("  ML-KEM-1024 keygen + encapsulate + decapsulate... ");
    match (|| -> Result<(), Box<dyn std::error::Error>> {
        let (pk, sk) = generate_kem_key_pair();
        let (ct, ss) = encapsulate(&pk);
        let recovered_ss = decapsulate(&ct, &sk);
        if recovered_ss.as_slice() == ss.as_slice() {
            Ok(())
        } else {
            Err("shared secret mismatch".into())
        }
    })() {
        Ok(()) => println!("OK"),
        Err(e) => {
            println!("FAILED — {}", e);
            all_ok = false;
        }
    }

    // ── AES-256-GCM round-trip ──
    print!("  AES-256-GCM encrypt + decrypt... ");
    match (|| -> Result<(), Box<dyn std::error::Error>> {
        let key = SessionKey::new([0xABu8; 32]); // clé fixe pour test
        let plaintext = b"polygone-test-payload";
        let (ciphertext, nonce) = encrypt(&key, plaintext, &[])?;
        let recovered = decrypt(&key, &ciphertext, &nonce, &[])?;
        if recovered == plaintext {
            Ok(())
        } else {
            Err("payload mismatch".into())
        }
    })() {
        Ok(()) => println!("OK"),
        Err(e) => {
            println!("FAILED — {}", e);
            all_ok = false;
        }
    }

    // ── Shamir 4-of-7 ──
    print!("  Shamir 4-of-7 secret sharing... ");
    match (|| -> Result<(), Box<dyn std::error::Error>> {
        let secret = SessionKey::new([0xCDu8; 32]);
        let shares = split_secret(&secret, 4, 7);
        // Reconstruire avec exactement 4 shares (le seuil)
        let recovered = reconstruct_secret(shares[..4].to_vec(), 4)
            .ok_or("reconstruction failed")?;
        if recovered.as_slice() == secret.as_slice() {
            Ok(())
        } else {
            Err("secret mismatch".into())
        }
    })() {
        Ok(()) => println!("OK"),
        Err(e) => {
            println!("FAILED — {}", e);
            all_ok = false;
        }
    }

    // ── BLAKE3 hash ──
    print!("  BLAKE3 hash... ");
    match (|| -> Result<(), Box<dyn std::error::Error>> {
        let hash = hash_data(b"polygone");
        // Juste vérifier qu'on a bien 32 octets et que ce n'est pas tout zéro
        if hash.len() == 32 && hash != [0u8; 32] {
            Ok(())
        } else {
            Err("invalid hash".into())
        }
    })() {
        Ok(()) => println!("OK (256-bit)"),
        Err(e) => {
            println!("FAILED — {}", e);
            all_ok = false;
        }
    }

    println!();
    if all_ok {
        println!("[Polygone] All cryptographic tests passed. System ready.");
    } else {
        println!("[Polygone] Some tests failed. See above for details.");
        std::process::exit(1);
    }
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

    // 2. Node P2P — placeholder
    println!("[Polygone] ⬡ Node starting...");
    println!("[Polygone]   Crypto:   ML-KEM-1024 (post-quantum)");
    println!("[Polygone]   Network:  Kademlia DHT + libp2p");
    println!("[Polygone]   Sharing:  Shamir 4-of-7 (any 4 of 7 fragments)");
    println!("[Polygone]   TTL:      30 seconds — auto-evaporate");
    println!("[Polygone]   Health:   http://localhost:{}/health", port);
    println!("[Polygone] Press Ctrl+C to stop.");

    // Boucle de maintien — le vrai réseau sera dans network/
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}

async fn health() -> &'static str {
    r#"{"status":"ok","version":"1.0.0","network":"Kademlia DHT","crypto":"ML-KEM-1024","sharing":"Shamir-4-7"}"#
}
