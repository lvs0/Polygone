# ⬢ POLYGONE : GUIDE DE PASSAGE À L'ÉCHELLE (SCALING)

Ce document explique comment transformer ce service en un réseau mondial indestructible composé de dizaines de nœuds "Vapor".

## 1. Stratégie Multi-VPS (Gratuit)
Tu peux multiplier ton nœud sur plusieurs fournisseurs pour créer une "Grille de Puissance" :
- **Render** : (Déjà fait) Nœud de contrôle.
- **Oracle Cloud** : Offre "Always Free" avec 4 instances ARM (Très puissantes pour Polygone).
- **Fly.io / Railway / Northflank** : Excellents pour des petits nœuds de transit.

## 2. Configuration Technique (Variables)
Pour chaque nouveau nœud, utilise l'image Docker `ghcr.io/lvs0/polygone:latest` et configure les variables :

| Variable | Valeur | Description |
| :--- | :--- | :--- |
| `POLY_P2P_SEED` | `Une-Clé-Base64-32-Octets` | Unique pour chaque nœud (ton identité) |
| `RENDER_URL` | `https://ton-app.onrender.com` | Utilise ton lien public pour le "Pulse" |
| `BOOTSTRAP_ADDR` | `/dns4/polygone-one.onrender.com/tcp/443/wss/p2p/TON_ID` | L'adresse de ton 1er nœud maillé |

## 3. Résilience : Que se passe-t-il si un PC s'arrête ?
Polygone utilise le principe de la **Multi-Localisation**. Chaque fragment de donnée est répliqué sur 20 nœuds différents (K=20).
- Si un PC s'arrête brutalement : **Zéro perte**. Le réseau redirige automatiquement les requêtes vers les 19 autres en temps réel.
- Le réseau se "re-cicatrise" tout seul dès que le nœud est de nouveau en ligne.

## 4. Vision : Prêt de Puissance Intelligent
En connectant ton PC au réseau (variable copiée-collée), ton PC délègue le travail réseau lourd aux VPS gratuits. En retour, tu accumules du **Karma**. 
Plus tu as de Karma, plus le réseau te donne la priorité pour les calculs lourds, rendant ton utilisation locale fluide.

---
**L'information n'existe pas. Elle traverse.** ⬡
