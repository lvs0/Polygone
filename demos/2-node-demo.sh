#!/usr/bin/env bash
# POLYGONE — 2-Node Interactive Demo
# =====================================
# Runs a full protocol session with two simulated nodes (Alice + Bob)
# on the same machine, showing:
#   1. Keypair generation (Bob)
#   2. ML-KEM-1024 encapsulation (Alice → Bob)
#   3. Topology derivation (identical on both sides)
#   4. AES-256-GCM encryption + Shamir 4-of-7 fragmentation
#   5. Fragment assignment display
#   6. Reassembly + decryption (Bob)
#   7. Session dissolution + zeroization
#
# Usage: ./demos/2-node-demo.sh [message]
#   message  — optional, default: "Information does not exist. It drifts."

set -euo pipefail

POLYGONE="${POLYGONE:-./target/debug/polygone}"
MSG="${1:-Information does not exist. It drifts.}"

# ── Colour codes ───────────────────────────────────────────────────────────
RED='\033[0;31m'
GRN='\033[0;32m'
YEL='\033[0;33m'
BLU='\033[0;34m'
CYN='\033[0;36m'
GRY='\033[0;90m'
BOLD='\033[1m'
RESET='\033[0m'

header() {
  echo -e "\n${BOLD}${CYN}═══════════════════════════════════════════════════════════${RESET}"
  echo -e "${BOLD}${CYN}  $1${RESET}"
  echo -e "${BOLD}${CYN}═══════════════════════════════════════════════════════════${RESET}\n"
}

step() {
  echo -e "${BOLD}[$1]${RESET} ${GRN}$2${RESET}"
}

info() {
  echo -e "  ${GRY}$1${RESET}"
}

sub() {
  echo -e "    ${BLU}↳ $1${RESET}"
}

ok() {
  echo -e "  ${GRN}✔ $1${RESET}"
}

warn() {
  echo -e "  ${YEL}⚠ $1${RESET}"
}

section() {
  echo -e "\n${BOLD}── $1 ──${RESET}"
}

# ── Pre-flight ──────────────────────────────────────────────────────────────
echo -e "${BOLD}"
echo "  ╔═══════════════════════════════════════════╗"
echo "  ║   ⬡  POLYGONE — 2-Node Interactive Demo   ║"
echo "  ╚═══════════════════════════════════════════╝"
echo -e "${RESET}"

if [[ ! -x "$POLYGONE" ]]; then
  warn "Binary not found at: $POLYGONE"
  echo "  Building…"
  cargo build -p polygone 2>/dev/null
  POLYGONE="./target/debug/polygone"
fi

step "PRE-FLIGHT" "Checking binary"
VERSION=$("$POLYGONE" --version 2>/dev/null | awk '{print $NF}' || echo "unknown")
ok "polygone $VERSION"

# ── Step 1: Bob generates keys (cached demo keys to avoid regenerate each run)
header "STEP 1 — Bob generates a post-quantum keypair"

KEYDIR="${HOME}/.polygone/keys-demo"
mkdir -p "$KEYDIR"

if [[ ! -f "${KEYDIR}/kem.sk" ]]; then
  info "Generating new demo keypair in $KEYDIR"
  "$POLYGONE" keygen --output "$KEYDIR" --force >/dev/null 2>&1
  ok "Keypair generated"
else
  ok "Demo keypair already exists — skipping generation"
fi

info "Files:"
sub "kem.pk  — KEM public key (share freely)"
sub "kem.sk  — KEM secret key (keep private, chmod 600)"
sub "sign.pk — Signing public key"
sub "sign.sk — Signing secret key"

# Extract Bob's public key hex
BOB_PK_HEX=$(cat "${KEYDIR}/kem.pk")
info "Bob's KEM public key (first 48 chars): ${BOB_PK_HEX:0:48}…"

# ── Step 2: Alice encapsulates (Bob's public key)
header "STEP 2 — Alice encapsulates a session secret with Bob's public key"

info "Alice uses Bob's KEM public key to run ML-KEM-1024 encapsulation."
info "Output: a ciphertext (1568 bytes) that Alice sends to Bob OUT-OF-BAND."

# We simulate this by running the send command which does full protocol
# In a real scenario Alice would send ct to Bob via any channel

# ── Step 3: Full round-trip via polygone send demo ─────────────────────────
header "STEP 3 — Full protocol round-trip: encrypt → fragment → reassemble → decrypt"

info "Running: polygone send --peer-pk demo --message \"$MSG\""

echo
"$POLYGONE" send --peer-pk demo --message "$MSG"

# ── Step 4: Topology explanation ──────────────────────────────────────────
section "TOPOLOGY EXPLANATION"

echo "  Both Alice and Bob derive the SAME 7-node topology independently"
echo "  from the shared secret (no communication needed)."
echo
info "  Node IDs are 8-byte random values derived via BLAKE3 XOF from"
sub "the shared secret + a domain separator 'polygone-topo-nodes-v1'."
echo
info "  Fragment assignment is a Fisher-Yates shuffle of 7 elements"
sub "derived from BLAKE3 XOF with domain 'polygone-fragment-assign-v1'."
echo
info "  Any 4 of 7 fragments can reconstruct the full encrypted payload."
sub "No single fragment, and no set of <4 fragments, reveals any info."

# ── Step 5: Node lifecycle ────────────────────────────────────────────────
section "NODE LIFECYCLE"

info "Each of the 7 nodes:"
sub "• Lives for at most 30 seconds (TTL)"
sub "• Holds at most 1 fragment"
sub "• Dissolves automatically when TTL expires or session ends"
sub "• Zeroizes fragment bytes on dissolution (ZeroizeOnDrop)"

# ── Step 6: Session dissolution ──────────────────────────────────────────
section "SESSION DISSOLUTION"

info "When session.dissolve() is called:"
sub "• All 7 EphemeralNode.dissolve() called in sequence"
sub "• Each node zeroizes its fragment via ZeroizeOnDrop"
sub "• session_key, shared_secret, topology all dropped"
sub "• No persistent state — the exchange DID NOT HAPPEN"

# ── Step 7: What happens in v2.0 ─────────────────────────────────────────
section "V2.0: REAL P2P NETWORKING"

info "In v1.0, all steps run in a single process (local demo mode)."
info "In v2.0, fragments are dispatched over real network:"
echo
sub "1. Alice encrypts and fragments locally"
sub "2. Each fragment sent to its assigned DHT node via libp2p"
sub "3. Bob queries DHT for nodes holding his fragments (threshold query)"
sub "4. Bob retrieves ≥4 fragments over libp2p connections"
sub "5. Bob reconstructs and decrypts locally"
sub "6. Session dissolves — DHT entries expire"

# ── Step 8: Quick self-test ──────────────────────────────────────────────
section "VERIFICATION"

info "Run the self-test suite to verify the crypto stack is sound:"
echo
echo "  ${BOLD}polygone self-test${RESET}"
echo

if command -v timeout &>/dev/null; then
  if timeout 30 "$POLYGONE" self-test 2>&1; then
    ok "Self-test passed — POLYGONE is operational"
  else
    warn "Self-test failed — check your build"
  fi
fi

# ── Done ───────────────────────────────────────────────────────────────────
echo
echo -e "${BOLD}${GRN}─────────────────────────────────────────────────────────${RESET}"
echo -e "${BOLD}${GRN}  ✔ Demo complete — run with a custom message:${RESET}"
echo -e "${BOLD}${GRN}    ./demos/2-node-demo.sh \"Your message here\"${RESET}"
echo -e "${BOLD}${GRN}─────────────────────────────────────────────────────────${RESET}"
echo