# ⬡ POLYGONE — BRAND IDENTITY

> *Le système nerveux d'Internet.*

---

## Positionnement

**Polygone n'est pas un outil.** C'est un organisme. Un réseau neuronal distribué qui fonctionne comme un cerveau collectif — chaque nœud est un neurone, chaque message est une synapse, chaque attaque est guérie.

Le positionnement n'est PAS "messagerie sécurisée" ou "alternative à Signal". C'est **une nouvelle forme d'intelligence distribuée**, chiffrée contre les machines quantiques.

**Target** : développeurs, chercheurs IA, crypto-anarchistes, gens qui comprennent que la vie privée est un droit pas un luxe.

---

## Personality

### Ce que Polygone EST
- **Vivante** — le réseau respire, grandit, se répare
- **Provocatrice** — on dit ce que les GAFAM cachent
- **Précise** — la crypto c'est du sérieux, pas du marketing
- **Inclusive** — open-source, tout le monde peut contribuer

### Ce que Polygone n'est PAS
- ~~Corporate~~ — pas de jargon vide, pas de "we value your privacy"
- ~~Froid~~ — on a une personnalité, pas juste du code
- ~~Apple-like~~ — propre mais distant. Polygone est chaleureux mais sérieux.
- ~~Crypto-bros~~ — pas de token, pas de NFT, pas de bullshit

### Le ton
- Direct : on dit les choses sans tourner
- Intelligent : on suppose que le lecteur est intelligent
- Surnaturel : "le réseau est vivant" — on y croit vraiment
- Anti-GAFAM quand c'est pertinent : "RSA sera mort en 2030. Polygone non."

---

## Palette de couleurs

```
╔═══════════════════════════════════════════════════════════╗
║  COULEUR          HEX       UTILISATION                    ║
╠═══════════════════════════════════════════════════════════╣
║  Black Void       #0a0a0f   Fond principal, sécurité       ║
║  Deep Black       #050508   Profondeur, headers            ║
║  Violet Neural    #7c3aed   Synapse, neuro, vivant         ║
║  Violet Light     #a78bfa   Accents, hovers                ║
║  Cyan Quantum     #22d3ee   Précision, quantum, tech        ║
║  Amber Warning    #f59e0b   Alerts, Shamir, attention      ║
║  Green Healthy    #22c55e   Status OK, auto-guérison       ║
║  Red Attack       #ef4444   Erreurs, attaques              ║
║  White Ghost      #f8fafc   Texte principal (dark bg)       ║
║  Gray Neutral     #94a3b8   Texte secondaire               ║
╚═══════════════════════════════════════════════════════════╝
```

### Règles d'usage
- **Fond toujours noir ou très sombre** — le violet et cyan "émergent" du noir
- **Jamais de blanc pur** — trop Apple, trop corporate
- **Violet = vivant, Cyan = précis** — violet pour le bio, cyan pour la tech
- **Amber = attention** — pour Shamir et les moments critiques

---

## Typographie

### Fonts

```css
/* Display — pour les titres */
@import url('https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@300;400;500;600;700&display=swap');

/* Mono — pour le code et la crypto */
@import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@300;400;500;700&display=swap');

/* Body — texte principal */
font-family: 'Space Grotesk', system-ui, sans-serif;
```

### Hiérarchie

| Niveau | Font | Size | Weight | Couleur |
|--------|------|------|--------|---------|
| H1 | Space Grotesk | 4rem | 700 | White |
| H2 | Space Grotesk | 2.5rem | 600 | White |
| H3 | Space Grotesk | 1.5rem | 600 | Violet Light |
| Body | Space Grotesk | 1rem | 400 | Gray Neutral |
| Code | JetBrains Mono | 0.9rem | 400 | Cyan |
| Accent | Space Grotesk | 1.1rem | 500 | Amber |

---

## Icônes et Symboles

### Le Logo

Le logo est une **structure hexagonale** — le polygone régulier le plus stable. Chaque côté représente un pilier de Polygone :
- Cryptographie post-quantique
- Partage de clé distribué (Shamir)
- Auto-guérison (Byzantine fault tolerance)
- Réseau neuronal distribué
- Open-source à 100%
- Sans serveur central

```
        ⬡
       /  \
      /    \
     /      \
    \       /
     \     /
      \   /
       ⬡⬡⬡

  Ou version simplifiée :

    ⬡────⬡
   /      \
  ⬡        ⬡
   \      /
    ⬡────⬡
```

### Icônes par domaine

```
Cryptographie :     🔐 ou ⬡ avec halo violet
Quantum :           ⚛ ou ◆ cyan
Shamir :            🔑 ou ⬡ amber
Auto-guérison :     🛡 ou ⬡ green
Neural :            🧠 ou ◇ violet
Network :           🌐 ou ⬡ cyan
Rust :              🦀 ou ⬡ orange
```

### Badges Shield

```
┌──────────────────────────┐
│  ⬡ POLYGONE              │
│  ──────────────────      │
│  Post-Quantum Ready      │
│  NIST Standard           │
└──────────────────────────┘
```

---

## Animations

### Principe

Le réseau est **vivant**. Les animations doivent refléter :
- **Pulse** : activité continue, le réseau "respire"
- **Propagation** : les données circulent comme des influx nerveux
- **Self-heal** : quand un nœud est réparé, une animation de "guérison"
- **Quantum glitch** : effet subtil pour rappeler la menace quantique

### CSS Keyframes

```css
/* Pulse neural — respiration continue */
@keyframes neural-pulse {
  0%, 100% { 
    opacity: 0.7;
    filter: drop-shadow(0 0 8px #7c3aed);
  }
  50% { 
    opacity: 1;
    filter: drop-shadow(0 0 20px #7c3aed);
  }
}

/* Propagation — données en transit */
@keyframes data-propagate {
  0% { transform: translateX(-100%); opacity: 0; }
  50% { opacity: 1; }
  100% { transform: translateX(100%); opacity: 0; }
}

/* Self-heal — réparation */
@keyframes self-heal {
  0% { transform: scale(1); filter: brightness(1); }
  25% { transform: scale(1.1); filter: brightness(0.5) hue-rotate(180deg); }
  50% { transform: scale(0.9); filter: brightness(2); }
  100% { transform: scale(1); filter: brightness(1); }
}

/* Quantum glitch — menace quantique */
@keyframes quantum-glitch {
  0%, 100% { clip-path: inset(0 0 0 0); }
  20% { clip-path: inset(20% 0 60% 0); transform: translateX(-2px); }
  40% { clip-path: inset(60% 0 10% 0); transform: translateX(2px); }
  60% { clip-path: inset(40% 0 40% 0); transform: translateX(-1px); }
  80% { clip-path: inset(10% 0 80% 0); transform: translateX(1px); }
}
```

---

## Landing Page — Structure

### Hero Section
- **Background** : animation neural network en continue (CSS pur)
- **Titre** : "Le système nerveux d'Internet" en violet
- **Sous-titre** : "Chaque nœud est un neurone. Chaque message est une synapse. Chaque attaque est guérie."
- **CTA** : bouton violet avec glow, "Commencer" → docs
- **Stats live** : "X nœuds actifs • Y messages/seconde • Z pays"

### Section "Pourquoi"
- 3 cards avec icons animés
- ML-KEM post-quantique
- Shamir 4-of-7
- Auto-guérison Byzantine

### Section "Comment ça marche"
- Diagramme animé du réseau
- Code snippet qui montre l'envoi d'un message chiffré
- Terminal interactif (optionnel)

### Section "PETALS_NEURO"
- Le protocole neuronal
- "Intelligence is not individual. It emerges from connection."
- Vidéo/animation du transfert d'état neuronal

### Footer
- GitHub + contributeurs
- "Made with 🦀 by l-vs"
- License MIT

---

## Voices — Comment on écrit

### ✅ Correct
- "Polygone chiffre contre les machines quantiques."
- "Le réseau guérit tout seul."
- "RSA sera mort en 2030. Nous, non."
- "Chaque nœud est un neurone."

### ❌ Incorrect
- ~~"Polygone est une solution de messagerie sécurisée..."~~
- ~~"Nous valorisons votre vie privée..."~~
- ~~"L'équipe de Polygone est fière de..."~~
- ~~"Contactez-nous pour plus d'informations."~~

---

## Anti-patterns — Ce qu'on ne fait JAMAIS

1. **Pas de "privacy by design"** — on dit "chiffré post-quantique"
2. **Pas de "we"* — on dit "Polygone" ou "nous"
3. **Pas de testimonials fake** — "Le meilleur outil" etc.
4. **Pas de "contact us"** — le GitHub est suffisant
5. **Pas de roadmap vague** — soit c'est fait, soit c'est "en cours", soit c'est "planned"
6. **Pas de crypto-bros speak** — pas de "HODL", pas de "to the moon"
7. **Pas de jargon inutile** — si "chiffré de bout en bout" suffit, on n'ajoute pas "E2EE"

---

## Assets à créer

- [ ] **Logo SVG** — version colorée + monochrome + favicon
- [ ] **Hero animation** — neural network en CSS/JS
- [ ] **Icons** — set complet pour chaque feature
- [ ] **Diagrammes** — architecture + flux + Shamir
- [ ] **Badges shields** — ready/secure/quantum/... 
- [ ] **Polygone app mockup** — UI du client
- [ ] **PETALS_NEURO visualization** — neural state transfer

---

*Brand document — Polygone*
*Version 1.0 — 19 juin 2026*
*Maintainer : Zoe*