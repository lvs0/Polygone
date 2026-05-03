#!/bin/bash
set -e
CYAN='\033[0;36m'; GREEN='\033[0;32m'; BLUE='\033[0;34m'; NC='\033[0m'
P="$HOME/.polygone"
R="$HOME/Polygone-Network"
echo -e "${CYAN}🌸 POLYGONE Installer${NC}"
mkdir -p "$P"/{config,data,logs,ssl} "$P/docker"
echo -e "${BLUE}Generating config...${NC}"
cat > "$P/config/config.toml" << T
[polygone]
node_name = "${1:-lvs0-node}"
data_dir = "$P/data"
log_dir = "$P/logs"
T
if [ -d "$R" ]; then echo "Repo found at $R"; else git clone https://github.com/lvs0/Polygone-Network.git "$R" 2>/dev/null || true; fi
cat > "$P/docker/docker-compose.yml" << Y
version: '3.8'
services:
  polygone:
    build:
      context: ${R}
      dockerfile: Dockerfile
    ports: ["8080:8080", "4001:4001"]
    environment:
      NODE_PSEUDO: ${1:-lvs0-node}
      PUBLIC_URL: http://localhost:8080
    restart: unless-stopped
Y
openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout "$P/ssl/key.pem" -out "$P/ssl/cert.pem" -subj "/CN=localhost" 2>/dev/null || true
echo -e "${GREEN}✅ Installation complete!${NC}"
echo "Repo: $R"
echo "Config: $P/config/config.toml"
echo "Run: cd $R && cargo build --release && ./target/release/polygone"
