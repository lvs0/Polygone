# Security Policy

## Supported Versions

Currently, only the `main` branch is supported for security updates.

| Version | Supported          |
| ------- | ------------------ |
| v0.2.x | :white_check_mark: |
| < v0.2  | :x:                |

## Reporting a Vulnerability

**DO NOT open a public GitHub issue for security vulnerabilities.**

If you discover a security-critical bug or a flaw in the cryptographic protocol, please report it via one of the following methods:

1. **Email**: Send a detailed report to `contact@soe-ai.dev`.
2. **Encrypted Channel**: (Optional) If you have a preferred encrypted method, email us first to coordinate keys.

### What is considered a security vulnerability?
- Flaws in the ML-KEM or ML-DSA implementation.
- Insecure deterministic topology derivation (predictability).
- Failures in memory zeroing (`ZeroizeOnDrop`).
- Potential for DHT correlation attacks beyond what is documented in `README.md`.

We aim to respond to all security reports within 48 hours and provide a fix as soon as possible. Thank you for helping keep the network safe.
