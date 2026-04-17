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

1. **Post-Quantum Handshake**: ML-KEM-1024 key exchange
2. **Deterministic Topology**: BLAKE3 derives 7 random nodes
3. **Shamir Dispersion**: AES-256-GCM + 4-of-7 secret sharing
4. **Vaporization**: 30s TTL, data auto-evaporates

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

## Known Limitations

- NAT traversal in progress (v0.3)
- DHT spam protection planned
- Static quorum (4-of-7)

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
