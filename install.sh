#!/bin/bash
# ⬡ POLYGONE — Interactive TUI Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash

set -e

# ═══════════════════════════════════════════════════════════════════════════
# CONFIGURATION
# ═══════════════════════════════════════════════════════════════════════════
INSTALL_DIR="$HOME/.local/bin"
ECOSYSTEM_DIR="$HOME/Polygone-Ecosystem"
GITHUB_API="https://api.github.com/repos/lvs0/Polygone/releases/latest"

# ═══════════════════════════════════════════════════════════════════════════
# COLORS (ANSI)
# ═══════════════════════════════════════════════════════════════════════════
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
WHITE='\033[1;37m'
DIM='\033[2m'
BOLD='\033[1m'
NC='\033[0m'

# Box drawing characters
VERT="│"
HORIZ="─"
TOP_LEFT="┌"
TOP_RIGHT="┐"
BOT_LEFT="└"
BOT_RIGHT="┘"
CROSS="┼"
T_RIGHT="├"
T_LEFT="┤"
T_DOWN="┬"
T_UP="┴"
BULLET="•"

# ═══════════════════════════════════════════════════════════════════════════
# LANGUAGE SYSTEM
# ═══════════════════════════════════════════════════════════════════════════
UI_LANG="en"

declare -A LANG_EN
LANG_EN[title]="⬡ POLYGONE — Installer"
LANG_EN[subtitle]="Post-Quantum Ephemeral Network"
LANG_EN[welcome]="Welcome to Polygone Installer"
LANG_EN[select_lang]="Select Language"
LANG_EN[select_modules]="Select Modules to Install"
LANG_EN[installing]="Installing..."
LANG_EN[complete]="Installation Complete!"
LANG_EN[exit_msg]="Information does not exist. It traverses."
LANG_EN[core]="Core"
LANG_EN[core_desc]="Post-quantum encryption & routing"
LANG_EN[drive]="Drive"
LANG_EN[drive_desc]="Sharded distributed storage"
LANG_EN[hide]="Hide"
LANG_EN[hide_desc]="SOCKS5 VPN tunnel"
LANG_EN[petals]="Petals"
LANG_EN[petals_desc]="Collaborative LLM inference"
LANG_EN[shell]="Shell"
LANG_EN[shell_desc]="Interactive TUI dashboard"
LANG_EN[brain]="Brain"
LANG_EN[brain_desc]="AI swarm diagnostics"
LANG_EN[server]="Server"
LANG_EN[server_desc]="Backend server components"
LANG_EN[all_modules]="All Modules"
LANG_EN[next]="Next →"
LANG_EN[back]="← Back"
LANG_EN[install]="Install"
LANG_EN[cancel]="Cancel"
LANG_EN[quit]="Quit"
LANG_EN[downloading]="Downloading Polygone Core..."
LANG_EN[downloading_mod]="Downloading"
LANG_EN[cloning]="Cloning repository..."
LANG_EN[configuring]="Configuring..."
LANG_EN[done]="Done ✓"
LANG_EN[error]="Error ✗"
LANG_EN[success]="Success!"
LANG_EN[press_enter]="Press ENTER to continue..."
LANG_EN[checking]="Checking..."
LANG_EN[already_installed]="Already installed"
LANG_EN[skipping]="Skipping"
LANG_EN[thanks]="Thank you for choosing Polygone!"
LANG_EN[launch_cmd]="Run: polygone help"
LANG_EN[update_available]="Update available"
LANG_EN[up_to_date]="Up to date"
LANG_EN[yes]="Yes"
LANG_EN[no]="No"
LANG_EN[ok]="OK"
LANG_EN[canceled]="Canceled"
LANG_EN[removing]="Removing..."
LANG_EN[removed]="Removed"

declare -A LANG_FR
LANG_FR[title]="⬡ POLYGONE — Installeur"
LANG_FR[subtitle]="Réseau Éphémère Post-Quantique"
LANG_FR[welcome]="Bienvenue dans l'Installeur Polygone"
LANG_FR[select_lang]="Sélectionner la Langue"
LANG_FR[select_modules]="Sélectionner les Modules"
LANG_FR[installing]="Installation en cours..."
LANG_FR[complete]="Installation Terminée!"
LANG_FR[exit_msg]="L'information n'existe pas. Elle traverse."
LANG_FR[core]="Cœur"
LANG_FR[core_desc]="Chiffrement post-quantique & routage"
LANG_FR[drive]="Drive"
LANG_FR[drive_desc]="Stockage distribué fragmenté"
LANG_FR[hide]="Hide"
LANG_FR[hide_desc]="Tunnel VPN SOCKS5"
LANG_FR[petals]="Petals"
LANG_FR[petals_desc]="Inférence LLM collaborative"
LANG_FR[shell]="Shell"
LANG_FR[shell_desc]="Dashboard TUI interactif"
LANG_FR[brain]="Brain"
LANG_FR[brain_desc]="Diagnostics IA de l'essaim"
LANG_FR[server]="Server"
LANG_FR[server_desc]="Composants serveur backend"
LANG_FR[all_modules]="Tous les Modules"
LANG_FR[next]="Suivant →"
LANG_FR[back]="← Retour"
LANG_FR[install]="Installer"
LANG_FR[cancel]="Annuler"
LANG_FR[quit]="Quitter"
LANG_FR[downloading]="Téléchargement du Cœur Polygone..."
LANG_FR[downloading_mod]="Téléchargement"
LANG_FR[cloning]="Clonage du dépôt..."
LANG_FR[configuring]="Configuration..."
LANG_FR[done]="Fait ✓"
LANG_FR[error]="Erreur ✗"
LANG_FR[success]="Succès!"
LANG_FR[press_enter]="Appuyez sur ENTRÉE pour continuer..."
LANG_FR[checking]="Vérification..."
LANG_FR[already_installed]="Déjà installé"
LANG_FR[skipping]="Ignoré"
LANG_FR[thanks]="Merci d'avoir choisi Polygone!"
LANG_FR[launch_cmd]="Lancez: polygone help"
LANG_FR[update_available]="Mise à jour disponible"
LANG_FR[up_to_date]="À jour"
LANG_FR[yes]="Oui"
LANG_FR[no]="Non"
LANG_FR[ok]="OK"
LANG_FR[canceled]="Annulé"
LANG_FR[removing]="Suppression..."
LANG_FR[removed]="Supprimé"

declare -A LANG_DE
LANG_DE[title]="⬡ POLYGONE — Installationsprogramm"
LANG_DE[subtitle]="Post-Quantisches ephemeres Netzwerk"
LANG_DE[welcome]="Willkommen beim Polygone Installer"
LANG_DE[select_lang]="Sprache auswählen"
LANG_DE[select_modules]="Module auswählen"
LANG_DE[installing]="Installation..."
LANG_DE[complete]="Installation abgeschlossen!"
LANG_DE[exit_msg]="Information existiert nicht. Sie durchquert."
LANG_DE[core]="Kern"
LANG_DE[core_desc]="Post-quanten Verschlüsselung & Routing"
LANG_DE[drive]="Drive"
LANG_DE[drive_desc]="Verteilte fragmentierte Speicherung"
LANG_DE[hide]="Hide"
LANG_DE[hide_desc]="SOCKS5 VPN Tunnel"
LANG_DE[petals]="Petals"
LANG_DE[petals_desc]="Kollaborative LLM Inferenz"
LANG_DE[shell]="Shell"
LANG_DE[shell_desc]="Interaktives TUI Dashboard"
LANG_DE[brain]="Brain"
LANG_DE[brain_desc]="KI Schwarmdiagnose"
LANG_DE[server]="Server"
LANG_DE[server_desc]="Backend Server Komponenten"
LANG_DE[all_modules]="Alle Module"
LANG_DE[next]="Weiter →"
LANG_DE[back]="← Zurück"
LANG_DE[install]="Installieren"
LANG_DE[cancel]="Abbrechen"
LANG_DE[quit]="Beenden"
LANG_DE[downloading]="Polygone Kern wird heruntergeladen..."
LANG_DE[downloading_mod]="Herunterladen"
LANG_DE[cloning]="Repository wird geklont..."
LANG_DE[configuring]="Konfiguration..."
LANG_DE[done]="Fertig ✓"
LANG_DE[error]="Fehler ✗"
LANG_DE[success]="Erfolg!"
LANG_DE[press_enter]="ENTER drücken um fortzufahren..."
LANG_DE[checking]="Überprüfung..."
LANG_DE[already_installed]="Bereits installiert"
LANG_DE[skipping]="Überspringen"
LANG_DE[thanks]="Danke für die Wahl von Polygone!"
LANG_DE[launch_cmd]="Ausführen: polygone help"
LANG_DE[update_available]="Update verfügbar"
LANG_DE[up_to_date]="Aktuell"
LANG_DE[yes]="Ja"
LANG_DE[no]="Nein"
LANG_DE[ok]="OK"
LANG_DE[canceled]="Abgebrochen"
LANG_DE[removing]="Entfernen..."
LANG_DE[removed]="Entfernt"

t() {
    local key="$1"
    case "$UI_LANG" in
        fr) echo "${LANG_FR[$key]}" ;;
        de) echo "${LANG_DE[$key]}" ;;
        *) echo "${LANG_EN[$key]}" ;;
    esac
}

# ═══════════════════════════════════════════════════════════════════════════
# TUI FUNCTIONS
# ═══════════════════════════════════════════════════════════════════════════

# Clear screen and hide cursor
clear_screen() {
    printf '\033[2J'
    printf '\033[H'
}

hide_cursor() {
    printf '\033[?25l'
}

show_cursor() {
    printf '\033[?25h'
}

move_to() {
    printf '\033[%d;%dH' "$1" "$2"
}

draw_box() {
    local width="$1"
    local height="$2"
    local x="$3"
    local y="$4"
    
    move_to "$y" "$x"
    printf "${TOP_LEFT}%${width}s${TOP_RIGHT}" | tr ' ' "$HORIZ"
    
    for ((i=1; i<height-1; i++)); do
        move_to $((y+i)) "$x"
        printf "${VERT}"
        move_to $((y+i)) $((x+width+1))
        printf "${VERT}"
    done
    
    move_to $((y+height-1)) "$x"
    printf "${BOT_LEFT}%${width}s${BOT_RIGHT}" | tr ' ' "$HORIZ"
}

draw_title_box() {
    local width=60
    local height=8
    local y=2
    local x=$(( (80 - width) / 2 ))
    
    clear_screen
    
    # Main box
    move_to $y $x
    printf "${TOP_LEFT}"
    for ((i=0; i<width; i++)); do printf "$HORIZ"; done
    printf "${TOP_RIGHT}"
    
    for ((i=1; i<height-1; i++)); do
        move_to $((y+i)) $x
        printf "${VERT}"
        move_to $((y+i)) $((x+width+1))
        printf "${VERT}"
    done
    
    move_to $((y+height-1)) $x
    printf "${BOT_LEFT}"
    for ((i=0; i<width; i++)); do printf "$HORIZ"; done
    printf "${BOT_RIGHT}"
    
    # Title
    move_to $((y+1)) $((x+2))
    printf "${CYAN}${BOLD}⬡ POLYGONE${NC} ${WHITE}— Installer${NC}"
    
    move_to $((y+3)) $((x+2))
    printf "${DIM}%s${NC}" "$(t subtitle)"
    
    move_to $((y+5)) $((x+2))
    printf "${WHITE}%s${NC}" "$(t welcome)"
}

draw_menu() {
    local title="$1"
    local start_y="$2"
    local items=("${@:3}")
    local width=70
    local height=$(( ${#items[@]} / 2 + 6 ))
    local x=$(( (80 - width) / 2 ))
    local y="$start_y"
    
    # Box
    move_to $y $x
    printf "${TOP_LEFT}"
    for ((i=0; i<width; i++)); do printf "$HORIZ"; done
    printf "${TOP_RIGHT}"
    
    for ((i=1; i<height-1; i++)); do
        move_to $((y+i)) $x
        printf "${VERT}"
        move_to $((y+i)) $((x+width+1))
        printf "${VERT}"
    done
    
    move_to $((y+height-1)) $x
    printf "${BOT_LEFT}"
    for ((i=0; i<width; i++)); do printf "$HORIZ"; done
    printf "${BOT_RIGHT}"
    
    # Title
    move_to $((y+1)) $((x+2))
    printf "${BOLD}${WHITE}%s${NC}" "$title"
}

draw_progress_bar() {
    local current="$1"
    local total="$2"
    local width=50
    local percent=$((current * 100 / total))
    local filled=$((width * current / total))
    
    local x=$(( (80 - width) / 2 ))
    local y=20
    
    move_to $y $x
    printf "${CYAN}[${NC}"
    
    for ((i=0; i<filled; i++)); do
        printf "${GREEN}█${NC}"
    done
    for ((i=filled; i<width; i++)); do
        printf "${DIM}▒${NC}"
    done
    
    printf "${CYAN}] ${WHITE}%d%%${NC}" "$percent"
}

draw_checkbox() {
    local checked="$1"
    local text="$2"
    local x="$3"
    local y="$4"
    
    move_to $y $x
    if [ "$checked" = "1" ]; then
        printf "${GREEN}[✓]${NC} ${WHITE}%s${NC}" "$text"
    else
        printf "${DIM}[ ]${NC} %s" "$text"
    fi
}

# ═══════════════════════════════════════════════════════════════════════════
# SCREEN: LANGUAGE SELECTION
# ═══════════════════════════════════════════════════════════════════════════
screen_lang_selection() {
    while true; do
        clear_screen
        hide_cursor
        
        local width=50
        local height=15
        local x=$(( (80 - width) / 2 ))
        local y=8
        
        # Box
        move_to $y $x
        printf "${TOP_LEFT}"
        for ((i=0; i<width; i++)); do printf "$HORIZ"; done
        printf "${TOP_RIGHT}"
        
        for ((i=1; i<height-1; i++)); do
            move_to $((y+i)) $x
            printf "${VERT}"
            move_to $((y+i)) $((x+width+1))
            printf "${VERT}"
        done
        
        move_to $((y+height-1)) $x
        printf "${BOT_LEFT}"
        for ((i=0; i<width; i++)); do printf "$HORIZ"; done
        printf "${BOT_RIGHT}"
        
        # Title
        move_to $((y+1)) $((x+2))
        printf "${BOLD}${WHITE} $(t select_lang) ${NC}"
        
        # Options
        local options=("English" "Français" "Deutsch")
        local keys=("en" "fr" "de")
        
        for ((i=0; i<${#options[@]}; i++)); do
            local opt_y=$((y+4+i))
            move_to $opt_y $((x+2))
            if [ "$LANG" = "${keys[$i]}" ]; then
                printf "${GREEN}▶ %s${NC}" "${options[$i]}"
            else
                printf "  %s" "${options[$i]}"
            fi
        done
        
        # Footer
        move_to $((y+height-2)) $((x+2))
        printf "${DIM}↑↓ Navigate  |  ENTER Select${NC}"
        
        # Read key
        read -rsn1 key
        
        case "$key" in
            $'\x1b')
                read -rsn2 key
                case "$key" in
                    '[A') # Up
                        for ((i=0; i<${#keys[@]}; i++)); do
                            [ "$LANG" = "${keys[$i]}" ] && prev=$i
                        done
                        UI_LANG="${keys[$prev]}"
                        ;;
                    '[B') # Down
                        for ((i=0; i<${#keys[@]}; i++)); do
                            [ "$LANG" = "${keys[$i]}" ] && next=$(( (i+1) % ${#keys[@]} ))
                        done
                        UI_LANG="${keys[$next]}"
                        ;;
                esac
                ;;
            "") # Enter
                return 0
                ;;
            'q'|'Q')
                screen_quit
                exit 0
                ;;
        esac
    done
}

# ═══════════════════════════════════════════════════════════════════════════
# SCREEN: MODULE SELECTION
# ═══════════════════════════════════════════════════════════════════════════
screen_module_selection() {
    local modules=("core" "drive" "hide" "petals" "shell" "brain" "server")
    local repos=("Polygone" "Polygone-Drive" "Polygone-Hide" "Polygone-Petals" "Polygone-Shell" "Polygone-Brain" "Polygone-Server")
    
    # Default: all selected except core (always selected)
    declare -A selected
    selected[core]=1
    for mod in drive hide petals shell brain server; do
        selected[$mod]=0
    done
    
    local cursor=0
    local num_options=9  # 7 modules + All + Next
    
    while true; do
        clear_screen
        hide_cursor
        
        local width=70
        local height=$((num_options + 10))
        local x=$(( (80 - width) / 2 ))
        local y=3
        
        # Header Box
        move_to $y $x
        printf "${TOP_LEFT}"
        for ((i=0; i<width; i++)); do printf "$HORIZ"; done
        printf "${TOP_RIGHT}"
        
        for ((i=1; i<6; i++)); do
            move_to $((y+i)) $x
            printf "${VERT}"
            move_to $((y+i)) $((x+width+1))
            printf "${VERT}"
        done
        
        move_to $((y+6)) $x
        printf "${T_RIGHT}"
        for ((i=0; i<width; i++)); do printf "$HORIZ"; done
        printf "${T_LEFT}"
        
        # Title
        move_to $((y+1)) $((x+2))
        printf "${BOLD}${WHITE} $(t select_modules) ${NC}"
        
        # Module list
        local start_y=$((y+4))
        for ((i=0; i<${#modules[@]}; i++)); do
            local mod_y=$((start_y+i))
            local mod="${modules[$i]}"
            
            move_to $mod_y $((x+2))
            if [ "$cursor" = "$i" ]; then
                printf "${CYAN}▶${NC} "
            else
                printf "  "
            fi
            
            if [ "${selected[$mod]}" = "1" ]; then
                printf "${GREEN}[✓]${NC} ${BOLD}${WHITE}$(t ${mod})${NC}"
            else
                printf "${DIM}[ ]${NC} $(t ${mod})"
            fi
            
            move_to $mod_y $((x+35))
            printf "${DIM}%s${NC}" "$(t ${mod}_desc)"
        done
        
        # All option
        local all_y=$((start_y+7))
        move_to $all_y $((x+2))
        if [ "$cursor" = "7" ]; then
            printf "${CYAN}▶${NC} "
        else
            printf "  "
        fi
        printf "${YELLOW}$(t all_modules)${NC}"
        
        # Next/Install option
        local next_y=$((start_y+8))
        move_to $next_y $((x+2))
        if [ "$cursor" = "8" ]; then
            printf "${CYAN}▶${NC} "
        else
            printf "  "
        fi
        printf "${GREEN}${BOLD}$(t install)${NC}"
        
        # Footer
        move_to $((y+height-1)) $((x+2))
        printf "${DIM}↑↓ Navigate  |  SPACE Select  |  ENTER $(t next)  |  Q $(t quit)${NC}"
        
        read -rsn1 key
        
        case "$key" in
            $'\x1b')
                read -rsn2 key
                case "$key" in
                    '[A') # Up
                        cursor=$(( (cursor - 1 + num_options) % num_options ))
                        ;;
                    '[B') # Down
                        cursor=$(( (cursor + 1) % num_options ))
                        ;;
                esac
                ;;
            " ")
                if [ "$cursor" -lt 7 ]; then
                    if [ "${modules[$cursor]}" != "core" ]; then
                        selected[${modules[$cursor]}]=$(( 1 - selected[${modules[$cursor]}] ))
                    fi
                fi
                ;;
            "")
                if [ "$cursor" = "7" ]; then
                    # All
                    for mod in "${modules[@]}"; do
                        selected[$mod]=1
                    done
                elif [ "$cursor" = "8" ]; then
                    # Install
                    return 0
                fi
                ;;
            'q'|'Q')
                screen_quit
                exit 0
                ;;
        esac
    done
}

# ═══════════════════════════════════════════════════════════════════════════
# SCREEN: INSTALLATION
# ═══════════════════════════════════════════════════════════════════════════
screen_installation() {
    clear_screen
    hide_cursor
    
    local modules=("core" "drive" "hide" "petals" "shell" "brain" "server")
    local repos=("Polygone" "Polygone-Drive" "Polygone-Hide" "Polygone-Petals" "Polygone-Shell" "Polygone-Brain" "Polygone-Server")
    local to_install=()
    
    # Count selected
    local count=0
    for mod in "${modules[@]}"; do
        if [ "${selected[$mod]}" = "1" ]; then
            ((count++))
        fi
    done
    
    # Header
    local width=70
    local x=$(( (80 - width) / 2 ))
    local y=5
    
    move_to $y $x
    printf "${TOP_LEFT}"
    for ((i=0; i<width; i++)); do printf "$HORIZ"; done
    printf "${TOP_RIGHT}"
    
    for ((i=1; i<3; i++)); do
        move_to $((y+i)) $x
        printf "${VERT}"
        move_to $((y+i)) $((x+width+1))
        printf "${VERT}"
    done
    
    move_to $((y+3)) $x
    printf "${T_RIGHT}"
    for ((i=0; i<width; i++)); do printf "$HORIZ"; done
    printf "${T_LEFT}"
    
    move_to $((y+1)) $((x+2))
    printf "${BOLD}${WHITE}$(t installing)${NC}"
    
    local idx=0
    for ((i=0; i<${#modules[@]}; i++)); do
        local mod="${modules[$i]}"
        local repo="${repos[$i]}"
        
        if [ "${selected[$mod]}" = "1" ]; then
            ((idx++))
            local line_y=$((y+6+i))
            
            move_to $line_y $((x+2))
            printf "${YELLOW}[%d/%d]${NC} $(t ${mod})" "$idx" "$count"
            
            # Progress bar
            local bar_x=$((x+35))
            move_to $line_y $bar_x
            printf "${DIM}[${NC}"
            for ((b=0; b<25; b++)); do printf "${DIM}▒${NC}"; done
            printf "${DIM}]${NC} ${DIM}0%%${NC}"
            
            # Perform installation
            move_to $line_y $((x+width-10))
            if [ "$mod" = "core" ]; then
                printf "${CYAN}%s${NC}" "$(t downloading)"
                
                # Download core
                mkdir -p "$INSTALL_DIR"
                
                # Get version
                LATEST_VERSION="v0.1.0"
                
                # Detect binary
                OS=$(uname -s)
                ARCH=$(uname -m)
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
                
                if [ -n "$BINARY_NAME" ]; then
                    local URL="https://github.com/lvs0/Polygone/releases/download/$LATEST_VERSION/$BINARY_NAME"
                    
                    move_to $line_y $((bar_x+2))
                    printf "${CYAN}█${NC}"
                    
                    wget -q "$URL" -O "$INSTALL_DIR/polygone" 2>/dev/null && {
                        chmod +x "$INSTALL_DIR/polygone"
                        
                        move_to $line_y $((x+width-15))
                        printf "${GREEN}%20s${NC}" "$(t done)"
                    } || {
                        move_to $line_y $((x+width-15))
                        printf "${RED}%20s${NC}" "$(t error)"
                    }
                fi
            else
                printf "${CYAN}%s${NC}" "$(t cloning)"
                
                # Clone repo
                MODULE_DIR="$ECOSYSTEM_DIR/$repo"
                mkdir -p "$MODULE_DIR"
                
                if [ -d "$MODULE_DIR/.git" ]; then
                    (cd "$MODULE_DIR" && git pull -q origin main 2>/dev/null) && {
                        move_to $line_y $((x+width-15))
                        printf "${GREEN}%20s${NC}" "$(t done)"
                    }
                else
                    git clone -q --depth 1 "https://github.com/lvs0/$repo" "$MODULE_DIR" 2>/dev/null && {
                        move_to $line_y $((x+width-15))
                        printf "${GREEN}%20s${NC}" "$(t done)"
                    }
                fi
            fi
        fi
    done
    
    # Add to PATH
    local shell_rc="$HOME/.bashrc"
    [ -f "$HOME/.zshrc" ] && shell_rc="$HOME/.zshrc"
    
    if ! grep -q '.local/bin' "$shell_rc" 2>/dev/null; then
        echo "" >> "$shell_rc"
        echo '# Polygone' >> "$shell_rc"
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$shell_rc"
    fi
    
    # Footer
    move_to $((y+18)) $((x+2))
    printf "${GREEN}${BOLD}$(t complete)${NC}"
    
    move_to $((y+20)) $((x+2))
    printf "${CYAN}$(t thanks)${NC}"
    
    move_to $((y+22)) $((x+2))
    printf "${WHITE}$(t launch_cmd)${NC}"
    
    move_to $((y+24)) $((x+2))
    printf "${DIM}$(t exit_msg)${NC}"
    
    move_to $((y+26)) $((x+2))
    printf "${DIM}Press ENTER to exit...${NC}"
    
    show_cursor
    read -rsn1
}

# ═══════════════════════════════════════════════════════════════════════════
# SCREEN: QUIT
# ═══════════════════════════════════════════════════════════════════════════
screen_quit() {
    clear_screen
    printf '\033[H'
    printf "\n"
    printf "  ${CYAN}⬡ POLYGONE${NC}\n"
    printf "\n"
    printf "  $(t exit_msg)\n"
    printf "\n"
    show_cursor
}

# ═══════════════════════════════════════════════════════════════════════════
# MAIN
# ═══════════════════════════════════════════════════════════════════════════
main() {
    # Trap for cleanup
    trap 'show_cursor; exit 1' INT TERM
    
    # Check if terminal supports ANSI
    if ! command -v tput &> /dev/null || ! tput colors &> /dev/null; then
        # Fallback for limited terminals
        exec bash "$0" --no-tui "$@"
    fi
    
    # Main flow
    clear_screen
    hide_cursor
    
    # Splash screen
    local splash="
    
    
    
        ${CYAN}${BOLD}
        ██████╗  █████╗ ██╗ █████╗ 
        ██╔══██╗██╔══██╗██║██╔══██╗
        ██║  ██║███████║██║███████║
        ██║  ██║██╔══██║██║██╔══██║
        ██████╔╝██║  ██║██║██║  ██║
        ╚═════╝ ╚═╝  ╚═╝╚═╝╚═╝  ╚═╝
        ${NC}
        
        ${WHITE}Post-Quantum Ephemeral Network${NC}
        
        ${DIM}Loading installer...${NC}
    
    "
    printf "$splash"
    sleep 1
    
    # Language selection
    screen_lang_selection
    
    # Module selection
    screen_module_selection
    
    # Installation
    screen_installation
    
    # Cleanup
    show_cursor
    clear_screen
}

# Fallback: Simple text mode
main_text() {
    echo ""
    echo "  ⬡ POLYGONE — Installer"
    echo ""
    echo "  Language / Langue / Sprache:"
    echo "    [1] English"
    echo "    [2] Français"
    echo "    [3] Deutsch"
    echo ""
    read -p "  Choice: " choice
    
    case "$choice" in
        2) UI_LANG="fr" ;;
        3) UI_LANG="de" ;;
        *) UI_LANG="en" ;;
    esac
    
    echo ""
    echo "  $(t select_modules):"
    echo "    [1] $(t core) - $(t core_desc)"
    echo "    [2] $(t drive) - $(t drive_desc)"
    echo "    [3] $(t hide) - $(t hide_desc)"
    echo "    [4] $(t petals) - $(t petals_desc)"
    echo "    [5] $(t shell) - $(t shell_desc)"
    echo "    [6] $(t brain) - $(t brain_desc)"
    echo "    [7] $(t server) - $(t server_desc)"
    echo "    [A] $(t all_modules)"
    echo ""
    read -p "  $(t install)? [Y/n]: " confirm
    
    if [ "$confirm" != "n" ] && [ "$confirm" != "N" ]; then
        echo ""
        echo "  $(t installing)"
        echo ""
        
        # Install all by default in text mode
        mkdir -p "$INSTALL_DIR" "$ECOSYSTEM_DIR"
        
        LATEST_VERSION="v0.1.0"
        OS=$(uname -s)
        ARCH=$(uname -m)
        
        case "$OS" in
            Linux)
                case "$ARCH" in
                    x86_64) wget -q "https://github.com/lvs0/Polygone/releases/download/$LATEST_VERSION/polygone-x86_64-linux" -O "$INSTALL_DIR/polygone" && chmod +x "$INSTALL_DIR/polygone" && echo "  ✓ Core" ;;
                esac
                ;;
        esac
        
        for repo in Polygone-Drive Polygone-Hide Polygone-Petals Polygone-Shell Polygone-Brain Polygone-Server; do
            git clone -q --depth 1 "https://github.com/lvs0/$repo" "$ECOSYSTEM_DIR/$repo" 2>/dev/null && echo "  ✓ $repo"
        done
        
        echo ""
        echo "  ${GREEN}$(t complete)${NC}"
        echo "  $(t thanks)"
        echo "  $(t launch_cmd)"
    else
        echo ""
        echo "  $(t canceled)"
    fi
    
    echo ""
    echo "  $(t exit_msg)"
    echo ""
}

# Entry point
if [ "$1" = "--no-tui" ]; then
    shift
    main_text "$@"
else
    main "$@"
fi
