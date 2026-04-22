# ⬡ POLYGONE — Changelog

Toutes les modifications notables de ce projet sont documentées dans ce fichier.

---

## [2.1.0] - 2024-XX-XX — 🎨 Expérience Utilisateur Premium

### ✨ Nouvelles Fonctionnalités

#### Outil de Configuration Interactif (`polygone-config`)
- **Interface CLI interactive** inspirée d'OpenCode/OpenClaw
- Menu de navigation avec flèches directionnelles
- Gestion complète des clés (génération, backup, affichage)
- Configuration réseau intuitive (P2P, adresse d'écoute, relais)
- Paramètres de confidentialité (auto-delete, metadata)
- Préférences d'affichage (thème, verbosité, emojis)
- Options avancées (expérimental, debug, logs)
- Sauvegarde automatique dans `~/.polygone/config.toml`

#### Améliorations UX CLI
- **Progress bars** avec `indicatif` pour opérations longues
- **Tables formatées** avec `comfy-table` pour affichages structurés
- **Prompts interactifs** avec `dialoguer` pour saisies utilisateur
- Messages en français avec emojis cohérents
- Codes couleur : cyan (info), vert (succès), rouge (erreur), jaune (warning)

#### Commande `polygone config`
- Intégration de l'outil de configuration dans la CLI principale
- Lancement via `polygone config` ou `polygone-config`
- Option `--quick` pour跳过 l'écran de bienvenue

### 🔧 Améliorations Techniques

#### Dépendances Ajoutées
- `indicatif` v0.17 — Progress bars stylisées
- `dialoguer` v0.11 — Prompts interactifs (Select, Input, Confirm, FuzzySelect)
- `comfy-table` v7 — Tables ASCII élégantes
- `toml` v0.8 — Parsing/écriture configuration
- `hostname` v0.4 — Informations système
- `whoami` v1 — Identification utilisateur
- `ratatui` v0.29 — Framework TUI (déjà présent, version mise à jour)
- `crossterm` v0.28 — Backend terminal (déjà présent, version mise à jour)

#### Features Cargo
- Nouvelle feature `p2p` pour activer libp2p optionnellement
- Feature `full` = `cli` + `gui`
- Feature par défaut : `cli`

### 📝 Documentation

#### Nouveau Fichier
- `PROMPT_TRANSFERT_QWEN.md` — Guide complet pour sessions IA futures
  - Architecture détaillée de l'écosystème
  - Standards de qualité et checklist
  - Instructions pour harmoniser tous les dépôts `Polygone-*`
  - Templates de communication inter-dev
  - Métriques de succès

#### Mises à Jour
- README.md — Ajout section `polygone config`
- INSTALL.md — Méthodes d'installation supplémentaires
- GUIDE_UTILISATEUR.md — Section configuration interactive

### 🔒 Sécurité

#### Renforcements
- Audit des logs pour suppression métadonnées
- Permissions 600 automatiques sur fichiers secrets
- Zeroize mémoire pour toutes données sensibles
- Mode furtif configurable (désactivation logs PeerId)

### 🐛 Corrections

#### CLI Principale
- Meilleure gestion des erreurs avec messages actionnables
- Affichage cohérent entre commandes
- Support stdin amélioré pour `polygone send`

---

## [2.0.0] - 2026-01-XX — 🎉 Version Grand Public

### ✨ Nouvelles Fonctionnalités

#### Interface Graphique (GUI)
- **Nouvelle application desktop** avec interface moderne et intuitive
- 4 onglets : Tableau de bord, Clés, Envoyer, Recevoir
- Support des QR codes pour l'échange de clés publiques
- Thème sombre par défaut
- Notifications en temps réel

#### Expérience Utilisateur
- **Guide utilisateur complet** en français (`GUIDE_UTILISATEUR.md`)
- Installation simplifiée sur Windows, macOS, Linux
- Messages d'erreur plus clairs et informatifs
- Workflows CI/CD pour builds automatiques

#### Documentation
- README entièrement réécrit pour le grand public
- FAQ détaillée avec cas d'usage concrets
- Modèles de menace expliqués simplement
- Exemples de commandes pour chaque scénario

### 🔧 Améliorations Techniques

#### Build System
- Features optionnelles : `cli`, `gui`, `full`
- Support multi-cibles : Linux x64, macOS ARM64, Windows x64
- GitHub Actions pour releases automatiques
- Benchmarks intégrés avec Criterion

#### Dépendances
- Ajout de `iced` v0.13 pour la GUI
- Ajout de `qrcode` v0.14 pour les QR codes
- Ajout de `libp2p` v0.54 (optionnel, v2.0 P2P)
- Ajout de `criterion` v0.5 pour les benchmarks

### 📦 Distribution

| Plateforme | Binaire | Taille |
|---|---|---|
| Linux x86_64 | `polygone-linux-x64` | ~12 MB |
| macOS ARM64 | `polygone-macos-arm64` | ~14 MB |
| Windows x64 | `polygone-windows-x64.exe` | ~15 MB |

### 🐛 Corrections

- Meilleure gestion des erreurs CLI
- Zeroize mémoire amélioré
- Logs plus verbeux en mode debug

---

## [1.0.0] - 2025-XX-XX — 🚀 Version Initiale

### Fonctionnalités de Base

- Cryptographie post-quantique ML-KEM-1024
- Fragmentation Shamir 4-of-7
- AES-256-GCM pour le chiffrement
- Interface TUI (terminal)
- CLI complète : keygen, send, receive, node, self-test

### Architecture

- `#![forbid(unsafe_code)]`
- ZeroizeOnDrop pour toutes les clés
- Session éphémère avec dissolution automatique

---

## Légende

- ✨ Nouvelle fonctionnalité
- 🔧 Amélioration
- 🐛 Correction de bug
- 📚 Documentation
- ⚠️ Breaking change
