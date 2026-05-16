#!/usr/bin/env bash
# Polygone DePIN Setup — Grass + io.net nodes
# Run once: bash ~/Polygone/.hermes-scripts/setup-depin.sh

set -euo pipefail

WORKSPACE="$HOME/Polygone"
LOG_DIR="$WORKSPACE/.hermes-logs"
WALLET_FILE="$WORKSPACE/.hermes-data/depin-wallet.json"

mkdir -p "$LOG_DIR" "$WORKSPACE/.hermes-data"

echo "=== POLYGONE DePIN Node Setup ==="
echo "1. Grass (bandwidth sharing — CPU only, low resource)"
echo "2. io.net (GPU compute — requires NVIDIA GPU)"
echo ""

read -p "Which node to install? [1/2/both]: " choice

install_grass() {
    echo "[GRASS] Starting installation..."
    
    # Check for Docker
    if ! command -v docker &>/dev/null; then
        echo "[GRASS] Docker not found. Installing..."
        sudo apt update && sudo apt install -y docker.io 2>&1 | tail -3
        sudo systemctl enable docker 2>&1 | tail -1
        sudo systemctl start docker 2>&1 | tail -1
    fi
    
    # Check if wallet exists
    if [ ! -f "$WALLET_FILE" ]; then
        echo "[GRASS] No wallet found. Generate one first."
        echo "You'll need a Solana wallet address."
        echo "Enter your Solana wallet address (or press Enter to skip):"
        read -r SOLANA_WALLET
        if [ -z "$SOLANA_WALLET" ]; then
            echo "[GRASS] Skipped. Run script again when you have a wallet."
            return 1
        fi
        echo "{\"type\":\"solana\",\"address\":\"$SOLANA_WALLET\"}" > "$WALLET_FILE"
    fi
    
    SOLANA_WALLET=$(python3 -c "import json; print(json.load(open('$WALLET_FILE'))['address'])")
    
    echo "[GRASS] Pulling Docker image..."
    sudo docker pull layerio/grass-node:latest 2>&1 | tail -3
    
    echo "[GRASS] Starting node..."
    sudo docker run -d \
        --name grass-node \
        --restart unless-stopped \
        -e WALLET_ADDRESS="$SOLANA_WALLET" \
        layerio/grass-node
    
    echo "[GRASS] Checking status..."
    sleep 3
    sudo docker logs grass-node 2>&1 | tail -10
    
    echo "[GRASS] Installation complete!"
    echo "Monitor: sudo docker logs -f grass-node"
    echo "Stop: sudo docker stop grass-node && sudo docker rm grass-node"
}

install_ionet() {
    echo "[IO.NET] Starting installation..."
    
    # Check for NVIDIA GPU
    if ! command -v nvidia-smi &>/dev/null; then
        echo "[IO.NET] No NVIDIA GPU found. io.net requires CUDA GPU."
        echo "[IO.NET] Skipping."
        return 1
    fi
    
    GPU=$(nvidia-smi --query-gpu=name --format=csv,noheader | head -1)
    echo "[IO.NET] Found GPU: $GPU"
    
    # Install io.net agent
    echo "[IO.NET] Installing io.net agent..."
    curl -fsSL https://cdn.io.net/install.sh | sh 2>&1 | tail -10
    
    # Check if installed
    if command -v ionet &>/dev/null; then
        echo "[IO.NET] Authenticate with: ionet auth login"
        echo "[IO.NET] Configure GPU: ionet config --gpu-type '$GPU'"
        echo "[IO.NET] Start node: ionet start"
    else
        echo "[IO.NET] Installation failed. Check docs at io.net"
    fi
}

case "$choice" in
    1) install_grass ;;
    2) install_ionet ;;
    both) install_grass; install_ionet ;;
    *) echo "Invalid choice"; exit 1 ;;
esac

echo ""
echo "=== Setup Complete ==="
echo "Log file: $LOG_DIR/depin-setup-$(date +%Y%m%d).log"