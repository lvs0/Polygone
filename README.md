<div align="center">
  <h1>⬡ POLYGONE</h1>
  <p><strong>Post-quantum privacy ecosystem. One command install. P2P. Zero trust.</strong></p>
</div>

---

## Quick Install

### Linux / macOS

```bash
curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/installer/install.sh | bash
```

### Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/lvs0/Polygone/main/installer/install.ps1 | iex
```

Then run:

```bash
polygone
```

---

## 🚀 Become a Node

### Option 1 — Desktop (recommended)

Install with the command above and run `polygone`. The TUI guides you through setup.

### Option 2 — Docker

```bash
docker run -d -p 8080:8080 \
  -e NODE_PSEUDO=your-node \
  -e PUBLIC_URL=https://your-domain.com \
  -e BOOTSTRAP_PEERS=https://node1.polygone.network,https://node2.polygone.network \
  lvs0/polygone-node:latest
```

### Option 3 — Cloud (free tier)

[![Deploy to Render](https://render.com/images/deploy-to-render-button.svg)](https://render.com/deploy?template=https://github.com/lvs0/Polygone)

---

## What is Polygone?

A complete privacy ecosystem built on post-quantum cryptography:

- **ML-KEM-1024** (FIPS 203) — post-quantum key exchange
- **Shamir SSS 4-of-7** — distributed secret sharing
- **AES-256-GCM** — authenticated encryption
- **BLAKE3** — cryptographic hashing
- **libp2p + Kademlia DHT** — decentralized routing

| Module | Description |
|--------|-------------|
| `polygone-tui` | Unified terminal dashboard |
| `polygone-drive` | Distributed encrypted storage + web UI |
| `polygone-msg` | E2E encrypted P2P messaging |
| `polygone-mesh` | Local network mesh (mDNS + Bluetooth) |
| `polygone-brain` | Local AI with provider fallback |
| `polygone-petals` | Distributed LLM inference |
| `polygone-server` | DHT relay node |
| `polygone-hide` | SOCKS5 proxy tunnel |

---

## License

MIT — Lévy Verpoort Scherpereel ([@lvs0](https://github.com/lvs0))
