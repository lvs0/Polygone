# ⬡ POLYGONE

> *"Information does not exist. It drifts."*

**A prototype implementation of Fisher-Yates topology derivation + Shamir secret sharing + ML-KEM-1024, demonstrating the cryptographic stack for a future distributed privacy network.**

⚠️ **v1.0.0 is a local prototype.** Real P2P networking with the anonymity properties described below is targeted for v2.0.

[![MIT License](https://img.shields.io/badge/License-MIT-purple.svg)](https://github.com/lvs0/Polygone/blob/main/LICENSE)
[![Rust](https://img.shields.io/badge/Rust-API%20%2B%20app-00.svg)](https://www.rust-lang.org/)
[![Memory safety](https://img.shields.io/badge/memory%20safety-ZeroizeOnDrop-2a4.svg)](https://github.com/lvs0/Polygone)
[![v1.0.0](https://img.shields.io/badge/version-1.0.0-8b6dff.svg)](https://github.com/lvs0/Polygone/releases)
[![Binary: 2.1 MB](https://img.shields.io/badge/binary-2.1%20MB-purple.svg)](https://github.com/lvs0/Polygone/releases)
[![Self-test: 5/5 PASS](https://img.shields.io/badge/self--test-5%2F5%20PASS-green.svg)](https://github.com/lvs0/Polygone#how-it-works)

```
$ polygone self-test
⬡ POLYGONE SELF-TEST
  [1/5] ML-KEM-1024 round-trip …… PASS ✔
  [2/5] AES-256-GCM encrypt/decrypt …… PASS ✔
  [3/5] Shamir 4-of-7 (all 35 combinations) …… PASS ✔
  [4/5] Full session round-trip (Alice → Bob) …… PASS ✔
  [5/5] Insufficient fragments → rejected …… PASS ✔
  ✔ All 5 tests passed. POLYGONE is operational.
```


---

## For Developers

```bash
# Install the binary
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash

# Or build from source
git clone https://github.com/lvs0/Polygone
cd Polygone && cargo build --release
./target/release/polygone self-test

# Use as a library
# Add to Cargo.toml:
# polygone = { git = "https://github.com/lvs0/Polygone" }
```

**API:** `polygone send <recipient_pubkey> <message>` — sends an ephemeral message (local mode in v1.0.0).

**Rust library:** Full crypto stack exposed via `lib.rs` for integration into other Rust projects.

> ⚠️ **Status note:** v1.0.0 operates in **local/simulated mode**. The cryptographic primitives are fully implemented and tested (self-test: 5/5 PASS). The distributed P2P network with real nodes is targeted for v2.0. Do not interpret the comparison table below as a claim about the current shipped implementation — it describes the design intent.

---

## The Problem

Classical encryption protects **content** — but an observer can always prove a communication happened.

| Solution | Hides Content | Hides Existence | Status |
|----------|:-------------:|:---------------:|:------:|
| TLS/SSL | ✓ | ✗ | Shipped |
| Signal | ✓ | ✗ | Shipped |
| Tor | Partial | ✗ | Shipped |
| **POLYGONE** | ✓ | **✓** | **v2.0 target** |

POLYGONE doesn't build a tunnel between A and B. It turns a message into a distributed computational state across nodes selected via Fisher-Yates shuffle — then makes it evaporate.

> ⚠️ **Honesty note**: v1.0.0 is a local prototype. "Hides existence" is the **design goal for v2.0** when real P2P networking is implemented.

---

## Quick Start

```bash
# Install (30 seconds)
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash

# Verify installation
polygone self-test

# ── Try it now — local demo ────────────────────────────────
polygone send --peer-pk demo --message "Hello, Polygone!"
# Output: Alice → Bob full round-trip, encrypted + fragmented locally
#
# What just happened:
#  1. Bob generated a fresh keypair
#  2. Alice encapsulated a session secret with Bob's public key
#  3. Both derived the same 7-node topology (deterministic, from shared secret)
#  4. Alice's message was AES-256-GCM encrypted
#  5. Ciphertext split into 7 Shamir fragments (threshold: 4)
#  6. Bob collected fragments and reconstructed the message
#  7. Session dissolved, keying material zeroed
#
#  NOTE: This is a LOCAL demo. Real P2P networking is v2.0.

# Start a node (v1.0: local hold, v2.0: real P2P participation)
polygone node start

# Get help
polygone help
```

---

## How It Works

```
Sender              Local Process          Receiver
  │                      │                       │
  │  1. ML-KEM-1024      │                       │
  │     Key Exchange     │                       │
  │─────────────────────►│                       │
  │                      │                       │
  │  2. AES-256-GCM      │                       │
  │     Payload Encrypt  │                       │
  │─────────────────────►│                       │
  │                      │                       │
  │  3. Shamir 4-of-7    │                       │
  │     Fragment         │                       │
  │─────────────────────►│───────────────────────►│
  │                      │                       │
  │  4. Fisher-Yates     │                       │
  │     7 selected nodes │                       │
  │     (v1.0: local sim, v2.0: P2P)            │
  │                      │                       │
  │  5. Vaporize         │                       │
  │     Fragments DELETE │                       │
  ✗                      ✗                       ✗
```

**v1.0.0**: Local prototype demonstrating the crypto stack. No real network traffic.
**v2.0 target**: Distributed P2P network where no observer can prove a message existed.

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

> ⚠️ These are **design goals** and **cryptographic guarantees at the primitive level**. v1.0.0 implements these primitives locally. The distributed P2P network (v2.0) is required before these properties hold at the network level.

```rust
// POLYGONE cryptographic design (v1.0.0 implemented primitives):

1. POST-QUANTUM (✓ implemented)
   → ML-KEM-1024 key exchange (FIPS 203)
   → Resists quantum computers that break RSA/ECC

2. ZERO METADATA (design goal for v2.0 — NOT yet active)
   → No source IP, no target IP, no timing correlation
   → Message becomes distributed state, not traffic
   
   ⚠️ In v1.0.0: local/simulated mode. No real network traffic exists.

3. INFORMATION-THEORETIC (✓ Shamir implemented)
   → Shamir k-1 fragments = 0 information leaked
   → Even with infinite compute, observer learns nothing

4. FORWARD SECRECY (✓ implemented)
   → Unique keys per session
   → Compromised keys don't threaten future comms

5. EPHEMERAL BY DEFAULT (✓ design intent)
   → 30-second TTL (local), auto-expire
   → Fragments deleted, no forensic trail
   → ⚠️ TTL behavior is local in v1.0.0; network TTL is v2.0

6. MEMORY SAFETY (✓ binary, ⚠️ deps)
   → Binary: `#![forbid(unsafe_code)]` (main.rs)
   → Library: pure Rust APIs; pqcrypto wraps libcrux (has unsafe internals)
   → Session keys: `ZeroizeOnDrop`, no heap leaks
   → ⚠️ ML-KEM/DSA implementations (libcrux via pqcrypto) contain unsafe assembly
```

---

## Architecture (v2.0 target)

> ⚠️ This diagram represents the **target v2.0 architecture**, not v1.0.0. In v1.0.0, all operations occur locally in a single process.

```
┌─────────────────────────────────────────────────────────┐
│                     POLYGONE STACK                       │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌──────────────┐     ┌──────────────┐                  │
│  │   Sender     │────▶│   7 Nodes    │                  │
│  │  ML-KEM-1024  │     │  (Fisher-Yates)                 │
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

| Property | Traditional VPN | Tor | POLYGONE v1.0.0 (prototype) | POLYGONE v2.0 (target) |
|----------|:---------------:|:---:|:-----------------------------:|:---------------------:|
| Hides content | ✓ | ✓ | ✓ (AES-256-GCM) | ✓ |
| Hides source IP | ✓ | ✓ | ✗ (local only) | ✓ |
| Hides destination | ✓ | Partial | ✗ (local only) | ✓ |
| Hides existence | ✗ | ✗ | ✗ (simulated network) | **✓** (design goal) |
| Post-quantum KEM | ✗ | ✗ | ✓ (ML-KEM-1024) | ✓ |
| Forward secrecy | ✓ | ✓ | ✓ (session keys) | ✓ |
| No telemetry | Rare | ✗ | ✓ (no network in v1) | ✓ |
| 100% Rust | Rare | ✗ | ✓ (API + app, ⚠️ libcrux internals) | ✓ |

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

# Windows → use WSL or build from source (see below)
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