#!/bin/sh
while true; do
  sleep 840
  wget -qO- "http://localhost:${PORT:-8080}/health" > /dev/null 2>&1
done
