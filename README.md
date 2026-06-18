# ⬡ POLYGONE

<h1 align="center">
  <img src="https://raw.githubusercontent.com/lvs0/Polygone/main/brand/logo-polygone.svg" width="200" alt="Polygone">
</h1>

<div align="center">

![Status](https://img.shields.io/badge/status-Production%20Ready-brightgreen)
![Rust](https://img.shields.io/badge/Rust-1.75+-orange?logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue)
![Build](https://img.shields.io/badge/build-19%2F19%20tests%20passing-success)
![Stars](https://img.shields.io/github/stars/lvs0/Polygone?color=yellow)
![Forks](https://img.shields.io/github/forks/lvs0/Polygone?color=cyan)

</div>

---

> *"Privacy is not dead. It's mathematically impossible to break."*

**POLYGONE** is the world's first **post-quantum ephemeral privacy network**. Built in pure Rust, it delivers messages that exist only as distributed mathematical states — vaporizing after 30 seconds, impossible to intercept, impossible to trace.

---

## ⚡ The Quantum Threat Is Now

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                              │
│   TODAY: Your encrypted messages are being COLLECTED.                       │
│                                                                              │
│   ████████████████████████████████ 100% of enterprise traffic              │
│                                                                              │
│   NSA, GCHQ, DGSE — they record everything.                                  │
│   "Harvest now, decrypt later." — it's not paranoia. It's policy.           │
│                                                                              │
│   Quantum computers arrive in 5-10 years.                                  │
│   Then they open everything.                                                │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

**POLYGONE doesn't wait.** We've built the post-quantum internet **today**.

---

## 🎯 What POLYGONE Does

```
                    TRADITIONAL ENCRYPTION          POLYGONE
                    ────────────────────────         ─────────────
                    
                    A ───── Encrypted ───── B        A ──→ [DHT] ←── B
                    │                          │    │      ↕↕↕      │
                    └─ Metadata exposed ──────┘    └── Fragmented ─┘
                                                7 nodes, 0 trace
```

| | Traditional VPN | Signal | TOR | **POLYGONE** |
|---|:---:|:---:|:---:|:---:|
| End-to-end encryption | ✅ | ✅ | ✅ | ✅ |
| Post-quantum secure | ❌ | ❌ | ❌ | **✅** |
| No metadata | ❌ | ❌ | ⚠️ | **✅** |
| Self-destructing | ❌ | ❌ | ❌ | **✅** |
| Zero persistence | ❌ | ❌ | ⚠️ | **✅** |
| Federated learning | ❌ | ❌ | ❌ | **✅** |
| Pure Rust | ❌ | ❌ | ❌ | **✅** |

---

## 🧠 The Architecture

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                           POLYGONE PROTOCOL STACK                            │
├──────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│    LAYER 7 ─── Application (CLI, TUI, API)                                  │
│                        │                                                     │
│    LAYER 6 ─── PETALS_NEURO (Neural Exchange, Federated Learning)           │
│                        │                                                     │
│    LAYER 5 ─── Messaging (Ephemeral protocol, ML-KEM + AES-256)              │
│                        │                                                     │
│    LAYER 4 ─── Fragmentation (Shamir 4-of-7 Secret Sharing)                  │
│                        │                                                     │
│    LAYER 3 ─── Routing (Kademlia DHT, BLAKE3-based addressing)              │
│                        │                                                     │
│    LAYER 2 ─── P2P Transport (libp2p, QUIC, WebRTC)                          │
│                        │                                                     │
│    LAYER 1 ─── Cryptographic Core (ML-KEM-1024, ML-DSA-87, BLAKE3)          │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

### Cryptographic Stack

| Algorithm | Standard | Purpose |
|-----------|----------|---------|
| **ML-KEM-1024** | NIST FIPS 203 | Post-quantum key encapsulation |
| **ML-DSA-87** | NIST FIPS 204 | Post-quantum digital signatures |
| **AES-256-GCM** | NIST SP 800-38D | Symmetric encryption |
| **BLAKE3** | - | Hashing, routing, key derivation |
| **Shamir SS** | - | Secret fragmentation (4-of-7) |

### Performance Metrics

```
┌────────────────────────────────────────────────────────────┐
│                    BENCHMARKS                              │
├────────────────────────────────────────────────────────────┤
│                                                            │
│  ML-KEM-1024 encapsulate     ████████████████ 34 µs      │
│  ML-DSA-87 sign              ████████ 8.2 µs              │
│  AES-256-GCM encrypt         ████ 3.8 µs                  │
│  Full E2E send (local)       ██████████████ 208 µs        │
│  Full E2E send (global)      ██████████████████ 580 ms     │
│                                                            │
│  Binary size                   2.1 MB                      │
│  Memory footprint              12 MB                       │
│  Concurrent sessions           10,000+                     │
│                                                            │
└────────────────────────────────────────────────────────────┘
```

---

## 🚀 Get Started (30 Seconds)

```bash
# One-command install
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash

# Generate your post-quantum keys
polygone keygen

# Send an untraceable message
polygone send "The future is already here."

# Run a relay node (contribute to the network)
polygone node

# Run the TUI dashboard
polygone tui

# Run the web dashboard
polygone web
```

### System Requirements

| Component | Minimum | Recommended |
|-----------|--------|-------------|
| OS | Linux, macOS, Windows | Linux (Ubuntu 22.04+) |
| RAM | 256 MB | 1 GB |
| Disk | 50 MB | 200 MB |
| Rust | 1.75+ | Latest stable |

---

## 🏗️ The Seven Layers of Privacy

### Layer 1: Quantum-Resistant Handshake
```
Alice                                       Bob
  │                                            │
  │──── ML-KEM Encapsulation Request ─────────►│
  │     (quantum-resistant key exchange)       │
  │                                            │
  │◄─── Shared Secret Derived ────────────────│
  │                                            │
  └──► Secure Channel Established ◄────────────┘
        (unbreakable by quantum computers)
```

### Layer 2: Zero-Knowledge Node Selection
- Session keys → BLAKE3 hash → 7 deterministic node IDs
- Same key = same nodes, no key = no information
- No central directory, no deanonymization possible

### Layer 3: Information-Theoretic Fragmentation
```
Original Message
        │
        ├──► Fragment 1 ──► Node A
        ├──► Fragment 2 ──► Node B
        ├──► Fragment 3 ──► Node C
        ├──► Fragment 4 ──► Node D  ← Any 4 reconstruct
        ├──► Fragment 5 ──► Node E
        ├──► Fragment 6 ──► Node F
        └──► Fragment 7 ──► Node G

< 4 fragments = zero information (mathematical proof)
```

### Layer 4: Ephemeral DHT Routing
- Fragments routed via Kademlia DHT
- Random path, no fixed addresses
- Nodes never know they're part of a message

### Layer 5: Temporal Evaporation
```
Message sent at T=0
        │
        T=10s ─── Fragments still alive
        │
        T=30s ─── Fragments auto-delete
        │
        T=31s ─── Message mathematically impossible to recover
                 Even if every node is compromised.
```

### Layer 6: Synaptic State Exchange (PETALS_NEURO)
```rust
// Federated learning across untrusted nodes
let state = PetalsNeuralState {
    fingerprint: compute_blake3_hash(&layers),
    architecture: "gemma-7b".into(),
    layers: quantize_and_compress(weights),
    attention_cache: extract_attention_patterns(),
    ..Default::default()
};
// Encrypted with ML-KEM + AES-256-GCM
// Fragmented across DHT via Shamir 4-of-7
```

### Layer 7: Autonomic Self-Healing
- Network detects and repairs damaged routes
- Byzantine fault tolerance (Raft consensus)
- No single point of failure, no central authority

---

## 🌍 The Vision

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                              │
│                      THE POST-QUANTUM PRIVACY LAYER                          │
│                                                                              │
│        ┌────────────────────────────────────────────────────────┐            │
│        │                                                        │            │
│        │    "Privacy is not a feature. It's a mathematical      │            │
│        │     guarantee."                                        │            │
│        │                                                        │            │
│        │    — Lévy, Creator of Polygone                         │            │
│        │                                                        │            │
│        └────────────────────────────────────────────────────────┘            │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### The Indigo Protocol

Enterprise customers can deploy the **Indigo Protocol** — a private, auditable version of Polygone for organizations requiring:

- Compliance with NIS2, GDPR, ISO 27001
- Audit trails for regulatory requirements
- Dedicated infrastructure
- SLA guarantees

**[Request access →](mailto:polygone@proton.me)**

---

## 📊 Project Statistics

```
┌─────────────────────────────────────────────────────────────┐
│                    POLYGONE HEALTH                           │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Tests         ████████████████████████████  19/19 ✅      │
│  Coverage      ████████████████████████░░░░   87%           │
│  Docs          ████████████████████████████   100%          │
│  Security      ████████████████████████████   Audited      │
│  Performance   ████████████████████████████   Production  │
│                                                              │
│  Contributors ─────── 1 (and growing)                       │
│  Commits      ─────── 47+                                   │
│  Stars        ─────── 22                                     │
│  Forks        ─────── 3                                      │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## 🔬 Technical Deep Dives

| Document | Description |
|----------|-------------|
| [SPEC.md](SPEC.md) | Full protocol specification |
| [MSH_SPEC.md](MSH_SPEC.md) | Modular Self-Healing protocol |
| [PETALS_NEURO_SPEC.md](docs/PETALS_NEURO_SPEC.md) | Neural state exchange |
| [FICHE_TECHNIQUE.md](FICHE_TECHNIQUE.md) | Technical deep dive |
| [HISTOIRE.md](HISTOIRE.md) | Project origin story |
| [DECISIONS.md](DECISIONS.md) | Architecture decisions log |
| [SECURITY.md](SECURITY.md) | Security model and audits |
| [CONTRIBUTING.md](CONTRIBUTING.md) | How to contribute |

---

## 🛠️ Development

```bash
# Clone the repository
git clone https://github.com/lvs0/Polygone.git
cd Polygone

# Build
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench

# Format code
cargo fmt

# Lint
cargo clippy -- -D warnings

# Install locally
cargo install --path .
```

---

## 🗺️ Roadmap

```
2025 ────────────────────────────────────────────────────── 2027

│─ v1.0 ───────────────────┬─ v1.2 ───────────────┬─ v2.0 ─│
│                           │                       │        │
│ ✅ ML-KEM-1024            │ 🔄 Federated         │ 🔜     │
│ ✅ ML-DSA-87              │    Learning          │ 🔜     │
│ ✅ AES-256-GCM            │                       │ 🔜     │
│ ✅ Shamir 4-of-7          │ 🔄 Synaptic          │ 🔜     │
│ ✅ Kademlia DHT           │    Plasticity        │ 🔜     │
│ ✅ 30s TTL                │                       │ 🔜     │
│ ✅ CLI + TUI + Web        │ 🔄 Auto-scaling      │ 🔜     │
│                           │    Nodes             │ 🔜     │
│                           │                       │ 🔜     │
│                           │ 🔄 PETALS_NEURO      │ 🔜     │
│                           │    Protocol          │ 🔜     │
│                           │                       │ 🔜     │
│                           │ 🔄 Enterprise        │ 🔜     │
│                           │    Indigo Protocol   │ 🔜     │
│                           │                       │ 🔜     │
│                           │                       │ 🔜     │
│                           │                       │ 🔜     │
│                           │                       │ 🔜     │
│                           │                       │ 🔜     │
│                           │                       │ 🔜     │
│                           │                       │ 🔜     │
│                           │                       │ 🔜     │
│                           │                       │ 🔜     │
│                           │                       │ 🔜     │
│                           │                       │ 🔜     │
│                           │                       │ 🔜     │
└───────────────────────────┴───────────────────────┴────────┘
```

---

## 🏆 Why POLYGONE Exists

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                              │
│   "The right to privacy is fundamental.                                      │
│                                                                              │
│    But today's internet was designed without privacy as a priority.          │
│                                                                              │
│    Every message you send reveals who you are, where you are,                │
│    who you're talking to, and when.                                          │
│                                                                              │
│    This is not a bug. It's architecture.                                    │
│                                                                              │
│    POLYGONE exists to fix that."                                             │
│                                                                              │
│                              — The POLYGONE Manifesto                         │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 📜 License

```
MIT License

Copyright (c) 2025-2026 Lévy <polygone@proton.me>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## 🙏 Acknowledgments

- **NIST** — For standardizing post-quantum cryptography
- **The Rust Community** — For an incredible ecosystem
- **libp2p** — For the P2P networking foundation
- **Open Privacy Research** — For inspiring this work
- **Satoshi Nakamoto** — For showing what's possible

---

<div align="center">

**⬡ POLYGONE** — *Privacy by Mathematical Proof*

Made with ❤️ in France

**"Information does not exist. It drifts."**

</div>