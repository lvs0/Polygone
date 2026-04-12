#!/bin/bash
# ⬡ POLYGONE — Installer — by Hope 🇫🇷
set -e

CYAN='\033[0;36m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${CYAN}"
echo "  ⬡ POLYGONE — by Hope"
echo "  Installing ephemeral post-quantum network..."
echo -e "${NC}"

# --- Check Rust ---
if ! command -v cargo &>/dev/null; then
    echo -e "${YELLOW}  [!] Rust not found. Installing...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
    source "$HOME/.cargo/env"
else
    echo -e "${GREEN}  [✓] Rust found: $(rustc --version)${NC}"
fi

# --- Build ---
echo -e "${CYAN}  Building release binary...${NC}"
cargo build --release 2>&1

# --- Install to PATH ---
BIN_DIR="$HOME/.local/bin"
mkdir -p "$BIN_DIR"
cp target/release/polygone "$BIN_DIR/polygone"

# Add to PATH if not there
if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
    echo "export PATH=\"$BIN_DIR:\$PATH\"" >> "$HOME/.bashrc"
    echo "export PATH=\"$BIN_DIR:\$PATH\"" >> "$HOME/.zshrc" 2>/dev/null || true
    export PATH="$BIN_DIR:$PATH"
fi

echo ""
echo -e "${GREEN}  ✓ Polygone installed successfully!${NC}"
echo ""
echo "  Run:  polygone start      → Launch the interactive shell"
echo "  Run:  polygone help       → Full command reference"
echo "  Run:  polygone self-test  → Verify everything works"
echo ""
echo -e "${CYAN}  ⬡ \"L'information n'existe pas. Elle traverse.\" — Hope${NC}"
echo ""
