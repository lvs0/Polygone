# ⬡ POLYGONE

[![Post-Quantum](https://img.shields.io/badge/Post--Quantum-ML--KEM--1024-blue)](https://csrc.nist.gov/pubs/ir/8439/ipd)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org)
[![Security: ZeroizeOnDrop](https://img.shields.io/badge/Security-ZeroizeOnDrop-brightgreen)](https://crates.io/crates/zeroize)
[![Privacy](https://img.shields.io/badge/Privacy-Ephemeral%20Metadata-red)](https://github.com/lvs0/Polygone)
[![Status](https://img.shields.io/badge/Status-v0.1%20Alpha-orange)](https://github.com/lvs0/Polygone)

> *"Information does not exist. It drifts."*  
> *"L'information n'existe pas. Elle traverse."*

**POLYGONE** is an **ephemeral post-quantum privacy network** that solves the **Metadata Problem**. Built with Rust.

---

## ⚠️ Current Status: v0.1 Alpha

**This is experimental software.** The cryptographic primitives are fully implemented and tested. The P2P network layer is functional but requires bootstrap nodes to operate in production.

### What's Implemented ✅

| Component | Status | Notes |
|-----------|--------|-------|
| ML-KEM-1024 Key Exchange | ✅ Complete | NIST Level 5 |
| ML-DSA-87 Signatures | ✅ Complete | NIST Level 5 |
| AES-256-GCM Encryption | ✅ Complete | |
| Shamir Secret Sharing | ✅ Complete | 4-of-7 threshold |
| Session Protocol | ✅ Complete | Full cryptographic flow |
| libp2p/Kademlia DHT | ✅ Complete | Memory-based, needs bootstrap |
| CLI Interface | ✅ Complete | All commands functional |
| Memory Protection | ✅ Complete | ZeroizeOnDrop |
| Path Sanitization | ✅ Complete | Traversal protection |
| Polygone-Server Module | ✅ Complete | Resource management, rate limiting |
| Polygone-Drive Module | ✅ Complete | Erasure coding, encrypted storage, ephemeral links |
| Polygone-Hide Module | 🚧 Planned | Anonymous browsing |
| Polygone-Petals Module | 🚧 Planned | AI/ML inference network |
| Polygone-Shell Module | 🚧 Planned | Secure CLI |
| Polygone-Brain Module | 🚧 Planned | AI reasoning engine |
| Polygone-Compute Module | 🚧 In Progress | Distributed idle power computing |

### What's Planned 🚧

| Feature | Target | Status |
|---------|--------|--------|
| Persistent DHT Storage | v0.2 | Bootstrap nodes needed |
| NAT Traversal | v0.3 | Relay infrastructure |
| Mobile Clients | v0.4 | iOS/Android |
| Anonymous Credentials | v0.5 | |
| Mainnet | v1.0 | |

---

## The Problem Traditional Encryption Can't Solve

```
┌─────────────────────────────────────────────────────────┐
│  Alice ──────── [encrypted tunnel] ──────── Bob        │
│   IP │                                        │ IP     │
│   │  │              METADATA LEAK             │  │     │
│   └──┴───────────────────────────────────────┴──┘     │
│   Source/Destination/Timing always visible             │
└─────────────────────────────────────────────────────────┘

POLYGONE turns messages into distributed mathematical waves
that evaporate. No tunnel. No metadata.
```

---

## Quick Start

### Install (One-Line)

```bash
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash
```

### Install Options

```bash
# Interactive installer (recommended)
./install.sh

# Core only
./install.sh --core

# Core + Drive
./install.sh --core --drive

# System-wide with systemd
./install.sh --core --systemd --background

# Custom directory
./install.sh --core --custom-dir /opt/polygone

# Expert mode (no prompts)
./install.sh --core --expert --no-verify
```

### Manual Install

```bash
git clone https://github.com/lvs0/Polygone
cd Polygone
cargo build --release
cargo install --path .
```

### Demo Mode (No Network Required)

```bash
polygone send --peer-pk demo --message "Hello, anonymous world."
```

This runs a complete cryptographic session locally to verify the protocol works.

### Real Network Mode

```bash
# 1. Generate your keypair
polygone keygen --output ~/.polygone/keys

# 2. Share your kem.pk with your contact (out-of-band)

# 3. Send a message
polygone send --peer-pk /path/to/contact/kem.pk --message "Hello"

# 4. Receive the session ciphertext from your contact
polygone receive --sk ~/.polygone/keys/kem.sk --ciphertext session.ct
```

### Start a Relay Node

```bash
polygone node start --listen /ip4/0.0.0.0/tcp/4001
```

---

## Architecture

### How It Works

```
┌─────────────────────────────────────────────────────────────────┐
│                         SENDER (Alice)                           │
│                                                                 │
│  1. ML-KEM-1024 Encapsulation                                   │
│     Alice + Bob's PK → Ciphertext + Shared Secret               │
│                                                                 │
│  2. Topology Derivation (BLAKE3)                                │
│     Shared Secret → 7 node IDs + AES-256-GCM key               │
│                                                                 │
│  3. Encryption + Fragmentation                                   │
│     Message → AES-256-GCM → 7 fragments (4-of-7 Shamir)        │
│                                                                 │
│  4. DHT Storage (30s TTL)                                       │
│                         ↓                                       │
└─────────────────────────────────────────────────────────────────┘
                          ↓
         ┌─────────────────────────────────┐
         │              DHT                │
         │  Fragments evaporate after 30s   │
         └─────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────────┐
│                         RECEIVER (Bob)                           │
│                                                                 │
│  1. Collect 4 random fragments from DHT                         │
│  2. Reconstruct via Shamir                                       │
│  3. Decrypt with shared secret                                  │
│  4. Session dissolves, keys zeroized                            │
└─────────────────────────────────────────────────────────────────┘
```

### Cryptographic Stack

| Layer | Algorithm | Standard |
|-------|-----------|----------|
| Key Exchange | ML-KEM-1024 | NIST FIPS 203 |
| Signing | ML-DSA-87 | NIST FIPS 204 |
| Encryption | AES-256-GCM | NIST SP 800-38D |
| Fragmentation | Shamir (4-of-7) | Information-theoretic |
| Hashing | BLAKE3 | |

---

## Security Properties

### Memory Protection
- `#![forbid(unsafe_code)]` - No unsafe Rust
- **ZeroizeOnDrop** on all key material
- **mlock()** support to prevent swap
- Secure memory wrapper for pqcrypto types

### Forward Secrecy
- Unique session keys per message
- Keys destroyed after use

### File Permissions
- Secret keys: `0o600`
- Public keys: `0o644`

---

## Documentation

| Document | Description |
|----------|-------------|
| [PROTOCOL.md](docs/PROTOCOL.md) | Out-of-band key exchange specification |
| [ARCHITECTURE.md](docs/ARCHITECTURE.md) | System design and components |
| [SECURITY.md](docs/SECURITY.md) | Security guarantees and threat model |

---

## Contributing

Contributions welcome. See [CONTRIBUTING.md](CONTRIBUTING.md).

For security issues: **contact@soe-ai.dev** (do not open public issues)

```bash
cargo build --release
cargo test
cargo clippy -- -D warnings
```

---

## Roadmap

- [x] v0.1: Core cryptography + CLI
- [ ] v0.2: Persistent DHT + bootstrap network
- [ ] v0.3: NAT traversal (relay)
- [ ] v0.4: Mobile clients
- [ ] v0.5: Anonymous credentials
- [ ] v1.0: Mainnet

---

## License

MIT - See [LICENSE](LICENSE)

---

## Credits

Created by **Lévy** (@lvs0), 14 years old, France.

*"L'information n'existait pas. Elle a traversé."*

Built with:
- [ml-kem](https://github.com/Argyle-Software/kyber) - Post-quantum KEM
- [libp2p](https://github.com/libp2p/rust-libp2p) - P2P networking
- [BLAKE3](https://github.com/BLAKE3-team/BLAKE3) - Fast hashing
- [zeroize](https://crates.io/crates/zeroize) - Secure memory

---

<div align="center">

**⬡ POLYGONE** — *Information does not exist. It drifts.*

</div>
