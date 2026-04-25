# Changelog

All notable changes to POLYGONE are documented here.

## [Unreleased] — 2026-04-25

### New in April 25, 2026 — Ecosystem Polish

**Landing page (index.html):**
- Cinematic terminal aesthetic with self-test animation preview
- Protocol flow: ML-KEM → AES → Shamir → DHT → Vaporize
- OS tabs (Linux/macOS/Windows) with copy buttons
- Stats row: 34µs / 3.8µs / 208µs / 0 metadata leaked
- 6 crypto feature cards, comparison table (VPN/Tor/POLYGONE)

**Installer (install.sh):**
- Smart installer: downloads pre-built binary OR builds from source
- GitHub Releases support (download v1.0.0 binary directly)
- Fallback to `cargo build --release` if no pre-built available
- Self-test run after install to confirm operation

**Documentation:**
- Complete README rewrite with comparison table
- ASCII architecture diagram
- Security properties in rust comments
- Full command reference
- MIT License badge, Rust badge, version badge

**CI/CD:**
- GitHub Actions workflow: auto-build on `git tag v*`
- Binary uploaded to GitHub Releases automatically

### Bug Fixes

- `cargo fix`: removed unused `Duration` import in compute/idle.rs
- `cargo fix`: removed unused `MessageKind` in tui/views.rs
- `cargo fix`: removed unused `app` variable in tui/widgets.rs
- Cargo.toml cleaned (features section, optional deps)

### Known Issues

- `polygone-server` bin has broken libp2p 0.54→0.56 API — disabled from main build
  - Run `cargo build --bin polygone-server --features server` to attempt fix
  - Requires: add `libp2p = "0.56"` to Cargo.toml + fix SwarmBuilder API

---

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

### Bug Fixes from v0.1 skeleton

- **CRITICAL**: `Cargo.toml` referenced `src/lib.rs` but files were at root → project did not compile
- `keygen`: keys were generated but never written to disk
- `send` (non-demo): immediately exited with `std::process::exit(1)`
- `receive`: was an empty stub (`"coming in v0.2"`)
- `node start`: waited for Ctrl-C with no logic
- No TUI implementation existed
- Module tree `src/crypto/`, `src/network/`, `src/protocol/` did not exist

---

## [0.1.0] — 2026-04-15 (skeleton)

- Initial commit with architecture sketch
- CLI structure with clap v4