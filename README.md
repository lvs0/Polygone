# POLYGONE ⬡

> *L'information n'existe pas. Elle traverse.*

Post-quantum ephemeral privacy network — built in Rust.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-nightly-orange.svg)]()
[![Status: v0.2 Operational](https://img.shields.io/badge/status-v0.2_Operational-green.svg)]()
[![unsafe: forbidden](https://img.shields.io/badge/unsafe-forbidden-red.svg)]()

---

## The Concept

La cryptographie classique protège le **contenu** d'une communication, mais elle ne peut pas cacher que la communication **a eu lieu**. Les métadonnées restent : l'IP source, la cible, et l'heure de transmission.

**POLYGONE** change ce paradigme.

Un réseau P2P éphémère naît via une Distributed Hash Table (Kademlia DHT). Il transporte une computation mathématique brisée en fragments de Shamir, puis disparaît en vaporisant la clé de session. Un observateur externe ne voit pas un flux de messages chiffrés de A vers B ; il observe du bruit ambiant asynchrone éclaté à travers le DHT mondial.

**S'il n'y a pas de cible, il n'y a pas de surface d'attaque.**

---

## Architecture de Résilience

```text
1. 🔐 SYNCHRONISATION (Hors-Bande)
   Alice et Bob échangent une clé publique ML-KEM-1024.
   La clé ne chiffre aucun payload ; elle initie unifiant l'architecture du réseau de transit.

2. 🧬 DÉRIVATION DÉTERMINISTE
   Un graphe théorique de 7 noeuds virtuels est généré en mémoire via le secret partagé post-quantique.
   Un hash (BLAKE3) avec séparation de domaine agit comme KDF. Absolument personne d'autre
   ne peut deviner quelles clés Kademlia seront générées et ciblées.

3. 🌪️ DISPERSION KADEMLIA
   Le payload est chiffré (AES-256-GCM), puis fragmenté (Shamir, t=4, n=7).
   Chaque fragment est largué sur le réseau P2P natif via `put_record` avec
   un Quorum Absolu de Majorité. Aucun relais ne voit plus d'un fragment (mathématiquement non reconstructible).

4. 💨 VAPORISATION (TTL)
   La donnée possède un Time-To-Live agressif (ex: 30s) imposé au réseau. Elle se désintègre
   de la RAM des relais à la seconde T. Localement, des contraintes Rust (`ZeroizeOnDrop`) nettoient les OS d'Alice et Bob.
   L'échange n'a jamais eu lieu.
```

---

## Stack Technique

| Couche        | Technologie              | Rôle |
|---------------|--------------------------|---|
| **KEM**           | ML-KEM-1024 (FIPS 203)   | Accords de clés (Résistance algorithme de Shor) |
| **Signature**     | ML-DSA-87 (FIPS 204)     | Authentification |
| **Symétrique**    | AES-256-GCM              | Chiffrement authentifié du payload |
| **Dérivation**    | BLAKE3                   | KDF ultra-rapide / Topologies Seeds déterministes |
| **Secret share**  | Shamir Secrets (t=4, n=7)| Fragmentation Information-theoretic |
| **Réseau**        | libp2p / Kademlia DHT    | Transport P2P mondial & Stockage Asynchrone Ephemère |
| **Langage**       | Rust (nightly)           | Memory-safety stricte, Drop-Traits nettoyants |

---

## Run The Testnet (Local Proof of Concept)

Polygone P2P v0.2 est complètement fonctionnel via CLI. Vous pouvez lancer le cluster local pour explorer l'architecture P2P :

```bash
# 1. Utiliser le profil sécurisé nightly
rustup toolchain install nightly && rustup default nightly

# 2. Cloner le dépôt
git clone https://github.com/lvs0/Polygone && cd Polygone

# 3. Lancer un cluster de 7 relais P2P Kademlia en fond (simule l'Internet)
./testnet.sh

# 4. Dans un terminal, exécuter l'Auto-Test P2P End-to-End
cargo run -- self-test
```

Voir le code de livraison de fragment P2P en temps réel via :
```bash
cargo run -- send --peer-pk demo --message "Hello Hacker News"
```

---

## Roadmap Accomplie ✅

- [x] Primitives crypto post-quantiques (KEM, DSA, AES, Shamir, BLAKE3)
- [x] Dérivation de topologie déterministe depuis un shared secret
- [x] Lifecycle noeuds éphémères (Drop safe memory clearing avec protection UNIX `0600`)
- [x] Couche compute/stockage distribué
- [x] Intégration complète `libp2p` (Swarm asynchrone, connectivité)
- [x] **Protocole MPC transit réel** (Fragments distribués via Records Kademlia + Majority Quorum)
- [x] Contre-mesures DHT (Records TTL : auto-évaporation du réseau à 30s)
- [x] Benchmarks (Criterion intégrés aux primitives ML-KEM)
- [x] P2P & Cryptographic Security Audit

---

## Participer

Issues et PRs bienvenues. La critique honnête et technique (cryptanalyse, failles réseaux, memory leak) est préférée à l'encouragement poli.

Opérateurs réseau : un VPS Linux 512 MB RAM (sans swap ! la prudence exige d'empêcher les dumps mémoires du kernel) suffit pour héberger un relais `cargo run --release -- node start`.

---

## Licence

MIT — Pas d'investisseurs. Pas de token. Pas de collecte de données.
Construit par Lévy, 14 ans, France.

*"La confidentialité n'est pas un paramètre. C'est une propriété architecturale."*
