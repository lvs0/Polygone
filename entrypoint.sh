#!/bin/sh
set -e

# Lancer health server
/app/health_server &

# Lancer Web UI
/app/webui &

# Lancer keepalive
/app/keepalive.sh &

# Lancer nœud principal
exec /app/polygone
