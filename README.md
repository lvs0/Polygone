# ⬡ POLYGONE

> *"Information does not exist. It drifts."* / *"L'information n'existe pas. Elle traverse."*

**POLYGONE** is a post-quantum ephemeral privacy network designed to solve the **Metadata Problem**. Built in pure Rust.

---

## English | [Français](#français)

### The Problem: The Metadata Leak

Traditional encryption protects **content**, but it cannot hide that a **communication occurred**. Source IPs, target IPs, timing, and packet sizes remain visible to observers.

**POLYGONE changes the paradigm.** Instead of an encrypted tunnel between A and B, POLYGONE turns a message into a distributed, transient mathematical state—a wave that crosses a global DHT and then vaporizes.

---

## Quick Start (30 seconds)

```bash
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash
```

That's it! Then run:
```bash
polygone help
```

### Commands

| Command | Description |
|---------|-------------|
| `polygone help` | Show all commands |
| `polygone self-test` | Verify installation |
| `polygone keygen` | Generate encryption keys |
| `polygone send` | Send a message |
| `polygone node` | Start relay node |
| `polygone update` | Update to latest |
| `polygone uninstall` | Remove Polygone |

---

## How It Works

> **Note:** POLYGONE's security guarantee is architectural — not a setting. Each layer addresses a specific dimension of the metadata problem.

1. **Post-Quantum Handshake**: ML-KEM-1024 key exchange. The shared secret is quantum-resistant — no future quantum computer can recover it.
2. **Deterministic Node Selection**: BLAKE3 hashes the session key to derive exactly 7 DHT peers. The selection is *reproducible* (same key → same nodes) but *unpredictable* (no key → no information). Nodes are selected via Kademlia DHT — no central directory.
3. **Shamir Dispersion**: Message is encrypted with AES-256-GCM, then split into 7 fragments via Shamir 4-of-7 secret sharing. Each fragment is sent to a different node. Any 4 fragments reconstruct the message; fewer than 4 provide zero information (information-theoretic security).
4. **30s Vaporization**: Fragments have a 30-second TTL. After 30 seconds, the message ceases to exist — no server stores it, no log records it.

### Security

- **Post-Quantum**: ML-KEM-1024 + ML-DSA-87
- **Information-Theoretic**: Shamir (k-1 fragments = 0 info)
- **Memory Safety**: `#![forbid(unsafe_code)]` + ZeroizeOnDrop
- **Forward Secrecy**: Unique keys per session

### Benchmarks

| Operation | Latency |
|-----------|---------|
| ML-KEM Encapsulation | ~34 µs |
| AES-256-GCM Encrypt | ~3.8 µs |
| Full Send (E2E) | ~208 µs |

---

## Known Limitations (v1.0.0)

- Static quorum: 4-of-7 (non-configurable in this release)
- NAT traversal: basic hole-punching; full relay fallback available
- DHT: Kademlia-based; Sybil resistance improvements in progress
- Ephemeral-only: no persistent message storage (by design)

---

## Contributing

Issues and PRs welcome. Privacy is an architectural property, not a setting. ⬡

---

## Français

### Le Problème

Le chiffrement traditionnel ne cache pas qu'une communication a eu lieu.

**POLYGONE change le paradigme.** Un message devient un état mathématique distribué transient qui s'évapore.

### Installation

```bash
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash
polygone help
```

### Sécurité

- **Post-Quantique**: ML-KEM-1024 + ML-DSA-87
- **Information-Théorique**: Shamir Secret Sharing
- **Mémoire**: `#![forbid(unsafe_code)]` + ZeroizeOnDrop

---

*by Lévy, 14 ans, France*
