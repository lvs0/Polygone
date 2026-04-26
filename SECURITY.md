# POLYGONE — Security Model

## What POLYGONE protects against

### Threat model

| Threat                                               | Protection                          |
|------------------------------------------------------|-------------------------------------|
| Content interception                                 | AES-256-GCM (IND-CCA2)             |
| Traffic analysis (who sent to whom)                  | Shamir distribution — partial       |
| Quantum computer breaking asymmetric crypto          | ML-KEM-1024 (FIPS 203)             |
| Single node compromise leaking the message           | Shamir: ≤3 nodes → zero info        |
| Key material lingering in memory after session       | ZeroizeOnDrop on all key types      |
| Unsafe memory operations                             | `#[forbid(unsafe_code)]`            |

### What POLYGONE does NOT protect against (v1.0)

- **Full P2P anonymity**: v1.0 is local-mode only. Real transport metadata protection requires libp2p (v2.0).
- **Nation-state adversaries**: No external audit has been performed.
- **Endpoint compromise**: If your machine is compromised, no protocol helps.
- **Long-term key compromise**: Your ML-KEM secret key must be stored securely.

---

## Cryptographic design

### Key derivation

```
shared_secret (32 bytes, from ML-KEM-1024)
    │
    ├── BLAKE3("polygone-topo-nodes-v1", secret) → topology_seed (32 bytes)
    │       → 7 node IDs (8 bytes each, BLAKE3 XOF)
    │
    └── BLAKE3("polygone-sess-v1",       secret) → session_key (32 bytes)
            → AES-256-GCM key
```

Domain separation guarantees: knowing the topology structure reveals zero bits about the encryption key, and vice versa.

### Shamir information-theoretic security

With threshold=4, n=7:
- Any 3 or fewer fragments reveal **zero information** about the secret (information-theoretically, not just computationally).
- Any 4 or more fragments reconstruct the secret exactly.

### AES-256-GCM nonce handling

A fresh 96-bit nonce is generated from `OsRng` for every encryption operation. Nonce reuse under the same key would be catastrophic for GCM; this is prevented by design.

---

## Known limitations

1. **`pqcrypto` zeroize**: The ML-KEM types from `ml-kem` wrap internal state that may not be fully zeroed on drop at the byte level on all platforms. We zero what we control (our own byte arrays). This is a crate limitation, not a POLYGONE-specific issue.

2. **Memory swap**: On systems with enabled swap, zeroed key material may have been written to disk before zeroing. Use encrypted swap or `mlock` (v2.0) to prevent this.

3. **Side channels**: POLYGONE does not currently implement constant-time operations beyond what the underlying crates provide. The `subtle` crate is available for future use.

---

## Reporting vulnerabilities

Open a GitHub issue marked `[SECURITY]`, or email directly.  
PGP key available on request.

**Do not exploit vulnerabilities. Report them.**
