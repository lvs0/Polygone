#!/bin/bash
# ⬡ POLYGONE — Installation Simple
# One-command install for everyone!

set -e

CYAN='\033[0;36m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
NC='\033[0m'

echo -e "${CYAN}${BOLD}"
echo "  ⬡ POLYGONE — Installation"
echo "  Réseau de Confidentialité Post-Quantique"
echo -e "${NC}"
echo ""

# Detect OS
OS=$(uname -s)
ARCH=$(uname -m)

echo -e "${BOLD}Détection du système...${NC}"
echo "  OS: $OS | Arch: $ARCH"
echo ""

# Check if already installed
if command -v polygone-cli &> /dev/null; then
    echo -e "${GREEN}✓ Polygone est déjà installé!${NC}"
    polygone-cli --version 2>/dev/null || echo "Version: $(polygone-cli --version 2>&1 | head -1)"
    echo ""
    echo "Pour mettre à jour: rm \$(which polygone-cli) && $0"
    exit 0
fi

# Create install directory
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# Check for prebuilt binary first
PREBUILT_URL=""
if [ "$OS" = "Linux" ] && [ "$ARCH" = "x86_64" ]; then
    PREBUILT_URL="https://github.com/lvs0/Polygone/releases/latest/download/polygone-cli-x86_64-linux"
fi

if [ -n "$PREBUILT_URL" ]; then
    echo -e "${BOLD}Téléchargement du binaire précompilé...${NC}"
    if curl -fsSL "$PREBUILT_URL" -o "$INSTALL_DIR/polygone-cli"; then
        chmod +x "$INSTALL_DIR/polygone-cli"
        echo -e "${GREEN}✓ Binaire installé!${NC}"
        
        # Add to PATH if needed
        if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
            echo ""
            echo -e "${YELLOW}Ajoute ~/.local/bin à ton PATH:${NC}"
            echo '  echo "export PATH=\$HOME/.local/bin:\$PATH" >> ~/.bashrc'
            echo '  source ~/.bashrc'
        fi
        
        echo ""
        echo -e "${GREEN}✓ Installation terminée!${NC}"
        echo ""
        echo "  Lance: polygone-cli self-test"
        echo "  Docs:  https://github.com/lvs0/Polygone"
        exit 0
    fi
fi

# Fallback: Install from source
echo -e "${YELLOW}Pas de binaire précompilé. Installation depuis les sources...${NC}"
echo ""

# Check for Rust
if ! command -v rustc &> /dev/null; then
    echo -e "${BOLD}Installation de Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env" 2>/dev/null || true
fi

# Check for cargo
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}✗ Cargo non disponible. Installe Rust d'abord:${NC}"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo -e "${BOLD}Compilation de Polygone (ça prend ~3-5 minutes)...${NC}"
echo ""

# Clone or update repo
POLYGONE_DIR="$HOME/Polygone"
if [ -d "$POLYGONE_DIR" ]; then
    echo "  Mise à jour du dépôt existant..."
    cd "$POLYGONE_DIR"
    git pull origin main 2>/dev/null || true
else
    echo "  Clonage du dépôt..."
    git clone https://github.com/lvs0/Polygone "$POLYGONE_DIR"
    cd "$POLYGONE_DIR"
fi

# Build release
cargo build --release --bin polygone-cli 2>&1 | tail -5

# Copy binary
cp "$POLYGONE_DIR/target/release/polygone-cli" "$INSTALL_DIR/polygone-cli"
chmod +x "$INSTALL_DIR/polygone-cli"

# Add to PATH
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo ""
    echo -e "${YELLOW}Ajoute ~/.local/bin à ton PATH:${NC}"
    echo '  echo "export PATH=$HOME/.local/bin:$PATH" >> ~/.bashrc'
    echo '  source ~/.bashrc'
fi

echo ""
echo -e "${GREEN}✓ Installation terminée!${NC}"
echo ""
echo "  Lance: polygone-cli self-test"
echo "  Docs:  https://github.com/lvs0/Polygone"
echo ""
echo -e "${CYAN}L'information n'existe pas. Elle traverse. ⬡${NC}"
