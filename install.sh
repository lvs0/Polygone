#!/bin/bash
# ⬡ POLYGONE — One-line installer — by Lévy
# Usage: curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash
# Or:    curl -fsSL https://lvs0.github.io/Polygone/install.sh | bash
set -e

CYAN='\033[0;36m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'
BOLD='\033[1m'
MAGENTA='\033[0;35m'

INSTALL_BASE="${POLYGONE_HOME:-$HOME/.polygone}"
BIN_DIR="$INSTALL_BASE/bin"

REPO_BASE="https://lvs0.github.io/Polygone"
RELEASE_URL="https://github.com/lvs0/Polygone/releases/download/v1.0.0"
RELEASE_TAG="v1.0.0"

# Pre-built binary URL (GitHub Release)
BINARY_URL="${RELEASE_URL}/polygone"
CHECKSUM_URL="${RELEASE_URL}/checksums.txt"

# Parse flags
SKIP_SOURCE=0
FORCE_REBUILD=0
while [[ $# -gt 0 ]]; do
  case $1 in
    --skip-source) SKIP_SOURCE=1 ;;
    --force-rebuild) FORCE_REBUILD=1 ;;
    --help|-h)
      echo "⬡ Polygone installer"
      echo "  curl -fsSL https://.../install.sh | bash"
      echo "  Flags: --skip-source  (binary download only)"
      echo "         --force-rebuild (build from source)"
      exit 0 ;;
  esac
  shift
done

echo -e "${MAGENTA}"
echo "  ╔═══════════════════════════════════════════════╗"
echo "  ║  ⬡  P O L Y G O N E  —  Post-Quantum Privacy ║"
echo "  ║  ML-KEM · Shamir · AES-256-GCM · BLAKE3      ║"
echo "  ╚═══════════════════════════════════════════════╝"
echo -e "${NC}"
echo -e "  ${CYAN}Installing to: $INSTALL_BASE${NC}"
echo ""

# ===== FAST PATH: Download pre-built binary =====
download_binary() {
  local url="$1"
  local dest="$2"
  echo -e "  ${CYAN}↓${NC} Downloading pre-built binary..."

  if command -v curl >/dev/null 2>&1; then
    DL_CMD="curl -fsSL --proto '=https' --tlsv1.2 -o"
  elif command -v wget >/dev/null 2>&1; then
    DL_CMD="wget -q -O"
  else
    echo -e "  ${RED}✗ Neither curl nor wget found. Install curl first.${NC}"
    return 1
  fi

  # Try primary URL
  if eval "$DL_CMD \"$dest\" \"$url\"" 2>/dev/null; then
    return 0
  fi

  # Try fallback mirror
  local fallback="${url//lvs0.github.io/raw.githubusercontent.com}"
  echo -e "  ${YELLOW}↺ Retry via GitHub raw...${NC}"
  eval "$DL_CMD \"$dest\" \"$fallback\"" 2>/dev/null
}

install_fast() {
  mkdir -p "$BIN_DIR"

  local dest="$BIN_DIR/polygone"

  # Download binary from GitHub Release
  echo -e "  ${CYAN}↓${NC} Downloading binary from GitHub Release..."
  if download_binary "$BINARY_URL" "$dest"; then
    chmod +x "$dest"

    # Verify checksum if available
    local sha_file="$INSTALL_BASE/checksums.txt"
    if download_binary "$CHECKSUM_URL" "$sha_file" 2>/dev/null; then
      echo -e "  ${CYAN}✓${NC} Verifying SHA256..."
      local expected_sha
      expected_sha=$(awk '{print $1}' "$sha_file")
      local actual_sha
      actual_sha=$(sha256sum "$dest" | awk '{print $1}')
      if [[ "$expected_sha" != "$actual_sha" ]]; then
        echo -e "  ${RED}✗ Checksum mismatch! Removing corrupted binary.${NC}"
        rm -f "$dest"
        return 1
      fi
      echo -e "  ${GREEN}  ✓ SHA256 verified${NC}"
      rm -f "$sha_file"
    else
      echo -e "  ${YELLOW}  ⚠ No checksums.txt found — trust at your own risk${NC}"
    fi

    # Add to PATH
    add_to_path
    return 0
  fi

  return 1
}

# ===== SLOW PATH: Build from source =====
install_from_source() {
  echo -e "  ${CYAN}⚙${NC} Building from source (requires Rust, ~5-10 min)..."

  if ! command -v cargo &>/dev/null; then
    echo -e "  ${YELLOW}  [!] Rust not found. Installing...${NC}"
    if ! command -v curl &>/dev/null; then
      echo -e "  ${RED}✗ curl required to install Rust${NC}"
      return 1
    fi
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
    source "$HOME/.cargo/env" 2>/dev/null || export PATH="$HOME/.cargo/bin:$PATH"
  fi

  local tmpdir="$INSTALL_BASE/build-$$"
  mkdir -p "$tmpdir"
  trap "rm -rf $tmpdir" EXIT

  echo -e "  ${CYAN}↓${NC} Cloning Polygone..."
  if command -v git >/dev/null 2>&1; then
    git clone --depth=1 https://github.com/lvs0/Polygone.git "$tmpdir" 2>/dev/null
  fi

  if [[ -d "$tmpdir/Polygone" ]]; then
    tmpdir="$tmpdir/Polygone"
  fi

  if [[ -f "$tmpdir/Cargo.toml" ]]; then
    echo -e "  ${CYAN}⚙${NC} Compiling (this may take a while the first time)..."
    (cd "$tmpdir" && cargo build --release -p polygone-app 2>&1) | grep -E "^  Compiling|^   Finished|error\[|^warning:" || true
    local binary="$tmpdir/target/release/polygone"
    if [[ -f "$binary" ]]; then
      mkdir -p "$BIN_DIR"
      cp "$binary" "$BIN_DIR/polygone"
      chmod +x "$BIN_DIR/polygone"
      add_to_path
      return 0
    fi
  fi

  echo -e "  ${RED}✗ Source build failed${NC}"
  return 1
}

add_to_path() {
  local profile_file=""
  for f in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile"; do
    if [[ -f "$f" ]]; then
      profile_file="$f"
      break
    fi
  done

  local path_line="export PATH=\"$BIN_DIR:\$PATH\" # Polygone"
  if [[ -n "$profile_file" ]] && ! grep -q "Polygone" "$profile_file" 2>/dev/null; then
    echo "$path_line" >> "$profile_file"
    echo -e "  ${CYAN}→${NC} Added $BIN_DIR to $profile_file"
  fi

  export PATH="$BIN_DIR:$PATH"
}

# ===== MAIN =====
if [[ $FORCE_REBUILD -eq 1 ]]; then
  install_from_source
else
  # Try fast path first, fall back to source
  if [[ $SKIP_SOURCE -eq 0 ]] && install_fast; then
    echo -e "  ${GREEN}  ✓ Binary installed successfully!${NC}"
  else
    if [[ $SKIP_SOURCE -eq 1 ]]; then
      echo -e "  ${RED}✗ Binary not available. Use --force-rebuild to build from source.${NC}"
      exit 1
    fi
    echo -e "  ${YELLOW}  Pre-built binary not available. Building from source...${NC}"
    install_from_source
  fi
fi

# Verify install
if command -v polygone &>/dev/null; then
  echo ""
  echo -e "  ${GREEN}✓ Polygone v$(polygone --version 2>/dev/null || echo 'installed') ready!${NC}"
  echo ""
  echo -e "  ${BOLD}Quick start:${NC}"
  echo -e "    ${CYAN}polygone self-test${NC}   → Verify crypto stack"
  echo -e "    ${CYAN}polygone start${NC}       → Launch P2P node"
  echo -e "    ${CYAN}polygone --help${NC}      → All commands"
  echo ""
  echo -e "  ${MAGENTA}⬡ Information does not exist. It traverses.${NC}"
else
  echo -e "  ${YELLOW}⚠ Polygone not found in PATH."
  echo "  Add $BIN_DIR to your PATH, then run 'polygone self-test'"
fi