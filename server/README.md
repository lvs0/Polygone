# ⬡ Polygone-Server

**The Infrastructure Core: Persistent Node Deployment.**

Polygone-Server is a production-ready wrapper designed to run a Polygone node 24/7 on servers and VPS using Docker. It ensures your PeerId persists across restarts and manages caching for the Drive network.

## 🚀 Key Features

- **Persistent Identity**: Automatically manages `identity.key` to ensure your node keeps its PeerId and reputation.
- **Dockerized**: Multi-stage build for a tiny footprint.
- **Smart Resource Sharing**: Integrated Karma system to automatically lend idle CPU/RAM.

## 🛠️ Quick Start

### One-Line Deploy
```bash
git clone https://github.com/lvs0/Polygone-Server
cd Polygone-Server
docker-compose up -d
```

### Manual Configuration
Edit `docker-compose.yml` to change:
- `POLY_CACHE_GB`: Maximum storage allocated for Drive shards.
- `POLY_LISTEN`: Public port (default 4001).

## 🏗️ Stack

- **Container**: Alpine Linux (slim)
- **Orchestration**: Docker Compose
- **Healthchecks**: Internal monitoring of swarm connectivity.

## ⚖️ License
MIT License - 2026 Lévy / Polygone Ecosystem.
by Hope
