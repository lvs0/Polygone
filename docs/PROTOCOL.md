# Out-of-Band Key Exchange Protocol

## Overview

POLYGONE uses a **hybrid classical/post-quantum key exchange** that separates the problem of:

1. **Key Exchange** → ML-KEM-1024 (post-quantum)
2. **Key Transport** → Out-of-band (human-mediated)

This design choice prioritizes **security over convenience**.

---

## Why Out-of-Band?

### The Problem with Online Key Exchange

Traditional messaging apps (Signal, WhatsApp, etc.) use online key exchange:
```
Server: "Here are Alice's public keys"
Client: "OK, I'll use these"
```

**Problems:**
- Server is a trusted third party
- Keys can be rotated server-side
- Metadata: who requested whose keys

### POLYGONE's Approach

```
Alice → [Some Medium] → Bob: "Here is my ML-KEM public key"
Bob → (derives topology) → Alice: "I received your message"
```

**Benefits:**
- No trusted server for key exchange
- Key exchange is auditable (human can verify)
- No metadata about key requests

---

## Protocol Specification

### Phase 1: Key Generation

```bash
polygone keygen --output ~/.polygone/keys
```

This generates:
```
~/.polygone/keys/
├── kem.pk    # ML-KEM-1024 public key (2592 bytes, shareable)
└── kem.sk    # ML-KEM-1024 secret key (4000 bytes, KEEP SECRET)
```

### Phase 2: Key Exchange

Alice shares her `kem.pk` via any medium:

| Method | Security | Usability |
|--------|----------|-----------|
| QR Code | ★★★★★ | Manual, limited size |
| Secure File Transfer | ★★★★ | Good for initial setup |
| Encrypted Email | ★★★ | Convenient, some trust needed |
| Signal/WhatsApp | ★★★ | Easy, requires third party |
| SSH Copy | ★★★★ | Good for power users |

### Phase 3: Message Exchange

```
┌─────────────────────────────────────────────────────────┐
│ ALICE                                                    │
│ ─────                                                   │
│ 1. Load Bob's public key: kem.pk                        │
│ 2. Run: polygone send --peer-pk kem.pk "message"       │
│ 3. Output: session.ct (ciphertext)                      │
│ 4. Send session.ct to Bob via any channel              │
└─────────────────────────────────────────────────────────┘
                            │
                            │ session.ct (out-of-band)
                            ▼
┌─────────────────────────────────────────────────────────┐
│ BOB                                                      │
│ ────                                                    │
│ 1. Load session.ct from Alice                           │
│ 2. Run: polygone receive --sk kem.sk --ciphertext session.ct │
│ 3. Output: decrypted message                            │
└─────────────────────────────────────────────────────────┘
```

---

## Security Considerations

### What the Protocol Provides

- **Post-quantum confidentiality** via ML-KEM-1024
- **Forward secrecy** (unique session keys)
- **Information-theoretic fragment privacy** (Shamir)
- **No server dependency** for key exchange

### What the Protocol Requires

- **Authenticity** of the shared public key
  - Verify the key actually belongs to your contact
  - Use a secure channel or out-of-band verification
- **Confidentiality** of your secret key
  - Never share `kem.sk`
  - Store securely (encrypted disk, HSM, etc.)
  - Consider backup (encrypted)

### Threat Model

| Threat | Protected By |
|--------|--------------|
| Passive eavesdropping | ML-KEM-1024 encryption |
| Quantum computer attack | ML-KEM-1024 (NIST Level 5) |
| Server compromise | No server in key exchange |
| Key impersonation | User verification required |
| Man-in-the-middle | Out-of-band verification |

---

## Key Verification

### Manual Verification (Recommended)

```
1. Alice shares key fingerprint via secure channel
2. Bob verifies fingerprint matches kem.pk
3. Key authenticity confirmed
```

### Fingerprint Format

```bash
polygone keygen --output keys
hexdump -C keys/kem.pk | sha256sum
# Output: xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx:xxxx
```

### Visual Verification

For QR codes:
```
1. Alice generates QR of kem.pk
2. Bob scans and displays fingerprint
3. Alice confirms (or rejects) via separate channel
```

---

## Future Improvements

### Planned: DHT-Based Key Resolution (v0.2)

```
Bob → Publishes kem.pk to DHT
Alice → Resolves Bob's PeerId → fetches kem.pk from DHT
Alice → Sends message
```

**Trade-offs:**
- ✅ More convenient
- ⚠️ Server can see who resolves whose key
- ⚠️ Requires bootstrapping

### Considered: Social Key Verification

Like Signal's safety numbers:
```
SHA-256(kem_alice || kem_bob) → safety number
Users verify safety number out-of-band once
```

---

## Implementation

See [`src/protocol/session.rs`] for the cryptographic session protocol.

See [`docs/SECURITY.md`] for memory safety and key protection details.
