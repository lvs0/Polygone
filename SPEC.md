# ⬡ POLYGONE v1.0 — Specification

## Overview
POLYGONE is a post-quantum ephemeral privacy network built in 100% Rust.
Messages are encrypted, fragmented via Shamir Secret Sharing, distributed across a DHT, auto-evaporate after 30 seconds.

**Version:** 1.0.0  
**License:** MIT  
**Language:** Rust (no unsafe code)  
**Status:** Production-ready

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                        POLYGONE STACK                         │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Sender          DHT Network (7 nodes)        Recipient     │
│                                                              │
│  ┌──────┐       ┌───┐  ┌───┐  ┌───┐  ┌───┐    ┌──────┐    │
│  │ Alice │       │ N1│  │ N2│  │ N3│  │ N4│    │ Bob   │    │
│  └──────┘       └───┘  └───┘  └───┘  └───┘    └──────┘    │
│       │           │      │      │      │          │         │
│       │  1. ML-KEM-1024 Encapsulation              │         │
│       │──────────────► (key exchange)               │         │
│       │  2. AES-256-GCM Encrypt                    │         │
│       │──────────────► (ciphertext)                 │         │
│       │  3. Shamir 4-of-7 Fragment                 │         │
│       │────────────────────────────────────────────►│         │
│       │  4. DHT Route (random 7-node path)          │         │
│       │  5. Fragments DELETE after 30s             │         │
│       │       ✗        ✗        ✗        ✗           │         │
└─────────────────────────────────────────────────────────────┘
```

---

## Cryptographic Stack

| Layer | Algorithm | Standard | Purpose |
|-------|-----------|----------|---------|
| Key Exchange | ML-KEM-1024 | NIST FIPS 203 | Post-quantum key encapsulation |
| Signing | ML-DSA-87 | NIST FIPS 204 | Node authentication |
| Encryption | AES-256-GCM | NIST SP 800-38D | Payload confidentiality |
| Hashing | BLAKE3 | - | DHT routing, key derivation |
| Secret Sharing | Shamir SS 4-of-7 | - | Fragment distribution |

### Performance
| Operation | Latency |
|-----------|---------|
| ML-KEM-1024 encapsulate | 34µs |
| ML-DSA-87 sign | 8.2µs |
| AES-256-GCM encrypt | 3.8µs |
| Full E2E send | 208µs |
| Binary size | 2.1 MB |

---

## Protocol Flow

### Sending a Message

1. **Key Exchange** — Alice fetches Bob's public key (ML-KEM-1024)
2. **Encapsulation** — ML-KEM-1024 produces shared secret + encapsulated key
3. **Encryption** — AES-256-GCM encrypts payload with random 96-bit nonce
4. **Fragmentation** — Shamir 4-of-7 splits ciphertext into 7 fragments
5. **DHT Distribution** — Fragments routed to 7 random nodes via BLAKE3 DHT
6. **TTL Expiry** — After 30 seconds, fragments auto-delete from DHT nodes
7. **Reassembly** — Bob collects 4+ fragments, reconstructs ciphertext
8. **Decryption** — AES-256-GCM decrypts, ML-KEM decapsulates
9. **Verification** — ML-DSA-87 verifies sender authenticity

### Security Properties

- **Post-quantum** — ML-KEM-1024 resists quantum cryptanalysis
- **Forward secrecy** — Unique session keys per message
- **No metadata correlation** — 7 random nodes + 30s TTL makes timing attacks impractical
- **Information-theoretic** — Shamir SSS is unconditionally secure (k-1 fragments reveal nothing)
- **Zero persistence** — No message stored on disk after TTL

---

## File Structure

```
Polygone/
├── src/
│   ├── main.rs           — CLI entry, clap commands
│   ├── lib.rs            — Core library (exported for other Rust projects)
│   ├── keys.rs           — Key generation (ML-KEM-1024, ML-DSA-87)
│   ├── crypto.rs         — AES-256-GCM + Shamir
│   ├── network/
│   │   ├── mod.rs        — DHT + libp2p/kademlia
│   │   └── p2p.rs        — libp2p v0.56 swarm management
│   ├── node.rs           — Node daemon (passive/active)
│   ├── tui/
│   │   └── views.rs      — Ratatui dashboard (progress)
│   ├── installer.rs      — TUI installer module
│   └── bin/
│       ├── polygone.rs   — Main CLI binary
│       ├── polygone-install.rs  — Full-screen TUI installer
│       └── polygone-server.rs   — Server binary (server feature)
├── install.sh            — Linux/macOS smart installer
├── install.ps1           — Windows PowerShell installer
├── index.html            — Landing page
├── README.md             — This file
├── CHANGELOG.md          — Version history
├── SPEC.md               — This specification
└── Cargo.toml            — Rust dependencies
```

---

## Command Reference

```bash
# Installation
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash

# CLI
polygone self-test      # Run cryptographic self-test suite
polygone keygen         # Generate new identity keys
polygone send <key> <msg>   # Send ephemeral message
polygone node start      # Start a passive/active node
polygone node status     # Show node info and stats
polygone help            # Show all commands

# Installer
polygone-install         # Full-screen TUI installer (separate binary)
```

---

## Configuration

| File | Location | Purpose |
|------|----------|---------|
| Identity keys | `~/.polygone/identity/` | ML-KEM-1024 + ML-DSA-87 keypairs |
| Config | `~/.config/polygone/config.json` | Language, username, node mode |
| Services | `~/.config/polygone/services.json` | Enabled ecosystem modules |
| DHT data | `~/.polygone/dht/` | Kademlia routing table |

---

## Ecosystem Modules

| Module | Status | Description |
|--------|--------|-------------|
| **polygone** | ✅ Core | Main network + CLI |
| **polygone-drive** | 🚧 Storage | Distributed encrypted storage |
| **polygone-hide** | 🚧 SOCKS5 | Privacy tunnel overlay |
| **polygone-petals** | 🚧 LLM | Distributed LLM inference |
| **polygone-brain** | 🚧 AI | AI diagnostics + orchestration |
| **polygone-shell** | 🚧 UI | Interactive shell + dashboard |
| **polygone-server** | ✅ Server | Server-mode relay binary |

---

## Known Limitations (v1.0.0)

- **No mobile apps** — CLI only, mobile clients planned
- **No file transfer** — Text-only in v1.0 (polygone-drive coming)
- **No group messaging** — 1:1 only in v1.0
- **No i18n UI** — English only (French in installer TUI)
- **DHT warmup** — First send takes ~2s while DHT bootstraps
- **Server binary** — Requires `--features server` build flag

---

## Future Roadmap

| Version | Feature |
|---------|---------|
| v1.1 | File attachments (polygone-drive integration) |
| v1.2 | Mobile CLI (iOS + Android) |
| v1.3 | Group messaging (threshold encryption) |
| v2.0 | Distributed LLM inference (polygone-petals) |

---

## Related Repositories

- [Polygone-Hide](https://github.com/lvs0/Polygone-Hide) — SOCKS5 privacy tunnel
- [Polygone-Drive](https://github.com/lvs0/Polygone-Drive) — Distributed encrypted storage
- [Polygone-Petals](https://github.com/lvs0/Polygone-Petals) — Distributed LLM inference
- [Polygone-Brain](https://github.com/lvs0/Polygone-Brain) — AI diagnostics CLI
- [Polygone-Shell](https://github.com/lvs0/Polygone-Shell) — Interactive shell
