#!/bin/bash
# ⬡ POLYGONE — Installer
# Usage: 
#   curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash
#   curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash -s -- --all
#   ./install.sh --core --drive

INSTALL_DIR="$HOME/.local/bin"
ECOSYSTEM_DIR="$HOME/.Polygone-Ecosystem"
LATEST_VERSION="v0.1.0"

# SHA256 checksums for released binaries (updated per release)
declare -A SHA256_SUMS
SHA256_SUMS["polygone-x86_64-linux"]="e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"

RED=$'\033[0;31m'
GREEN=$'\033[0;32m'
CYAN=$'\033[0;36m'
NC=$'\033[0m'

UI_LANG="en"
sel_core=1 sel_drive=0 sel_hide=0 sel_petals=0 sel_shell=0 sel_brain=0 sel_server=0

t() {
    local key="${1}"
    local val
    case "$UI_LANG" in
        fr) val="${T_FR[$key]}" ;;
        de) val="${T_DE[$key]}" ;;
        *) val="${T_EN[$key]}" ;;
    esac
    echo "${val:-$1}"
}

declare -A T_EN
T_EN[title]="⬡ POLYGONE — Installer"
T_EN[welcome]="Welcome! Select language:"
T_EN[modules]="Select modules:"
T_EN[core]="Core (required)"
T_EN[drive]="Drive"
T_EN[hide]="Hide"
T_EN[petals]="Petals"
T_EN[shell]="Shell"
T_EN[brain]="Brain"
T_EN[server]="Server"
T_EN[all]="All"
T_EN[done]="Done ✓"
T_EN[err]="Error ✗"
T_EN[complete]="Installation Complete!"
T_EN[thanks]="Thank you for choosing Polygone!"
T_EN[cmd]="Run: polygone help"
T_EN[piper]="Pipe mode. Install Core? Press ENTER."
T_EN[auto]="Installing Polygone..."

declare -A T_FR
T_FR[title]="⬡ POLYGONE — Installeur"
T_FR[welcome]="Bienvenue! Langue:"
T_FR[modules]="Modules:"
T_FR[core]="Cœur (requis)"
T_FR[drive]="Drive"
T_FR[hide]="Hide"
T_FR[petals]="Petals"
T_FR[shell]="Shell"
T_FR[brain]="Brain"
T_FR[server]="Server"
T_FR[all]="Tout"
T_FR[done]="Fait ✓"
T_FR[err]="Erreur ✗"
T_FR[complete]="Installation Terminée!"
T_FR[thanks]="Merci d'avoir choisi Polygone!"
T_FR[cmd]="Lancez: polygone help"
T_FR[piper]="Mode pipe. Installer Core? ENTRÉE."
T_FR[auto]="Installation Polygone..."

declare -A T_DE
T_DE[title]="⬡ POLYGONE — Installationsprogramm"
T_DE[welcome]="Willkommen! Sprache:"
T_DE[modules]="Module:"
T_DE[core]="Kern (erforderlich)"
T_DE[drive]="Drive"
T_DE[hide]="Hide"
T_DE[petals]="Petals"
T_DE[shell]="Shell"
T_DE[brain]="Brain"
T_DE[server]="Server"
T_DE[all]="Alle"
T_DE[done]="Fertig ✓"
T_DE[err]="Fehler ✗"
T_DE[complete]="Installation abgeschlossen!"
T_DE[thanks]="Danke für die Wahl von Polygone!"
T_DE[cmd]="Ausführen: polygone help"
T_DE[piper]="Pipemodus. Kern installieren? ENTER."
T_DE[auto]="Polygone wird installiert..."

while [ $# -gt 0 ]; do
    case "$1" in
        --core)   sel_core=1 ;;
        --drive)  sel_drive=1 ;;
        --hide)   sel_hide=1 ;;
        --petals) sel_petals=1 ;;
        --shell)  sel_shell=1 ;;
        --brain)  sel_brain=1 ;;
        --server) sel_server=1 ;;
        --all)    sel_core=1 sel_drive=1 sel_hide=1 sel_petals=1 sel_shell=1 sel_brain=1 sel_server=1 ;;
        --lang)   shift; UI_LANG="$1" ;;
        --help|-h)
            echo "Usage: install.sh [OPTIONS]"
            echo "  --core --drive --hide --petals --shell --brain --server --all --lang en|fr|de"
            exit 0 ;;
    esac
    shift
done

# Check if we should use interactive mode
# Default to interactive, unless stdin is clearly piped (not a terminal)
INTERACTIVE=1
if [ ! -t 0 ] && [ -z "$*" ]; then
    INTERACTIVE=0
fi

if [ "$INTERACTIVE" = "1" ]; then
    clear
    echo ""
    echo "  $(t title)"
    echo ""
    echo "  $(t welcome)"
    echo "  [1] English"
    echo "  [2] Français"
    echo "  [3] Deutsch"
    echo ""
    echo ""
    echo -n "  Choice [1-3]: "
    read choice
    case "$choice" in
        2) UI_LANG="fr" ;;
        3) UI_LANG="de" ;;
    esac

    clear
    echo ""
    echo "  $(t title)"
    echo ""
    echo "  $(t modules)"
    echo ""
    echo "  [1] $(t core)          [✓]"
    echo "  [2] $(t drive)         [ ]"
    echo "  [3] $(t hide)          [ ]"
    echo "  [4] $(t petals)        [ ]"
    echo "  [5] $(t shell)         [ ]"
    echo "  [6] $(t brain)         [ ]"
    echo "  [7] $(t server)        [ ]"
    echo ""
    echo "  [A] $(t all)"
    echo "  [ENTER] Install"
    echo ""
    echo -n "  Choice: "
    read choice
    case "$choice" in
        2) sel_drive=1 ;;
        3) sel_hide=1 ;;
        4) sel_petals=1 ;;
        5) sel_shell=1 ;;
        6) sel_brain=1 ;;
        7) sel_server=1 ;;
        a|A) sel_core=1 sel_drive=1 sel_hide=1 sel_petals=1 sel_shell=1 sel_brain=1 sel_server=1 ;;
    esac
else
    clear
    echo ""
    echo "  $(t title)"
    echo ""
    echo "  $(t piper)"
    echo ""
    read -r
fi

clear
echo ""
echo "  $(t title)"
echo "  $(t auto)"
echo ""

mkdir -p "$INSTALL_DIR" "$ECOSYSTEM_DIR"

echo -n "  Core... "
OS=$(uname -s)
ARCH=$(uname -m)
case "$OS" in
    Linux)
        case "$ARCH" in
            x86_64)
                wget -q "https://github.com/lvs0/Polygone/releases/download/$LATEST_VERSION/polygone-x86_64-linux" -O "$INSTALL_DIR/polygone" && \
                chmod +x "$INSTALL_DIR/polygone" && \
                ACTUAL_SHA256=$(sha256sum "$INSTALL_DIR/polygone" 2>/dev/null | cut -d' ' -f1) && \
                EXPECTED_SHA256="${SHA256_SUMS[polygone-x86_64-linux]}" && \
                if [ -n "$EXPECTED_SHA256" ] && [ "$ACTUAL_SHA256" != "$EXPECTED_SHA256" ]; then \
                    printf "%bSHA256 Mismatch! Removing corrupted binary.%b\n" "$RED" "$NC" && \
                    rm -f "$INSTALL_DIR/polygone" && \
                    printf "%bError ✗%b\n" "$RED" "$NC"; \
                else \
                    printf "%bDone ✓%b\n" "$GREEN" "$NC"; \
                fi || printf "%bError ✗%b\n" "$RED" "$NC"
                ;;
            *) echo "Unsupported" ;;
        esac
        ;;
    *) echo "OS not supported" ;;
esac

[ $sel_drive -eq 1 ] && {
    echo -n "  Drive... "
    git clone -q --depth 1 "https://github.com/lvs0/Polygone-Drive" "$ECOSYSTEM_DIR/Polygone-Drive" 2>/dev/null && printf "%bDone ✓%b\n" "$GREEN" "$NC" || printf "%bError ✗%b\n" "$RED" "$NC"
}

[ $sel_hide -eq 1 ] && {
    echo -n "  Hide... "
    git clone -q --depth 1 "https://github.com/lvs0/Polygone-Hide" "$ECOSYSTEM_DIR/Polygone-Hide" 2>/dev/null && printf "%bDone ✓%b\n" "$GREEN" "$NC" || printf "%bError ✗%b\n" "$RED" "$NC"
}

[ $sel_petals -eq 1 ] && {
    echo -n "  Petals... "
    git clone -q --depth 1 "https://github.com/lvs0/Polygone-Petals" "$ECOSYSTEM_DIR/Polygone-Petals" 2>/dev/null && printf "%bDone ✓%b\n" "$GREEN" "$NC" || printf "%bError ✗%b\n" "$RED" "$NC"
}

[ $sel_shell -eq 1 ] && {
    echo -n "  Shell... "
    git clone -q --depth 1 "https://github.com/lvs0/Polygone-Shell" "$ECOSYSTEM_DIR/Polygone-Shell" 2>/dev/null && printf "%bDone ✓%b\n" "$GREEN" "$NC" || printf "%bError ✗%b\n" "$RED" "$NC"
}

[ $sel_brain -eq 1 ] && {
    echo -n "  Brain... "
    git clone -q --depth 1 "https://github.com/lvs0/Polygone-Brain" "$ECOSYSTEM_DIR/Polygone-Brain" 2>/dev/null && printf "%bDone ✓%b\n" "$GREEN" "$NC" || printf "%bError ✗%b\n" "$RED" "$NC"
}

[ $sel_server -eq 1 ] && {
    echo -n "  Server... "
    git clone -q --depth 1 "https://github.com/lvs0/Polygone-Server" "$ECOSYSTEM_DIR/Polygone-Server" 2>/dev/null && printf "%bDone ✓%b\n" "$GREEN" "$NC" || printf "%bError ✗%b\n" "$RED" "$NC"
}

for rc in "$HOME/.bashrc" "$HOME/.zshrc"; do
    [ -f "$rc" ] && ! grep -q '.local/bin' "$rc" && echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$rc"
done

echo ""
printf "  %b%s%b\n" "$GREEN" "$(t complete)" "$NC"
echo ""
echo "  $(t thanks)"
printf "  %b%s%b\n" "$CYAN" "$(t cmd)" "$NC"
echo ""
