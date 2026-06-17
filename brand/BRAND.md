# ⬡ POLYGONE — Brand Guide

> **Version:** 1.0.0 — 2026-06-18
> **Author:** Lévy
> **License:** MIT

---

## Brand Concept

**Polygone** (from Greek: πολύγωνον, *many angles*) is a post-quantum privacy network. The brand must embody three tensions simultaneously:

1. **Geometric precision** — hexagon, lines, structure
2. **Quantum void** — information does not exist; it drifts
3. **Distributed vitality** — 7 nodes, 7 fragments, 7 angles

The hexagon is the atomic symbol of Polygone. It is not decorative. It represents the Shamir secret sharing (7 fragments, 4 required to reconstruct), the geometric convergence of distributed nodes, and the mathematical state that becomes the message.

---

## Logo

### Primary Logo (`brand/logo-main.svg`)

Full-color logo with quantum particle effect. Use on:
- Hero sections
- Landing pages
- Print materials
- Presentation decks

### Logomark Only (`brand/favicon.svg`)

Hexagon only, no particles. Use for:
- Browser tab (favicon)
- App icons
- Minimal contexts where the hexagon is enough

### Clear Space

Minimum clear space = **1 hexagon width** on all sides.

Do not place the logo on backgrounds with contrast lower than 4.5:1.

### Color Variants

| Variant | When to Use |
|---------|-------------|
| Full logo (SVG) | Digital, dark backgrounds |
| Favicon SVG | Browser tabs, mobile home screen |
| Hexagon outline only | Watermarks, subtle branding |

---

## Color Palette

```
Primary Background    #04040c    (deep space black — never pure #000)
Secondary Background  #07071a    (elevated surface)
Card Background       #0a0a1f    (content containers)
Primary Blue          #00c8ff    (the core brand color — electric cyan)
Blue Dim              #0090cc    (secondary interactions)
Blue Deep             #0055aa    (borders, accents)
Violet Accent         #8855ff    (AI / quantum / advanced features)
Border                rgba(0, 200, 255, 0.12)
Glow (subtle)         rgba(0, 200, 255, 0.15)
Glow (strong)         rgba(0, 200, 255, 0.30)
```

### Text Colors

```
Body Text             #dde0f0    (primary content)
Dim Text              #6b6f8a    (secondary, captions, placeholders)
Blue Text             #00c8ff    (links, emphasis, calls-to-action)
```

### Usage Rules

- **Never** use `#000` or `#fff` as background/text — always desaturate slightly
- **Never** use the brand blue as body text (only for CTAs and accents)
- The background is **deep space**, not midnight — it has a blue-purple undertone

---

## Typography

```
Display / Headlines:  Orbitron (Google Fonts)
                      Weight: 700–900
                      Tracking: 0.04–0.15em
                      Use: section titles, logo text, CLI names

Monospace:            Share Tech Mono (Google Fonts)
                      Use: code blocks, install commands, technical data

Body:                 Inter
                      Weight: 300–400
                      Line-height: 1.7–1.8
                      Use: paragraphs, descriptions
```

### Font Stack Fallbacks

```css
font-family: 'Orbitron', 'Share Tech Mono', monospace;  /* display */
font-family: 'Share Tech Mono', 'Courier New', monospace;  /* mono */
font-family: 'Inter', system-ui, sans-serif;  /* body */
```

### Scale

```
Hero Title:           clamp(2.4rem, 6vw, 4.5rem)  — Orbitron 900
Section Title:        clamp(1.8rem, 4vw, 2.8rem)  — Orbitron 700
Card Title:           0.85–0.9rem                  — Orbitron 700
Body:                 1rem–1.05rem                 — Inter 300
Code / Mono:          13–14px                      — Share Tech Mono
Label / Badge:        10–11px                      — Share Tech Mono, uppercase
```

---

## Visual Principles

### 1. Hexagons are not decoration

The hexagon grid (as background) and the hexagon logo are the identity mark. Every visual decision should be traceable back to the hexagon or the void (the "information that does not exist").

**Do:** Use hexagons as section dividers, as the logo, as bullet points.
**Don't:** Use random gradients or abstract shapes that have nothing to do with the hex structure.

### 2. Glow is information

The electric cyan glow is not "futuristic" decoration. It represents the quantum state — the wave function before collapse. Use glow sparingly: it should highlight, not flood.

**Do:** Glow on the logo, on active states, on hover.
**Don't:** Glow on entire sections or backgrounds (only subtle radial gradients centered on content).

### 3. The void is part of the message

The black hexagon at the center of the logo (the "void") represents that information is absent until reconstructed. This concept should appear in visual storytelling: black backgrounds, deep spaces, not "clean white SaaS."

### 4. Precision over spectacle

Every element should be traceable to a concept. No random particles, no animated gradients for the sake of movement. If it doesn't serve the concept, it doesn't exist.

---

## What NOT to Do

- **Don't** use gradients that go from blue to purple to pink — that's AI aesthetic, not Polygone
- **Don't** use pure white (`#fff`) on pure black (`#000`) — desaturate both
- **Don't** add glowing particles to every section — one at a time, with intent
- **Don't** use Roboto, Arial, or system-ui for display text — Orbitron only
- **Don't** use the logo on a light background without a dark version
- **Don't** stretch or distort the hexagon logo — it has a precise aspect ratio
- **Don't** use more than 3 colors in any composition (deep space + blue + white accent)

---

## Component Patterns

### Badge / Status Tag

```html
<span style="
  padding: 4px 14px;
  border: 1px solid rgba(0, 200, 255, 0.2);
  border-radius: 100px;
  font-family: 'Share Tech Mono';
  font-size: 11px;
  color: #00c8ff;
  background: rgba(0, 200, 255, 0.04);
">
  ● post-quantum · ephemeral
</span>
```

### Install Box

Dark background (`#07071a`), cyan border (`rgba(0,200,255,0.12)`), monospace command, copy button in brand blue.

### Section Divider

A single hexagon outline (SVG or CSS border), centered, with 1px `rgba(0,200,255,0.2)` border.

### Cards

Background: `#0a0a1f`. Border: `1px solid rgba(0,200,255,0.08)`. Radius: 10px. On hover: border brightens to `rgba(0,200,255,0.3)` + subtle `translateY(-4px)` lift.

---

## Hexagon Geometry

For CSS/SVG reference — the standard hexagon points (centered at origin, radius R):

```
R = 100:
  (R·sin(0°),   -R·cos(0°))   = (0,    -100)   ← top
  (R·sin(60°),  -R·cos(60°))  = (86.6,  -50)   ← top-right
  (R·sin(120°), -R·cos(120°)) = (86.6,   50)   ← bottom-right
  (R·sin(180°), -R·cos(180°)) = (0,     100)   ← bottom
  (R·sin(240°), -R·cos(240°)) = (-86.6,  50)   ← bottom-left
  (R·sin(300°), -R·cos(300°)) = (-86.6, -50)   ← top-left
```

SVG polygon for hexagon (viewBox 256×256, center 128,128, R=112):
```
points="128,16 228,71 228,185 128,240 28,185 28,71"
```

---

## Version History

| Date | Version | Changes |
|------|---------|---------|
| 2026-06-18 | 1.0.0 | Initial brand guide. Logo SVG, favicon SVG, color palette, typography, component patterns. |

---

*Privacy is an architectural property, not a setting. ⬡*