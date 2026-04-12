#!/bin/bash
# ⬡ POLYGONE — Installation Simple
# Usage: curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash

set -e

CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
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

BINARY_NAME=""
case "$OS" in
    Linux)
        case "$ARCH" in
            x86_64) BINARY_NAME="polygone-x86_64-linux" ;;
            aarch64) BINARY_NAME="polygone-aarch64-linux" ;;
        esac
        ;;
    Darwin)
        case "$ARCH" in
            x86_64) BINARY_NAME="polygone-x86_64-macos" ;;
            arm64) BINARY_NAME="polygone-aarch64-macos" ;;
        esac
        ;;
esac

if [ -z "$BINARY_NAME" ]; then
    echo -e "${RED}✗ Pas de binaire pour ${OS}-${ARCH}${NC}"
    echo ""
    echo "Installe Rust et compile depuis les sources:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "  git clone https://github.com/lvs0/Polygone && cd Polygone"
    echo "  cargo build --release && cp target/release/polygone ~/.local/bin/"
    exit 1
fi

if [ -x "$INSTALL_DIR/polygone" ]; then
    echo -e "${BOLD}Polygone est déjà installé.${NC}"
    echo ""
    echo -n "Voulez-vous mettre à jour? [Y/n] "
    read -r answer
    if [ "$answer" != "n" ] && [ "$answer" != "N" ]; then
        echo ""
        echo "  → Lancement de la mise à jour..."
        "$INSTALL_DIR/polygone" update --force
        exit 0
    fi
    echo ""
    echo "  Lance: polygone help"
    echo "  Docs:  https://github.com/lvs0/Polygone"
    exit 0
fi

echo -e "${BOLD}Téléchargement de Polygone...${NC}"

GITHUB_API="https://api.github.com/repos/lvs0/Polygone/releases/latest"
LATEST_VERSION=$(curl -s "$GITHUB_API" | grep -o '"tag_name": "[^"]*"' | cut -d'"' -f4)

if [ -n "$LATEST_VERSION" ]; then
    echo "  → Version: $LATEST_VERSION"
    DOWNLOAD_URL="https://github.com/lvs0/Polygone/releases/download/$LATEST_VERSION/$BINARY_NAME"
    echo "  → Binary: $BINARY_NAME"
    
    if curl -fsSL "$DOWNLOAD_URL" -o "$INSTALL_DIR/polygone"; then
        chmod +x "$INSTALL_DIR/polygone"
        echo -e "${GREEN}✓ Polygone installé!${NC}"
    else
        echo -e "${RED}✗ Erreur lors du téléchargement${NC}"
        exit 1
    fi
else
    echo -e "${RED}✗ Impossible de contacter GitHub${NC}"
    exit 1
fi

SHELL_RC="$HOME/.bashrc"
if [ -f "$HOME/.zshrc" ]; then
    SHELL_RC="$HOME/.zshrc"
fi

if ! grep -q '.local/bin' "$SHELL_RC" 2>/dev/null; then
    echo "" >> "$SHELL_RC"
    echo '# Polygone' >> "$SHELL_RC"
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_RC"
    echo -e "${GREEN}✓ PATH mis à jour${NC}"
fi

echo ""
echo -e "${GREEN}✓ Installation terminée!${NC}"
echo ""
echo "  Lance maintenant:"
echo -e "    ${BOLD}source ~/.bashrc && polygone help${NC}"
echo ""
echo "  Commandes disponibles:"
echo "    polygone self-test    — Vérifier que tout fonctionne"
echo "    polygone keygen       — Générer des clés"
echo "    polygone send         — Envoyer un message"
echo "    polygone node         — Lancer un node relay"
echo "    polygone update       — Mettre à jour"
echo "    polygone uninstall    — Désinstaller"
echo ""
echo -e "${CYAN}L'information n'existe pas. Elle traverse. ⬡${NC}"
