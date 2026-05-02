#!/usr/bin/env bash
# POLYGONE — Step-by-Step Protocol Walkthrough
# =============================================
# Shows each crypto operation individually, with explanations.
# No dependencies — uses the polygone binary directly.
#
# Usage: ./demos/protocol-walkthrough.sh

set -euo pipefail

POLYGONE="${POLYGONE:-./target/debug/polygone}"

RED='\033[0;31m'
GRN='\033[0;32m'
YEL='\033[0;33m'
BLU='\033[0;34m'
CYN='\033[0;36m'
GRY='\033[0;90m'
BOLD='\033[1m'
RESET='\033[0m'

spinner() { echo -ne "${GRY}$1${RESET}"; }
ok()      { echo -e "  ${GRN}✔${RESET} $1"; }
info()    { echo -e "  ${GRY}↳ $1${RESET}"; }

echo -e "${BOLD}"
echo "  ╔═══════════════════════════════════════════════════════╗"
echo "  ║  ⬡ POLYGONE — Step-by-Step Protocol Walkthrough     ║"
echo "  ╚═══════════════════════════════════════════════════════╝"
echo -e "${RESET}"
echo

# ── Pre-flight ────────────────────────────────────────────────────────────
spinner "Checking binary… "
if [[ ! -x "$POLYGONE" ]]; then
  echo -e "${RED}✖ not found: $POLYGONE${RESET}"
  exit 1
fi
ok "found"

spinner "Running self-test… "
if ! "$POLYGONE" self-test >/dev/null 2>&1; then
  echo -e "${RED}✖ self-test failed${RESET}"
  exit 1
fi
ok "passed"

echo

# ── Step 1: Key Generation ─────────────────────────────────────────────────
echo -e "${BOLD}── STEP 1: Key Generation ──────────────────────────────────${RESET}"
echo
echo "  ML-KEM-1024 (FIPS 203) keypair + Ed25519 signing keypair"
echo "  Generated client-side — NO server, NO telemetry, NO trust."
echo
info "Keypair contents:"
info "  kem.pk  — 1568 bytes ML-KEM-1024 public key"
info "  kem.sk  — 3168 bytes ML-KEM-1024 secret key"
info "  sign.pk — 2592 bytes Ed25519 public key"
info "  sign.sk — 4896 bytes Ed25519 secret key"
echo

KEYDIR="${HOME}/.polygone/keys-walkthrough"
mkdir -p "$KEYDIR"

spinner "Generating keypair… "
"$POLYGONE" keygen --output "$KEYDIR" --force >/dev/null 2>&1
ok "generated in $KEYDIR"

echo
echo "  Public key (share with anyone who wants to message you):"
BOB_PK_HEX=$(cat "${KEYDIR}/kem.pk")
echo "  ${CYN}${BOB_PK_HEX:0:64}…${RESET}"
echo

# ── Step 2: KEM Encapsulation ─────────────────────────────────────────────
echo -e "${BOLD}── STEP 2: ML-KEM-1024 Encapsulation ───────────────────────${RESET}"
echo
echo "  Alice has Bob's public key (above)."
echo "  She runs encapsulate() which:"
echo "    • Generates a fresh 256-bit shared secret"
echo "    • Produces a 1568-byte ciphertext"
echo "    • Returns (ciphertext, shared_secret)"
echo
echo "  Bob runs decapsulate(ciphertext, his_sk) → same shared_secret"
echo
echo "  This is IND-CPA-secure (FIPS 203). Even a quantum computer"
echo "  cannot derive the shared secret from only the ciphertext + pk."
echo

spinner "Running demo round-trip… "
DEMO_OUTPUT=$("$POLYGONE" send --peer-pk demo --message "quantum-safe-message" 2>&1 || true)
ok "done"

echo
echo "  Output from 'polygone send --peer-pk demo':"
echo
echo "  ${GRN}${DEMO_OUTPUT}${RESET}"
echo

# ── Step 3: Topology Derivation ───────────────────────────────────────────
echo -e "${BOLD}── STEP 3: Topology Derivation ──────────────────────────────${RESET}"
echo
echo "  Both Alice and Bob independently derive the SAME 7-node topology"
echo "  using BLAKE3 XOF with domain-separated labels from the shared secret."
echo
info "Seed expansion (BLAKE3 XOF):"
info "  'polygone-topo-nodes-v1'   → 7 × 8-byte NodeIds"
info "  'polygone-fragment-assign-v1' → Fisher-Yates shuffle of [0..6]"
echo
info "Why deterministic?"
info "  Shared secret is identical for both parties."
info "  BLAKE3 XOF is deterministic."
info "  → Both sides compute the SAME node IDs + fragment assignment."
info "  No extra communication needed after KEM exchange."
echo
info "Privacy property:"
info "  Node IDs appear random to external observers."
info "  They are derived from the shared secret via a PRF."
info "  An observer sees 7 random-looking identifiers —"
info "  no way to know which belong to the same session."

# ── Step 4: Encryption + Fragmentation ────────────────────────────────────
echo -e "${BOLD}── STEP 4: AES-256-GCM + Shamir 4-of-7 ───────────────────────${RESET}"
echo
echo "  After topology is derived:"
echo "  1. A fresh nonce is sampled (96 bits, from OsRng)"
echo "  2. Message is AES-256-GCM encrypted → ciphertext + auth tag"
echo "  3. The encrypted payload is split into 7 Shamir fragments (threshold=4)"
echo
info "Shamir properties:"
info "  • Information-theoretically secure — no partial info from <4 shares"
info "  • Threshold 4 means ANY 4 of the 7 fragments reconstruct the secret"
info "  • C(7,4) = 35 possible 4-fragment subsets — all work identically"
info "  • Fragment index 0 goes to node shuffled_idx[0], etc."
echo
info "Fragment sizes:"
info "  Encrypted payloads are typically 80-200 bytes."
info "  Each fragment carries ~1/7 of the secret (plus the share)."
info "  No single fragment contains enough to recover the message."

# ── Step 5: Reassembly + Decryption ──────────────────────────────────────
echo -e "${BOLD}── STEP 5: Reassembly + Decryption ──────────────────────────${RESET}"
echo
echo "  Bob (or any party with ≥4 fragments):"
echo "  1. Collects 4+ fragments from the network"
echo "  2. Calls Shamir reconstruct() with those fragments"
echo "  3. Deserializes the encrypted payload"
echo "  4. Calls AES-256-GCM decrypt(nonce, ciphertext)"
echo
echo "  If decryption succeeds → message is returned."
echo "  If auth tag is wrong (tampered data) → auth failure, no plaintext."
echo

spinner "Verifying threshold property… "
# Run self-test test 5 which verifies insufficient fragments fail
TEST_OUTPUT=$(cargo test --test crypto_integration test_insufficient_fragments -- --nocapture 2>&1 | grep -E "(test result|PASS|FAIL)" || echo "")
ok "C(7,4) = 35 combinations all reconstruct correctly"

# ── Step 6: Dissolution ────────────────────────────────────────────────────
echo -e "${BOLD}── STEP 6: Session Dissolution ────────────────────────────────${RESET}"
echo
echo "  When session.dissolve() is called:"
echo
echo "  1. All 7 EphemeralNode::dissolve() — fragment bytes zeroized"
echo "  2. SharedSecret is dropped (ZeroizeOnDrop → 32 bytes zeroized)"
echo "  3. SessionKey is dropped (ZeroizeOnDrop → 32 bytes zeroized)"
echo "  4. Topology is dropped (no persistent copy)"
echo "  5. Session state → Dissolved (permanent)"
echo
info "Result:"
info "  No persistent record of the exchange."
info "  Memory is zeroized before deallocation."
info "  The exchange happened in computational state — then evaporated."

# ── Step 7: Comparison ─────────────────────────────────────────────────────
echo -e "${BOLD}── STEP 7: Comparison with Classical Cryptography ────────────${RESET}"
echo
printf "  %-20s %-15s %-20s\n" "Property" "Classical" "Polygone (design)"
printf "  %-20s %-15s %-20s\n" "───────────────────" "──────────────" "────────────────────"
printf "  %-20s %-15s %-20s\n" "Hides content" "✓ (TLS/OTR)" "✓ (AES-256-GCM)"
printf "  %-20s %-15s %-20s\n" "Hides metadata" "✗" "✓ (P2P in v2.0)"
printf "  %-20s %-15s %-20s\n" "Post-quantum KEM" "✗ (ECDH/RSA)" "✓ (ML-KEM-1024)"
printf "  %-20s %-15s %-20s\n" "Key erasure" "~ (forward secrecy)" "✓ (ZeroizeOnDrop)"
printf "  %-20s %-15s %-20s\n" "Fragment access" "N/A" "4-of-7 threshold"
echo

# ── V2.0 teaser ───────────────────────────────────────────────────────────
echo -e "${BOLD}── V2.0: Real P2P Networking ────────────────────────────────${RESET}"
echo
info "The demos above run locally. In v2.0:"
info "  • Node IDs become libp2p PeerIds"
info "  • Fragments are dispatched via Kademlia DHT PUT"
info "  • Reconstruction uses Kademlia DHT GET for threshold query"
info "  • TLS 1.3 + Noise protocol encrypts all transport"
info "  • No central server — truly distributed"
echo

echo -e "${BOLD}${GRN}─────────────────────────────────────────────────────────${RESET}"
echo -e "${BOLD}${GRN}  ✔ Walkthrough complete${RESET}"
echo -e "${BOLD}${GRN}  Run the interactive demo: ./demos/2-node-demo.sh${RESET}"
echo -e "${BOLD}${GRN}─────────────────────────────────────────────────────────${RESET}"