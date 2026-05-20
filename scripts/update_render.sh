#!/bin/bash
# Usage : RENDER_API_KEY=rct_xxx ./scripts/update_render.sh

if [ -z "$RENDER_API_KEY" ]; then
  echo "Erreur : export RENDER_API_KEY=rct_..."
  exit 1
fi

# Liste des IDs de services Render (à remplacer par les vrais)
SERVICES=("srv-xxx1" "srv-xxx2" "srv-xxx3" "srv-xxx4" "srv-xxx5" "srv-xxx6" "srv-xxx7" "srv-xxx8" "srv-xxx9" "srv-xxx10")

for SVC in "${SERVICES[@]}"; do
  echo "🔄 Redéploiement de $SVC..."
  curl -s -X POST "https://api.render.com/v1/services/$SVC/deploys"     -H "Authorization: Bearer $RENDER_API_KEY"     -H "Accept: application/json"
  echo ""
done
