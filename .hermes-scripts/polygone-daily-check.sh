#!/usr/bin/env bash
# Polygone Daily Check — vérifications complètes
set -euo pipefail

WORKSPACE="$HOME/Polygone"
LOG_DIR="$WORKSPACE/.hermes-logs"
DATE=$(date +%Y%m%d)
LOG="$LOG_DIR/daily-check-$DATE.log"

mkdir -p "$LOG_DIR"
echo "=== Polygone Daily Check $DATE ===" > "$LOG"

# 1. Vérification sauvegarde récente (< 7 jours)
BACKUP_DIR="$HOME/Téléchargements/Polygone-Backups"
if [ -d "$BACKUP_DIR" ]; then
    LATEST=$(ls -t "$BACKUP_DIR"/polygone-backup-*.tar.gz 2>/dev/null | head -1 || echo "")
    if [ -n "$LATEST" ]; then
        AGE=$(( ($(date +%s) - $(stat -c %Y "$LATEST")) / 86400 ))
        echo "Latest backup: $LATEST ($AGE days old)" >> "$LOG"
        if [ "$AGE" -gt 7 ]; then
            echo "⚠️  Latest backup > 7 days!" >> "$LOG"
        fi
    else
        echo "❌ No backup found" >> "$LOG"
    fi
else
    echo "❌ Backup dir missing" >> "$LOG"
fi

# 2. Vérification taille workspace
TOTAL=$(du -sh "$WORKSPACE" 2>/dev/null | cut -f1)
echo "Workspace size: $TOTAL" >> "$LOG"

# 3. Vérification cibles
echo "--- Target size ---" >> "$LOG"
df -h "$WORKSPACE/target" 2>/dev/null | tail -1 >> "$LOG" || echo "target/ not mounted" >> "$LOG"

# 4. Vérification Cargo.lock cohérence
echo "--- Cargo.lock ---" >> "$LOG"
if [ -f "$WORKSPACE/Cargo.lock" ]; then
    echo "✅ Cargo.lock present" >> "$LOG"
else
    echo "❌ Cargo.lock missing" >> "$LOG"
fi

# 5. Vérification workspace structure
echo "--- Workspace structure ---" >> "$LOG"
for crate in polygone-core polygone-brain polygone-shell polygone-app crates/common crates/crypto crates/network msh; do
    if [ -d "$WORKSPACE/$crate/Cargo.toml" ]; then
        echo "✅ $crate" >> "$LOG"
    else
        echo "❌ $crate missing" >> "$LOG"
    fi
done

echo "--- End ---" >> "$LOG"
exit 0
