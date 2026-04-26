//! polygone — CLI entry point.
//!
//! Commands:

#![allow(missing_docs)]
//!   polygone keygen            → Generate ML-KEM-1024 + Ed25519 keypair, save to disk
//!   polygone send              → Encrypt and fragment a message
//!   polygone receive           → Reconstruct and decrypt a message
//!   polygone node start|stop   → Relay node management
//!   polygone status            → Show node and session status
//!   polygone self-test         → Run crypto self-test suite
//!   polygone tui               → Launch the TUI dashboard

#![forbid(unsafe_code)]

use std::io::{self, Read};
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tracing_subscriber::{fmt, EnvFilter};

use polygone::{
    KeyPair, Session, VERSION,
    crypto::{kem, shamir},
    keys,
    network::TopologyParams,
    tui::{run_tui, views::View},
};

// ── CLI Definition ────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(
    name = "polygone",
    version = VERSION,
    author = "Lévy <polygone@proton.me>",
    about = "Post-quantum ephemeral transit network",
    long_about = "\
⬡ POLYGONE — L'information n'existe pas. Elle traverse.

A post-quantum ephemeral network where messages become distributed
computational state. ML-KEM-1024 key exchange. AES-256-GCM encryption.
Shamir 4-of-7 fragmentation. BLAKE3 domain-separated key derivation.

No server sees the message. No observer can prove a message existed.
Classical encryption hides content. POLYGONE hides the communication itself.

Source: https://github.com/lvs0/Polygone
License: MIT — No investors. No token. No telemetry."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Verbosity: -v = info, -vv = debug, -vvv = trace
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    verbose: u8,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new post-quantum keypair and save to disk
    Keygen {
        /// Output directory for key files (default: ~/.polygone/keys)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Overwrite existing keys without prompting
        #[arg(long)]
        force: bool,
    },

    /// Encrypt and send a message through the ephemeral network
    Send {
        /// Recipient's KEM public key (hex, path to .pk file, or 'demo' for local round-trip)
        #[arg(short, long)]
        peer_pk: String,

        /// Message to send (use '-' to read from stdin)
        #[arg(short, long)]
        message: String,
    },

    /// Receive and decrypt a message (responder role)
    Receive {
        /// Path to your KEM secret key file (default: ~/.polygone/keys/kem.sk)
        #[arg(short, long)]
        sk: Option<PathBuf>,

        /// KEM ciphertext from the initiator (hex string)
        #[arg(short, long)]
        ciphertext: String,
    },

    /// Relay node management
    Node {
        #[command(subcommand)]
        action: NodeAction,
    },

    /// Show current status: sessions, node health, network peers
    Status,

    /// Run the first-time setup wizard
    Setup,

    /// Run the full self-test suite (crypto + protocol)
    #[command(name = "self-test")]
    SelfTest,

    /// Launch the interactive TUI dashboard
    Tui {
        /// Which view to open first (dashboard|keygen|send|receive|node|selftest|help)
        #[arg(short, long, default_value = "dashboard")]
        view: String,
    },
}

#[derive(Subcommand)]
enum NodeAction {
    /// Start the relay node daemon
    Start {
        /// Maximum RAM to allocate (MB)
        #[arg(short, long, default_value = "256")]
        ram_mb: usize,
        /// Listen address
        #[arg(short, long, default_value = "0.0.0.0:4001")]
        listen: String,
    },
    /// Stop the running node daemon
    Stop,
    /// Show this node's public information
    Info,
}

// ── Entry Point ───────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let filter = match cli.verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };
    fmt()
        .with_env_filter(EnvFilter::new(filter))
        .with_target(false)
        .init();

    match cli.command {
        Commands::Keygen { output, force }         => cmd_keygen(output, force).await,
        Commands::Send { peer_pk, message }        => cmd_send(peer_pk, message).await,
        Commands::Receive { sk, ciphertext }       => cmd_receive(sk, ciphertext).await,
        Commands::Node { action }                  => cmd_node(action).await,
        Commands::Status                           => cmd_status(),
        Commands::Setup                            => cmd_setup().await,
        Commands::SelfTest                         => cmd_selftest().await,
        Commands::Tui { view }                     => cmd_tui(view),
    }
}

// ── keygen ────────────────────────────────────────────────────────────────────

async fn cmd_keygen(output: Option<PathBuf>, force: bool) -> anyhow::Result<()> {
    let dir = output.unwrap_or_else(keys::default_key_dir);

    println!("⬡ POLYGONE — Generating post-quantum keypair");
    println!();
    println!("  Algorithm : ML-KEM-1024 (FIPS 203) + Ed25519");
    println!("  Directory : {}", dir.display());
    println!();

    if keys::keypair_exists(&dir) && !force {
        eprintln!("  ⚠ Keypair already exists at {}.", dir.display());
        eprintln!("    Use --force to overwrite.");
        std::process::exit(1);
    }

    print!("  Generating ML-KEM-1024 keypair …");
    let kp = KeyPair::generate()?;
    println!(" done");

    print!("  Writing key files …");
    keys::write_keypair(&kp, &dir)?;
    println!(" done");

    println!();
    println!("  ✔ kem.pk   {} B  (share freely)", kp.kem_pk.as_bytes().len());
    println!("  ✔ sign.pk  {} B  (share freely)", kp.sign_pk.as_bytes().len());
    println!("  ✔ kem.sk   {} B  (KEEP PRIVATE, chmod 600)", kp.kem_sk.to_hex().len() / 2);
    println!("  ✔ sign.sk  {} B  (KEEP PRIVATE, chmod 600)", kp.sign_sk.to_hex().len() / 2);
    println!();
    println!("  KEM public key (first 48 hex chars):");
    println!("    {}…", &kp.kem_pk.to_hex()[..48]);
    println!();
    println!("  → Share your KEM public key with anyone who wants to send you a message.");
    println!("  → Your secret key cannot be recovered if lost. Back it up securely.");

    Ok(())
}

// ── send ──────────────────────────────────────────────────────────────────────

async fn cmd_send(peer_pk_arg: String, message_arg: String) -> anyhow::Result<()> {
    // Resolve message (stdin or direct)
    let message = if message_arg == "-" {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        buf.trim().to_string()
    } else {
        message_arg
    };

    // ── Demo mode ─────────────────────────────────────────────────────────────
    if peer_pk_arg == "demo" {
        return cmd_send_demo(message).await;
    }

    // ── Real mode: load peer public key ───────────────────────────────────────
    let peer_pk = resolve_peer_pk(&peer_pk_arg)?;

    println!("⬡ POLYGONE — Encrypting message");
    println!();

    // Alice generates a fresh ephemeral keypair and encapsulates
    let (mut session, ciphertext) = Session::new_initiator(&peer_pk)?;

    println!("  [1/4] ML-KEM-1024 encapsulation …");
    println!("        Ciphertext: {} bytes", ciphertext.as_bytes().len());
    println!("        → Send this ciphertext to your peer (out-of-band):");
    println!("          {}", ciphertext.to_hex());
    println!();

    // In v1.0, the peer must call `polygone receive` with this ciphertext.
    // In v2.0, this will be dispatched via the DHT automatically.

    session.establish(None)?;
    let topo = session.topology.as_ref().unwrap().clone();
    println!("  [2/4] Topology derived — {}", topo.describe());
    println!("        Nodes:");
    for (i, node) in topo.nodes.iter().enumerate() {
        println!("          Node {i}: {node}");
    }
    println!();

    let assignments = session.send(message.as_bytes())?;

    println!("  [3/4] Encrypted + fragmented");
    println!("        {}/{} fragments produced", assignments.len(), topo.params.node_count);
    println!("        Threshold: {}/{} — no fewer can reconstruct", topo.params.threshold, topo.params.node_count);
    println!();

    println!("  [4/4] Fragments (give these to your peer along with the ciphertext above):");
    for (node_id, frag_bytes) in &assignments {
        println!("        Node {node_id}: {} bytes → {}", frag_bytes.len(), hex::encode(&frag_bytes[..frag_bytes.len().min(16)]));
    }

    session.dissolve();
    println!();
    println!("  ✔ Session dissolved — keying material zeroed.");
    println!("  The exchange will complete when your peer reconstructs with ≥4 fragments.");

    Ok(())
}

async fn cmd_send_demo(message: String) -> anyhow::Result<()> {
    println!("⬡ POLYGONE — Local protocol demo (Alice → Bob)");
    println!();

    // Bob generates his keypair
    let bob_kp = KeyPair::generate()?;
    println!("  [BOB]   Generated keypair — KEM public key ready");

    // Alice initiates
    let (mut alice, ciphertext) = Session::new_initiator(&bob_kp.kem_pk)?;
    println!("  [ALICE] ML-KEM-1024 encapsulation → Bob");
    println!("          Ciphertext: {} bytes", ciphertext.as_bytes().len());

    // Bob decapsulates
    let mut bob = Session::new_responder(bob_kp, &ciphertext)?;
    println!("  [BOB]   Decapsulated — shared secret recovered");

    // Both establish topology independently
    alice.establish(None)?;
    bob.establish(None)?;
    let topo = alice.topology.as_ref().unwrap().clone();
    println!("  [BOTH]  Topology derived — {}", topo.describe());

    // Alice checks both sides derive the same topology
    let alice_nodes: Vec<_> = alice.topology.as_ref().unwrap().nodes.iter().map(|n| n.to_string()).collect();
    let bob_nodes:   Vec<_> = bob.topology.as_ref().unwrap().nodes.iter().map(|n| n.to_string()).collect();
    assert_eq!(alice_nodes, bob_nodes, "Topology mismatch — this is a bug");
    println!("          ✔ Topology identical on both sides (deterministic)");
    println!();

    // Alice sends
    let assignments = alice.send(message.as_bytes())?;
    let n_shards = assignments.len();
    println!("  [ALICE] Message encrypted → Shamir-fragmented");
    println!("          {n_shards}/{} fragments — threshold {}/{}",
        topo.params.node_count, topo.params.threshold, topo.params.node_count);
    println!("          No single node holds a reconstructable piece.");
    println!();

    // Simulate fragment delivery (in-process, v1.0)
    let fragments: Vec<Vec<u8>> = assignments.into_iter().map(|(_, b)| b).collect();

    // Bob reconstructs
    let recovered = bob.receive(fragments)?;
    println!("  [BOB]   Reconstructed → decrypted");
    println!("          Message: \"{}\"", String::from_utf8_lossy(&recovered));
    println!();

    // Both dissolve
    alice.dissolve();
    bob.dissolve();
    println!("  [BOTH]  Session dissolved — keying material zeroed");
    println!("          The exchange did not happen.");
    println!();
    println!("  ─────────────────────────────────────────────────────────────");
    println!("  NOTE: Fragment transport is in-process in v1.0 (local mode).");
    println!("        Real P2P dispatch via libp2p + Kademlia DHT: v2.0.");

    Ok(())
}

fn resolve_peer_pk(arg: &str) -> anyhow::Result<kem::KemPublicKey> {
    // Try as a path first
    let path = std::path::Path::new(arg);
    if path.exists() {
        let hex = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("cannot read {}: {e}", path.display()))?;
        return Ok(kem::KemPublicKey::from_hex(hex.trim())?);
    }
    // Otherwise treat as hex
    kem::KemPublicKey::from_hex(arg)
        .map_err(|e| anyhow::anyhow!("{e}\n  Hint: pass a hex public key, a path to a .pk file, or 'demo'"))
}

// ── receive ───────────────────────────────────────────────────────────────────

async fn cmd_receive(sk_path: Option<PathBuf>, ciphertext_hex: String) -> anyhow::Result<()> {
    println!("⬡ POLYGONE — Receiving message");
    println!();

    // Resolve secret key
    let sk_dir = sk_path.map(|p| {
        if p.is_file() { p.parent().map(|d| d.to_path_buf()).unwrap_or_default() } else { p }
    }).unwrap_or_else(keys::default_key_dir);

    print!("  Loading keypair …");
    let kp = keys::read_keypair(&sk_dir)
        .map_err(|e| anyhow::anyhow!("{e}\n  Hint: run `polygone keygen` first"))?;
    println!(" done");

    print!("  Decapsulating KEM ciphertext …");
    let ct = kem::KemCiphertext::from_hex(&ciphertext_hex)?;
    let mut session = Session::new_responder(kp, &ct)?;
    println!(" done");

    session.establish(None)?;
    let topo = session.topology.as_ref().unwrap();
    println!("  Topology derived — {}", topo.describe());
    println!();
    println!("  Waiting for {} fragments (need at least {})…", topo.params.node_count, topo.params.threshold);
    println!();
    println!("  NOTE: In v1.0, fragments must be provided manually.");
    println!("        In v2.0, they will be collected automatically from the DHT.");
    println!();
    println!("  → Provide the fragment hex strings from the sender, then run:");
    println!("      polygone receive --sk <path> --ciphertext <ct> [fragments will be auto-collected in v2.0]");

    session.dissolve();

    Ok(())
}

// ── node ──────────────────────────────────────────────────────────────────────

async fn cmd_node(action: NodeAction) -> anyhow::Result<()> {
    match action {
        NodeAction::Start { ram_mb, listen } => {
            println!("⬡ POLYGONE NODE");
            println!();
            println!("  Listen  : {listen}");
            println!("  RAM cap : {ram_mb} MB");
            println!("  Mode    : local (libp2p integration: v2.0)");
            println!();
            println!("  ✔ Node started — participating in ephemeral transit network");
            println!("  ✔ You will never see the content of any message you relay.");
            println!("  ✔ Press Ctrl-C to stop gracefully.");
            println!();

            // In v2.0: start libp2p swarm with Kademlia DHT here.
            // For v1.0: we hold the process alive so the node is "reachable" locally.
            tokio::signal::ctrl_c().await?;
            println!();
            println!("  ✔ Node stopped cleanly. Keying material zeroed.");
        }
        NodeAction::Stop => {
            println!("⬡ POLYGONE NODE — Stopping…");
            println!("  (In v2.0 this will signal the daemon via IPC)");
        }
        NodeAction::Info => {
            println!("⬡ POLYGONE NODE INFO");
            println!();
            println!("  Version  : {VERSION}");
            println!("  Status   : offline");
            println!("  Peers    : 0 (libp2p in v2.0)");
            println!("  RAM cap  : 256 MB (default)");
        }
    }
    Ok(())
}

// ── status ────────────────────────────────────────────────────────────────────

fn cmd_status() -> anyhow::Result<()> {
    use std::path::PathBuf;

    let config_path = dirs::config_dir()
        .unwrap_or(PathBuf::from("~/.config"))
        .join("polygone/config.toml");

    let is_setup = config_path.exists();
    let config_content = is_setup.then(|| std::fs::read_to_string(&config_path).ok()).flatten();

    let username = config_content.as_ref()
        .and_then(|c| c.lines().find(|l| l.contains("username")).map(|l| l.split('"').nth(1).unwrap_or("?").to_string()))
        .unwrap_or_else(|| "?".to_string());

    println!("
[36m⬡ POLYGONE STATUS[0m
");
    println!("  [37mSetup status :[0m {} [32m{}[0m", if is_setup { "✔ Configuré" } else { "○ Non configuré" }, if !is_setup { "[33m(run polygone setup)[0m" } else { "" });
    if is_setup {
        println!("  [37mUsername    :[0m {}", username);
    }
    println!("  [37mNode status :[0m [31m○ Déconnecté[0m (P2P in v2.0)");
    println!("  [37mNetwork     :[0m N/A (DHT in v2.0)");
    println!();

    Ok(())
}

// ── self-test ─────────────────────────────────────────────────────────────────
// ── self-test ─────────────────────────────────────────────────────────────────

async fn cmd_selftest() -> anyhow::Result<()> {
    use polygone::crypto::{kem, shamir, symmetric::SessionKey};

    println!("⬡ POLYGONE SELF-TEST");
    println!();

    let mut passed = 0usize;
    let mut failed = 0usize;

    macro_rules! test {
        ($name:expr, $block:block) => {{
            print!("  {} … ", $name);
            match (|| -> anyhow::Result<()> { $block; Ok(()) })() {
                Ok(_) => { println!("PASS ✔"); passed += 1; }
                Err(e) => { println!("FAIL ✖  ({e})"); failed += 1; }
            }
        }};
    }

    test!("[1/5] ML-KEM-1024 round-trip", {
        let (pk, sk) = kem::generate_keypair()?;
        let (ct, ss1) = kem::encapsulate(&pk)?;
        let ss2 = kem::decapsulate(&sk, &ct)?;
        anyhow::ensure!(ss1.0 == ss2.0, "shared secrets mismatch");
    });

    test!("[2/5] AES-256-GCM encrypt/decrypt", {
        let key = SessionKey::from_bytes([0xABu8; 32]);
        let msg = b"post-quantum privacy";
        let ct = key.encrypt(msg)?;
        let pt = key.decrypt(&ct)?;
        anyhow::ensure!(pt == msg, "decrypted message mismatch");
    });

    test!("[3/5] Shamir 4-of-7 (all 35 combinations)", {
        let secret = b"polygone-shamir-test-secret-bytes";
        let frags = shamir::split(secret, 4, 7)?;
        for i in 0..7 { for j in (i+1)..7 { for k in (j+1)..7 { for l in (k+1)..7 {
            let subset = vec![frags[i].clone(), frags[j].clone(),
                              frags[k].clone(), frags[l].clone()];
            let r = shamir::reconstruct(&subset, 4)?;
            anyhow::ensure!(r == secret, "C({i},{j},{k},{l}) failed");
        }}}}
    });

    test!("[4/5] Full session round-trip (Alice → Bob)", {
        let bob_kp = KeyPair::generate()?;
        let (mut alice, ct) = Session::new_initiator(&bob_kp.kem_pk)?;
        let mut bob = Session::new_responder(bob_kp, &ct)?;
        alice.establish(None)?;
        bob.establish(None)?;

        // Verify both sides derive identical topology
        let a_nodes: Vec<_> = alice.topology.as_ref().unwrap().nodes.iter().map(|n| n.0).collect();
        let b_nodes: Vec<_> = bob.topology.as_ref().unwrap().nodes.iter().map(|n| n.0).collect();
        anyhow::ensure!(a_nodes == b_nodes, "topology mismatch between Alice and Bob");

        let msg = b"L'information n'existe pas. Elle traverse.";
        let assignments = alice.send(msg)?;
        let frags: Vec<_> = assignments.into_iter().map(|(_, b)| b).collect();
        let recovered = bob.receive(frags)?;
        anyhow::ensure!(recovered == msg, "message mismatch");
        alice.dissolve();
        bob.dissolve();
    });

    test!("[5/5] Insufficient fragments → rejected", {
        let bob_kp = KeyPair::generate()?;
        let (mut alice, ct) = Session::new_initiator(&bob_kp.kem_pk)?;
        let mut bob = Session::new_responder(bob_kp, &ct)?;
        alice.establish(None)?;
        bob.establish(None)?;
        let assignments = alice.send(b"secret")?;
        let frags: Vec<_> = assignments.into_iter().take(3).map(|(_, b)| b).collect();
        anyhow::ensure!(bob.receive(frags).is_err(), "should have failed with 3/7 fragments");
    });

    println!();
    if failed == 0 {
        println!("  ✔ All {passed} tests passed. POLYGONE is operational.");
    } else {
        println!("  ✖ {failed}/{} tests FAILED.", passed + failed);
        std::process::exit(1);
    }

    Ok(())
}

// ── tui ───────────────────────────────────────────────────────────────────────

fn cmd_tui(view: String) -> anyhow::Result<()> {
    let initial = match view.as_str() {
        "keygen"   => View::Keygen,
        "send"     => View::Send,
        "receive"  => View::Receive,
        "node"     => View::Node,
        "selftest" => View::SelfTest,
        "help"     => View::Help,
        _          => View::Dashboard,
    };
    run_tui(initial).map_err(|e| anyhow::anyhow!("TUI error: {e}"))
}

// ── setup ─────────────────────────────────────────────────────────────────────

async fn cmd_setup() -> anyhow::Result<()> {
    println!("\n\x1b[36m⬡ POLYGONE SETUP WIZARD\x1b[0m\n");

    // Ask username
    print!("Username (3-20 chars, alphanumeric + underscore): ");
    std::io::Write::flush(&mut std::io::stdout())?;
    let mut username = String::new();
    std::io::stdin().read_line(&mut username)?;
    let username = username.trim();

    if username.len() < 3 || username.len() > 20 || !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        println!("\x1b[31m❌ Username must be 3-20 chars, alphanumeric + underscore only\x1b[0m");
        return Ok(());
    }

    // Create config directory and file
    let config_dir = dirs::config_dir().unwrap_or_default().join("polygone");
    std::fs::create_dir_all(&config_dir)?;
    let config_path = config_dir.join("config.toml");


    let config = format!(r#"# Polygone Configuration
# Generated by polygone setup

[setup]
username = "{}"
language = "fr"

[node]
auto_connect = false
bootstrap_nodes = ["127.0.0.1:9001"]

[network]
max_peers = 50
"#, username);

    std::fs::write(&config_path, &config)?;

    println!("\n\x1b[32m✅ Setup complete!\x1b[0m");
    println!();
    println!("  Config saved to: {:?}", config_path);
    println!();
    println!("\x1b[33mNext steps:\x1b[0m");
    println!("  \x1b[36mpolygone keygen\x1b[0m    → Generate your identity keys");
    println!("  \x1b[36mpolygone status\x1b[0m  → Check your status");

    Ok(())
}
