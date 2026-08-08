# ⬡ Polygone

**Réseau de transit éphémère post-quantique, écrit en Rust.**

Un message chiffré est découpé en fragments (Shamir 4-of-7), fait transiter par un relais qui ne voit que du routage, puis est reconstruit et déchiffré chez le destinataire. Rien n'est persisté.

Post-quantique : ML-KEM-1024 (FIPS 203) · AES-256-GCM · BLAKE3 · Shamir 4-of-7 · Ed25519.
Licence MIT. Pas de compte, pas de télémétrie, pas de serveur central.

---

## État du projet

**v1.0.0 — livré (avril 2026).** Le protocole local complet fonctionne de bout en bout : génération de clés, chiffrement, fragmentation, reconstruction, déchiffrement — 19 tests verts.

| Module | État |
|--------|------|
| `polygone-core` | ✅ Stable |
| `polygone-crypto` | ✅ Stable (audité) |
| `polygone-network` | ✅ Stable |
| `polygone-msg` | ✅ Stable |
| `polygone-msh` | ✅ Stable |
| `polygone-app` | ✅ Stable |
| `polygone-petals` | 🚧 En cours (IA locale) |
| `polygone-brain` | 🚧 En cours |
| `polygone-shell` | 🚧 En cours |

Ce qui est marqué ✅ est testé et utilisable. Ce qui est 🚧 n'est pas encore promis.

---

## Installation

```bash
git clone https://github.com/lvs0/Polygone.git
cd Polygone
cargo build --release
```

## Démarrage rapide

```bash
# 1. Générer ta paire de clés (ML-KEM-1024 + Ed25519, chmod 600)
polygone keygen

# 2. Envoyer un message chiffré (démo Alice→Bob ou clé réelle)
polygone send --peer-pk demo --message "salut"
polygone send --peer-pk <hex|fichier> --message "message secret"

# 3. Recevoir et reconstruire
polygone receive

# 4. Vérifier que tout fonctionne
polygone self-test

# 5. Node + TUI
polygone node start
polygone tui
```

`polygone status` montre tes clés, sessions et l'état du nœud.

## Ce que ça fait, concrètement

1. **Chiffre** — ML-KEM-1024 encapsule la clé de session, AES-256-GCM chiffre le message (nonce frais par message).
2. **Fragmente** — Shamir 4-of-7 : le message n'existe nulle part en entier.
3. **Transite** — le relais route les fragments sans les lire ni les stocker.
4. **Reconstruit** — le destinataire réunit 4 fragments sur 7 et déchiffre.

Ce que ça ne fait pas : pas de compte, pas de persistance, pas de cloud. Les fragments meurent.

## Sécurité

- Le modèle de menace est documenté (voir `docs/` et `THREAT_MODEL.md`).
- La crypto est implémentée avec les crates RustCrypto (ml-kem, ed25519-dalek, aes-gcm, blake3) — pas de primitives maison.
- ML-DSA (FIPS 204) : chemin de migration documenté pour les signatures.

## Documentation

- [`CHANGELOG.md`](CHANGELOG.md) — historique des versions
- [`docs/DESIGN_PHILOSOPHY.md`](docs/DESIGN_PHILOSOPHY.md) — choix d'architecture
- [`docs/ECOSYSTEM_MAP.md`](docs/ECOSYSTEM_MAP.md) — les projets voisins (CLI, Drive, Hide, Server, Shell, Petals, Brain)
- [`CONTRIBUTING.md`](CONTRIBUTING.md) — comment contribuer

## Licence

MIT. Chaque ligne est auditable.

---

⬡ Polygone — *l'information n'existe pas, elle traverse.*
