// ⬡ POLYGONE — the organism's visual heartbeat
//
// Every startup emits a banner. We chose not to draw a logo.
// We chose to draw a moment.
//
// The banner is: 30 seconds wide. Quiet by design.
// Read it slowly. It will be gone in the next release.

/// Banner displayed at startup. Read it like a poem.
pub const BANNER: &str = r#"




                          ⬡
                          △
                         /|\
                        / | \
                       /  |  \
                      ⬡   ⬡   ⬡




       p o s t - q u a n t u m   ·   e p h e m e r a l
       c h a i n e d   i n   t h i r t y   s e c o n d s   t h e n   g o n e


"#;

/// Sentinel heartbeat (printed when starting the node).
/// Apple says "Hello", we say nothing. Just a pulse.
pub const NODE_STARTING: &str = r#"
[Polygone] boot
[Polygone] crypto ··· ML-KEM-1024 (post-quantum)
[Polygone] network · Kademlia DHT + libp2p
[Polygone] sharing ·· Shamir 4-of-7 (information-theoretic)
[Polygone] ttl ····· 30 seconds (auto-evaporate)
[Polygone] health ·· http://localhost:{port}/health
[Polygone] version · v1.0.0
[Polygone] onyx ···· privacy.is
"#;

/// Hexagram (inline icon for transcripts)
pub const HEX_ICON: &str = "⬡";

/// Self-test expected output — minimal, monastic.
pub const SELF_TEST_EXPECTED: &str = r#"
[Polygone] self-test
[Polygone] ✓ ML-KEM-1024 key generation
[Polygone] ✓ ML-KEM encapsulate/decapsulate
[Polygone] ✓ AES-256-GCM encrypt/decrypt
[Polygone] ✓ Shamir 4-of-7 split/reconstruct
[Polygone] ✓ BLAKE3 hash
[Polygone] ✓ information-theoretic (k-1 = 0)

[Polygone] ready.
"#;

/// Compact help — only what you need to know.
pub const HELP_TEXT: &str = r#"
⬡ polygone

Commands:
  start           launch node (p2p + health endpoint)
  keygen          generate node keys (ml-kem-1024)
  send  <msg>     send ephemeral message
  node            start as relay node
  self-test       cryptographic self-test
  status          network status
  update          update to latest release
  uninstall       remove polygone and all local data

Flags:
  -v, --version
  -h, --help
  --no-banner

Environment:
  POLYGONE_PORT       health port (default: 8080)
  POLYGONE_PEER_ID    fixed peer ID (auto if unset)
  POLYGONE_LOG        error|warn|info|debug (default: info)

⬡ privacy.is — your message, gone in 30s.
"#;
