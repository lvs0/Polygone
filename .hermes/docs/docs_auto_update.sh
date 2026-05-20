#!/usr/bin/env bash
set -euo pipefail

SRC="/home/l-vs/Polygone"
DEST="/home/l-vs/Polygone/docs"
LOG_DIR="/home/l-vs/Polygone/.hermes-logs"
TIMESTAMP="$(date +%Y%m%d)"
LOG_FILE="${LOG_DIR}/doc-${TIMESTAMP}.log"
TEMP_OUTPUT="/tmp/doc_build_${TIMESTAMP}.txt"

mkdir -p "$LOG_DIR" "$DEST"

cd "$SRC"
cargo doc --workspace --no-deps 2>&1 | tee "$TEMP_OUTPUT" | tail -5

# Check build status: cargo returns 0 on success
if ! grep -qiE "error|failed|\[ERROR\]" "$TEMP_OUTPUT" 2>/dev/null; then
    rm -rf "$DEST"
    cp -r "${SRC}/target/doc" "$DEST"
    rm -f "$TEMP_OUTPUT"
else
    mkdir -p "$LOG_DIR"
    cp "$TEMP_OUTPUT" "$LOG_FILE"
    rm -f "$TEMP_OUTPUT"
fi
exit 0
