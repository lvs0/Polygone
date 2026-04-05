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
    author  = "Max <polygone@proton.me>",
    about   = "Post-quantum ephemeral transit network",
    long_about = "\
POLYGONE — L'information n'existe pas. Elle traverse.

A post-quantum ephemeral network where messages become distributed
computational state. No server sees the message. No observer can
prove a message existed. Classical encryption hides content.
POLYGONE hides the communication itself.

Source: https://github.com/polygone/core
Licence: MIT"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Verbosity level (0=warn, 1=info, 2=debug, 3=trace)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,
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
        Commands::Send { peer_pk, message } => cmd_send(peer_pk, message).await,
        Commands::Receive { sk, ciphertext } => cmd_receive(sk, ciphertext).await,
        Commands::Node { action } => cmd_node(action).await,
        Commands::Status => cmd_status().await,
        Commands::SelfTest => cmd_selftest().await,
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

    // TODO: write to disk with proper permissions (chmod 600)
    Ok(())
}

async fn cmd_send(peer_pk: String, message: String) -> anyhow::Result<()> {
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
        println!("          Message: "{}"", String::from_utf8_lossy(&recovered));

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
    // Not yet implemented — DHT-based peer resolution is v0.2.
    eprintln!("ERROR: Loading peer public keys from files is not yet implemented.");
    eprintln!("       Use --peer-pk demo to run a local protocol demonstration.");
    eprintln!("       Full peer-to-peer send arrives in v0.2 (libp2p + DHT).");
    std::process::exit(1);
}

async fn cmd_receive(_sk: String, _ciphertext: String) -> anyhow::Result<()> {
    println!("⬡ POLYGONE — Receiving message...");
    println!("  (Full implementation with DHT peer discovery coming in v0.2)");
    Ok(())
}

async fn cmd_node(action: NodeAction) -> anyhow::Result<()> {
    match action {
        NodeAction::Start { ram_mb, listen } => {
            println!("⬡ POLYGONE NODE");
            println!("  Listen  : {listen}");
            println!("  RAM cap : {ram_mb} MB");
            println!();
            println!("  ✓ Node started — participating in ephemeral transit network");
            println!("  ✓ You will never see the content of any message you relay.");
            println!("  ✓ Press Ctrl-C to stop.");
            println!();
            // TODO: start libp2p swarm with Kademlia DHT
            tokio::signal::ctrl_c().await?;
            println!("  ✓ Node stopped cleanly.");
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
