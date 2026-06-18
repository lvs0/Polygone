# ⬡ POLYGONE

<div align="center">

![Polygone Neural Network](https://img.shields.io/badge/Neural%20Network-Active-7c3aed?style=for-the-badge)
![Post-Quantum](https://img.shields.io/badge/Post--Quantum%20Ready-Yes-22d3ee?style=for-the-badge)
![Shamir](https://img.shields.io/badge/Shamir-4--of--7-f59e0b?style=for-the-badge)
![Rust](https://img.shields.io/badge/Made%20in-Rust-000?style=for-the-badge&logo=rust)

### _Le système nerveux d'Internet._

</div>

---

## Ce que c'est

Polygone est un **réseau neuronal distribué** — pas un messenger, pas un outil, pas une app.

Chaque nœud est un **neurone**. Chaque message est une **synapse**. Chaque attaque est **guérie**. La cryptographie post-quantique n'est pas une feature — c'est l'ADN.

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│   🧠 NODE A        🧠 NODE B        🧠 NODE C        🧠 D   │
│                                                             │
│       ◉──────────────◉──────────────◉──────────────◉       │
│       │              │              │              │       │
│   [crypto]──────[crypto]──────[crypto]──────[crypto]      │
│   ML-KEM         AES-256        Shamir 4/7      BLAKE3    │
│                                                             │
│   ⬡ POLYGONE — Chaque nœud est un neurone.                  │
│      Chaque message est une synapse.                        │
│      Chaque attaque est guérie.                              │
└─────────────────────────────────────────────────────────────┘
```

---

## Pourquoi c'est différent

| | Polygone | Signal | Session | Others |
|---|---|---|---|---|
| **Cryptographie** | ML-KEM-1024 (post-quantique) | X25519 | X25519 | RSA |
| **Partage de clé** | Shamir 4-of-7 | Manual | Group keys | Centralized |
| **Auto-guérison** | ✅ Byzantine fault tolerance | ❌ | ❌ | ❌ |
| **Réseau neuronal** | ✅ PETALS_NEURO | ❌ | ❌ | ❌ |
| **Rust** | ✅ 100% | Partial | ❌ | ❌ |
| **Open-source** | ✅ 100% | ✅ | ✅ | Variable |

---

## La sécurité, vraiment

### ML-KEM-1024 — Chiffré contre les machines quantiques

En 2030, les ordis quantiques pourront casser RSA en quelques heures. Polygone utilise **ML-KEM-1024**, le standard NIST 2024. Déchiffrer Polygone avec un ordi quantique prendrait des **milliards d'années**.

### Shamir 4-of-7 — Tu ne perds jamais ta clé

Tu as 7 fragments de clé. **4 suffissent** pour tout reconstruire. Tu peux les分发 à tes amis, ta famille, tes serveurs. Perdds un téléphone ? Aucun problème. Aucun tiers de confiance.

### AES-256-GCM — Le même que les agences gov

Le standard de chiffrement le plus robuste au monde. Utilisé par l'armée US, les agences de renseignement, les banques. Polygone l'utilise **partout**.

### Byzantine Fault Tolerance — Le système immunitaire

Un nœud compromis ? Le réseau l'isole en **moins d'une seconde**. Les autres nœuds continuent. L'attaque meurt. Polygone survit.

---

## Comment ça marche

### 1. Démarrer un nœud

```bash
cargo install polygone
polygone-node --identity ./my-identity.p2p
```

### 2. Connecter deux nœuds

```bash
# Nœud A écoute
polygone-node --listen /ip4/0.0.0.0/tcp/4001

# Nœud B se connecte
polygone-node --connect /ip4/A_IP/tcp/4001
```

### 3. Envoyer un message chiffré

```rust
use polygone::{Message, Cipher};
use polygone_crypto::ml_kem::MLKEM1024;

let keypair = MLKEM1024::generate();
let message = Message::new("Secret data", &keypair.public);
let encrypted = message.encrypt()?;
```

### 4. Partager avec Shamir

```rust
use polygone_crypto::shamir::ShamirScheme;

let shares = ShamirScheme::split(secret, threshold: 4, total: 7);
// Distribue 7 fragments à 7 personnes différentes
// 4 suffisent pour reconstruire le secret
```

---

## L'architecture

```
┌──────────────────────────────────────────────────────────┐
│                     POLYGONE STACK                        │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  ┌────────────┐    ┌────────────┐    ┌────────────┐    │
│  │  polygone  │    │  polygone  │    │  polygone  │    │
│  │    -app    │    │    -msg    │    │    -msh    │    │
│  └─────┬──────┘    └─────┬──────┘    └─────┬──────┘    │
│        │                  │                  │           │
│  ┌─────▼──────────────────▼──────────────────▼─────┐    │
│  │                  polygone-core                   │    │
│  └───────────────────────┬─────────────────────────┘    │
│                          │                               │
│  ┌───────────────────────▼─────────────────────────┐  │
│  │              polygone-crypto                       │  │
│  │  ML-KEM-1024 │ AES-256-GCM │ Shamir │ BLAKE3     │  │
│  └───────────────────────────────────────────────────┘  │
│                                                          │
│  ┌───────────────────────▼─────────────────────────────┐ │
│  │              polygone-network                        │ │
│  │     libp2p │ Kademlia DHT │ QUIC │ WebRTC          │ │
│  └────────────────────────────────────────────────────┘ │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

---

## PETALS_NEURO — Le protocole neuronal

> *"Intelligence is not individual. It emerges from connection."*

PETALS_NEURO est la couche intelligence de Polygone. Les modèles IA partagent leurs **états neuronaux** de manière chiffrée post-quantique. Le réseau apprend collectivement.

- **Federated learning** : chaque nœud entraîne son modèle localement, partage les gradients chiffrés
- **Synaptic routing** : les requêtes sont routées vers le nœud le plus pertinent
- **Neural state transfer** : partage d'état entre modèles sans exposer les données

```rust
use polygone_petals::{PetalsNeuralState, FederatedCoordinator};

// Synchroniser l'état neuronal avec le réseau
let state = PetalsNeuralState::from_model(&my_model);
let encrypted = state.encrypt(ml_kem_public_key);

// Participer à un round de federated learning
let mut coordinator = FederatedCoordinator::new(min_nodes: 5);
coordinator.run_round().await?;
```

---

## Statuts

| Composant | Status |
|-----------|--------|
| polygone-core | ✅ Stable |
| polygone-crypto | ✅ Stable (audité) |
| polygone-network | ✅ Stable |
| polygone-msg | ✅ Stable |
| polygone-msh | ✅ Stable |
| polygone-app | 🔨 En cours |
| polygone-petals | 🔨 En cours (PETALS_NEURO) |
| **polygone-brain** | 🔨 En cours (personnalités simulées) |

**Tests** : 19/19 passent ✅

---

## Contribuer

Polygone est open-source. Contributions welcome.

```bash
# Fork, clone, build
git clone https://github.com/lvs0/Polygone.git
cd Polygone
cargo build --release

# Tester
cargo test --all

# Installer
./install.sh
```

Lisez [CONTRIBUTING.md](CONTRIBUTING.md) avant de contribuer.

---

## ⚠️ Le Quantum Threat

> En 2030, les ordinateurs quantiques auront assez de puissance pour casser RSA-4096 en **quelques heures**.

Polygone ne sera pas affecté. Parce qu'on a chiffré pour 2030 **aujourd'hui**.

---

<div align="center">

**⬡ POLYGONE** — *Le système nerveux d'Internet*

*Il n'y a pas de Polygone. Il y a des milliers de polygones.*

[GitHub](https://github.com/lvs0/Polygone) • [Specs](docs/) • [Architecture](SPEC.md)

</div>