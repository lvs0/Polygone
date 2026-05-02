#!/bin/bash
# Polygone TUI — Install script
# Usage: bash install_polygone.sh

set -e

echo "⬡ Installing Polygone TUI..."

# Check Python
if ! command -v python3 &>/dev/null; then
    echo "ERROR: python3 not found"
    exit 1
fi

# Install the package
echo "[1/3] Installing Python package..."
pip install -e . --quiet

# Check polygone CLI
echo "[2/3] Checking polygone CLI..."
if command -v polygone &>/dev/null; then
    echo "  ✓ polygone found: $(polygone --version | head -1)"
else
    echo "  ⚠ polygone not found in PATH"
    echo "  Build from source: cargo build --release && cargo install --path ."
fi

# Verify installation
echo "[3/3] Verifying..."
python3 -c "from polygone_tui.app import PolygoneApp; print('  ✓ TUI import OK')"

echo ""
echo "⬡ Polygone TUI installed!"
echo ""
echo "Usage:"
echo "  polygone-tui              # Launch TUI"
echo "  polygone-tui start        # Start node (legacy)"
echo "  polygone-tui stop         # Stop node (legacy)"
echo "  polygone-tui status       # Show status (legacy)"
echo ""
echo "Or use the Rust CLI:"
echo "  polygone tui              # Launch Rust TUI (ratatui)"
echo "  polygone help             # Show all commands"
