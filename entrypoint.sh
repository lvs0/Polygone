#!/bin/bash
# ⬡ POLYGONE VPS RUNNER

echo "Launching Polygone Pulse (Anti-Sleep)..."
python3 pulse.py &

echo "Launching Polygone Node..."
# Pass all arguments to polygone-server
exec polygone-server "$@"
