#!/bin/bash
# ⬡ POLYGONE — Smart Installer — by Lévy, France 🇫🇷
# Downloads pre-built binary or builds from source

set -e

CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

VERSION="1.0.0"
INSTALL_DIR="${HOME}/.local/bin"
BINARY_URL="https://github.com/lvs0/Polygone/releases/download/v${VERSION}/polygone"

echo -e "${CYAN}"
echo "  ⬡ POLYGONE v${VERSION}"
echo "  Post-quantum ephemeral privacy network"
echo -e "${NC}"

# Create install dir
mkdir -p "$INSTALL_DIR"

# ─── Method 1: Download pre-built binary (fast) ───────────────────
download_binary() {
    echo -e "${CYAN}  ↓ Downloading pre-built binary...${NC}"
    
    if curl -fsSL --retry 3 --retry-delay 2 -o "${INSTALL_DIR}/polygone" "${BINARY_URL}"; then
        chmod +x "${INSTALL_DIR}/polygone"
        
        if "${INSTALL_DIR}/polygone" --version &>/dev/null; then
            echo -e "${GREEN}  ✓ Binary installed from GitHub Releases${NC}"
            return 0
        fi
    fi
    
    rm -f "${INSTALL_DIR}/polygone"
    return 1
}

# ─── Method 2: Build from source (fallback) ────────────────────
build_from_source() {
    echo -e "${YELLOW}  ! Pre-built binary not available, building from source...${NC}"
    
    # Check Rust
    if ! command -v cargo &>/dev/null; then
        echo -e "${CYAN}  ↓ Installing Rust...${NC}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
        source "${HOME}/.cargo/env" 2>/dev/null || true
    fi
    
    echo -e "${CYAN}  ⚙ Building release binary...${NC}"
    
    if [ ! -f "Cargo.toml" ]; then
        git clone https://github.com/lvs0/Polygone.git /tmp/polygone-build
        cd /tmp/polygone-build
    else
        cd "$(dirname "$0")"
    fi
    
    cargo build --release 2>/dev/null || cargo build --release
    
    cp target/release/polygone "${INSTALL_DIR}/polygone"
    chmod +x "${INSTALL_DIR}/polygone"
    
    echo -e "${GREEN}  ✓ Built from source and installed${NC}"
}

# ─── Post-install ─────────────────────────────────────────────────
post_install() {
    echo ""
    echo -e "${GREEN}  ✓ POLYGONE v${VERSION} installed!${NC}"
    echo ""
    echo "  Location: ${INSTALL_DIR}/polygone"
    echo ""
    
    # Add to PATH
    if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
        echo "export PATH=\"${INSTALL_DIR}:\$PATH\"" >> "${HOME}/.bashrc"
        export PATH="${INSTALL_DIR}:$PATH"
        echo "  → Added ${INSTALL_DIR} to PATH"
    fi
    
    # Run self-test
    echo ""
    echo -e "${CYAN}  Running self-test...${NC}"
    "${INSTALL_DIR}/polygone" self-test
    
    echo ""
    echo -e "${GREEN}  ⬡ POLYGONE is ready.${NC}"
    echo ""
}

# ─── Main ─────────────────────────────────────────────────────────
main() {
    if download_binary; then
        post_install
    else
        build_from_source
        post_install
    fi
}

main "$@"