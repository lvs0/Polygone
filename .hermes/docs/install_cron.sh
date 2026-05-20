#!/usr/bin/env bash
SCRIPT="/home/l-vs/Polygone/.hermes/docs/docs_auto_update.sh"
CRON_LINE="30 8 * * * ${SCRIPT} >> /tmp/doc_cron_${TIMESTAMP}.log 2>&1"

# Remove existing identical cron if present
crontab -l 2>/dev/null | grep -vF "docs_auto_update.sh" | crontab -
# Add the job
(crontab -l 2>/dev/null; echo "30 8 * * * /home/l-vs/Polygone/.hermes/docs/docs_auto_update.sh >> /tmp/doc_cron_$(date +%Y%m%d_%H%M%S).log 2>&1") | crontab -
echo "Cron installed/updated."
