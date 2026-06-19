#!/usr/bin/env bash
# ⬡ POLYGONE PRE-COMMIT HOOK
#
# Ce hook refuse tout commit qui ne respecte pas la philosophie Polygone.
# Il lit une règle de cohérence à chaque commit. Il est sévère.
# Mais il est honnête.

set -e
cd "$(git rev-parse --show-toplevel)"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'

echo -e "${YELLOW}⬡ POLYGONE — committing...${NC}"

# 1. Refuse tout commit avec tracking (telemetry, analytics, beacons)
TRACKING_PATTERN='(mi[t]\\?segment|google-analytics|sentry-|datadog|amplitude|mixpanel|posthog|firebase|crashlytics)'
if grep -RInE "$TRACKING_PATTERN" --include='*.toml' --include='*.rs' --include='*.py' --include='*.js' --include='*.ts' . 2>/dev/null | grep -v target/ | grep -v node_modules/ | grep -v ".git/"; then
  echo -e "${RED}❌ FAIL — tracking détecté.${NC}"
  echo "Polygone refuse les dépendances de tracking (analytics, telemetry, error reporting)."
  echo "Si c'est nécessaire (edge self-test), documente dans ECHO.md pourquoi."
  exit 1
fi

# 2. Refuse tout ajout de persistance longue durée dans le code principal
PERSISTENCE_PATTERN='(persist|forever|eternal|permanent)'
MATCHES=$(grep -RInE "$PERSISTENCE_PATTERN" --include='*.rs' crates/ polygone-core/ polygone-brain/ polygone-petals/ 2>/dev/null | grep -v "noqa" | grep -v "test" || true)
if [ -n "$MATCHES" ]; then
  echo -e "${RED}❌ FAIL — persistance suspecte.${NC}"
  echo "Le code Polygone doit être conçu autour du TTL 30s."
  echo "Toute persistance doit être expliquée dans un commit message."
  echo "$MATCHES"
  exit 1
fi

# 3. Imprime le mantra
echo -e "${GREEN}⬡ Privacy is the new oxygen.${NC}"
echo -e "${GREEN}⬡ Ce que vous venez d'écrire respecte la philosophie de base.${NC}"
exit 0
