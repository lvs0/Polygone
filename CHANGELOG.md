# Changelog

All notable changes to POLYGONE are documented here.

## [1.0.0] — 2026-04-17

### New in v1.0

**Fully functional local protocol:**

- `polygone keygen` — generates ML-KEM-1024 + Ed25519 keypair, writes to disk with `chmod 600`
- `polygone send --peer-pk demo --message <text>` — full local Alice→Bob round-trip with topology verification
- `polygone send --peer-pk <hex|file> --message <text>` — encrypt for a real peer public key
- `polygone receive` — decapsulate and establish session (fragment collection: v2.0)
- `polygone node start/stop/info` — node lifecycle management
- `polygone status` — show keypair status, sessions, node health
- `polygone self-test` — 5 tests covering KEM, AES, Shamir, full session, insufficient fragments
- `polygone tui` — full TUI dashboard with ratatui (6 views: Dashboard, Keygen, Send, Receive, Node, Help)

**Crypto stack:**
- ML-KEM-1024 (FIPS 203) via `ml-kem` RustCrypto crate
- Ed25519 signatures via `ed25519-dalek` (ML-DSA upgrade path documented)
- AES-256-GCM with fresh 96-bit OsRng nonce per message
- BLAKE3 domain-separated KDF (labels: `polygone-topo-nodes-v1`, `polygone-sess-v1`)
- Shamir 4-of-7 with C(7,4)=35 combinations tested

**Safety:**
- `#[forbid(unsafe_code)]` crate-wide
- `ZeroizeOnDrop` on all key material (KEM keys, session keys, shared secret)
- Key files written with `chmod 600`, directory with `chmod 700`

**Project structure fixed:**
- All source in `src/` with proper module hierarchy
- `Cargo.toml` paths corrected (`src/lib.rs`, `src/main.rs`)
- 100% of TODOs from previous skeleton resolved

### Bug fixes from v0.1 skeleton

- **CRITICAL**: `Cargo.toml` referenced `src/lib.rs` but files were at root → project did not compile
- `keygen`: keys were generated but never written to disk
- `send` (non-demo): immediately exited with `std::process::exit(1)`
- `receive`: was an empty stub (`"coming in v0.2"`)
- `node start`: waited for Ctrl-C with no logic
- No TUI implementation existed
- Module tree `src/crypto/`, `src/network/`, `src/protocol/` did not exist

## [0.1.0] — 2026-04-15 (skeleton)

- Initial commit with architecture sketch
- CLI structure with clap v4
- `send --peer-pk demo` local demo working
- Crypto primitives designed but not in correct module layout
- No TUI, no key persistence, no receive, no real send
