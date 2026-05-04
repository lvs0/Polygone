<div align="center">
  <h1>⬡ POLYGONE</h1>
[![CI](https://github.com/lvs0/Polygone/actions/workflows/ci.yml/badge.svg)](https://github.com/lvs0/Polygone/actions/workflows/ci.yml)

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

n## Public demo node

A public demo node is available (subject to platform sleep policies):

- URL: `https://polygone-demo.onrender.com` (or your own deployed instance)
- Health: `https://polygone-demo.onrender.com/health`

To deploy your own free node in 5 minutes, see [CONTRIBUTING.md — Deploy a node (free)](CONTRIBUTING.md#deploy-a-node-free).

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
| `gateway` | Gateway HTTP → P2P (bridge) |
| `polygone-hide` | SOCKS5 proxy tunnel |

---

## Community

Join us on Discord: [![Discord](https://img.shields.io/badge/Discord-Join%20Polygone-5865F2?logo=discord)](https://discord.gg/kSqe38NbJM)

## Contributing & Local Development

See [CONTRIBUTING.md](CONTRIBUTING.md) for details on building, testing, and running a local cluster with Docker.

---

## License

MIT — Lévy Verpoort Scherpereel ([@lvs0](https://github.com/lvs0))
