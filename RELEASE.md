# Polygone Network — v1.0.0

> **Premier lancement public** — 27 avril 2026

---

## Ce que c'est

**Polygone** est un réseau de messagerie éphémère post-quantique. Tes messages existent le temps de traverser le réseau, puis s'évaporent. Personne ne peut prouver qu'une conversation a eu lieu.

- **ML-KEM-1024** — Échange de clés résistant aux ordinateurs quantiques
- **AES-256-GCM** — Chiffrement symétrique militaire
- **Shamir 4-of-7** — Message dispersé en fragments, aucun nœud ne voit le message complet
- **Effacement 30s** — Les fragments expirent automatiquement
- **Zéro telemetry** — Aucune donnée collectée

---

## Installation (30 secondes)

```bash
# Linux / macOS / Windows (WSL)
curl -fsSL https://polygone.network/install.sh | bash

# Ou depuis le source
git clone https://github.com/lvs0/Polygone.git
cd Polygone && cargo build --release
```

---

## Démarrage rapide

```bash
polygone keygen                    # Génère ta clé d'identité
polygone setup                     # Assistant de configuration
polygone send <clé> <message>      # Envoie un message
polygone receive                   # Reçois un message
polygone tui                       # Interface interactive
```

---

## Nouveautés v1.0.0

| Feature | Status |
|---|---|
| Crypto ML-KEM-1024 + AES-256 | ✅ |
| Shamir Secret Sharing (4-of-7) | ✅ |
| CLI complet (7 commandes) | ✅ |
| TUI Python interactif | ✅ |
| TUI Rust (ratatui) | ✅ |
| Setup wizard interactif | ✅ |
| Node boost CPU | ✅ |
| Auto-update | ✅ |
| Tests: 24/24 PASS | ✅ |
| Self-test: 5/5 PASS | ✅ |

---

## Écosystème

| Module | Status |
|---|---|
| Polygone Core | ✅ v1.0.0 |
| Polygone Drive | 🚧 En développement |
| Polygone Hide | 🚧 En développement |
| Polygone Petals | 🚧 En développement |
| Polygone Shell | 🚧 En développement |
| Polygone Brain | 🚧 En développement |

---

## Équipe

- **Créateur:** Lévy Verpoort Scherpereel — polygone@proton.me
- **License:** MIT — Aucun investisseur, aucun token, aucune telemetry
- **GitHub:** https://github.com/lvs0/Polygone

---

*Polygone: L'information n'existe pas. Elle traverse.*
