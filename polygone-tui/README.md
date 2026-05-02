# Polygone TUI

**Terminal UI qui centralise tout.** Une seule commande, tout accessible.

## Installation

```bash
pip install -e .
# ou
bash install_polygone.sh
```

## Utilisation

```bash
# Lancer le TUI (TOUT depuis ici)
polygone-tui

# Raccourcis clavier
# q          Quitter
# 1          Accueil (statut réseau)
# 2          Identité (générer/afficher clés)
# 3          Messages (envoyer/recevoir)
# 4          Réseau (démarrer/arrêter nœud)
# 5          Paramètres (setup, config)
```

## Ce que le TUI centralise

| Fonction | Description |
|---|---|
| **Accueil** | Statut réseau, version, connecter/déconnecter |
| **Identité** | Générer clé, afficher clé publique, exporter |
| **Messages** | Envoyer message chiffré, recevoir et déchiffrer |
| **Réseau** | Démarrer/arrêter nœud, info, boost CPU, updates |
| **Config** | Setup wizard, afficher config, danger zone |

## Depuis le CLI Rust

```bash
polygone tui              # TUI Rust (ratatui)
polygone tui accueil      # TUI → onglet accueil
polygone tui parametres   # TUI → onglet paramètres
```

## Dépendances

- Python 3.8+
- `textual>=0.80.0`
- `rich>=13.0.0`
- `polygone` CLI (doit être dans PATH)

## Pour les développeurs

```bash
# Lancer depuis le source
python3 -m polygone_tui.app

# Mode debug
RUST_BACKTRACE=1 polygone-tui
```
