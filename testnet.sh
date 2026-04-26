#!/bin/bash
set -e

# Build the project
cargo build

# Terminate all background nodes on exit
trap 'kill $(jobs -p)' EXIT

echo "Starting bootstrap node..."
./target/debug/polygone node start --listen /ip4/127.0.0.1/tcp/4001 &
sleep 2

for i in {2..7}; do
    PORT=$((4000 + i))
    echo "Starting node $i on port $PORT..."
    ./target/debug/polygone node start --listen /ip4/127.0.0.1/tcp/$PORT -b /ip4/127.0.0.1/tcp/4001/p2p/mock &
done

echo "Ephemeral testnet running! Press Ctrl+C to stop."
wait
