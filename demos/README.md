# Polygone Demos

This directory contains interactive demonstrations of the Polygone protocol.

## Scripts

### `2-node-demo.sh`
Interactive 2-node session demo (Alice + Bob on same machine).
Shows the full protocol: KEM → topology → encryption → fragmentation → reassembly → dissolution.

```bash
./demos/2-node-demo.sh "Your message here"
```

### `protocol-walkthrough.sh`
Step-by-step educational walkthrough of each cryptographic operation.
Explains what happens at each layer and why the design choices were made.

```bash
./demos/protocol-walkthrough.sh
```

## Prerequisites

Both scripts use `./target/debug/polygone` by default.
Build it first (or the scripts will attempt to):

```bash
cargo build -p polygone
```

## Demo Flow (2-node-demo)

```
1. Bob generates a fresh keypair (or reuses cached demo keys)
2. Alice encapsulates a session secret with Bob's KEM public key
3. Both independently derive the same 7-node topology (BLAKE3 XOF)
4. Alice encrypts her message with AES-256-GCM
5. The ciphertext is split into 7 Shamir fragments (threshold=4)
6. Bob receives fragments, reconstructs the encrypted payload
7. Bob decrypts → recovers the plaintext message
8. Both sessions dissolve — all keying material zeroized
```

## What v1.0 vs v2.0 Means

| Feature | v1.0 (current) | v2.0 (target) |
|---------|----------------|---------------|
| Fragment transport | In-process (local demo) | libp2p + Kademlia DHT |
| Node discovery | N/A | Real P2P bootstrap |
| Topology derivation | Local | Local (same algorithm) |
| KEM exchange | Out-of-band (manual) | Via DHT |
| Relay network | None | Real distributed nodes |

## Security Notes

- The demo uses ephemeral in-process communication.
- In v1.0, there's no real network anonymity (it's a local prototype).
- The cryptographic stack (ML-KEM-1024, AES-256-GCM, Shamir 4-of-7, BLAKE3) is post-quantum secure.
- Session key erasure is enforced via `ZeroizeOnDrop` on all sensitive types.