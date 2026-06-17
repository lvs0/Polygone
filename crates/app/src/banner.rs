// Polygone — ASCII banner module
// Affiche le logo hex + informations de version au démarrage

/// Banner affiché au démarrage de `polygone help` / sans arguments
pub const BANNER: &str = r#"
╔═══════════════════════════════════════════════════════════╗
║                                                           ║
║          ⬡  P O L Y G O N E  —  v1.0.0               ║
║                                                           ║
║          Post-Quantum · Ephemeral · Zero Metadata        ║
║                                                           ║
║          ML-KEM-1024  ·  Shamir 4-of-7  ·  AES-256-GCM   ║
║          BLAKE3 · Kademlia DHT · 30s Vaporization        ║
║                                                           ║
╚═══════════════════════════════════════════════════════════╝
"#;

/// Petit logo hex pour affichage inline (CLI / logs)
pub const HEX_ICON: &str = r#"⬡"#;

/// Commandes disponibles
pub const HELP_TEXT: &str = r#"
Usage: polygone <COMMAND>

Commands:
  start              Launch P2P node + TUI dashboard
  keygen             Generate node keypair (ML-KEM-1024)
  send <MSG>         Send an ephemeral message
  node               Start as relay node (contribute to network)
  self-test          Run cryptographic self-test
  status             Show network status + peer count
  update             Update to latest version
  uninstall          Remove Polygone and all data

Flags:
  -v, --version      Show version
  -h, --help         Show this help
  --no-banner        Skip banner on start

Environment:
  POLYGONE_PORT      HTTP health port (default: 8080)
  POLYGONE_PEER_ID   Fixed peer ID (auto-generated if absent)
  POLYGONE_LOG       Log level: error|warn|info|debug (default: info)
"#;

/// Message de démarrage du nœud
pub const NODE_STARTING: &str = r#"
[Polygone] ⬡ Node starting...
[Polygone]   Crypto:   ML-KEM-1024 (post-quantum)
[Polygone]   Network:  Kademlia DHT + libp2p
[Polygone]   Sharing:  Shamir 4-of-7 (any 4 of 7 fragments)
[Polygone]   TTL:      30 seconds — auto-evaporate
[Polygone]   Health:   http://localhost:{port}/health

[Polygone] Press Ctrl+C to stop.
"#;

/// Test automatique (sortie attendue)
pub const SELF_TEST_EXPECTED: &str = r#"
[Polygone] Running self-test...
[Polygone] ✓ ML-KEM-1024 key generation: OK
[Polygone] ✓ ML-KEM encapsulation/decapsulation: OK
[Polygone] ✓ AES-256-GCM encrypt/decrypt: OK
[Polygone] ✓ Shamir 4-of-7 split/reconstruct: OK
[Polygone] ✓ BLAKE3 hash: OK
[Polygone] ✓ Information-theoretic security (k-1 = 0): OK

[Polygone] All tests passed. System ready.
"#;