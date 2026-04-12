# ⬡ POLYGONE — by Hope

> *"L'information n'existe pas. Elle traverse."*

**POLYGONE** is a French-built, post-quantum ephemeral privacy network. Built in pure Rust by the **Hope** collective.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-nightly-orange.svg)]()
[![Status: v0.2-alpha](https://img.shields.io/badge/status-v0.2_alpha-yellow.svg)]()
[![unsafe: forbidden](https://img.shields.io/badge/unsafe-forbidden-red.svg)]()

---

## The Problem

Classical encryption hides **content**. It cannot hide that a **communication occurred**.
Source IPs, timing, and packet sizes remain visible. For a global adversary, metadata is more dangerous than content.

**POLYGONE turns a message into a transient mathematical state — a wave that crosses a global DHT and vaporizes.**
To an outside observer: no message. Only ambient noise across 7 ephemeral nodes.

---

## How it Works

### 1. Post-Quantum Handshake
**ML-KEM-1024** (FIPS 203) key exchange. The key doesn't encrypt the payload — it defines the **network topology** for the transit.

### 2. Deterministic Topology
Both peers use **BLAKE3** to derive the same graph of 7 virtual nodes. No third party can predict which DHT keys will be targeted.

### 3. Shamir Dispersion
Payload encrypted with **AES-256-GCM**, split via **Shamir's Secret Sharing (t=4, n=7)**. Fragments go into the Kademlia DHT via `libp2p`. No relay holds more than one fragment.

### 4. Atmospheric Vaporization
**30s TTL.** Fragments evaporate from relay RAM automatically. `ZeroizeOnDrop` leaves no trace in Alice or Bob's memory.

---

## ⚡ Quickstart

```bash
# 1. Clone
git clone https://github.com/lvs0/Polygone && cd Polygone

# 2. Install
chmod +x install.sh && ./install.sh

# 3. Start
polygone start
```

After install, run **`polygone help`** at any time for the full command reference.

### `polygone start`
Launches the **Polygone-Shell** — an interactive TUI dashboard showing your PeerId, active sessions, relay logs, and DHT status in real time.

### Command Reference

| Command | Description |
|---|---|
| `polygone help` | Full usage reference |
| `polygone start` | Launch the interactive shell (TUI) |
| `polygone keygen` | Generate your ML-KEM + ML-DSA keypair |
| `polygone send --peer-pk demo` | Local Alice→Bob demo (no network required) |
| `polygone send --peer-pk <key.pk> -m "..."` | Send an ephemeral message through the network |
| `polygone node start` | Start a relay node and contribute bandwidth |
| `polygone self-test` | Run the full cryptographic self-test suite |
| `polygone status` | Show node health and active sessions |

---

## Benchmarks

| Primitive | Operation | Latency |
|---|---|---|
| **ML-KEM-1024** | Encapsulation | ~34.1 µs |
| **ML-KEM-1024** | Decapsulation | ~35.3 µs |
| **BLAKE3** | Topology Derivation | ~0.23 µs |
| **AES-256-GCM** | Encryption (1KB) | ~3.80 µs |
| **Shamir (4/7)** | Split | ~4.21 µs |
| **Full Lifecycle** | Alice Send (E2E) | **~207.6 µs** |

---

## Security Model

- **Post-Quantum**: ML-KEM-1024 + ML-DSA-87. Resistant to Shor's algorithm.
- **Forward Secrecy**: Each session uses a unique key and network topology.
- **Information-Theoretic**: Shamir guarantees zero information leakage below threshold.
- **Memory Safe**: `#![forbid(unsafe_code)]` + `ZeroizeOnDrop` everywhere.

## Known Limitations (v0.2-alpha)

- **NAT Traversal**: Optimized for stable connections. Mobile/home NAT in development.
- **DHT Spam**: No rate-limiting on `put_record` yet.
- **Static Quorum**: t=4, n=7 hardcoded. Dynamic tuning planned for v0.3.
- **No Formal Verification**: Protocol state machine not yet formally verified.

---

## Contributing

Issues and PRs welcome. We value **honest technical critique** over polite praise.
→ [CONTRIBUTING.md](CONTRIBUTING.md) · [SECURITY.md](SECURITY.md)

---

*A **Hope** project — 🇫🇷 France · [github.com/lvs0](https://github.com/lvs0)*

***"Privacy is not a setting. It is an architectural property."*** ⬡
