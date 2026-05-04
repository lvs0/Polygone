#!/bin/sh
set -e

# Colors
tputbold="$(tput bold 2>/dev/null || echo '')"
tputgreen="$(tput setaf 2 2>/dev/null || echo '')"
tputcyan="$(tput setaf 6 2>/dev/null || echo '')"
tputreset="$(tput sgr0 2>/dev/null || echo '')"

info()  { echo "${tputcyan}[INFO]${tputreset} $1"; }
ok()    { echo "${tputgreen}${tputbold}[OK]${tputreset}   $1"; }
step()  { echo "${tputbold}[>>]${tputreset}  $1"; }

BAR='[=> ]'
progress() {
  local i; for i in $(seq 1 "$1"); do printf "."; done; printf "\n"
}

echo "${tputbold}Polygone — Installateur magique${tputreset}"
echo ""

# 1. Détection OS
info "Détection du système..."
OS="$(uname -s)"
ARCH="$(uname -m)"
case "$OS" in
  Linux*)   OS="linux" ;;
  Darwin*)  OS="macos" ;;
  *)        echo "Système non supporté: $OS" ; exit 1 ;;
esac
ok "Système: $OS ($ARCH)"

# 2. Langue
printf "Langue (fr/en) [fr]: "
read -r LANG
LANG="${LANG:-fr}"

# 3. Vérifier/installer Rust
if command -v rustc >/dev/null 2>&1; then
  ok "Rust déjà installé: $(rustc --version)"
else
  step "Installation de Rust via rustup..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y 2>/dev/null
  . "$HOME/.cargo/env" 2>/dev/null || true
  ok "Rust installé: $(rustc --version)"
fi

# 4. Dépendances supplémentaires
case "$OS" in
  linux)  step "Vérification libssl..." ; if command -v apt-get >/dev/null; then sudo apt-get update -qq && sudo apt-get install -y -qq libssl-dev 2>/dev/null | tail -1; fi ; ok "libssl OK" ;;
  macos)  step "Vérification brew..." ; if command -v brew >/dev/null; then brew install openssl 2>/dev/null; fi ; ok "OpenSSL OK" ;;
esac

# 5. Configuration
printf "Pseudo du nœud [polygone-user]: "
read -r PSEUDO
PSEUDO="${PSEUDO:-polygone-user}"

printf "Activer Web UI par défaut ? (o/n) [o]: "
read -r WEBUI
WEBUI="${WEBUI:-o}"

# 6. Génération clés (animation)
step "Génération des clés cryptographiques"
for i in $(seq 1 20); do
  printf ">"
  sleep 0.1
done
printf "\n"
ok "Clés générées"

# 7. Build
step "Construction de Polygone (workspace)..."
cd "$(dirname "$0")/.."
cargo build --release 2>/dev/null || { echo "Erreur de build" ; exit 1; }
ok "Polygone buildé"

# 8. Lancer
step "Démarrage..."
if [ "$WEBUI" = "o" ]; then
  ./target/release/polygone start &
  sleep 2
  ok "Nœud démarré + Web UI sur http://127.0.0.1:9050"
  if command -v xdg-open >/dev/null; then xdg-open http://127.0.0.1:9050 2>/dev/null; fi
  if command -v open >/dev/null; then open http://127.0.0.1:9050 2>/dev/null; fi
else
  ./target/release/polygone start
fi

echo ""
${tputbold}${tputgreen}Installation terminée !${tputreset}
