#!/bin/sh
# Keep-alive pour Render (sleep après 15m d'inactivité)
while true; do
  curl -s -o /dev/null "http://localhost:8080/health" || true
  sleep 840
done
