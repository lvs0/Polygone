# ⬡ POLYGONE

> *"Information does not exist. It drifts."*

**A post-quantum ephemeral privacy network where messages become distributed mathematical waves — then evaporate.**

[![MIT License](https://img.shields.io/badge/License-MIT-purple.svg)](https://github.com/lvs0/Polygone/blob/main/LICENSE)
[![Rust](https://img.shields.io/badge/Rust-100%25-b71200.svg)](https://www.rust-lang.org/)
[![No unsafe code](https://img.shields.io/badge/unsafe-forbidden-b00000.svg)](https://github.com/lvs0/Polygone)
[![v1.0.0](https://img.shields.io/badge/version-1.0.0-8b6dff.svg)](https://github.com/lvs0/Polygone/releases)

---

## The Problem

Classical encryption protects **content** — but an observer can always prove a communication happened.

| Solution | Hides Content | Hides Existence |
|----------|:-------------:|:---------------:|
| TLS/SSL | ✓ | ✗ |
| Signal | ✓ | ✗ |
| Tor | Partial | ✗ |
| **POLYGONE** | ✓ | **✓** |

POLYGONE doesn't build a tunnel between A and B. It turns a message into a distributed computational state across a global DHT — then makes it evaporate.

---

## Quick Start

```bash
# Install (30 seconds)
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash

# Verify installation
polygone self-test

# Generate keys
polygone keygen

# Start a node
polygone node start

# Get help
polygone help
```

---

## How It Works

```
Sender                Network                 Receiver
  │                      │                       │
  │  1. ML-KEM-1024      │                       │
  │     Key Exchange     │                       │
  │─────────────────────►│                       │
  │                      │                       │
  │  2. AES-256-GCM      │                       │
  │     Payload Encrypt  │                       │
  │─────────────────────►│                       │
  │                      │                       │
  │  3. Shamir 4-of-7     │                       │
  │     Fragment          │                       │
  │─────────────────────►│───────────────────────►│
  │                      │                       │
  │  4. BLAKE3 DHT Route  │                       │
  │     7 random nodes    │                       │
  │     30s TTL          │                       │
  │                      │                       │
  │  5. Vaporize          │                       │
  │     Fragments DELETE │                       │
  ✗                      ✗                       ✗
```

**No tunnel. No observer can prove a message existed.**

---

## Cryptographic Stack

| Layer | Algorithm | Standard |
|-------|-----------|----------|
| Key Exchange | ML-KEM-1024 | NIST FIPS 203 |
| Signatures | ML-DSA-87 | NIST FIPS 204 |
| Encryption | AES-256-GCM | 96-bit nonce |
| Fragmentation | Shamir SSS 4-of-7 | Information-theoretic |
| Hashing | BLAKE3 | Domain-separated |

**Post-quantum**: Resists Shor's algorithm and quantum cryptanalysis.

**Information-theoretic**: k-1 Shamir fragments reveal zero information.

**Memory-safe**: `#![forbid(unsafe_code)]` + `zeroize` on drop.

---

## Performance

| Operation | Latency |
|-----------|---------|
| ML-KEM Encapsulation | **34 µs** |
| AES-256-GCM Encrypt | **3.8 µs** |
| Full E2E Send | **208 µs** |
| Self-test | 100ms |

Measured on AMD Ryzen 5 5600X. Run `polygone self-test` to verify on your hardware.

---

## Security Properties

```rust
// POLYGONE is built around these guarantees:

1. POST-QUANTUM
   → ML-KEM-1024 key exchange
   → Resists quantum computers that break RSA/ECC

2. ZERO METADATA
   → No source IP, no target IP, no timing correlation
   → Message becomes distributed state, not traffic

3. INFORMATION-THEORETIC
   → Shamir k-1 fragments = 0 information leaked
   → Even with infinite compute, observer learns nothing

4. FORWARD SECRECY
   → Unique keys per session
   → Compromised keys don't threaten future comms

5. EPHEMERAL BY DEFAULT
   → 30-second TTL, auto-expire
   → Fragments deleted, no forensic trail

6. MEMORY SAFETY
   → Zero unsafe code (#![forbid(unsafe_code)])
   → ZeroizeOnDrop, no heap leaks
```

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     POLYGONE STACK                       │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌──────────────┐     ┌──────────────┐                  │
│  │   Sender     │────▶│   7 Nodes    │                  │
│  │  ML-KEM-1024  │     │  (random)     │                  │
│  │  AES-256-GCM │     │  BLAKE3 DHT   │                  │
│  │  Shamir 4-7  │     │  30s TTL      │                  │
│  └──────────────┘     └──────┬───────┘                  │
│                              │                          │
│                              ▼                          │
│  ┌──────────────┐     ┌──────────────┐                  │
│  │  Receiver    │◀────│   Reconstruct│                  │
│  │  Decrypt     │     │   4-of-7      │                  │
│  │  Verify      │     │   Decrypt     │                  │
│  └──────────────┘     └──────────────┘                  │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

---

## Comparison

| Property | Traditional VPN | Tor | POLYGONE |
|----------|:---------------:|:---:|:--------:|
| Hides content | ✓ | ✓ | ✓ |
| Hides source IP | ✓ | ✓ | ✓ |
| Hides destination | ✓ | Partial | ✓ |
| Hides existence | ✗ | ✗ | **✓** |
| Post-quantum | ✗ | ✗ | **✓** |
| Forward secrecy | ✓ | ✓ | ✓ |
| No telemetry | Rare | ✗ | **✓** |
| 100% Rust | Rare | ✗ | **✓** |

---

## Why Polygone?

*"We don't need a faster horse. We need a different vehicle."*

- **Adversarial AI**: Traffic analysis + quantum computing will break current encryption in 5-10 years
- **Metadata leaks kill**: Knowing WHO talked to WHO is often more valuable than WHAT they said
- **Privacy as architecture**: Not a setting — a fundamental property of the protocol

---

## Install

```bash
# Linux / macOS
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash

# Windows (PowerShell)
irm https://polygone.network/install.ps1 | iex
```

Or build from source:

```bash
git clone https://github.com/lvs0/Polygone.git
cd Polygone
cargo build --release
./target/release/polygone help
```

---

## Commands

| Command | Description |
|---------|-------------|
| `polygone self-test` | Run cryptographic test suite |
| `polygone keygen` | Generate ML-KEM-1024 + Ed25519 keys |
| `polygone send <msg>` | Encrypt and send a message |
| `polygone node start` | Start relay node |
| `polygone node stop` | Stop relay node |
| `polygone status` | Show node and network status |
| `polygone update` | Update to latest version |
| `polygone uninstall` | Remove Polygone completely |

---

## Status

- ✅ ML-KEM-1024 key exchange
- ✅ AES-256-GCM encryption
- ✅ Shamir Secret Sharing (4-of-7)
- ✅ Full session round-trip (tested)
- 🚧 NAT traversal (v0.3)
- 🚧 DHT spam protection (planned)
- 🚧 Configurable quorum (planned)

---

## Contributing

Issues and PRs welcome. Privacy is an **architectural property**, not a setting.

**Security disclosures**: Please report vulnerabilities to [polygone@proton.me](mailto:polygone@proton.me) with full details.

---

## License

MIT License — No investors. No token. No telemetry.

Built with ❤️ by [Lévy Verpoort Scherpereel](https://github.com/lvs0), 14 years old, France.

*"The best encryption is the one that looks like nothing happened at all."*