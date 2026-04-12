#!/bin/bash
# ⬡ POLYGONE — Menu Principal Simple
# Usage: ./polygone.sh

CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
NC='\033[0m'

POLYGONE_DIR="$(cd "$(dirname "$0")" && pwd)"
BINARY="$POLYGONE_DIR/target/release/polygone-cli"

show_menu() {
    clear
    echo -e "${CYAN}${BOLD}"
    echo "  ⬡ POLYGONE — Menu Principal"
    echo "  Réseau de Confidentialité Post-Quantique"
    echo -e "${NC}"
    echo ""
    
    # Check if binary exists
    if [ ! -f "$BINARY" ]; then
        echo -e "${YELLOW}⚠ Polygone n'est pas compilé!${NC}"
        echo ""
        echo "  Lance d'abord: ./install_simple.sh"
        echo ""
        exit 1
    fi
    
    echo "  1)  Self-Test       — Vérifier que tout fonctionne"
    echo "  2)  Générer Clés    — Créer une paire de clés"
    echo "  3)  Envoyer Message — Mode demo Alice → Bob"
    echo "  4)  Statut          — État du réseau"
    echo "  5)  Portefeuille    — Karma et rewards"
    echo "  6)  Node            — Lancer un node relay"
    echo ""
    echo "  0)  Quitter"
    echo ""
    echo -n "  Choix: "
}

while true; do
    show_menu
    read choice
    
    case $choice in
        1)
            echo ""
            $BINARY self-test
            echo ""
            echo -n "Appuie sur Entrée pour continuer..."
            read
            ;;
        2)
            echo ""
            $BINARY keygen
            echo ""
            echo -n "Appuie sur Entrée pour continuer..."
            read
            ;;
        3)
            echo ""
            echo -n "Message à envoyer: "
            read msg
            if [ -n "$msg" ]; then
                $BINARY send --peer-pk demo --message "$msg"
            fi
            echo ""
            echo -n "Appuie sur Entrée pour continuer..."
            read
            ;;
        4)
            echo ""
            $BINARY status
            echo ""
            echo -n "Appuie sur Entrée pour continuer..."
            read
            ;;
        5)
            echo ""
            $BINARY power wallet
            echo ""
            echo -n "Appuie sur Entrée pour continuer..."
            read
            ;;
        6)
            echo ""
            echo "Lancement du node..."
            $BINARY node
            ;;
        0)
            echo ""
            echo -e "${CYAN}L'information n'existe pas. Elle traverse. ⬡${NC}"
            break
            ;;
        *)
            echo ""
            echo -e "${YELLOW}Choix invalide${NC}"
            sleep 1
            ;;
    esac
done
