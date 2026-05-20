#!/usr/bin/env bash
set -euo pipefail
WORKSPACE="$HOME/Polygone"
LOG_DIR="$WORKSPACE/.hermes-logs"
DATE=$(date +%Y%m%d)
LOG="$LOG_DIR/cleanup-$DATE.log"
mkdir -p "$LOG_DIR"
echo "=== Polygone Cleanup $DATE ===" | tee "$LOG"
echo -n "Espace avant: " | tee -a "$LOG"
df -h "$WORKSPACE" | tail -1 | awk '{print $5 " used on " $2}' | tee -a "$LOG"
TARGET_SIZE=$(du -sb "$WORKSPACE/target/" 2>/dev/null | cut -f1 || echo "0")
echo "target/ size: $(numfmt --to=iec-i --suffix=B "$TARGET_SIZE")" | tee -a "$LOG"
rm -rf "$WORKSPACE/target/"
echo "target/ deleted" | tee -a "$LOG"
rm -rf /tmp/polygone-cache-* 2>/dev/null || true
echo "tmp caches cleaned" | tee -a "$LOG"
if command -v cargo-cache &>/dev/null; then
    cargo cache --autoclean 2>&1 | tee -a "$LOG" || true
fi
echo -n "Espace après: " | tee -a "$LOG"
df -h "$WORKSPACE" | tail -1 | awk '{print $5 " used on " $2}' | tee -a "$LOG"
TOTAL=$((TARGET_SIZE))
echo "Freed: $(numfmt --to=iec-i --suffix=B "$TOTAL")" | tee -a "$LOG"
exit 0
