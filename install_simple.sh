#!/bin/bash
# ⬡ POLYGONE — Installation Simple (No Rust needed!)
# One-command install for everyone!

set -e

CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
NC='\033[0m'

echo -e "${CYAN}${BOLD}"
echo "  ⬡ POLYGONE — Installation"
echo "  Réseau de Confidentialité Post-Quantique"
echo -e "${NC}"
echo ""

OS=$(uname -s)
ARCH=$(uname -m)

echo -e "${BOLD}Détection du système...${NC}"
echo "  OS: $OS | Arch: $ARCH"
echo ""

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

POLYGONE_SH_URL="https://raw.githubusercontent.com/lvs0/Polygone/main/polygone.sh"

# First, download the menu script
echo -e "${BOLD}Téléchargement de Polygone...${NC}"
echo "  → Menu: polygone.sh"
curl -fsSL "$POLYGONE_SH_URL" -o "$INSTALL_DIR/polygone.sh"
chmod +x "$INSTALL_DIR/polygone.sh"

# Try to download prebuilt binary from GitHub releases
GITHUB_API="https://api.github.com/repos/lvs0/Polygone/releases/latest"
LATEST_VERSION=$(curl -s "$GITHUB_API" | grep -o '"tag_name": "[^"]*"' | cut -d'"' -f4)

if [ -n "$LATEST_VERSION" ]; then
    echo "  → Release: $LATEST_VERSION"
    
    # Detect binary name based on OS/Arch
    case "$OS" in
        Linux)
            case "$ARCH" in
                x86_64) BINARY_NAME="polygone-cli-x86_64-linux" ;;
                aarch64) BINARY_NAME="polygone-cli-aarch64-linux" ;;
            esac
            ;;
        Darwin)
            case "$ARCH" in
                x86_64) BINARY_NAME="polygone-cli-x86_64-macos" ;;
                arm64) BINARY_NAME="polygone-cli-aarch64-macos" ;;
            esac
            ;;
    esac
    
    if [ -n "$BINARY_NAME" ]; then
        BINARY_URL="https://github.com/lvs0/Polygone/releases/download/$LATEST_VERSION/$BINARY_NAME"
        echo "  → Binaire: $BINARY_NAME"
        
        if curl -fsSL "$BINARY_URL" -o "$INSTALL_DIR/polygone-cli"; then
            chmod +x "$INSTALL_DIR/polygone-cli"
            echo -e "${GREEN}✓ Binaire précompilé installé!${NC}"
        else
            unset BINARY_NAME
        fi
    fi
fi

# If no prebuilt binary, compile from source (needs Rust)
if [ ! -f "$INSTALL_DIR/polygone-cli" ] || [ ! -x "$INSTALL_DIR/polygone-cli" ]; then
    echo ""
    echo -e "${YELLOW}Pas de binaire précompilé. Compilation depuis les sources...${NC}"
    echo ""
    
    if ! command -v rustc &> /dev/null; then
        echo -e "${BOLD}Installation de Rust...${NC}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env" 2>/dev/null || true
    fi
    
    if ! command -v cargo &> /dev/null; then
        echo -e "${YELLOW}⚠ Rust non disponible. Téléchargement du code source uniquement.${NC}"
        echo ""
        echo "  Installe Rust puis lance:"
        echo "    cd ~/Polygone && cargo build --release --bin polygone-cli"
        echo "    cp target/release/polygone-cli ~/.local/bin/"
    else
        echo -e "${BOLD}Compilation (3-5 minutes)...${NC}"
        cd /tmp
        rm -rf Polygone
        git clone --depth 1 https://github.com/lvs0/Polygone
        cd Polygone
        cargo build --release --bin polygone-cli 2>&1 | tail -3
        cp target/release/polygone-cli "$INSTALL_DIR/polygone-cli"
        chmod +x "$INSTALL_DIR/polygone-cli"
        rm -rf /tmp/Polygone
        echo -e "${GREEN}✓ Compilé et installé!${NC}"
    fi
fi

# Add to PATH
SHELL_RC="$HOME/.bashrc"
if [ -f "$HOME/.zshrc" ]; then
    SHELL_RC="$HOME/.zshrc"
fi

if ! grep -q '.local/bin' "$SHELL_RC" 2>/dev/null; then
    echo "" >> "$SHELL_RC"
    echo '# Polygone' >> "$SHELL_RC"
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_RC"
fi

echo ""
echo -e "${GREEN}✓ Installation terminée!${NC}"
echo ""
echo "  Lance maintenant:"
echo -e "    ${BOLD}source ~/.bashrc && polygone-cli self-test${NC}"
echo ""
echo -e "${CYAN}L'information n'existe pas. Elle traverse. ⬡${NC}"
