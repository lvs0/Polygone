# POLYGONE — Brand Book v0.1

> *« Post-quantique. Pair à pair. Local. »*

Polygone est la marque technique de l'**infrastructure de confidentialité post-quantique** dans l'écosystème NYX.

Ce brand book définit l'identité visuelle de Polygone, alignée sur le **NYX design system**.

---

## 1. Pourquoi ce design

- **Sobriété cinétique** : on ne te montre pas tout, mais ce qu'on te montre compte.
- **Vocabulaire post-quantique** : hexagones emboîtés, nœuds périphériques, dégradés cyan→violet→magenta. KEM, Shamir, mesh.
- **Pas de criardise** : fond noir profond, glows discrets, motion lente.

## 2. Promesse

- **Zéro confiance** : pas de serveur central.
- **Zéro télémétrie** : pas d'envoi réseau non sollicité.
- **Une commande** : `polygone` suffit — installer, contrôler, lancer.

## 3. Token stack

Hérite du **NYX brand system** (`tokens.css`). Les tokens Polygone préfixent `--pol-*` pour ne pas se mélanger avec NYX générique.

| Token group | Variable | Valeur | Usage |
|-------------|----------|--------|-------|
| Background | `--pol-bg` | `#05060a` | Tout le site |
| Cyan signature | `--pol-cyan` | `#00f5d4` | Actions, signal post-quantique |
| Violet crypto | `--pol-violet` | `#9d4edd` | Cards IA/crypto/data |
| Magenta scan | `--pol-magenta` | `#ff3d8c` | Alertes, temps réel |
| Gradient hero | `--pol-grad-main` | cyan→violet→magenta | Wordmark, boutons électriques |

## 4. Logo

- **SVG hexagonal** : `logo-polygone.svg` (3 hexagones concentriques + 6 nœuds périphériques + centre).
- **Center mark** : toujours afficher le mark avec un padding de protection = 12 % de sa taille.
- **Minimum size** : 32 px (en ligne). En print : 18 mm.
- **Ne jamais** : étirer, déformer, changer le gradient, ou placer sur fond sans contraste.

## 5. Typographie

- **Display** : Fraunces 300/500 italic — pour les titres et le hero.
- **Sans** : Space Grotesk 400/500/700 — UI et body.
- **Mono** : JetBrains Mono 400/700 — tags, signatures, code.

## 6. Motion

Héritée de NYX :

- Micro-interactions : 180 ms `ease-soft`
- Section transitions : 360 ms `ease-soft`
- Hero animations : 720–1400 ms `ease-soft`
- Hex mark : `pol-drift` 6 s (mouvement perpétuel lent)

**Règle** : un élément qui apparaît doit *venir de quelque part* (translate-Y + opacity). Jamais pop-in brutal.

## 7. Voix

- Français.
- Phrases courtes. Verbes simples.
- Jamais de mention "révolutionnaire", "game-changer", "next-gen".
- Toujours dire *ce qu'on sait* et *ce qu'on ne sait pas*.
- Le visiteur est *un pair*, pas un client.

## 8. Déclinaisons par produit

| Module | Accent | Page |
|--------|--------|------|
| **Polygone Core** | `--pol-cyan` | `/` |
| **Polygone Hide** | `--pol-magenta` | `/hide.html` |
| **Polygone Drive** | `--pol-violet` | `/drive.html` |
| **Polygone Mesh** | `--pol-cyan` | `/mesh.html` |
| **Polygone Brain** | `--pol-violet` | `/brain.html` |

## 9. Ce que Polygone n'est PAS

- ❌ Un VPN classique.
- ❌ Une blockchain.
- ❌ Un produit freemium avec tracking.
- ❌ Un concurrent de Mullvad ou Tailscale (philosophie opposée : pas de serveurs centralisés du tout).

## 10. Crédits

- Brand aligné sur NYX — Zoe & Lévy, 12 juin 2026.
- Adapté pour Polygone — Zoe, 16 juin 2026.

— Zoe & Lévy, 16 juin 2026 — Europe/Paris.
