#!/bin/bash
# ⬡ POLYGONE — Universal Installer
# Usage: 
#   curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash
#   ./install.sh --core --drive --hide --all --lang fr

# ══════════════════════════════════════════════════════════════
# COLORS
# ══════════════════════════════════════════════════════════════
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'
BOLD='\033[1m'

# ══════════════════════════════════════════════════════════════
# DETECT SYSTEM
# ══════════════════════════════════════════════════════════════
OS=$(uname -s)
ARCH=$(uname -m)

echo -e "${CYAN}${BOLD}"
echo "  ⬡ POLYGONE — Universal Installer"
echo "  Post-Quantum Ephemeral Privacy Network"
echo -e "${NC}"
echo ""

echo "  Detecting system..."
echo "  OS: $OS | Arch: $ARCH"
echo ""

# ══════════════════════════════════════════════════════════════
# CONFIG
# ══════════════════════════════════════════════════════════════
INSTALL_DIR="$HOME/.local/bin"
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

# ══════════════════════════════════════════════════════════════
# PARSE ARGUMENTS
# ══════════════════════════════════════════════════════════════
INSTALL_CORE=0
INSTALL_DRIVE=0
INSTALL_HIDE=0
INSTALL_PETALS=0
INSTALL_SHELL=0
INSTALL_BRAIN=0
INSTALL_SERVER=0
FORCE=0

while [[ $# -gt 0 ]]; do
    case "$1" in
        --all)
            INSTALL_CORE=1
            INSTALL_DRIVE=1
            INSTALL_HIDE=1
            INSTALL_PETALS=1
            INSTALL_SHELL=1
            INSTALL_BRAIN=1
            INSTALL_SERVER=1
            ;;
        --core)
            INSTALL_CORE=1
            ;;
        --drive)
            INSTALL_DRIVE=1
            ;;
        --hide)
            INSTALL_HIDE=1
            ;;
        --petals)
            INSTALL_PETALS=1
            ;;
        --shell)
            INSTALL_SHELL=1
            ;;
        --brain)
            INSTALL_BRAIN=1
            ;;
        --server)
            INSTALL_SERVER=1
            ;;
        --force)
            FORCE=1
            ;;
        --help|-h)
            echo "  Usage: $0 [OPTIONS]"
            echo ""
            echo "  Options:"
            echo "    --all      Install everything"
            echo "    --core     Install Polygone Core"
            echo "    --drive    Install Drive module"
            echo "    --hide     Install Hide module"
            echo "    --petals   Install Petals module"
            echo "    --shell    Install Shell module"
            echo "    --brain    Install Brain module"
            echo "    --server   Install Server module"
            echo "    --force    Force update if installed"
            echo "    --help     Show this help"
            echo ""
            echo "  Examples:"
            echo "    $0 --all                    # Install everything"
            echo "    $0 --core                  # Core only"
            echo "    $0 --core --drive --hide  # Core + Drive + Hide"
            echo ""
            exit 0
            ;;
        *)
            echo -e "  ${RED}Unknown option: $1${NC}"
            echo "  Use --help for usage"
            exit 1
            ;;
    esac
    shift
done

# If no modules specified, default to core
if [[ $INSTALL_CORE -eq 0 ]] && [[ $INSTALL_DRIVE -eq 0 ]] && [[ $INSTALL_HIDE -eq 0 ]] && [[ $INSTALL_PETALS -eq 0 ]] && [[ $INSTALL_SHELL -eq 0 ]] && [[ $INSTALL_BRAIN -eq 0 ]] && [[ $INSTALL_SERVER -eq 0 ]]; then
    INSTALL_CORE=1
fi

mkdir -p "$INSTALL_DIR"

# ══════════════════════════════════════════════════════════════
# INSTALL CORE
# ══════════════════════════════════════════════════════════════
install_core() {
    echo -e "${BOLD}Installing Polygone Core...${NC}"
    
    # Check if already installed
    if [[ -x "$INSTALL_DIR/polygone" ]] && [[ $FORCE -eq 0 ]]; then
        echo "  Polygone Core is already installed."
        echo "  Use --force to update."
        return 0
    fi
    
    if [[ -z "$BINARY_NAME" ]]; then
        echo -e "  ${RED}✗ No binary for $OS-$ARCH${NC}"
        echo "  Build from source:"
        echo "    git clone https://github.com/lvs0/Polygone && cd Polygone"
        echo "    cargo build --release && cp target/release/polygone ~/.local/bin/"
        return 1
    fi
    
    # Get latest release
    GITHUB_API="https://api.github.com/repos/lvs0/Polygone/releases/latest"
    API_RESPONSE=$(curl -s "$GITHUB_API")
    LATEST_VERSION=$(echo "$API_RESPONSE" | python3 -c "import sys,json; print(json.load(sys.stdin)['tag_name'])")
    
    if [[ -n "$LATEST_VERSION" ]]; then
        echo "  → Version: $LATEST_VERSION"
    fi
    
    DOWNLOAD_URL="https://github.com/lvs0/Polygone/releases/download/$LATEST_VERSION/$BINARY_NAME"
    echo "  → Downloading: $BINARY_NAME"
    
    if curl -fL "$DOWNLOAD_URL" -o "$INSTALL_DIR/polygone"; then
        chmod +x "$INSTALL_DIR/polygone"
        echo -e "  ${GREEN}✓ Polygone Core installed!${NC}"
    else
        echo -e "  ${RED}✗ Download failed${NC}"
        return 1
    fi
}

# ══════════════════════════════════════════════════════════════
# INSTALL MODULE
# ══════════════════════════════════════════════════════════════
install_module() {
    local name="$1"
    local repo="$2"
    
    echo -e "${BOLD}Installing $name...${NC}"
    
    MODULE_DIR="$HOME/Polygone-Ecosystem/$name"
    mkdir -p "$MODULE_DIR"
    
    if [[ -d "$MODULE_DIR/.git" ]]; then
        echo "  → Updating existing..."
        cd "$MODULE_DIR" && git pull origin main 2>/dev/null || true
    else
        echo "  → Cloning..."
        git clone --depth 1 "https://github.com/lvs0/$repo" "$MODULE_DIR" 2>/dev/null || true
    fi
    
    if [[ -f "$MODULE_DIR/install.sh" ]]; then
        echo "  → Running installer..."
        cd "$MODULE_DIR" && chmod +x install.sh && bash install.sh 2>/dev/null || true
    fi
    
    echo -e "  ${GREEN}✓ $name installed${NC}"
}

# ══════════════════════════════════════════════════════════════
# MAIN INSTALL
# ══════════════════════════════════════════════════════════════
echo ""
echo "  Modules to install:"

[[ $INSTALL_CORE -eq 1 ]] && echo "    • Core (Polygone)"
[[ $INSTALL_DRIVE -eq 1 ]] && echo "    • Drive (Sharded storage)"
[[ $INSTALL_HIDE -eq 1 ]] && echo "    • Hide (VPN tunnel)"
[[ $INSTALL_PETALS -eq 1 ]] && echo "    • Petals (LLM inference)"
[[ $INSTALL_SHELL -eq 1 ]] && echo "    • Shell (TUI dashboard)"
[[ $INSTALL_BRAIN -eq 1 ]] && echo "    • Brain (AI diagnostics)"
[[ $INSTALL_SERVER -eq 1 ]] && echo "    • Server (Backend)"

echo ""

# Install Core first
[[ $INSTALL_CORE -eq 1 ]] && install_core

# Install other modules
[[ $INSTALL_DRIVE -eq 1 ]] && install_module "Drive" "Polygone-Drive"
[[ $INSTALL_HIDE -eq 1 ]] && install_module "Hide" "Polygone-Hide"
[[ $INSTALL_PETALS -eq 1 ]] && install_module "Petals" "Polygone-Petals"
[[ $INSTALL_SHELL -eq 1 ]] && install_module "Shell" "Polygone-Shell"
[[ $INSTALL_BRAIN -eq 1 ]] && install_module "Brain" "Polygone-Brain"
[[ $INSTALL_SERVER -eq 1 ]] && install_module "Server" "Polygone-Server"

# Add to PATH
SHELL_RC="$HOME/.bashrc"
[[ -f "$HOME/.zshrc" ]] && SHELL_RC="$HOME/.zshrc"

if ! grep -q '.local/bin' "$SHELL_RC" 2>/dev/null; then
    echo "" >> "$SHELL_RC"
    echo '# Polygone' >> "$SHELL_RC"
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_RC"
    echo -e "  ${GREEN}✓ PATH updated in $SHELL_RC${NC}"
fi

echo ""
echo -e "${GREEN}✓ Installation complete!${NC}"
echo ""
echo "  Launch:"
[[ $INSTALL_CORE -eq 1 ]] && echo "    source ~/.bashrc && polygone help"
[[ $INSTALL_DRIVE -eq 1 ]] && echo "    polygone-drive"
[[ $INSTALL_HIDE -eq 1 ]] && echo "    polygone-hide"
[[ $INSTALL_SHELL -eq 1 ]] && echo "    polygone-shell"
[[ $INSTALL_BRAIN -eq 1 ]] && echo "    polygone-brain"
echo ""
echo -e "${CYAN}L'information n'existe pas. Elle traverse. ⬡${NC}"
