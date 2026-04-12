#!/bin/bash
# ⬡ POLYGONE — Interactive Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash

set -e

INSTALL_DIR="$HOME/.local/bin"
ECOSYSTEM_DIR="$HOME/.Polygone-Ecosystem"
LATEST_VERSION="v0.1.0"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
DIM='\033[2m'
BOLD='\033[1m'
NC='\033[0m'

# Translations
UI_LANG="en"

declare -A T
T[en_title]="⬡ POLYGONE — Installer"
T[fr_title]="⬡ POLYGONE — Installeur"
T[de_title]="⬡ POLYGONE — Installer"
T[en_subtitle]="Post-Quantum Ephemeral Network"
T[fr_subtitle]="Réseau Éphémère Post-Quantique"
T[de_subtitle]="Post-Quantisches Netzwerk"
T[en_welcome]="Welcome! Select your language:"
T[fr_welcome]="Bienvenue! Sélectionnez votre langue:"
T[de_welcome]="Willkommen! Wählen Sie Ihre Sprache:"
T[en_modules]="Select modules to install:"
T[fr_modules]="Sélectionnez les modules à installer:"
T[de_modules]="Module auswählen:"
T[en_core]="Core (required)"
T[fr_core]="Cœur (requis)"
T[de_core]="Kern (erforderlich)"
T[en_drive]="Drive — Distributed storage"
T[fr_drive]="Drive — Stockage distribué"
T[de_drive]="Drive — Verteilte Speicherung"
T[en_hide]="Hide — VPN tunnel"
T[fr_hide]="Hide — Tunnel VPN"
T[de_hide]="Hide — VPN Tunnel"
T[en_petals]="Petals — LLM inference"
T[fr_petals]="Petals — Inférence LLM"
T[de_petals]="Petals — LLM Inferenz"
T[en_shell]="Shell — TUI dashboard"
T[fr_shell]="Shell — Dashboard"
T[de_shell]="Shell — TUI Dashboard"
T[en_brain]="Brain — AI diagnostics"
T[fr_brain]="Brain — Diagnostics IA"
T[de_brain]="Brain — KI Diagnose"
T[en_server]="Server — Backend"
T[fr_server]="Server — Backend"
T[de_server]="Server — Backend"
T[en_all]="All modules"
T[fr_all]="Tous les modules"
T[de_all]="Alle Module"
T[en_next]="Next →"
T[fr_next]="Suivant →"
T[de_next]="Weiter →"
T[en_done]="Done ✓"
T[fr_done]="Fait ✓"
T[de_done]="Fertig ✓"
T[en_err]="Error ✗"
T[fr_err]="Erreur ✗"
T[de_err]="Fehler ✗"
T[en_complete]="Installation Complete!"
T[fr_complete]="Installation Terminée!"
T[de_complete]="Installation abgeschlossen!"
T[en_thanks]="Thank you for choosing Polygone!"
T[fr_thanks]="Merci d'avoir choisi Polygone!"
T[de_thanks]="Danke für die Wahl von Polygone!"
T[en_cmd]="Run: polygone help"
T[fr_cmd]="Lancez: polygone help"
T[de_cmd]="Ausführen: polygone help"

tr() {
    echo "${T[${UI_LANG}_$1]}"
}

# Clear screen
clear_screen() {
    printf '\033[2J\033[H'
}

hide_cursor() {
    printf '\033[?25l'
}

show_cursor() {
    printf '\033[?25h'
}

# Simple menu without fancy TUI
menu_language() {
    while true; do
        clear_screen
        echo ""
        echo "  $(tr title)"
        echo "  $(tr subtitle)"
        echo ""
        echo "  $(tr welcome)"
        echo ""
        echo "  [1] English"
        echo "  [2] Français"  
        echo "  [3] Deutsch"
        echo ""
        echo -n "  > "
        
        read -r choice
        case "$choice" in
            1) UI_LANG="en"; return 0 ;;
            2) UI_LANG="fr"; return 0 ;;
            3) UI_LANG="de"; return 0 ;;
        esac
    done
}

menu_modules() {
    # Default: core selected, others off
    sel_core=1
    sel_drive=0
    sel_hide=0
    sel_petals=0
    sel_shell=0
    sel_brain=0
    sel_server=0
    
    while true; do
        clear_screen
        echo ""
        echo "  $(tr title)"
        echo ""
        echo "  $(tr modules)"
        echo ""
        echo "  [1] $(tr core)          $([ $sel_core -eq 1 ] && echo "${GREEN}[✓]${NC}" || echo "[ ]")"
        echo "  [2] $(tr drive)         $([ $sel_drive -eq 1 ] && echo "${GREEN}[✓]${NC}" || echo "[ ]")"
        echo "  [3] $(tr hide)           $([ $sel_hide -eq 1 ] && echo "${GREEN}[✓]${NC}" || echo "[ ]")"
        echo "  [4] $(tr petals)        $([ $sel_petals -eq 1 ] && echo "${GREEN}[✓]${NC}" || echo "[ ]")"
        echo "  [5] $(tr shell)          $([ $sel_shell -eq 1 ] && echo "${GREEN}[✓]${NC}" || echo "[ ]")"
        echo "  [6] $(tr brain)          $([ $sel_brain -eq 1 ] && echo "${GREEN}[✓]${NC}" || echo "[ ]")"
        echo "  [7] $(tr server)         $([ $sel_server -eq 1 ] && echo "${GREEN}[✓]${NC}" || echo "[ ]")"
        echo ""
        echo "  [A] $(tr all)"
        echo "  [ENTER] $(tr next)"
        echo ""
        echo "  Q = Quit"
        echo ""
        echo -n "  > "
        
        read -r choice
        case "$choice" in
            1) sel_core=$((1 - sel_core)) ;;
            2) sel_drive=$((1 - sel_drive)) ;;
            3) sel_hide=$((1 - sel_hide)) ;;
            4) sel_petals=$((1 - sel_petals)) ;;
            5) sel_shell=$((1 - sel_shell)) ;;
            6) sel_brain=$((1 - sel_brain)) ;;
            7) sel_server=$((1 - sel_server)) ;;
            a|A)
                sel_core=1; sel_drive=1; sel_hide=1; sel_petals=1
                sel_shell=1; sel_brain=1; sel_server=1
                ;;
            "")
                return 0
                ;;
            q|Q)
                clear_screen
                echo ""
                echo "  $(tr title)"
                echo "  L'information n'existe pas. Elle traverse."
                echo ""
                exit 0
                ;;
        esac
    done
}

do_install() {
    clear_screen
    echo ""
    echo "  $(tr title)"
    echo "  Installing..."
    echo ""
    
    mkdir -p "$INSTALL_DIR"
    
    # Download core
    echo -n "  Core... "
    OS=$(uname -s)
    ARCH=$(uname -m)
    case "$OS" in
        Linux)
            case "$ARCH" in
                x86_64)
                    wget -q "https://github.com/lvs0/Polygone/releases/download/$LATEST_VERSION/polygone-x86_64-linux" -O "$INSTALL_DIR/polygone" 2>/dev/null
                    chmod +x "$INSTALL_DIR/polygone"
                    echo "$(tr done)"
                    ;;
                *)
                    echo "Unsupported architecture: $ARCH"
                    ;;
            esac
            ;;
        Darwin)
            echo "macOS support coming soon..."
            ;;
        *)
            echo "Unsupported OS: $OS"
            ;;
    esac
    
    # Clone modules
    [ $sel_drive -eq 1 ] && {
        echo -n "  Drive... "
        git clone -q --depth 1 "https://github.com/lvs0/Polygone-Drive" "$ECOSYSTEM_DIR/Polygone-Drive" 2>/dev/null && echo "$(tr done)"
    }
    
    [ $sel_hide -eq 1 ] && {
        echo -n "  Hide... "
        git clone -q --depth 1 "https://github.com/lvs0/Polygone-Hide" "$ECOSYSTEM_DIR/Polygone-Hide" 2>/dev/null && echo "$(tr done)"
    }
    
    [ $sel_petals -eq 1 ] && {
        echo -n "  Petals... "
        git clone -q --depth 1 "https://github.com/lvs0/Polygone-Petals" "$ECOSYSTEM_DIR/Polygone-Petals" 2>/dev/null && echo "$(tr done)"
    }
    
    [ $sel_shell -eq 1 ] && {
        echo -n "  Shell... "
        git clone -q --depth 1 "https://github.com/lvs0/Polygone-Shell" "$ECOSYSTEM_DIR/Polygone-Shell" 2>/dev/null && echo "$(tr done)"
    }
    
    [ $sel_brain -eq 1 ] && {
        echo -n "  Brain... "
        git clone -q --depth 1 "https://github.com/lvs0/Polygone-Brain" "$ECOSYSTEM_DIR/Polygone-Brain" 2>/dev/null && echo "$(tr done)"
    }
    
    [ $sel_server -eq 1 ] && {
        echo -n "  Server... "
        git clone -q --depth 1 "https://github.com/lvs0/Polygone-Server" "$ECOSYSTEM_DIR/Polygone-Server" 2>/dev/null && echo "$(tr done)"
    }
    
    # Add to PATH
    for rc in "$HOME/.bashrc" "$HOME/.zshrc"; do
        [ -f "$rc" ] && ! grep -q '.local/bin' "$rc" && echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$rc"
    done
    
    echo ""
    echo "  ${GREEN}$(tr complete)${NC}"
    echo ""
    echo "  $(tr thanks)"
    echo "  ${CYAN}$(tr cmd)${NC}"
    echo ""
}

# Main
trap 'show_cursor; exit 1' INT

clear_screen
hide_cursor

menu_language
menu_modules
do_install

show_cursor
echo -n "Press ENTER to exit... "
read -r
clear_screen
