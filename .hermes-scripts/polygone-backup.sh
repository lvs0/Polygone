#!/usr/bin/env bash
set -euo pipefail
BACKUP_DIR="$HOME/Téléchargements/Polygone-Backups"
WORKSPACE="$HOME/Polygone"
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="$BACKUP_DIR/polygone-backup-$DATE.tar.gz"
mkdir -p "$BACKUP_DIR"
echo "=== Polygone Backup $DATE ===" >> "$BACKUP_DIR/backup.log"
echo "Source: $WORKSPACE" >> "$BACKUP_DIR/backup.log"
tar czf "$BACKUP_FILE" --exclude='target' --exclude='.git' --exclude='*.log' -C "$WORKSPACE" . 2>&1 | tail -3
SIZE=$(du -sh "$BACKUP_FILE" | cut -f1)
echo "Backup: $BACKUP_FILE ($SIZE)" >> "$BACKUP_DIR/backup.log"
cd "$BACKUP_DIR"
ls -t polygone-backup-*.tar.gz 2>/dev/null | tail -n +8 | xargs -r rm
echo "Old backups cleaned" >> "$BACKUP_DIR/backup.log"
