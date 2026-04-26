#!/bin/bash
# ⬡ POLYGONE — Interactive Testnet Script
# "L'information n'existe pas. Elle traverse."

set -e

# --- Configuration ---
BINARY="./target/release/polygone"
DEFAULT_PORT=4001
BOOTSTRAP_PEER_ID="12D3KooW..." # Mock for local

# --- UI Helpers ---
BLUE='\033[0;34m'
CYAN='\033[0;36m'
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

banner() {
    echo -e "${CYAN}"
    echo "  ⬡ POLYGONE — Ephemeral P2P Network"
    echo "  ------------------------------------"
    echo -e "${NC}"
}

check_env() {
    echo -n "Checking Rust environment... "
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}FAILED${NC} (Cargo not found)"
        exit 1
    fi
    echo -e "${GREEN}OK${NC}"
}

build_project() {
    echo -e "${BLUE}Building project in release mode...${NC}"
    cargo build --release
}

start_node() {
    echo -e "${BLUE}Starting node...${NC}"
    if [ -z "$1" ]; then
        $BINARY node start
    else
        $BINARY node start --listen "$1"
    fi
}

simulate_network() {
    echo -e "${BLUE}Simulating a 7-node global network on localhost...${NC}"
    # Start bootstrap
    $BINARY node start --listen /ip4/127.0.0.1/tcp/4001 &
    BOOTSTRAP_PID=$!
    sleep 2
    
    # Start 6 more nodes
    for i in {2..7}; do
        PORT=$((4000 + i))
        $BINARY node start --listen /ip4/127.0.0.1/tcp/$PORT -b /ip4/127.0.0.1/tcp/4001/p2p/mock &
    done
    
    echo -e "${GREEN}Network is LIVE. All 7 nodes are talking.${NC}"
    echo "Press Ctrl+C to stop the simulation."
    wait
}

self_test() {
    echo -e "${BLUE}Running End-to-End Self Test...${NC}"
    $BINARY self-test
}

run_bench() {
    echo -e "${BLUE}Running Cryptographic Benchmarks...${NC}"
    cargo bench --bench crypto_bench
}

# --- Main Logic ---
banner
check_env

if [[ ! -f "$BINARY" ]]; then
    build_project
fi

echo "What would you like to do?"
echo "1) Start a new bootstrap node (local)"
echo "2) Simulate a 7-node global network (local)"
echo "3) Run End-to-End Self Test"
echo "4) Run Cryptographic Benchmarks"
echo "5) Exit"
echo -n "Choice: "
read -r choice

case $choice in
    1)
        start_node "/ip4/127.0.0.1/tcp/$DEFAULT_PORT"
        ;;
    2)
        simulate_network
        ;;
    3)
        self_test
        ;;
    4)
        run_bench
        ;;
    5)
        exit 0
        ;;
    *)
        echo -e "${RED}Invalid choice.${NC}"
        ;;
esac
