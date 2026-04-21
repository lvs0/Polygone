#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════════════
# ⬡ POLYGONE — Installateur Multiplateforme v2.0
# ═══════════════════════════════════════════════════════════════════════════════
# Supporte: Linux (Fedora, Ubuntu, Debian, Arch), macOS, Windows (WSL)
# Usage: curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash
# ═══════════════════════════════════════════════════════════════════════════════

set -euo pipefail

CYAN='\033[0;36m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info()    { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[✔]${NC} $1"; }
log_warn()    { echo -e "${YELLOW}[⚠]${NC} $1"; }
log_error()   { echo -e "${RED}[✖]${NC} $1"; }

POLYGONE_VERSION="${POLYGONE_VERSION:-latest}"

echo -e "${CYAN}"
echo "╔═══════════════════════════════════════════════════════════╗"
echo "║                                                           ║"
echo "║   ⬡ POLYGONE — Post-Quantum Ephemeral Privacy Network    ║"
echo "║   Installation Script v2.0                                ║"
echo "║                                                           ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo -e "${NC}"

# Détection du système
detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        OS="linux"
        if command -v dnf &> /dev/null; then
            DISTRO="fedora"
            PKG_MANAGER="dnf"
        elif command -v apt-get &> /dev/null; then
            DISTRO="debian"
            PKG_MANAGER="apt-get"
        elif command -v pacman &> /dev/null; then
            DISTRO="arch"
            PKG_MANAGER="pacman"
        else
            DISTRO="unknown"
            PKG_MANAGER="unknown"
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        OS="macos"
        DISTRO="macos"
        PKG_MANAGER="brew"
    else
        log_error "Système non supporté: $OSTYPE"
        exit 1
    fi
    log_info "Système: $OS ($DISTRO)"
}

# Installation des dépendances
install_dependencies() {
    log_info "Installation des dépendances..."
    
    case "$PKG_MANAGER" in
        dnf)
            sudo dnf install -y rust cargo openssl-devel pkg-config make git curl wget
            ;;
        apt-get)
            sudo apt-get update -y
            sudo apt-get install -y rustc cargo libssl-dev pkg-config make git curl wget
            ;;
        pacman)
            sudo pacman -Sy --noconfirm rust cargo openssl pkg-config make git curl wget
            ;;
        brew)
            if ! command -v brew &> /dev/null; then
                log_warn "Homebrew manquant. Installation..."
                /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
            fi
            brew install rust openssl pkg-config make git curl wget
            ;;
        *)
            # Fallback: installer Rust via rustup
            if ! command -v cargo &> /dev/null; then
                log_warn "Rust manquant. Installation via rustup..."
                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
                source "$HOME/.cargo/env"
            fi
            ;;
    esac
    
    log_success "Dépendances installées"
}

# Compilation
build_from_source() {
    log_info "Compilation de POLYGONE..."
    cd "$(dirname "$0")"
    cargo build --release --bin polygone
    log_success "Compilation terminée"
}

# Installation
install_binary() {
    log_info "Installation du binaire..."
    
    local bin_dir="$HOME/.local/bin"
    mkdir -p "$bin_dir"
    cp target/release/polygone "$bin_dir/"
    chmod +x "$bin_dir/polygone"
    
    # Ajout au PATH
    if [[ ":$PATH:" != *":$bin_dir:"* ]]; then
        echo "export PATH=\"$bin_dir:\$PATH\"" >> "$HOME/.bashrc"
        echo "export PATH=\"$bin_dir:\$PATH\"" >> "$HOME/.zshrc" 2>/dev/null || true
        export PATH="$bin_dir:$PATH"
    fi
    
    log_success "POLYGONE installé dans $bin_dir"
}

# Vérification
verify_installation() {
    log_info "Vérification..."
    
    if command -v polygone &> /dev/null; then
        polygone --version 2>/dev/null || echo "POLYGONE ${POLYGONE_VERSION}"
        log_success "Installation réussie!"
        echo ""
        echo -e "${CYAN}╔════════════════════════════════════════════════════════╗${NC}"
        echo -e "${CYAN}║  Commandes utiles:                                     ║${NC}"
        echo -e "${CYAN}║    polygone tui          → Interface graphique         ║${NC}"
        echo -e "${CYAN}║    polygone keygen       → Générer vos clés            ║${NC}"
        echo -e "${CYAN}║    polygone node start   → Démarrer un nœud            ║${NC}"
        echo -e "${CYAN}║    polygone self-test    → Tests de validation         ║${NC}"
        echo -e "${CYAN}╚════════════════════════════════════════════════════════╝${NC}"
    else
        log_error "Échec: binaire introuvable"
        exit 1
    fi
}

# Main
main() {
    detect_os
    install_dependencies
    build_from_source
    install_binary
    verify_installation
    
    echo ""
    echo -e "${GREEN}  ⬡ \"L'information n'existe pas. Elle traverse.\"${NC}"
    echo ""
}

main "$@"
