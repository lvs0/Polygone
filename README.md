# POLYGONE

> *L'information n'existe pas. Elle traverse.*

Post-quantum ephemeral privacy network — built in Rust.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-nightly-orange.svg)]()
[![Status: Research](https://img.shields.io/badge/status-research-yellow.svg)]()
[![unsafe: forbidden](https://img.shields.io/badge/unsafe-forbidden-red.svg)]()

---

## Ce que c'est

La cryptographie classique cache le **contenu** d'une communication.
Elle ne peut pas cacher que la communication **a eu lieu**.

POLYGONE change la question.

Un réseau éphémère naît, transporte une computation distribuée en quelques
millisecondes, puis disparaît. Un observateur externe ne voit pas un message
chiffré. Il voit du bruit ambiant du réseau.

**La cible n'existe pas. Il n'y a pas de surface d'attaque.**

---

## Comment ça fonctionne

```
1. SYNCHRONISATION
   Alice et Bob échangent une clé ML-KEM-1024.
   Une fois. Hors-bande. La clé ne chiffre rien —
   elle définit l'architecture du réseau pour cet échange.
   Puis elle est détruite.

2. NAISSANCE
   Un réseau de 7 noeuds éphémères naît, dérivé
   déterministiquement du secret partagé.
   Aucune coordination réseau supplémentaire.

3. TRANSIT
   Le message devient un état distribué temporaire.
   4 fragments minimum pour reconstruire.
   Aucun noeud ne voit plus d'un fragment.

4. DISPARITION
   Le réseau se dissout. Clés zéroïsées.
   L'échange n'a pas eu lieu.
```

---

## Stack

| Couche        | Technologie              |
|---------------|--------------------------|
| KEM           | ML-KEM-1024 (FIPS 203)   |
| Signature     | ML-DSA-87 (FIPS 204)     |
| Symétrique    | AES-256-GCM              |
| Hash          | BLAKE3                   |
| Secret share  | Shamir threshold=4, n=7  |
| Réseau        | libp2p / Kademlia DHT    |
| Langage       | Rust (nightly)           |

---

## Installation

```bash
rustup toolchain install nightly
rustup default nightly
git clone https://github.com/[username]/polygone && cd polygone
cargo build --release
./target/release/polygone node
```

---

## Structure

```
src/
├── crypto/        # ML-KEM, ML-DSA, AES-GCM, Shamir, BLAKE3
├── network/       # NodeId, topology derivation, node lifecycle
├── protocol/      # Session state machine, transit, dissolution
├── compute_storage.rs  # P2P compute + encrypted storage layer
├── error.rs
├── lib.rs
└── main.rs        # CLI (clap v4)
```

---

## Statut honnête

Recherche active. Pas en production. Audit cryptographique externe non effectué.

- [x] Primitives crypto (KEM, DSA, AES, Shamir, BLAKE3)
- [x] Dérivation de topologie depuis shared secret
- [x] Lifecycle noeuds éphémères
- [x] Couche compute/stockage distribué
- [ ] libp2p intégration complète
- [ ] Protocole MPC transit réel
- [ ] Audit externe
- [ ] Benchmarks

---

## Participer

Issues et PRs bienvenues. La critique honnête est préférée à l'encouragement.

Opérateurs : un VPS 512 MB RAM suffit. Voir `TECHNICAL_SPEC.md`.

---

## Licence

MIT — Pas d'investisseurs. Pas de token. Pas de collecte de données.
Construit par Lévy, 14 ans, France.

*"La confidentialité n'est pas un paramètre. C'est une propriété architecturale."*
