# ⬡ POLYGONE

> *L'information n'existe pas. Elle traverse.*

**Post-quantum ephemeral privacy network — built in Rust.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://rust-lang.org)
[![Status: v1.0](https://img.shields.io/badge/status-v1.0--stable-green.svg)]()
[![unsafe: forbidden](https://img.shields.io/badge/unsafe-forbidden-red.svg)]()
[![no telemetry](https://img.shields.io/badge/telemetry-none-brightgreen.svg)]()

---

## What it is

Classical cryptography hides the **content** of a communication.  
It cannot hide that the communication **happened**.

POLYGONE changes the question.

A message becomes a distributed computational state across 7 ephemeral nodes.  
Any 4 fragments reconstruct it. No single node holds more than a fragment.  
The network dissolves. Keys are zeroed in memory. The exchange did not happen.

**The target doesn't exist. There is no attack surface.**

---

## How it works

```
1. KEY EXCHANGE
   Alice has Bob's ML-KEM-1024 public key (shared out-of-band).
   She encapsulates a shared secret → gets a KEM ciphertext.
   She sends the ciphertext to Bob.

2. TOPOLOGY DERIVATION (independent, both sides)
   Alice and Bob independently derive identical:
     ▸ 7 ephemeral node IDs   (BLAKE3 "polygone-topo-nodes-v1")
     ▸ AES-256-GCM session key (BLAKE3 "polygone-sess-v1")
   No extra communication. Pure determinism from the shared secret.

3. ENCRYPT + FRAGMENT
   plaintext → AES-256-GCM → ciphertext
   ciphertext → Shamir 4-of-7 → 7 fragments
   Each fragment → one ephemeral node

4. RECONSTRUCT
   Bob collects ≥4 fragments → Shamir reconstruct → decrypt → plaintext

5. DISSOLVE
   All nodes dissolve. Keys zeroed (ZeroizeOnDrop).
   The session is gone from memory.
```

---

## Crypto stack

| Layer          | Primitive                          | Standard        |
|----------------|------------------------------------|-----------------|
| KEM            | ML-KEM-1024                        | FIPS 203        |
| Signature      | Ed25519 (ML-DSA-87 upgrade path)   | RFC 8032        |
| Symmetric      | AES-256-GCM                        | NIST SP 800-38D |
| KDF            | BLAKE3 (domain-separated)          | —               |
| Secret sharing | Shamir threshold=4, n=7            | —               |
| Language       | Rust stable (`#[forbid(unsafe_code)]`) | —          |

**Key derivation domain separation:**
```
BLAKE3("polygone-topo-nodes-v1" + shared_secret) → topology seed
BLAKE3("polygone-sess-v1"       + shared_secret) → AES-256-GCM key
```
An adversary who learns the topology structure learns nothing about the encryption key.

---

## Installation

```bash
# Prerequisites: Rust stable (1.75+)
rustup toolchain install stable
rustup default stable

# Build
git clone https://github.com/lvs0/Polygone
cd Polygone
cargo build --release

# Install globally
cargo install --path .
```

---

## Usage

### Generate your keypair

```bash
polygone keygen
# Keys saved to ~/.polygone/keys/ with chmod 600
# Share your KEM public key: ~/.polygone/keys/kem.pk
```

### Local demo (no network)

```bash
polygone send --peer-pk demo --message "L'information n'existe pas"
```

### Send to a real peer

```bash
# Step 1 (Alice): Encrypt for Bob's public key
polygone send --peer-pk bob_kem.pk --message "Hello Bob"
# → Outputs: KEM ciphertext (hex) + 7 fragment bytes

# Step 2: Send ciphertext + ≥4 fragments to Bob out-of-band

# Step 3 (Bob): Reconstruct
polygone receive --ciphertext <ct_hex>
```

### TUI dashboard

```bash
polygone tui
```

Navigate with `1`–`6`, quit with `q`.

### Node operator

```bash
# Contribute to the network (VPS with 512 MB RAM is enough)
polygone node start --listen 0.0.0.0:4001 --ram-mb 128

# Check status
polygone status

# Verify everything works
polygone self-test
```

### Read from stdin

```bash
echo "Secret message" | polygone send --peer-pk demo --message -
cat secret.txt      | polygone send --peer-pk bob.pk --message -
```

---

## Project structure

```
src/
├── crypto/
│   ├── mod.rs          — SharedSecret, KeyPair, BLAKE3 KDF
│   ├── kem.rs          — ML-KEM-1024 (FIPS 203)
│   ├── sign.rs         — Ed25519 (ML-DSA upgrade path)
│   ├── symmetric.rs    — AES-256-GCM
│   └── shamir.rs       — Shamir 4-of-7
├── network/
│   ├── mod.rs          — NodeId
│   ├── topology.rs     — Deterministic topology derivation
│   └── node.rs         — EphemeralNode lifecycle, ZeroizeOnDrop
├── protocol/
│   ├── mod.rs          — SessionId, TransitState
│   └── session.rs      — Full session lifecycle + tests
├── tui/
│   ├── mod.rs
│   ├── app.rs          — TUI main loop (ratatui + crossterm)
│   ├── views.rs        — All screens (dashboard, keygen, send…)
│   └── widgets.rs      — Reusable widgets
├── keys.rs             — Key persistence with chmod 600
├── error.rs            — Unified PolygoneError
├── lib.rs              — Crate root + re-exports
└── main.rs             — CLI (clap v4) + all commands
```

---

## Honest status

**v1.0 — Stable local protocol.**

| Feature                                  | Status  |
|------------------------------------------|---------|
| ML-KEM-1024 key exchange                 | ✔ Done  |
| AES-256-GCM + BLAKE3 KDF                 | ✔ Done  |
| Shamir 4-of-7 (all C(7,4)=35 combos)    | ✔ Done  |
| Full session lifecycle + tests           | ✔ Done  |
| Key persistence with secure permissions  | ✔ Done  |
| TUI dashboard (ratatui)                  | ✔ Done  |
| `self-test` command (5 tests)            | ✔ Done  |
| P2P fragment transport (libp2p + DHT)    | v2.0    |
| Automatic peer discovery                 | v2.0    |
| External cryptographic audit             | Planned |

**No external audit has been performed.** Do not use in production for sensitive data until an audit is complete.

---

## Contribute

Issues and PRs welcome. Honest criticism is preferred over encouragement.

Node operators: a 512 MB RAM VPS is sufficient.

---

## License

MIT — No investors. No token. No telemetry. No data collection.

*Built by Lévy, 14, France.*

*"Privacy is not a setting. It is an architectural property."*
