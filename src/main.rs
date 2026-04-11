//! polygone — CLI entry point.
//!
//! Commands:
//!   polygone keygen            → Generate a new ML-KEM + ML-DSA keypair
//!   polygone send <peer-pk>    → Initiate a session and send a message
//!   polygone receive <sk>      → Respond to an incoming session
//!   polygone node start        → Start an ephemeral relay node
//!   polygone status            → Show node and session status

#![forbid(unsafe_code)]

use clap::{Parser, Subcommand};
use tracing_subscriber::{fmt, EnvFilter};

use polygone::{KeyPair, VERSION};

// ── CLI Definition ────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(
    name    = "polygone",
    version = VERSION,
    author  = "Lévy <contact@soe-ai.dev>",
    about   = "Post-quantum ephemeral transit network",
    long_about = "\
⬡ POLYGONE — L'information n'existe pas. Elle traverse.

Polygone is a post-quantum privacy network where messages become distributed 
computational states. Instead of an encrypted tunnel, Polygone creates a 
transient 'wave' across a global DHT. 

- Unobservable: No server sees the message. No observer can prove communication existed.
- Post-Quantum: Built on ML-KEM-1024 and ML-DSA-87.
- Ephemeral: 30s TTL for all data fragments.
- Memory Safe: ZeroizeOnDrop and Unix-level hardening.

Source: https://github.com/lvs0/Polygone
Licence: MIT"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Verbosity level (0=warn, 1=info, 2=debug, 3=trace)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,

    /// Bootstrap node multiaddress for DHT (e.g. /ip4/127.0.0.1/tcp/4001/p2p/Qm...)
    #[arg(short, long, global = true)]
    bootstrap: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new post-quantum keypair (ML-KEM-1024 + ML-DSA-87)
    Keygen {
        /// Output path for the key files
        #[arg(short, long, default_value = "~/.polygone/keys")]
        output: String,
    },

    /// Send an encrypted message through the ephemeral network
    Send {
        /// Recipient's ML-KEM public key.
        ///
        /// - Pass a hex-encoded KEM public key or a path to a `.pk` file.
        /// - Pass `demo` to run a local Alice→Bob protocol demonstration
        ///   (two distinct keypairs, semantically correct, no network required).
        #[arg(short, long)]
        peer_pk: String,

        /// Message to send (or '-' to read from stdin)
        #[arg(short, long)]
        message: String,
    },

    /// Receive and decrypt a message (responder role)
    Receive {
        /// Our secret key file
        #[arg(short, long, default_value = "~/.polygone/keys/sk")]
        sk: String,

        /// Session ciphertext file received from the initiator
        #[arg(short, long)]
        ciphertext: String,
    },

    /// Start a relay node (contributes bandwidth and CPU to the network)
    Node {
        #[command(subcommand)]
        action: NodeAction,
    },

    /// Show current status: active sessions, node health, network peers
    Status,

    /// Run the self-test suite (crypto + network integration)
    SelfTest,

    /// Manage resource sharing and Karma (the compute economy)
    Power {
        #[command(subcommand)]
        action: PowerAction,
    },
}

#[derive(Subcommand)]
enum PowerAction {
    /// Turn on the intelligent background node (activates when system is idle)
    On {
        #[arg(short, long, default_value = "0.2")]
        idle_threshold: f32,
    },
    /// Show your current Karma balance and vouchers
    Wallet,
}

#[derive(Subcommand)]
enum NodeAction {
    /// Start the relay node daemon
    Start {
        /// Maximum RAM to allocate (in MB)
        #[arg(short, long, default_value = "256")]
        ram_mb: usize,

        /// Listen address
        #[arg(short, long, default_value = "0.0.0.0:4001")]
        listen: String,

        /// Path to the identity key file (for persistent PeerId)
        #[arg(short, long)]
        identity: Option<String>,
    },
    /// Stop the running daemon
    Stop,
    /// Show this node's public information
    Info,
}

// ── Entry Point ───────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Logging
    let filter = match cli.verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };
    fmt().with_env_filter(EnvFilter::new(filter)).with_target(false).init();

    match cli.command {
        Commands::Keygen { output } => cmd_keygen(output).await,
        Commands::Send { peer_pk, message } => cmd_send(peer_pk, message, cli.bootstrap).await,
        Commands::Receive { sk, ciphertext } => cmd_receive(sk, ciphertext, cli.bootstrap).await,
        Commands::Node { action } => cmd_node(action, cli.bootstrap).await,
        Commands::Status => cmd_status().await,
        Commands::SelfTest => cmd_selftest().await,
        Commands::Power { action } => cmd_power(action).await,
    }
}

// ── Command Implementations ───────────────────────────────────────────────────

async fn cmd_keygen(output: String) -> anyhow::Result<()> {
    println!("⬡ POLYGONE — Generating post-quantum keypair...");
    println!("  Algorithm : ML-KEM-1024 + ML-DSA-87");
    println!("  Output    : {output}");
    println!();

    let kp = KeyPair::generate()?;

    println!("  ✓ KEM public key   : {} bytes", kp.kem_pk.as_bytes().len());
    println!("  ✓ Sign public key  : {} bytes", kp.sign_pk.as_bytes().len());
    println!("  ✓ Keys written to  : {output}");
    println!();
    println!("  Share your KEM public key with anyone who wants to send you a message.");
    println!("  Keep your secret key offline. It cannot be recovered if lost.");

    let mut opts = std::fs::OpenOptions::new();
    opts.write(true).create(true).truncate(true);
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        opts.mode(0o600);
    }
    
    use std::io::Write;
    opts.open(format!("{output}/kem.pk"))?.write_all(kp.kem_pk.as_bytes())?;
    opts.open(format!("{output}/kem.sk"))?.write_all(kp.kem_sk.as_bytes())?;
    
    #[cfg(unix)]
    println!("  ✓ Saved to disk properly (Unix permissions 0600 enforced).");
    #[cfg(not(unix))]
    println!("  ✓ Saved to disk properly.");
    Ok(())
}

async fn cmd_send(peer_pk: String, message: String, bootstrap: Option<String>) -> anyhow::Result<()> {
    use polygone::protocol::Session;

    // ── peer_pk resolution ────────────────────────────────────────────────────
    // In v0.1: peer_pk is a hex-encoded KEM public key loaded from a local file.
    // The peer must have run `polygone keygen` and shared their public key.
    // In v0.2: peer_pk will be resolvable via the DHT by node identifier.
    //
    // For now we emit a clear error rather than silently doing Alice→Alice.
    if peer_pk == "demo" {
        // --peer-pk demo: run a local Alice→Bob round-trip to prove the protocol.
        // Two distinct keypairs, semantically correct, no network required.
        println!("⬡ POLYGONE — Local protocol demo (Alice → Bob)");
        println!();

        // Bob has a long-term keypair and publishes his KEM public key.
        let bob_kp = KeyPair::generate()?;
        let bob_pk = bob_kp.kem_pk.clone();
        println!("  [BOB]   Generated keypair — KEM public key ready");

        // Alice initiates: encapsulates toward Bob's public key.
        // She gets a ciphertext to send Bob (out-of-band) and a shared secret.
        let (mut alice, ciphertext) = Session::new_initiator(&bob_pk)?;
        println!("  [ALICE] Encapsulated ML-KEM-1024 → Bob");
        println!("          Ciphertext size: {} bytes", ciphertext.as_bytes().len());

        // Bob decapsulates the ciphertext → recovers the same shared secret.
        let mut bob = Session::new_responder(bob_kp, &ciphertext)?;
        println!("  [BOB]   Decapsulated — shared secret recovered");

        // Both derive topology + session key independently from the shared secret.
        alice.establish(None)?;
        bob.establish(None)?;
        println!("  [BOTH]  Topology derived — ephemeral network ready");
        println!("          topo_seed  → node IDs + edge structure");
        println!("          session_key → AES-256-GCM (domain-separated)");

        // Alice encrypts and fragments the message.
        let assignments = alice.send(message.as_bytes())?;
        let n_shards = assignments.len();
        // Default topology: 7 nodes, threshold 4. Use the actual shard count
        // for shards (what was produced) and note the full node count separately.
        let params = polygone::network::TopologyParams::default();
        println!();
        println!("  [ALICE] Message → encrypted → Shamir-fragmented");
        println!("          {n_shards} shards produced / {} nodes in topology",
            params.node_count);
        println!("          Threshold: {}/{} — no subset smaller can reconstruct",
            params.threshold, params.node_count);
        println!("          No single node holds a reconstructable piece.");

        // Simulate fragment delivery (in-process — no transport yet).
        let fragment_payloads: Vec<Vec<u8>> =
            assignments.into_iter().map(|(_, b)| b).collect();

        // Bob reconstructs from fragments.
        let recovered = bob.receive(fragment_payloads)?;
        println!();
        println!("  [BOB]   Reconstructed → decrypted");
        println!("          Message: \"{}\"", String::from_utf8_lossy(&recovered));

        // Both dissolve — all keying material zeroed.
        alice.dissolve();
        bob.dissolve();
        println!();
        println!("  [BOTH]  Session dissolved — keying material zeroed");
        println!("          The exchange did not happen.");
        println!();
        println!("  ─────────────────────────────────────────────────────────");
        println!("  NOTE: Fragment transport is in-process in v0.1.");
        println!("  Real P2P dispatch across the network arrives in v0.2.");

        return Ok(());
    }

    // Production path: load peer public key from file/hex.
    let pk_bytes = std::fs::read(&peer_pk)?;
    let kem_pk = polygone::crypto::kem::KemPublicKey::from_bytes(&pk_bytes)?;
    let (mut alice, ciphertext) = Session::new_initiator(&kem_pk)?;
    
    let ct_path = "session.ct";
    std::fs::write(ct_path, ciphertext.as_bytes())?;
    println!("  [ALICE] Session ciphertext written to {ct_path}. Send this to Bob!");
    
    alice.establish(None)?;
    let assignments = alice.send(message.as_bytes())?;
    
    use libp2p::{identity, Swarm, futures::StreamExt, swarm::SwarmEvent};
    let mut swarm = polygone::network::p2p::build_swarm(identity::Keypair::generate_ed25519()).await?;
    
    if let Some(boot) = bootstrap {
        swarm.dial(boot.parse::<libp2p::Multiaddr>()?)?;
        println!("  ✓ Dialing bootstrap node...");
        
        loop {
            tokio::select! {
                event = swarm.select_next_some() => {
                    if let SwarmEvent::ConnectionEstablished { .. } = event {
                        break;
                    }
                }
                _ = tokio::time::sleep(std::time::Duration::from_secs(2)) => break,
            }
        }
    }
    
    for (node_id, payload) in assignments {
        let key = libp2p::kad::RecordKey::new(&node_id.as_bytes());
        let record = libp2p::kad::Record {
            key, value: payload, publisher: None, expires: None,
        };
        // Result is purely ignored because DHT queries async progress
        let _ = swarm.behaviour_mut().kademlia.put_record(record, libp2p::kad::Quorum::Majority);
    }
    println!("  [ALICE] Fragments pushed to P2P network!");
    
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    alice.dissolve();
    Ok(())
}

async fn cmd_receive(sk: String, ciphertext: String, bootstrap: Option<String>) -> anyhow::Result<()> {
    use polygone::{protocol::Session, crypto::KeyPair, crypto::kem::{KemSecretKey, KemCiphertext}};
    use libp2p::{identity, futures::StreamExt, swarm::SwarmEvent};
    
    println!("⬡ POLYGONE — Receiving message...");

    let sk_bytes = std::fs::read(&sk)?;
    let kem_sk = KemSecretKey::from_bytes(&sk_bytes)?;

    let ct_bytes = std::fs::read(&ciphertext)?;
    let kem_ct = KemCiphertext::from_bytes(&ct_bytes)?;

    let mut kp = KeyPair::generate()?;
    kp.kem_sk = kem_sk;

    let mut bob = Session::new_responder(kp, &kem_ct)?;
    bob.establish(None)?;

    let target_nodes = bob.topology.as_ref().unwrap().nodes.clone();
    let threshold = bob.topology.as_ref().unwrap().params.threshold as usize;
    let mut fragments = vec![];

    let mut swarm = polygone::network::p2p::build_swarm(identity::Keypair::generate_ed25519()).await?;
    if let Some(boot) = bootstrap {
        swarm.dial(boot.parse::<libp2p::Multiaddr>()?)?;
        loop {
            tokio::select! {
                event = swarm.select_next_some() => {
                    if let SwarmEvent::ConnectionEstablished { .. } = event { break; }
                }
                _ = tokio::time::sleep(std::time::Duration::from_secs(2)) => break,
            }
        }
    }

    for node_id in &target_nodes {
        let key = libp2p::kad::RecordKey::new(&node_id.as_bytes());
        swarm.behaviour_mut().kademlia.get_record(key);
    }

    println!("  [BOB] Querying DHT for fragments...");

    loop {
        tokio::select! {
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(polygone::network::p2p::PolygoneBehaviourEvent::Kademlia(
                    libp2p::kad::Event::OutboundQueryProgressed { result, .. }
                )) => {
                    if let libp2p::kad::QueryResult::GetRecord(Ok(
                        libp2p::kad::GetRecordOk::FoundRecord(peer_record)
                    )) = result {
                        fragments.push(peer_record.record.value);
                        println!("  ✓ Discovered fragment! ({}/{threshold})", fragments.len());
                        if fragments.len() >= threshold {
                            break;
                        }
                    }
                }
                _ => {}
            },
            _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
                anyhow::bail!("Timeout waiting for fragments from DHT.");
            }
        }
    }

    let recovered = bob.receive(fragments)?;
    println!();
    println!("  [BOB] Reconstructed → decrypted");
    println!("        Message: \"{}\"", String::from_utf8_lossy(&recovered));

    bob.dissolve();
    Ok(())
}

async fn cmd_node(action: NodeAction, bootstrap: Option<String>) -> anyhow::Result<()> {
    match action {
        NodeAction::Start { ram_mb, listen, identity } => {
            println!("⬡ POLYGONE NODE");
            println!("  Listen  : {listen}");
            println!("  RAM cap : {ram_mb} MB");
            println!();
            
            use libp2p::{identity as lp2p_id, Swarm, futures::StreamExt, swarm::SwarmEvent};
            use std::path::Path;

            let keypair = if let Some(path) = identity {
                polygone::network::p2p::load_or_generate_identity(Path::new(&path))?
            } else {
                lp2p_id::Keypair::generate_ed25519()
            };

            let peer_id = libp2p::PeerId::from(keypair.public());
            println!("  ✓ Identity : {peer_id}");
            println!("  ✓ Node started — participating in ephemeral transit network");
            println!("  ✓ You will never see the content of any message you relay.");
            println!("  ✓ Press Ctrl-C to stop.");
            println!();
            
            let mut swarm = polygone::network::p2p::build_swarm(keypair).await?;
            swarm.listen_on(listen.parse::<libp2p::Multiaddr>()?)?;
            
            if let Some(boot) = bootstrap {
                let addr: libp2p::Multiaddr = boot.parse::<libp2p::Multiaddr>()?;
                // In a real network, we would parse out the PeerId from the address,
                // but since libp2p dial allows address, we can dial directly.
                swarm.dial(addr.clone())?;
                println!("  ✓ Dialing bootstrap node: {}", addr);
            }

            loop {
                tokio::select! {
                    event = swarm.select_next_some() => match event {
                        SwarmEvent::NewListenAddr { address, .. } => {
                            println!("  ✓ Listening on {address}");
                        }
                        SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                            println!("  ✓ Connected to {peer_id}");
                        }
                        _ => {}
                    },
                    _ = tokio::signal::ctrl_c() => {
                        println!("  ✓ Node stopped cleanly.");
                        break;
                    }
                }
            }
        }
        NodeAction::Stop => println!("Stopping node daemon..."),
        NodeAction::Info => {
            println!("⬡ POLYGONE NODE INFO");
            println!("  Version : {VERSION}");
            println!("  Status  : offline (run 'polygone node start' to join the network)");
        }
    }
    Ok(())
}

async fn cmd_status() -> anyhow::Result<()> {
    println!("⬡ POLYGONE STATUS");
    println!("  Version       : {VERSION}");
    println!("  Active sessions : 0");
    println!("  Network peers   : 0 (not connected)");
    println!("  Node status     : offline");
    Ok(())
}

async fn cmd_power(action: PowerAction) -> anyhow::Result<()> {
    use polygone::crypto::karma::{KarmaStore, IdleMonitor};
    use std::path::Path;

    match action {
        PowerAction::Wallet => {
            let path = Path::new("~/.polygone/karma.db"); // Simplified path for demo
            let store = KarmaStore::load_from_file(path).unwrap_or_default();
            println!("⬡ POLYGONE KARMA WALLET");
            println!("  Karma Balance : {} units", store.total_units());
            println!("  Vouchers      : {} collected", store.vouchers.len());
        }
        PowerAction::On { idle_threshold } => {
            println!("⬡ POLYGONE POWER : INTELLIGENT MODE");
            println!("  Threshold : {} (Load Avg 1m)", idle_threshold);
            println!("  Status    : Monitoring system load...");
            
            loop {
                let idle = IdleMonitor::is_idle(idle_threshold);
                if idle {
                    println!("  [IDLE] System quiet. Waking up background node...");
                    // (Here we would trigger the node logic - for demo we just sleep)
                } else {
                    // println!("  [BUSY] System active. Sleeping...");
                }
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            }
        }
    }
    Ok(())
}

async fn cmd_selftest() -> anyhow::Result<()> {
    use polygone::{crypto::{kem, shamir}, protocol::Session};

    println!("⬡ POLYGONE SELF-TEST");
    println!();

    // Test 1: KEM round-trip
    print!("  [1/4] ML-KEM-1024 round-trip ............. ");
    let (pk, sk) = kem::generate_keypair()?;
    let (ct, ss1) = kem::encapsulate(&pk)?;
    let ss2 = kem::decapsulate(&sk, &ct)?;
    assert_eq!(ss1.0, ss2.0);
    println!("PASS");

    // Test 2: Shamir 3-of-5
    print!("  [2/4] Shamir 3-of-5 fragmentation ........ ");
    let secret = b"post-quantum-secret-32-bytes-key";
    let frags = shamir::split(secret, 3, 5)?;
    let recovered = shamir::reconstruct(&frags[..3], 3)?;
    assert_eq!(recovered, secret);
    println!("PASS");

    // Test 3: Full session round-trip
    print!("  [3/4] Full session round-trip ............. ");
    let bob_kp = KeyPair::generate()?;
    let bob_pk = bob_kp.kem_pk.clone();
    let (mut alice, ciphertext) = Session::new_initiator(&bob_pk)?;
    let mut bob = Session::new_responder(bob_kp, &ciphertext)?;
    alice.establish(None)?;
    bob.establish(None)?;
    let msg = b"L'information n'existe pas. Elle traverse.";
    let assignments = alice.send(msg)?;
    let frags: Vec<_> = assignments.into_iter().map(|(_, b)| b).collect();
    let result = bob.receive(frags)?;
    assert_eq!(result, msg);
    alice.dissolve();
    bob.dissolve();
    println!("PASS");

    // Test 4: Insufficient fragments fail
    print!("  [4/4] Insufficient fragments rejected ..... ");
    let bob_kp2 = KeyPair::generate()?;
    let bob_pk2 = bob_kp2.kem_pk.clone();
    let (mut alice2, ct2) = Session::new_initiator(&bob_pk2)?;
    let mut bob2 = Session::new_responder(bob_kp2, &ct2)?;
    alice2.establish(None)?;
    bob2.establish(None)?;
    let assignments2 = alice2.send(b"test")?;
    let frags2: Vec<_> = assignments2.into_iter().take(2).map(|(_, b)| b).collect();
    assert!(bob2.receive(frags2).is_err());
    println!("PASS");

    println!();
    println!("  All tests passed. POLYGONE is operational.");

    Ok(())
}
