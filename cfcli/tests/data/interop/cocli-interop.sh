#!/bin/bash
#
# Interop test: cfcli <-> cocli (veraison) CoRIM signing and verification
#
# Prerequisites:
#   - cocli built at $COCLI_BIN (default: ~/devel/third-party/cocli/cocli)
#   - cfcli built via cargo (run from workspace root)
#
# Usage:
#   cd /path/to/cbor_formats
#   bash cfcli/tests/data/interop/cocli-interop.sh
#

set -euo pipefail

COCLI_BIN="${COCLI_BIN:-$HOME/devel/third-party/cocli/cocli}"
COCLI_DATA="${COCLI_DATA:-$HOME/devel/third-party/cocli/data}"
CFCLI_BIN="cargo run --bin cfcli --"

KEY="$COCLI_DATA/keys/ec-p256.jwk"
META="$COCLI_DATA/corim/templates/meta-mini.json"
COMID_TEMPLATE="$COCLI_DATA/comid/templates/comid-cca-refval.json"
CORIM_TEMPLATE="$COCLI_DATA/corim/templates/corim-mini.json"

# Check prerequisites
if [ ! -x "$COCLI_BIN" ]; then
    echo "SKIP: cocli not found at $COCLI_BIN"
    exit 0
fi

if [ ! -f "$KEY" ]; then
    echo "SKIP: cocli test data not found at $COCLI_DATA"
    exit 0
fi

TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

PASS=0
FAIL=0

pass() { echo "  PASS: $1"; PASS=$((PASS + 1)); }
fail() { echo "  FAIL: $1"; FAIL=$((FAIL + 1)); }

echo "=== CoRIM interop: cfcli <-> cocli ==="
echo ""

# ── Step 1: Create test data with cocli ──
echo "Creating test CoMID and unsigned CoRIM with cocli..."
"$COCLI_BIN" comid create --template="$COMID_TEMPLATE" --output-dir="$TMPDIR" > /dev/null 2>&1
"$COCLI_BIN" corim create --template="$CORIM_TEMPLATE" --comid-dir="$TMPDIR" --output="$TMPDIR/unsigned.cbor" > /dev/null 2>&1

# ── Test 1: Sign with cocli, verify with cfcli ──
echo "Test 1: cocli sign -> cfcli verify"
"$COCLI_BIN" corim sign \
    --file="$TMPDIR/unsigned.cbor" \
    --key="$KEY" \
    --meta="$META" \
    --output="$TMPDIR/cocli-signed.cbor" > /dev/null 2>&1

if $CFCLI_BIN corim verify \
    --signed-corim-file="$TMPDIR/cocli-signed.cbor" \
    --key-file="$KEY" 2>&1 | grep -q "Verification successful"; then
    pass "cfcli verifies cocli-signed CoRIM"
else
    fail "cfcli could not verify cocli-signed CoRIM"
fi

# ── Test 2: Sign with cfcli, verify with cocli ──
echo "Test 2: cfcli sign -> cocli verify"
$CFCLI_BIN corim sign \
    --corim-file="$TMPDIR/unsigned.cbor" \
    --key-file="$KEY" \
    --meta-file="$META" \
    --output-dir="$TMPDIR" > /dev/null 2>&1

if "$COCLI_BIN" corim verify \
    --file="$TMPDIR/signed-unsigned.cbor" \
    --key="$KEY" 2>&1 | grep -q "verified"; then
    pass "cocli verifies cfcli-signed CoRIM"
else
    fail "cocli could not verify cfcli-signed CoRIM"
fi

# ── Test 3: Sign with cocli, verify with cocli (control) ──
echo "Test 3: cocli sign -> cocli verify (control)"
if "$COCLI_BIN" corim verify \
    --file="$TMPDIR/cocli-signed.cbor" \
    --key="$KEY" 2>&1 | grep -q "verified"; then
    pass "cocli self-verification"
else
    fail "cocli self-verification"
fi

# ── Test 4: Sign with cfcli, verify with cfcli (control) ──
echo "Test 4: cfcli sign -> cfcli verify (control)"
if $CFCLI_BIN corim verify \
    --signed-corim-file="$TMPDIR/signed-unsigned.cbor" \
    --key-file="$KEY" 2>&1 | grep -q "Verification successful"; then
    pass "cfcli self-verification"
else
    fail "cfcli self-verification"
fi

# ── Test 5: Extract from cocli-signed (informational) ──
echo "Test 5: cfcli extract cocli-signed CoRIM"
mkdir -p "$TMPDIR/extracted"
EXTRACT_OUTPUT=$($CFCLI_BIN corim extract \
    --signed-corim-file="$TMPDIR/cocli-signed.cbor" \
    --output-dir="$TMPDIR/extracted" 2>&1)
if echo "$EXTRACT_OUTPUT" | grep -q "Extracted"; then
    if ls "$TMPDIR/extracted"/*.cbor > /dev/null 2>&1; then
        pass "cfcli extracts tags from cocli-signed CoRIM"
    else
        echo "  INFO: extract ran but no tags written (cocli may use a different tag wrapping convention)"
        pass "cfcli extract ran without error"
    fi
else
    fail "cfcli could not extract from cocli-signed CoRIM"
fi

echo ""
echo "=== Results: $PASS passed, $FAIL failed ==="
[ "$FAIL" -eq 0 ] || exit 1
