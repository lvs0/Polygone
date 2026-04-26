# Polygone

**v1.0.0 | MIT | No token | No telemetry**  
*Privacy par conception, pas par politique*

## Comment ça marche

Polygone chiffre ton message avec ML-KEM + AES-256, le disperse en fragments via Shamir, et chaque fragment s'évapore en 30s.

```
Toi → [chiffre] → fragments → nœuds distribués → [efface]
```

## Pour commencer

```bash
# 1. Installer
curl -fsSL https://polygone.network/install.sh | bash

# 2. Configurer
polygone setup

# 3. Envoyer un message
polygone keygen                          # génère ta clé
polygone send <clé_publique> <message>   # envoie
polygone receive                         # reçoit
```

## Commandes

| Commande | Description |
|---|---|
| `polygone keygen` | Génère ta paire de clés |
| `polygone send <clé> <msg>` | Envoie un message chiffré |
| `polygone receive` | Récupère tes messages |
| `polygone status` | Affiche l'état du réseau |
| `polygone node` | Lance un nœud relay |

## Sécurité

- **ML-KEM** — Chiffrement post-quantique (résistant aux ordinateurs quantiques)
- **AES-256-GCM** — Chiffrement symétrique militaire
- **Shamir Secret Sharing** — Ton message est splité en N parts, aucune seule suffit

Zéro telemetry. Zéro tracking. Zéro tierce partie.

## Aide

- **Discord:** https://discord.gg/polygone
- **GitHub:** https://github.com/lvs0/Polygone

---

*License: MIT | Build your own node, own your privacy*