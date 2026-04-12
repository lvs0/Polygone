# ⬡ POLYGONE

[![Post-Quantum](https://img.shields.io/badge/Post--Quantum-ML--KEM--1024-blue)](https://csrc.nist.gov/pubs/ir/8439/ipd)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org)
[![Security: ZeroizeOnDrop](https://img.shields.io/badge/Security-ZeroizeOnDrop-brightgreen)](https://crates.io/crates/zeroize)
[![Privacy](https://img.shields.io/badge/Privacy-Ephemeral%20Metadata-red)](https://github.com/lvs0/Polygone)

> *"Information does not exist. It drifts."*  
> *"L'information n'existe pas. Elle traverse."*

**POLYGONE** is an **ephemeral post-quantum privacy network** that solves the **Metadata Problem**. Built with Rust.

---

## The Problem Traditional Encryption Can't Solve

```
Traditional VPN/Encryption:
┌─────────────────────────────────────────────────────────┐
│  Alice ──────── [encrypted tunnel] ──────── Bob        │
│   IP │                                        │ IP     │
│   │  │              METADATA LEAK             │  │     │
│   └──┴───────────────────────────────────────┴──┘     │
│   Source/Destination/Timing always visible             │
└─────────────────────────────────────────────────────────┘

POLYGONE:
┌─────────────────────────────────────────────────────────┐
│                                                         │
│         ┌───┐   ┌───┐   ┌───┐   ┌───┐   ┌───┐        │
│         │ N1│   │ N2│   │ N3│   │ N4│   │ N5│        │
│         └───┘   └───┘   └───┘   └───┘   └───┘        │
│              ↘       ↑       ↓       ↑       ↙         │
│                   (DHT Wave)                            │
│                                                         │
│   Alice sends fragments → Distributed → Bob reconstructs │
│   No tunnel. No connection. No metadata.                 │
└─────────────────────────────────────────────────────────┘
```

**Polygone turns messages into mathematical waves that evaporate.**

---

## Quick Start

### One-Line Install

```bash
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash
```

### Or From Source

```bash
git clone https://github.com/lvs0/Polygone
cd Polygone
cargo build --release
cargo install --path .
```

### Verify Installation

```bash
polygone selftest
```

---

## Usage

### Generate Keys

```bash
polygone keygen --output ~/.polygone/keys
```

### Send a Message (Demo Mode)

```bash
polygone send --peer-pk demo --message "Hello, anonymous world."
```

### Start a Relay Node

```bash
polygone node start --listen /ip4/0.0.0.0/tcp/4001
```

### Full Command List

| Command | Description |
|---------|-------------|
| `polygone keygen` | Generate ML-KEM-1024 + ML-DSA-87 keypair |
| `polygone send <pk> <msg>` | Send encrypted message |
| `polygone receive <sk> <ct>` | Receive and decrypt |
| `polygone node start` | Start relay node |
| `polygone node info` | Node information |
| `polygone status` | Network status |
| `polygone selftest` | Run self-tests |
| `polygone update` | Update to latest |
| `polygone uninstall` | Remove Polygone |

---

## Architecture

### How Messages Become Waves

```
┌─────────────────────────────────────────────────────────────────┐
│                         SENDER (Alice)                           │
│                                                                 │
│  1. Generate shared secret via ML-KEM-1024                     │
│     ┌─────────────────────────────────────────────────────┐     │
│     │  Alice + Bob's PK → Ciphertext + Shared Secret     │     │
│     └─────────────────────────────────────────────────────┘     │
│                                                                 │
│  2. Derive topology from shared secret                         │
│     ┌─────────────────────────────────────────────────────┐     │
│     │  BLAKE3(seed) → 7 random node IDs                 │     │
│     │  AES-256-GCM key derived per session               │     │
│     └─────────────────────────────────────────────────────┘     │
│                                                                 │
│  3. Encrypt + Fragment via Shamir                               │
│     ┌─────────────────────────────────────────────────────┐     │
│     │  Message → AES-256-GCM → 7 fragments (4-of-7)     │     │
│     │  Each fragment is useless without 4 others          │     │
│     └─────────────────────────────────────────────────────┘     │
│                                                                 │
│  4. Push to DHT (ephemeral, 30s TTL)                           │
│                         ↓                                       │
└─────────────────────────────────────────────────────────────────┘
                          ↓
        ┌─────────────────────────────────────────┐
        │              DHT WAVE                    │
        │  ┌───┐ ┌───┐ ┌───┐ ┌───┐ ┌───┐       │
        │  │N1 │ │N2 │ │N3 │ │N4 │ │N5 │ ...    │
        │  └───┘ └───┘ └───┘ └───┘ └───┘       │
        │  Fragments stored temporarily           │
        │  TTL = 30 seconds                       │
        └─────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────────────┐
│                         RECEIVER (Bob)                           │
│                                                                 │
│  1. Query DHT for 4 random fragments                           │
│  2. Reconstruct message from fragments                          │
│  3. Decrypt with shared secret                                  │
│  4. Session dissolves → keys zeroized                          │
│                                                                 │
│  THE MESSAGE NEVER EXISTED AS A UNIT.                          │
└─────────────────────────────────────────────────────────────────┘
```

### Cryptographic Stack

| Layer | Algorithm | Purpose |
|-------|-----------|---------|
| Key Exchange | ML-KEM-1024 (Kyber) | Post-quantum KEX |
| Signing | ML-DSA-87 (Dilithium) | Authentication |
| Encryption | AES-256-GCM | Symmetric encryption |
| Fragmentation | Shamir Secret Sharing (4-of-7) | Distributed storage |
| Hashing | BLAKE3 | Topology derivation |

---

## Security Properties

### Post-Quantum Security
- **ML-KEM-1024**: Resistant to quantum attacks (NIST Level 5)
- **ML-DSA-87**: Quantum-resistant signatures

### Information-Theoretic Privacy
- **Shamir Secret Sharing**: Even with 6/7 fragments = 0 information
- No subset of fragments can reveal anything

### Memory Safety
- `#![forbid(unsafe_code)]` - No unsafe Rust
- **ZeroizeOnDrop**: Keys zeroed from memory on drop
- No secrets in logs, stack traces, or core dumps

### Forward Secrecy
- Unique session keys per message
- Keys destroyed after use
- No long-term key reuse

### Path Traversal Protection
- All file paths canonicalized
- Tilde (`~`) expansion validated
- 0o600 permissions on key files

---

## Benchmarks

```
ML-KEM-1024 Encapsulation:    ~34 µs
AES-256-GCM Encrypt (1KB):     ~3.8 µs
Shamir Split (7 fragments):   ~12 µs
Full E2E Send (1KB message):   ~208 µs
```

*Measured on AMD Ryzen 9 5950X*

---

## Ecosystem

```
┌─────────────────────────────────────────────────────────────────┐
│                      POLYGONE ECOSYSTEM                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐            │
│  │  Polygone   │  │  Polygone   │  │  Polygone   │            │
│  │   Core      │  │   Drive     │  │   Hide      │            │
│  │  (Network)  │  │  (Storage)  │  │  (Routing)  │            │
│  └─────────────┘  └─────────────┘  └─────────────┘            │
│                                                                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐            │
│  │  Polygone   │  │  Polygone   │  │  Polygone   │            │
│  │   Shell     │  │   Brain     │  │   Server    │            │
│  │  (Terminal) │  │    (LLM)    │  │  (Hosting)  │            │
│  └─────────────┘  └─────────────┘  └─────────────┘            │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Modules

| Module | Description | Status |
|--------|-------------|--------|
| **Core** | Post-quantum network layer | ✅ Stable |
| **Drive** | Encrypted distributed storage | 🚧 WIP |
| **Hide** | Anonymous routing | 🚧 WIP |
| **Shell** | Terminal interface | 🚧 WIP |
| **Brain** | LLM integration (Granite) | ✅ Ready |
| **Server** | Self-hosting solutions | 🚧 WIP |

---

## Installation Options

### Full Installation

```bash
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash -s -- --all
```

### Selective Installation

```bash
# Core + LLM
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash -s -- --core --brain

# Core + Storage
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash -s -- --core --drive
```

### Modules Available

| Flag | Module | Description |
|------|--------|-------------|
| `--core` | Polygone Core | Required, network layer |
| `--drive` | Polygone-Drive | Distributed storage |
| `--hide` | Polygone-Hide | Anonymous routing |
| `--petals` | Petals | LLM inference |
| `--shell` | Polygone-Shell | Terminal UI |
| `--brain` | Polygone-Brain | AI assistant |
| `--server` | Polygone-Server | Self-hosting |

---

## Contributing

Contributions welcome. Privacy is an **architectural property**, not a setting.

```bash
# Fork, then:
git clone https://github.com/YOUR_USER/Polygone
cd Polygone
cargo build --release
cargo test
```

### Security

For security issues, please email `contact@soe-ai.dev` directly. Do not open public issues for vulnerabilities.

---

## Roadmap

- [ ] v0.2: Production DHT dispatch
- [ ] v0.3: NAT traversal (libp2p relay)
- [ ] v0.4: Mobile clients (iOS/Android)
- [ ] v0.5: Anonymous credentials
- [ ] v1.0: Mainnet launch

---

## License

MIT License - See [LICENSE](LICENSE)

---

## Credits

Created by **Lévy** (@lvs0)  
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
