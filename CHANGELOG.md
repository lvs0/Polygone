# ⬡ POLYGONE — Changelog

Toutes les modifications notables de ce projet sont documentées dans ce fichier.

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
