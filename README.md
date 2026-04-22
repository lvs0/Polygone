# ⬡ POLYGONE v2.0

> **Messagerie privée post-quantique pour tous** — Sécurisée, éphémère, invisible  
> *"L'information n'existe pas. Elle traverse."*

[![Version](https://img.shields.io/badge/version-2.0.0-blue)](https://github.com/lvs0/Polygone/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-nightly-orange)](https://www.rust-lang.org)
[![Post-Quantum](https://img.shields.io/badge/security-post--quantum-green)](docs/SECURITY.md)

---

## 🌟 Qu'est-ce que POLYGONE ?

**POLYGONE** est une application de messagerie privée qui utilise la cryptographie **post-quantique** pour protéger vos communications contre les ordinateurs quantiques futurs et la surveillance actuelle.

Contrairement aux applications classiques qui chiffrent seulement le **contenu**, POLYGONE rend la communication elle-même **inobservable** grâce à une architecture innovante inspirée du comportement des vagues.

### ✨ Fonctionnalités

| Fonctionnalité | Description |
|---|---|
| 🔐 **Cryptographie Post-Quantique** | ML-KEM-1024 (NIST FIPS 203) + AES-256-GCM |
| 👻 **Messages Éphémères** | Auto-destruction après lecture, aucune trace persistante |
| 🌊 **Architecture "Vague"** | Les messages deviennent un état distribué temporaire |
| 📱 **Interface Graphique** | Application desktop intuitive (optionnelle) |
| 💻 **CLI Puissante** | Interface en ligne de commande pour power users |
| 🔗 **Partage QR Code** | Échange facile de clés publiques via QR codes |
| 🛡️ **Sécurité Mémoire** | `#![forbid(unsafe_code)]` + zeroize automatique |

---

## 🚀 Installation Rapide

### Option 1 : Script Automatique (Recommandé)

```bash
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash
```

Puis vérifiez l'installation :
```bash
polygone --version
```

### Option 2 : Compilation Manuelles

```bash
# Prérequis : Rust nightly
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default nightly

# Cloner et compiler
git clone https://github.com/lvs0/Polygone.git
cd Polygone
cargo build --release

# Installer
cargo install --path .
```

### Option 3 : Avec Interface Graphique

```bash
cargo install --path . --features gui
```

---

## 📖 Utilisation

### Première Configuration

1. **Générer vos clés** :
```bash
polygone keygen
```

2. **Partager votre clé publique** avec vos contacts (ou scannez le QR code dans l'interface graphique)

3. **Envoyer un message** :
```bash
polygone send --peer <clé_publique> --message "Votre message secret"
```

### Commandes Principales

| Commande | Description |
|---|---|
| `polygone keygen` | Générer une paire de clés post-quantique |
| `polygone send` | Envoyer un message chiffré |
| `polygone receive` | Recevoir et déchiffrer un message |
| `polygone node start` | Démarrer un nœud relais |
| `polygone status` | Voir l'état du réseau |
| `polygone self-test` | Vérifier l'installation |
| `polygone tui` | Interface terminal interactive |
| `polygone-gui` | Lancer l'interface graphique |

---

## 🔬 Comment Ça Marche ?

### Le Problème : La Fuite des Métadonnées

Les systèmes traditionnels cachent le **contenu** mais pas l'**existence** de la communication. Un observateur peut voir :
- Qui communique avec qui
- Quand la communication a lieu
- La taille des données échangées

### La Solution POLYGONE : L'Approche "Vague"

Comme une vague dans l'eau n'a pas de molécules propres (les molécules oscillent sur place, seul le **motif** se propage), POLYGONE transforme chaque message en un **état computationnel distribué** :

```
┌─────────────────────────────────────────────────────────────┐
│  ÉTAPE 1 : SYNCHRONISATION (hors-bande, une seule fois)     │
│  Alice & Bob échangent une clé ML-KEM-1024                  │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│  ÉTAPE 2 : NAISSANCE DU RÉSEAU                              │
│  7 nœuds éphémères dérivés de la clé partagée               │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│  ÉTAPE 3 : FRAGMENTATION                                    │
│  Message → AES-256-GCM → Shamir 4-of-7 → 7 fragments        │
│  Aucun nœud ne voit plus d'un fragment                      │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│  ÉTAPE 4 : DISSOLUTION                                      │
│  Bob reconstruit avec 4+ fragments → message déchiffré      │
│  Tous les fragments et clés sont détruits                   │
│  L'échange n'a jamais eu lieu                               │
└─────────────────────────────────────────────────────────────┘
```

### Stack Cryptographique

| Composant | Technologie | Rôle |
|---|---|---|
| KEM | ML-KEM-1024 (FIPS 203) | Échange de clé post-quantique |
| Signature | Ed25519 (ML-DSA ready) | Authentification |
| Chiffrement | AES-256-GCM | Protection du contenu |
| Secret Sharing | Shamir 4-of-7 | Fragmentation sécuritaire |
| KDF | BLAKE3 | Dérivation de clés |

---

## 🎯 Cas d'Usage

### Pour les Particuliers
- Messages privés sans historique
- Partage de documents sensibles
- Communications familiales sécurisées

### Pour les Professionnels
- Avocats : protection du secret professionnel
- Journalistes : sources anonymes
- Médecins : données patients confidentielles
- Entreprises : R&D sensible, secrets industriels

### Pour les Développeurs
- Intégration dans vos applications
- Audit du code source ouvert
- Contribution à un projet open-source

---

## 📦 Téléchargements

| Plateforme | Binaire | Taille |
|---|---|---|
| Linux x86_64 | `polygone-linux-x64` | ~12 MB |
| macOS ARM64 | `polygone-macos-arm64` | ~14 MB |
| Windows x64 | `polygone-windows-x64.exe` | ~15 MB |

*[Voir tous les téléchargements sur la page des Releases](https://github.com/lvs0/Polygone/releases)*

---

## 🤝 Contribuer

POLYGONE est **open-source** sous licence MIT. Contributions bienvenues !

```bash
# Fork et clone
git clone https://github.com/votre-user/Polygone.git
cd Polygone

# Créer une branche
git checkout -b ma-fonctionnalite

# Tester
cargo test
cargo run -- self-test

# Soumettre
git commit -m "Ajout: description"
git push origin ma-fonctionnalite
```

### Bonnes Premières Issues
- 🐛 Correction de bugs
- 📝 Amélioration documentation
- 🎨 Interface utilisateur
- 🔒 Tests de sécurité

---

## 📚 Documentation

- [📖 Architecture Technique](docs/ARCHITECTURE.md)
- [🔐 Spécifications de Sécurité](docs/SECURITY.md)
- [📜 Protocole Détaillé](docs/PROTOCOL.md)
- [🚀 Guide de Déploiement](SCALING_GUIDE.md)
- [❓ FAQ](#faq)

---

## ❓ FAQ

### Q: Est-ce compatible avec Signal/Telegram ?
**R:** Non, POLYGONE est un protocole indépendant. Il offre des garanties différentes (post-quantique + inobservabilité).

### Q: Dois-je faire confiance aux nœuds relais ?
**R:** Non. Grâce au secret sharing de Shamir, aucun nœud individuel ne peut accéder à votre message. Il faudrait 4 nœuds qui collaborent malicieusement.

### Q: Que se passe-t-il si je perds mes clés ?
**R:** Elles sont irrécupérables. C'est intentionnel pour la sécurité. Sauvegardez-les dans un gestionnaire de mots de passe.

### Q: Est-ce légal en France ?
**R:** Oui. POLYGONE utilise des algorithmes standards (NIST FIPS 203/204). Nous documentons nos choix techniques pour la CNIL.

### Q: Puis-je héberger mon propre nœud ?
**R:** Oui ! Voir le [Guide de Déploiement](SCALING_GUIDE.md).

---

## 🛡️ Modèle de Menace

| Attaquant | Capacité | Protection POLYGONE |
|---|---|---|
| FAI / ISP | Voit le trafic réseau | ✅ Fragments chiffrés, métadonnées brouillées |
| État-nation | Surveillance de masse | ✅ Post-quantique, pas de logs |
| Ordinateur Quantique | Casser RSA/ECC | ✅ ML-KEM-1024 résistant à Shor |
| Nœud Malicieux | Intercepter fragments | ✅ Shamir : 1 fragment = 0 information |
| Attaque Physique | Vol de device | ✅ Clés en mémoire volatile, zeroized |

---

## 📜 Licence

**MIT License** — Utilisation libre, modification et distribution autorisées.

```
Copyright © 2025-2026 Lévy & Hope Collective

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files...
```

---

## 🙏 Remerciements

- **NIST** pour les standards FIPS 203/204
- **Rust Community** pour un langage sécurisé
- **Hope Collective** pour le soutien
- **Tous les contributeurs** open-source

---

<div align="center">

**⬡ POLYGONE** — *L'information n'existe pas. Elle traverse.*

[Site Web](https://polygone.dev) • [GitHub](https://github.com/lvs0/Polygone) • [Discord](#) • [Documentation](docs/)

**Projet français** · **l-vs** · **collectif Hope** (*by Hope*)

</div>
