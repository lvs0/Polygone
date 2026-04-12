# Security Model

## Overview

POLYGONE is designed with security as an **architectural property**, not a configuration option.

## Cryptographic Guarantees

### Post-Quantum Resistance

| Algorithm | Standard | NIST Level | Purpose |
|-----------|----------|------------|---------|
| ML-KEM-1024 | FIPS 203 | Level 5 | Key Exchange |
| ML-DSA-87 | FIPS 204 | Level 5 | Signatures |

**Why Level 5?** Maximum security for long-term confidentiality.

### Information-Theoretic Privacy

Shamir Secret Sharing with 4-of-7 threshold:

```
┌────────────────────────────────────────────────────────┐
│                    Original Message                      │
│                      (1 secret)                        │
└────────────────────────────────────────────────────────┘
                           │
                           ▼
┌────────────────────────────────────────────────────────┐
│  Fragment 1 │ Fragment 2 │ Fragment 3 │ Fragment 4   │
│      f₁     │     f₂     │     f₃     │     f₄       │
│  Fragment 5 │ Fragment 6 │ Fragment 7 │              │
│      f₅     │     f₆     │     f₇     │              │
└────────────────────────────────────────────────────────┘
                           │
            ┌──────────────┼──────────────┐
            ▼              ▼              ▼
         0 fragments    3 fragments    4+ fragments
            =            < threshold   = RECONSTRUCT
            0              = 0            = 1
            info           info           info
```

**Result**: Even with 6 fragments, attacker learns **zero** information.

### Memory Safety

```rust
// All secrets use ZeroizeOnDrop
pub struct SignSecretKey {
    #[zeroize(drop)]
    bytes: Zeroizing<Vec<u8>>,
}
```

Properties:
- Keys automatically zeroed when dropped
- No unsafe code anywhere (`#![forbid(unsafe_code)]`)
- No secrets in logs or error messages

## Attack Scenarios

### Passive Observer
- **Goal**: Learn who communicated with whom
- **Result**: IMPOSSIBLE - No persistent connections, no metadata

### Node Compromise
- **Goal**: Reconstruct messages
- **Result**: Each node has 1 fragment, useless alone

### Quantum Computer
- **Goal**: Break encryption
- **Result**: ML-KEM-1024 is quantum-resistant

### Man-in-the-Middle
- **Goal**: Modify messages
- **Result**: ML-DSA-87 signatures prevent tampering

## File Permissions

All secret keys are stored with restrictive permissions:

```bash
Secret Key (sk): 0o600 (rw-------)
Public Key (pk):  0o644 (rw-r--r--)
```

## Known Limitations

1. **NAT Traversal**: Full P2P requires relay nodes (v0.3)
2. **DHT Availability**: Network must have sufficient nodes
3. **Timing Analysis**: Traffic timing may leak patterns (future work)

## Reporting Security Issues

For security vulnerabilities, contact: **contact@soe-ai.dev**

Do NOT open public issues for security problems.
