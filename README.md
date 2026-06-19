# ⬡ POLYGONE

<div align="center">

### Le système nerveux d'Internet.

*Chiffré post-quantique. Distribué. Vivant.*

[![Post-Quantum](https://img.shields.io/badge/Post--Quantum-NIST%20ML--KEM--1024-22d3ee?style=flat-square)](https://csrc.nist.gov/projects/post-quantum-cryptography)
[![Shamir](https://img.shields.io/badge/Shamir-4--of--7-f59e0b?style=flat-square)](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing)
[![Rust](https://img.shields.io/badge/Made%20in-100%25%20Rust-000?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-7c3aed?style=flat-square)](LICENSE)

</div>

---

## En 2030, ils liront tout.

Les clés RSA qui protègent tes messages aujourd'hui — WhatsApp, Signal, tesbanques — seront une blague pour un ordi quantique à 1000$.

Tes conversations. Tes fichiers. Tes mots de passe. Tes pensées numériques. **Tout lu. Tout gardé. Tout exploité.**

On n'a pas le luxe d'attendre. La cryptographie post-quantique, c'est maintenant ou c'est trop tard.

**Polygone chiffre pour 2030, aujourd'hui.**

---

## Ce que c'est

Polygone n'est pas un messenger. Ce n'est pas une app. Ce n'est pas un outil.

C'est le **système immunitaire d'Internet**.

Chaque nœud est un neurone. Chaque message est une synapse chiffrée. Le réseaune stocke rien — chaque fragment meurt en 30 secondes. Et quand les machines quantiques arrivent en 2030, le chiffrement tient quand même.

**La cryptographie post-quantique n'est pas une feature. C'est l'ADN.**

---

## Pourquoi pas Signal ? Pourquoi pas Tor ?

| | Polygone | Signal | Tor | Telegram |
|---|---|---|---|---|
| **Post-quantique** | ML-KEM-1024 | X25519 ⚠️ | RSA ⚠️ | RSA ⚠️ |
| **Fragmentation Shamir** | 4-of-7 | ❌ | ❌ | ❌ |
| **Auto-guérison (BFT)** | ✅ | ❌ | ❌ | ❌ |
| **Zéro données persistées** | ✅ | ❌ | Partial | ❌ |
| **100% Rust** | ✅ | ❌ | Partial | ❌ |
| **Réseau neuronal IA** | PETALS_NEURO | ❌ | ❌ | ❌ |

⚠️ = vulnérable aux ordinateurs quantiques

---

## Comment ça marche

```
Nœud A                       Réseau Polygone                     Nœud B
  │                                │                                │
  │  1. ML-KEM-1024               │                                │
  │──────────────────────────────► │  (clé du destinataire)         │
  │                                │                                │
  │  2. AES-256-GCM               │                                │
  │     Chiffrement du message    │                                │
  │──────────────────────────────► │                                │
  │                                │                                │
  │  3. Shamir 4-of-7             │                                │
  │     Fragmentation             │                                │
  │     [F1]──►[F2]──►[F3]──►    │                                │
  │     [F4]──►[F5]──►[F6]──►    │                                │
  │     [F7]                      │                                │
  │            Chaque fragment prend un chemin différent             │
  │                                │                                │
  │  4. TTL 30s                   │                                │
  │     Auto-destruction          │                                │
  │                    ✗   ✗   ✗  (les fragments meurent)           │
  │                                │                                │
  │  5. Réassemblage (4 fragments suffisent)                        │
  │                                │        ◄────────────────────── │
  │  6. Déchiffrement             │        4-of-7 → message         │
  │                                │                                │
  │  7. ML-DSA-87                 │                                │
  │     Vérification authenticité │        ◄────────────────────── │
```

---

## Démarrer — 30 secondes

```bash
# Une ligne. Tu es dans le réseau.
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash
```

```bash
# Vérifier que ça marche
polygone self-test

# Générer tes clés post-quantiques
polygone keygen

# Démarrer ton nœud
polygone start

# Envoyer un message
polygone send "La vie privée est un droit."
```

---

## L'architecture

```
polygone/
├── polygone-app        # CLI — la porte d'entrée
├── polygone-core       # Le cœur — protocole neuronal
├── polygone-crypto     # ML-KEM-1024 · AES-256-GCM · Shamir · BLAKE3
├── polygone-network    # libp2p · Kademlia DHT · QUIC · WebRTC
├── polygone-msg        # Protocole de messagerie éphémère
├── polygone-msh        # Shell interactif
├── polygone-petals     # PETALS_NEURO — inférence LLM distribuée
├── polygone-brain      # Cerveau IA — personnalités simulées
├── polygone-shell      # Dashboard interactif
└── polygone-drive      # Stockage distribué chiffré (soon)
```

---

## PETALS_NEURO — Intelligence collective chiffrée

> *"L'intelligence n'est pas dans le cerveau individuel. C'est dans la synchronisation des neurones."*

PETALS_NEURO permet à des nœuds Polygone d'exécuter des modèles de langage massifs en collaboration — sans jamais exposer le modèle complet à un seul nœud.

Chaque couche du modèle est calculée par un nœud différent. Les hidden states voyagent chiffrés par ML-KEM entre chaque saut. Le réseau forme un **cerveau collectif**, résilient, et post-quantique.

```rust
use polygone_petals::{PetalsClient, InferenceParams};

let params = InferenceParams {
    model: "meta-llama/Llama-3-70b".to_string(),
    prompt: "Explique la conscience quantique.".to_string(),
    max_new_tokens: 100,
};

let response = client.generate(params).await?;
println!("{}", response);
```

---

## Le manifeste

> *En 2030, ton ordinateur quantique pourra ouvrir tous tes secrets.*
>
> *La cryptographie post-quantique n'est pas une option. C'est la seule façon de communiquer en 2026.*
>
> *Polygone n'est pas un produit. C'est une position.*
>
> *Rejoins le réseau.*

[Lis le manifeste complet →](POLYGONE_MANIFESTO.md)

---

## Statut des modules

| Module | Status |
|--------|--------|
| `polygone-core` | ✅ Stable |
| `polygone-crypto` | ✅ Stable — audité |
| `polygone-network` | ✅ Stable |
| `polygone-msg` | ✅ Stable |
| `polygone-msh` | ✅ Stable |
| `polygone-app` | ✅ Stable |
| `polygone-petals` | 🚧 PETALS_NEURO |
| `polygone-brain` | 🚧 Personnalités simulées |
| `polygone-shell` | 🚧 Dashboard interactif |

**Tests** : 19/19 passent ✅

---

## Contribuer

Polygone est open source — MIT. Chaque ligne est auditable.

```bash
git clone https://github.com/lvs0/Polygone.git
cd Polygone
cargo build --release
cargo test --all
```

[Lis CONTRIBUTING.md avant →](CONTRIBUTING.md)

---

## L'hexagone

En géométrie, l'hexagone est la forme qui utilise le **moins de matériau** pour enclose la plus grande surface. Optimal. Élégant.

Le vide au centre, c'est ce qu'on protège.

---

<div align="center">

**⬡ POLYGONE** — *Le système nerveux d'Internet*

*Il n'y a pas de Polygone. Il y a des milliers de polygones.*

[GitHub](https://github.com/lvs0/Polygone) · [Specs](SPEC.md) · [Manifeste](POLYGONE_MANIFESTO.md) · [PETALS_NEURO](docs/PETALS_NEURO.md)

</div>