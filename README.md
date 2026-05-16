# Polygone — Post-Quantum Privacy Network

**A privacy network built on mathematical waves. No tunnel. No metadata leak.**

> ⚠️ This is the local development workspace. For investors and partners, see our [docs/](docs/) folder.

## Architecture

```
polygone/
├── polygone-core/        ← Unified crate (re-exports common, crypto, network)
├── polygone-brain/       ← Orchestrator & diagnostics
├── polygone-shell/       ← TUI dashboard (ratatui)
├── polygone-app/         ← CLI application (axum)
├── crates/
│   ├── common/          ← Shared types (NodeId, Packet, Session…)
│   ├── crypto/          ← ML-KEM-1024, ML-DSA-87, AES-256-GCM, Shamir, BLAKE3
│   ├── network/         ← P2P libp2p/Kademlia, FragmentDispatcher
│   └── gateway/         ← HTTP→P2P gateway (axum)
└── docs/                ← Investor package, whitepaper, legal guide
```

## Quick Start

```bash
# Build all crates
cargo check --workspace

# Run the shell dashboard
cd polygone-shell && cargo run --release

# Run tests
cargo test --workspace
```

## Status

- ✅ Workspace: 9 crates, `cargo check --workspace` → 0 error, 0 warning
- ✅ CI/CD: GitHub Actions (`cargo check --workspace` + `cargo test --workspace --lib`)
- ✅ 49 tests passing in polygone-crypto
- ✅ GitHub synced: `origin/main` at `0f99c5f`
- ⚠️ polygone-petals: excluded (candle-core/rand incompatibility — monitored daily)

## Investor / Partner Documents

| Document | Purpose |
|---|---|
| `docs/investor-deck.html` | Pitch deck (12 slides, dark-themed) |
| `docs/whitepaper.md` | Technical whitepaper |
| `docs/executive-summary.txt` | One-page summary |
| `docs/legal-guide-france.md` | Legal structure guide (France) |
| `docs/partnership-template.md` | Friends/family investment template |
| `docs/recruitment-pitch.md` | Team recruitment pitch (French) |

## Links

- GitHub: https://github.com/lvs0/Polygone
- License: MIT