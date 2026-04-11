# Contributing to Polygone ⬡

First off, thank you for considering contributing to Polygone! We are building a post-quantum ephemeral privacy network, and community feedback is crucial for our security and stability.

## How Can I Contribute?

### Cryptanalysis & Security Review
Our primary need is technical review of the core protocol. If you find a flaw in our Shamir implementation, KEM derivation, or network topology, please report it! 
- Check `src/crypto/` for primitives.
- Check `src/network/topology.rs` for the deterministic derivation logic.

### Reporting Bugs
If you find a bug, please open an issue on GitHub with:
- Steps to reproduce.
- Expected vs. actual behavior.
- Your environment (Rust version, OS).

### Code Contributions
1. **Fork** the repository.
2. **Create a branch** for your feature or fix.
3. **Write tests**! We won't merge code without tests.
4. **Submit a Pull Request**.

## Technical Standards
- **No Unsafe**: We forbid unsafe code. Do not use it.
- **Documentation**: All public functions must have docstrings.
- **Zeroize**: Any sensitive material must be zeroed on drop.

## Philosophy
We value **honest technical critique** over polite praise. If you think an architectural choice is wrong, tell us why and how to fix it.

***"La confidentialité n'est pas un paramètre. C'est une propriété architecturale."***
