# ⬡ Polygone-Shell

**The Cyberpunk Control Terminal.**

Polygone-Shell is a lightweight, high-performance terminal UI (TUI) built in Rust using `ratatui`. It provides a real-time cockpit for monitoring your node, managing sessions, and visualizing the ephemeral drift.

---

**Project origin · Origine :** French engineering · **[l-vs](https://github.com/lvs0)** · collective **Hope** (*by Hope*). Public focus: **post-quantum** protocols — not hosting economics or free-tier setups.

---

## 🚀 Key Features

- **Real-time Swarm Monitor**: Watch peers connect and disconnect.
- **Vapor Visualization**: Interactive view of shard fragmentation.
- **Post-Quantum Dashboard**: Monitor ML-KEM handshake statuses.
- **Cyberpunk Aesthetics**: Immersive, minimalist design.

## 🛠️ Usage

### Build and Run
```bash
cd Polygone-Shell
cargo run --release
```

### Controls
- `Tab`: Switch between tabs (Network, Storage, Identity).
- `q`: Exit shell.
- `Arrows`: Navigate logs.

## 🏗️ Stack

- **UI Framework**: Ratatui (Rust)
- **Backend**: Libp2p Events
- **Graphics**: ANSI Unicode symbols

## ⚖️ License
MIT License — 2026 · **l-vs** · **Hope** (*by Hope*) · Polygone ecosystem
