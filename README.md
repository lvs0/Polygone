# ⬡ POLYGONE

> *"Information does not exist. It drifts."* / *"L'information n'existe pas. Elle traverse."*

**POLYGONE** is a post-quantum ephemeral privacy network designed to solve the **Metadata Problem**. Built in pure Rust with a beautiful TUI interface.

---

## English | [Français](#français)

### The Problem: The Metadata Leak

Traditional encryption protects **content**, but it cannot hide that a **communication occurred**. Source IPs, target IPs, timing, and packet sizes remain visible to observers.

**POLYGONE changes the paradigm.** Instead of an encrypted tunnel between A and B, POLYGONE turns a message into a distributed, transient mathematical state—a wave that crosses a global DHT and then vaporizes.

---

## 🚀 Quick Start (30 seconds)

### One-Line Installation

```bash
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash
```

That's it! Then run:
```bash
polygone tui    # Launch the beautiful terminal interface
```

### Pre-built Binaries

Download ready-to-use binaries for your platform from [Releases](https://github.com/lvs0/Polygone/releases):

| Platform | Download |
|----------|----------|
| 🐧 Linux (x86_64) | `polygone-linux-x86_64.tar.gz` |
| 🍎 macOS (Intel) | `polygone-macos-x86_64.tar.gz` |
| 🍎 macOS (Apple Silicon) | `polygone-macos-arm64.tar.gz` |
| 🪟 Windows (x86_64) | `polygone-windows-x86_64.zip` |

---

## 📱 Commands

| Command | Description |
|---------|-------------|
| `polygone tui` | 🎨 Launch the interactive TUI dashboard |
| `polygone help` | Show all commands |
| `polygone self-test` | ✅ Verify installation |
| `polygone keygen` | 🔑 Generate post-quantum keys |
| `polygone send` | 📤 Send an encrypted message |
| `polygone receive` | 📥 Receive a message |
| `polygone node start` | 🌐 Start a relay node |
| `polygone status` | 📊 View network status |

---

## 🎯 How It Works

1. **Post-Quantum Handshake**: ML-KEM-1024 key exchange
2. **Deterministic Topology**: BLAKE3 derives 7 random nodes
3. **Shamir Dispersion**: AES-256-GCM + 4-of-7 secret sharing
4. **Vaporization**: 30s TTL, data auto-evaporates

### 🔒 Security Stack

- **Post-Quantum KEM**: ML-KEM-1024 (FIPS 203)
- **Signatures**: Ed25519 (ML-DSA ready)
- **Symmetric**: AES-256-GCM
- **KDF**: BLAKE3 (domain-separated)
- **Secret Sharing**: Shamir 4-of-7
- **Memory Safety**: `#![forbid(unsafe_code)]` + ZeroizeOnDrop
- **Forward Secrecy**: Unique keys per session

### ⚡ Performance

| Operation | Latency |
|-----------|---------|
| ML-KEM Encapsulation | ~34 µs |
| AES-256-GCM Encrypt | ~3.8 µs |
| Full Send (E2E) | ~208 µs |

---

## 💻 Platform Support

| OS | Status | Notes |
|----|--------|-------|
| Fedora | ✅ Native | `dnf install rust cargo` |
| Ubuntu/Debian | ✅ Native | `apt install rustc cargo` |
| Arch Linux | ✅ Native | `pacman -S rust cargo` |
| macOS | ✅ Native | Intel + Apple Silicon |
| Windows | ✅ WSL2 / Native | PowerShell or WSL |

See detailed instructions in [INSTALL.md](INSTALL.md)

---

## 🛠️ Build from Source

### Prerequisites

- Rust >= 1.75 ([rustup.rs](https://rustup.rs))
- OpenSSL development files
- pkg-config
- Git

### Build Steps

```bash
git clone https://github.com/lvs0/Polygone.git
cd Polygone
cargo build --release
sudo cp target/release/polygone /usr/local/bin/
```

---

## 📸 TUI Features

The built-in terminal interface provides:

- 📊 **Dashboard**: Real-time network status
- 🔑 **Key Management**: Easy key generation and preview
- 📤 **Send/Receive**: Guided message workflows
- 🌐 **Node Control**: Start/stop relay nodes
- 🧪 **Self-Tests**: Verify crypto primitives
- 📖 **Help**: Built-in documentation

Navigate with number keys (1-6), quit with `q`.

---

## ⚠️ Known Limitations

- NAT traversal in progress (v2.0)
- DHT spam protection planned
- Static quorum (4-of-7)
- P2P network dispatch arrives in v2.0 (libp2p + Kademlia DHT)

---

## 🤝 Contributing

Issues and PRs welcome! Privacy is an architectural property, not a setting. ⬡

### Development

```bash
# Run tests
cargo test

# Run linter
cargo clippy --all-features -- -D warnings

# Format code
cargo fmt
```

---

## 📄 License

MIT License — No investors. No token. No telemetry.

---

## Français

### Le Problème

Le chiffrement traditionnel ne cache pas qu'une communication a eu lieu.

**POLYGONE change le paradigme.** Un message devient un état mathématique distribué transient qui s'évapore.

### Installation Rapide

```bash
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash
polygone tui    # Interface graphique terminal
```

### Sécurité

- **Post-Quantique**: ML-KEM-1024 + ML-DSA-87
- **Information-Théorique**: Shamir Secret Sharing
- **Mémoire**: `#![forbid(unsafe_code)]` + ZeroizeOnDrop

### Documentation Complète

- [Guide d'installation](INSTALL.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Protocole](docs/PROTOCOL.md)
- [Sécurité](docs/SECURITY.md)

---

*by Lévy, 14 ans, France*

> ⬡ *"L'information n'existe pas. Elle traverse."*
