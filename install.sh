#!/bin/bash
# ⬡ POLYGONE — Enhanced Interactive Installer
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/lvs0/Polygone/main/install.sh | bash
#   ./install.sh --all --docker --systemd
#   ./install.sh --core --custom-dir=/opt/polygone

set -euo pipefail

INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
ECOSYSTEM_DIR="${ECOSYSTEM_DIR:-$HOME/.polygone}"
LATEST_VERSION="v0.1.0"
REPO_URL="https://github.com/lvs0/Polygone"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

opt_core=1 opt_drive=0 opt_hide=0 opt_petals=0 opt_shell=0 opt_brain=0 opt_server=0 opt_compute=0
opt_systemd=0 opt_background=0 opt_custom_dir="" opt_verify=1 opt_expert=0

declare -A T_EN
T_EN[title]="⬡ POLYGONE — Enhanced Installer"
T_EN[welcome]="Welcome! This installer will set up Polygone ecosystem."
T_EN[modules]="Select modules to install:"
T_EN[core]="Core (required) — P2P networking, crypto"
T_EN[drive]="Drive — Decentralized encrypted storage"
T_EN[hide]="Hide — Anonymous browsing"
T_EN[petals]="Petals — AI/ML inference network"
T_EN[shell]="Shell — Command-line interface"
T_EN[brain]="Brain — AI reasoning engine"
T_EN[server]="Server — Full node for hosting"
T_EN[compute]="Compute — Distributed idle power"
T_EN[install_type]="Installation Type:"
T_EN[standard]="Standard — Local user installation"
T_EN[systemwide]="System-wide — Install for all users"
T_EN[docker]="Docker — Run in container"
T_EN[custom_dir]="Custom directory"
T_EN[options]="Additional Options:"
T_EN[background]="Run as background daemon"
T_EN[systemd]="Install systemd service"
T_EN[expert]="Expert mode (skip verifications)"
T_EN[verify]="Verify checksums"
T_EN[language]="Select language:"
T_EN[done]="Installation Complete!"
T_EN[thanks]="Thank you for choosing Polygone!"
T_EN[run_cmd]="Run: polygone help"
T_EN[start_daemon]="Start daemon: polygone daemon start"
T_EN[start_docker]="Start Docker: docker run -d polygone"
T_EN[error]="Error"
T_EN[success]="Success"
T_EN[skip]="Skipped"
T_EN[installing]="Installing..."
T_EN[exit_menu]="Exit"
T_EN[back]="Back"
T_EN[confirm]="Confirm Installation"
T_EN[summary]="Installation Summary"
T_EN[already]="already installed"

declare -A T_FR
T_FR[title]="⬡ POLYGONE — Installeur Amélioré"
T_FR[welcome]="Bienvenue! Cet installeur configurera l'écosystème Polygone."
T_FR[modules]="Sélectionnez les modules à installer:"
T_FR[core]="Cœur (requis) — Réseau P2P, cryptographie"
T_FR[drive]="Drive — Stockage chiffré décentralisé"
T_FR[hide]="Hide — Navigation anonyme"
T_FR[petals]="Petals — Réseau IA/ML"
T_FR[shell]="Shell — Interface ligne de commande"
T_FR[brain]="Brain — Moteur de raisonnement IA"
T_FR[server]="Server — Nœud complet pour hébergement"
T_FR[install_type]="Type d'installation:"
T_FR[standard]="Standard — Installation utilisateur local"
T_FR[systemwide]="Système — Installer pour tous les utilisateurs"
T_FR[docker]="Docker — Exécuter en conteneur"
T_FR[custom_dir]="Répertoire personnalisé"
T_FR[options]="Options additionnelles:"
T_FR[background]="Exécuter en arrière-plan"
T_FR[systemd]="Installer service systemd"
T_FR[expert]="Mode expert (sans vérification)"
T_FR[verify]="Vérifier les sommes de contrôle"
T_FR[language]="Sélectionnez la langue:"
T_FR[done]="Installation Terminée!"
T_FR[thanks]="Merci d'avoir choisi Polygone!"
T_FR[run_cmd]="Lancez: polygone help"
T_FR[start_daemon]="Démarrer daemon: polygone daemon start"
T_FR[start_docker]="Démarrer Docker: docker run -d polygone"
T_FR[error]="Erreur"
T_FR[success]="Succès"
T_FR[skip]="Ignoré"
T_FR[installing]="Installation..."
T_FR[exit_menu]="Quitter"
T_FR[back]="Retour"
T_FR[confirm]="Confirmer l'installation"
T_FR[summary]="Résumé de l'installation"
T_FR[already]="déjà installé"

UI_LANG="en"
LANG_SELECTION_DONE=0

t() {
    local key="${1}"
    local val
    case "$UI_LANG" in
        fr) val="${T_FR[$key]}" ;;
        *) val="${T_EN[$key]}" ;;
    esac
    echo "${val:-$1}"
}

usage() {
    cat << EOF
${BOLD}Usage:${NC} $0 [OPTIONS]

${BOLD}Module Options:${NC}
  --core          Install core (default)
  --drive         Install Drive module
  --hide          Install Hide module
  --petals        Install Petals module
  --shell         Install Shell module
  --brain         Install Brain module
  --server        Install Server module
  --compute       Install Compute module (distributed computing)
  --all           Install all modules

${BOLD}Installation Options:${NC}
  --docker        Install Docker container
  --systemd       Install systemd service
  --background    Run as background daemon
  --custom-dir    Custom installation directory
  --no-verify     Skip SHA256 verification
  --expert        Expert mode (minimal prompts)

${BOLD}Language Options:${NC}
  --lang LANG     Set language (en, fr)

${BOLD}Examples:${NC}
  $0 --all --docker --systemd
  $0 --core --server --custom-dir /opt/polygone
  curl -fsSL $REPO_URL/main/install.sh | bash

EOF
    exit 0
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --core)      opt_core=1 ;;
        --drive)     opt_drive=1 ;;
        --hide)      opt_hide=1 ;;
        --petals)    opt_petals=1 ;;
        --shell)     opt_shell=1 ;;
        --brain)     opt_brain=1 ;;
        --server)    opt_server=1 ;;
        --compute)   opt_compute=1 ;;
        --all)       opt_core=1 opt_drive=1 opt_hide=1 opt_petals=1 opt_shell=1 opt_brain=1 opt_server=1 opt_compute=1 ;;
        --systemd)   opt_systemd=1 ;;
        --background) opt_background=1 ;;
        --custom-dir) shift; opt_custom_dir="$1" ;;
        --no-verify) opt_verify=0 ;;
        --expert)    opt_expert=1 ;;
        --lang)      shift; UI_LANG="$1" ;;
        --help|-h)   usage ;;
        *)           echo "Unknown option: $1"; exit 1 ;;
    esac
    shift
done

if [[ -n "$opt_custom_dir" ]]; then
    INSTALL_DIR="$opt_custom_dir/bin"
    ECOSYSTEM_DIR="$opt_custom_dir"
fi

if [[ ! -t 0 ]] && [[ $opt_expert -eq 0 ]]; then
    echo "  $(t title)"
    echo "  ${YELLOW}Detected piped input. Installing core with defaults...${NC}"
    opt_core=1
    opt_drive=0
fi

show_language_menu() {
    clear
    echo ""
    echo -e "  ${BOLD}$(t title)${NC}"
    echo ""
    echo "  $(t language)"
    echo ""
    echo "  [1] English"
    echo "  [2] Français"
    echo ""
    echo -n "  Choice [1-2]: "
    read choice
    case "$choice" in
        2) UI_LANG="fr" ;;
    esac
}

show_main_menu() {
    local done=0
    while [[ $done -eq 0 ]]; do
        clear
        echo ""
        echo -e "  ${BOLD}$(t title)${NC}"
        echo ""
        echo "  $(t modules)"
        echo ""
        echo "  [1] $(t core)          [$(toggle $opt_core)]"
        echo "  [2] $(t drive)         [$(toggle $opt_drive)]"
        echo "  [3] $(t hide)          [$(toggle $opt_hide)]"
        echo "  [4] $(t petals)        [$(toggle $opt_petals)]"
        echo "  [5] $(t shell)         [$(toggle $opt_shell)]"
        echo "  [6] $(t brain)         [$(toggle $opt_brain)]"
        echo "  [7] $(t server)        [$(toggle $opt_server)]"
        echo ""
        echo "  [A] $(t all)"
        echo "  [S] $(t confirm)"
        echo "  [E] $(t exit_menu)"
        echo ""
        echo -n "  Choice: "
        read choice
        handle_main_choice "$choice"
    done
}

handle_main_choice() {
    case "$1" in
        1) opt_core=$(toggle $opt_core) ;;
        2) opt_drive=$(toggle $opt_drive) ;;
        3) opt_hide=$(toggle $opt_hide) ;;
        4) opt_petals=$(toggle $opt_petals) ;;
        5) opt_shell=$(toggle $opt_shell) ;;
        6) opt_brain=$(toggle $opt_brain) ;;
        7) opt_server=$(toggle $opt_server) ;;
        a|A) toggle_all ;;
        s|S) show_install_type_menu ;;
        e|E) echo "Exiting."; exit 0 ;;
    esac
}

toggle() { [[ $1 -eq 1 ]] && echo 0 || echo 1; }

toggle_all() {
    if [[ $opt_core -eq 1 ]] && [[ $opt_drive -eq 1 ]] && [[ $opt_hide -eq 1 ]] && \
       [[ $opt_petals -eq 1 ]] && [[ $opt_shell -eq 1 ]] && [[ $opt_brain -eq 1 ]] && \
       [[ $opt_server -eq 1 ]]; then
        opt_core=0 opt_drive=0 opt_hide=0 opt_petals=0 opt_shell=0 opt_brain=0 opt_server=0
    else
        opt_core=1 opt_drive=1 opt_hide=1 opt_petals=1 opt_shell=1 opt_brain=1 opt_server=1
    fi
}

show_install_type_menu() {
    clear
    echo ""
    echo -e "  ${BOLD}$(t title)${NC}"
    echo ""
    echo "  $(t install_type)"
    echo ""
    echo "  [1] $(t standard)"
    echo "  [2] $(t systemwide)"
    echo "  [3] $(t docker)"
    echo "  [4] $(t custom_dir)"
    echo ""
    echo "  $(t options)"
    echo "  [B] $(t background)         [$(toggle $opt_background)]"
    echo "  [D] $(t systemd)             [$(toggle $opt_systemd)]"
    echo "  [X] $(t expert)              [$(toggle $opt_expert)]"
    echo "  [V] $(t verify)             [$(toggle $opt_verify)]"
    echo ""
    echo "  [ENTER] $(t confirm)"
    echo "  [B] $(t back)"
    echo ""
    echo -n "  Choice: "
    read choice
    handle_install_choice "$choice"
}

handle_install_choice() {
    case "$1" in
        1) ;;
        2) [[ $EUID -ne 0 ]] && echo "Root required for system-wide install" ;;
        3) opt_docker=1 ;;
        4) echo -n "Directory: "; read opt_custom_dir ;;
        b|B) return ;;
        d|D) opt_systemd=$(toggle $opt_systemd) ;;
        x|X) opt_expert=$(toggle $opt_expert) ;;
        v|V) opt_verify=$(toggle $opt_verify) ;;
        "") show_summary ;;
        *) show_summary ;;
    esac
}

show_summary() {
    clear
    echo ""
    echo -e "  ${BOLD}$(t summary)${NC}"
    echo ""
    echo "  ${BOLD}Modules:${NC}"
    [[ $opt_core -eq 1 ]] && echo "    ✓ Core" || echo "    - Core"
    [[ $opt_drive -eq 1 ]] && echo "    ✓ Drive" || echo "    - Drive"
    [[ $opt_hide -eq 1 ]] && echo "    ✓ Hide" || echo "    - Hide"
    [[ $opt_petals -eq 1 ]] && echo "    ✓ Petals" || echo "    - Petals"
    [[ $opt_shell -eq 1 ]] && echo "    ✓ Shell" || echo "    - Shell"
    [[ $opt_brain -eq 1 ]] && echo "    ✓ Brain" || echo "    - Brain"
    [[ $opt_server -eq 1 ]] && echo "    ✓ Server" || echo "    - Server"
    echo ""
    echo "  ${BOLD}Options:${NC}"
    echo "    Directory: $INSTALL_DIR"
    [[ $opt_docker -eq 1 ]] && echo "    ✓ Docker" || echo "    - Docker"
    [[ $opt_systemd -eq 1 ]] && echo "    ✓ Systemd" || echo "    - Systemd"
    [[ $opt_background -eq 1 ]] && echo "    ✓ Background" || echo "    - Background"
    [[ $opt_expert -eq 1 ]] && echo "    ✓ Expert mode"
    echo ""
    echo -n "  [ENTER] Continue, [C] Cancel: "
    read choice
    if [[ "$choice" == "c" ]] || [[ "$choice" == "C" ]]; then
        show_main_menu
    else
        do_install
    fi
}

do_install() {
    clear
    echo ""
    echo -e "  ${BOLD}$(t title)${NC}"
    echo "  $(t installing)"
    echo ""

    mkdir -p "$INSTALL_DIR" "$ECOSYSTEM_DIR"

    if [[ $opt_core -eq 1 ]]; then
        install_core
    fi

    [[ $opt_drive -eq 1 ]] && install_module "Drive" "Polygone-Drive"
    [[ $opt_hide -eq 1 ]] && install_module "Hide" "Polygone-Hide"
    [[ $opt_petals -eq 1 ]] && install_module "Petals" "Polygone-Petals"
    [[ $opt_shell -eq 1 ]] && install_module "Shell" "Polygone-Shell"
    [[ $opt_brain -eq 1 ]] && install_module "Brain" "Polygone-Brain"
    [[ $opt_server -eq 1 ]] && install_module "Server" "Polygone-Server"
    [[ $opt_compute -eq 1 ]] && install_module "Compute" "Polygone-Compute"

    setup_path
    [[ $opt_systemd -eq 1 ]] && install_systemd
    [[ $opt_background -eq 1 ]] && start_daemon

    show_complete
}

install_core() {
    echo -ne "  Core... "
    OS=$(uname -s)
    ARCH=$(uname -m)
    case "$OS" in
        Linux)
            case "$ARCH" in
                x86_64)
                    if [[ -f "$INSTALL_DIR/polygone" ]] && [[ $opt_verify -eq 1 ]]; then
                        echo -e "${YELLOW}$(t already)${NC}"
                        return
                    fi
                    wget -q "${REPO_URL}/releases/download/${LATEST_VERSION}/polygone-x86_64-linux" \
                        -O "$INSTALL_DIR/polygone" 2>/dev/null
                    if [[ $? -eq 0 ]]; then
                        chmod +x "$INSTALL_DIR/polygone"
                        if [[ $opt_verify -eq 1 ]]; then
                            EXPECTED="f112eb3687ce59a7e17d840192a7770c694fbccca2564b9609f28bad74afb696"
                            ACTUAL=$(sha256sum "$INSTALL_DIR/polygone" 2>/dev/null | cut -d' ' -f1)
                            if [[ "$ACTUAL" != "$EXPECTED" ]]; then
                                rm -f "$INSTALL_DIR/polygone"
                                echo -e "${RED}SHA256 mismatch!${NC}"
                                return
                            fi
                        fi
                        echo -e "${GREEN}$(t success)${NC}"
                    else
                        echo -e "${RED}$(t error)${NC}"
                    fi
                    ;;
                aarch64)
                    wget -q "${REPO_URL}/releases/download/${LATEST_VERSION}/polygone-aarch64-linux" \
                        -O "$INSTALL_DIR/polygone" 2>/dev/null && \
                    chmod +x "$INSTALL_DIR/polygone" && \
                    echo -e "${GREEN}$(t success)${NC}" || \
                    echo -e "${RED}$(t error)${NC}"
                    ;;
                *) echo -e "${RED}Unsupported arch${NC}" ;;
            esac
            ;;
        Darwin)
            case "$ARCH" in
                x86_64|arm64)
                    wget -q "${REPO_URL}/releases/download/${LATEST_VERSION}/polygone-macos" \
                        -O "$INSTALL_DIR/polygone" 2>/dev/null && \
                    chmod +x "$INSTALL_DIR/polygone" && \
                    echo -e "${GREEN}$(t success)${NC}" || \
                    echo -e "${RED}$(t error)${NC}"
                    ;;
            esac
            ;;
        *) echo -e "${RED}Unsupported OS${NC}" ;;
    esac
}

install_module() {
    local name="$1" repo="$2"
    echo -ne "  $name... "
    if [[ -d "$ECOSYSTEM_DIR/$repo" ]]; then
        echo -e "${YELLOW}$(t already)${NC}"
    else
        git clone -q --depth 1 "${REPO_URL}/$repo" "$ECOSYSTEM_DIR/$repo" 2>/dev/null && \
        echo -e "${GREEN}$(t success)${NC}" || \
        echo -e "${RED}$(t error)${NC}"
    fi
}

install_systemd() {
    echo -ne "  Polygone service... "
    local svc="/etc/systemd/system/polygone.service"
    sudo tee "$svc" > /dev/null << 'EOF'
[Unit]
Description=Polygone P2P Network Node
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=polygone
Group=polygone
ExecStart=/usr/local/bin/polygone daemon start
Restart=on-failure
RestartSec=5
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
EOF
    sudo systemctl daemon-reload 2>/dev/null && \
    echo -e "${GREEN}OK${NC}" || echo -e "${RED}Error${NC}"
    
    if [[ $opt_compute -eq 1 ]]; then
        echo -ne "  Compute worker... "
        local csvc="/etc/systemd/system/polygone-compute.service"
        sudo tee "$csvc" > /dev/null << 'EOF'
[Unit]
Description=Polygone Compute - Distributed Idle Power
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=polygone
Group=polygone
ExecStart=/usr/local/bin/polygone-compute worker --max-cpu 50 --idle-threshold-secs 300
Restart=on-failure
RestartSec=5
CPUQuota=50%
MemoryMax=4G
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
EOF
        sudo systemctl daemon-reload 2>/dev/null && \
        echo -e "${GREEN}OK${NC}" || echo -e "${RED}Error${NC}"
    fi
}

start_daemon() {
    echo -ne "  Daemon... "
    if command -v polygone &> /dev/null; then
        polygone daemon start &> /dev/null &
        sleep 1
        echo -e "${GREEN}$(t success)${NC}" || \
        echo -e "${RED}$(t error)${NC}"
    else
        echo -e "${YELLOW}Not found in PATH${NC}"
    fi
}

setup_path() {
    for rc in "$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile"; do
        [[ -f "$rc" ]] && ! grep -q '.local/bin' "$rc" && \
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$rc"
    done
}

show_complete() {
    clear
    echo ""
    echo -e "  ${BOLD}${GREEN}$(t done)${NC}"
    echo ""
    echo "  $(t thanks)"
    echo ""
    echo "  $(t run_cmd)"
    [[ $opt_background -eq 1 ]] && echo "  $(t start_daemon)"
    [[ $opt_docker -eq 1 ]] && echo "  $(t start_docker)"
    echo ""
}

show_language_menu
show_main_menu
