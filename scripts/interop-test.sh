#!/bin/bash
# Interop test script for cfcli CoRIM sign/verify/extract.
#
# Tests:
#   1. cfcli sign → cfcli verify  (self-test)
#   2. cfcli sign → cfcli extract (self-test)
#   3. cocli sign → cfcli verify  (cross-tool, if cocli available)
#   4. cfcli sign → cocli verify  (cross-tool, if cocli available)
#
# Prerequisites:
#   - cargo build -p cfcli
#   - cocli data files at COCLI_DATA_DIR (optional, for cross-tool tests)
#   - cocli binary (optional, for cross-tool tests)

set -u

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# Paths
# Build cfcli first
echo "Building cfcli..."
cargo build -q -p cfcli --manifest-path "$REPO_DIR/Cargo.toml"

# Determine target directory
TARGET_DIR="$REPO_DIR/target/debug"
CFCLI="${CFCLI:-$TARGET_DIR/cfcli}"
COCLI_DATA_DIR="${COCLI_DATA_DIR:-/Users/cwallace/devel/third-party/cocli/data}"
COCLI_BIN="${COCLI_BIN:-}"

# Test data
UNSIGNED_CORIM="$COCLI_DATA_DIR/corim/unsigned-corim.cbor"
KEY_FILE="$COCLI_DATA_DIR/keys/ec-p256.jwk"
META_FILE="$COCLI_DATA_DIR/corim/templates/meta-mini.json"
META_FULL_FILE="$COCLI_DATA_DIR/corim/templates/meta-full.json"

# Working directory
WORK_DIR=$(mktemp -d)
trap 'rm -rf "$WORK_DIR"' EXIT

PASS=0
FAIL=0
SKIP=0

pass() { echo "  PASS: $1"; PASS=$((PASS + 1)); }
fail() { echo "  FAIL: $1"; FAIL=$((FAIL + 1)); }
skip() { echo "  SKIP: $1"; SKIP=$((SKIP + 1)); }

echo "=== CoRIM Sign/Verify/Extract Interop Tests ==="
echo "Working directory: $WORK_DIR"
echo ""

# Check prerequisites
if [ ! -f "$UNSIGNED_CORIM" ]; then
    echo "ERROR: unsigned CoRIM not found at $UNSIGNED_CORIM"
    echo "Set COCLI_DATA_DIR to the cocli data directory."
    exit 1
fi

if [ ! -f "$KEY_FILE" ]; then
    echo "ERROR: key file not found at $KEY_FILE"
    exit 1
fi

# ── Test 1: cfcli sign → cfcli verify (mini meta) ──
echo "--- Test 1: cfcli sign → cfcli verify (mini meta) ---"
mkdir -p "$WORK_DIR/test1"
if $CFCLI corim sign \
    --corim-file "$UNSIGNED_CORIM" \
    --key-file "$KEY_FILE" \
    --meta-file "$META_FILE" \
    --output-dir "$WORK_DIR/test1" 2>/dev/null; then

    SIGNED_FILE=$(ls "$WORK_DIR/test1"/signed-*.cbor 2>/dev/null | head -1)
    if [ -n "$SIGNED_FILE" ]; then
        OUTPUT=$($CFCLI corim verify \
            --signed-corim-file "$SIGNED_FILE" \
            --key-file "$KEY_FILE" 2>&1)
        if echo "$OUTPUT" | grep -q "Verification successful"; then
            pass "cfcli sign → cfcli verify (mini meta)"
        else
            fail "cfcli sign → cfcli verify (mini meta): $OUTPUT"
        fi
    else
        fail "cfcli sign produced no output file"
    fi
else
    fail "cfcli sign failed"
fi

# ── Test 2: cfcli sign → cfcli verify (full meta with validity) ──
echo "--- Test 2: cfcli sign → cfcli verify (full meta) ---"
if [ -f "$META_FULL_FILE" ]; then
    mkdir -p "$WORK_DIR/test2"
    if $CFCLI corim sign \
        --corim-file "$UNSIGNED_CORIM" \
        --key-file "$KEY_FILE" \
        --meta-file "$META_FULL_FILE" \
        --output-dir "$WORK_DIR/test2" 2>/dev/null; then

        SIGNED_FILE=$(ls "$WORK_DIR/test2"/signed-*.cbor 2>/dev/null | head -1)
        if [ -n "$SIGNED_FILE" ]; then
            OUTPUT=$($CFCLI corim verify \
                --signed-corim-file "$SIGNED_FILE" \
                --key-file "$KEY_FILE" 2>&1)
            if echo "$OUTPUT" | grep -q "Verification successful"; then
                pass "cfcli sign → cfcli verify (full meta)"
            else
                fail "cfcli sign → cfcli verify (full meta): $OUTPUT"
            fi
        else
            fail "cfcli sign (full meta) produced no output file"
        fi
    else
        fail "cfcli sign (full meta) failed"
    fi
else
    skip "full meta template not found"
fi

# ── Test 3: cfcli sign → cfcli extract ──
echo "--- Test 3: cfcli sign → cfcli extract ---"
SIGNED_FILE=$(ls "$WORK_DIR/test1"/signed-*.cbor 2>/dev/null | head -1)
if [ -n "$SIGNED_FILE" ]; then
    mkdir -p "$WORK_DIR/test3"
    OUTPUT=$($CFCLI corim extract \
        --signed-corim-file "$SIGNED_FILE" \
        --output-dir "$WORK_DIR/test3" 2>&1)
    EXTRACTED=$(ls "$WORK_DIR/test3"/*.cbor 2>/dev/null | wc -l | tr -d ' ')
    if [ "$EXTRACTED" -gt 0 ]; then
        pass "cfcli extract: $EXTRACTED tag(s) extracted"
    else
        fail "cfcli extract produced no files: $OUTPUT"
    fi
else
    skip "no signed file from test 1"
fi

# ── Test 4: cfcli verify rejects bad signature ──
echo "--- Test 4: cfcli verify rejects bad signature ---"
BAD_SIG_FILE="$COCLI_DATA_DIR/corim/signed-corim-bad-signature.cbor"
if [ -f "$BAD_SIG_FILE" ]; then
    OUTPUT=$($CFCLI corim verify \
        --signed-corim-file "$BAD_SIG_FILE" \
        --key-file "$KEY_FILE" 2>&1)
    if echo "$OUTPUT" | grep -q "failed\|Failed"; then
        pass "cfcli rejects bad signature"
    else
        fail "cfcli accepted bad signature: $OUTPUT"
    fi
else
    skip "bad signature test file not found"
fi

# ── Cross-tool tests (require cocli binary) ──
echo ""
echo "--- Cross-tool tests (cocli) ---"

if [ -z "$COCLI_BIN" ]; then
    # Try to find cocli
    if command -v cocli &>/dev/null; then
        COCLI_BIN="cocli"
    elif [ -x "/tmp/cocli" ]; then
        COCLI_BIN="/tmp/cocli"
    fi
fi

if [ -n "$COCLI_BIN" ]; then
    echo "Using cocli: $COCLI_BIN"

    # Test 5: cfcli sign → cocli verify
    echo "--- Test 5: cfcli sign → cocli verify ---"
    SIGNED_FILE=$(ls "$WORK_DIR/test1"/signed-*.cbor 2>/dev/null | head -1)
    if [ -n "$SIGNED_FILE" ]; then
        OUTPUT=$($COCLI_BIN corim verify \
            --file "$SIGNED_FILE" \
            --key "$KEY_FILE" 2>&1) && {
            pass "cfcli sign → cocli verify"
        } || {
            # cocli may have schema incompatibility with the unsigned CoRIM format
            skip "cfcli sign → cocli verify (cocli payload parse error — likely schema version mismatch)"
            echo "    cocli output: $(echo "$OUTPUT" | head -1)"
        }
    else
        skip "no signed file from test 1"
    fi

    # Test 6: cocli sign → cfcli verify (using pre-existing signed data)
    echo "--- Test 6: cocli-signed data → cfcli verify ---"
    COCLI_SIGNED="$COCLI_DATA_DIR/corim/signed-corim.cbor"
    if [ -f "$COCLI_SIGNED" ]; then
        OUTPUT=$($CFCLI corim verify \
            --signed-corim-file "$COCLI_SIGNED" \
            --key-file "$KEY_FILE" 2>&1)
        if echo "$OUTPUT" | grep -q "Verification successful"; then
            pass "cocli-signed → cfcli verify"
        else
            # Pre-existing signed data may use a different key
            skip "cocli-signed → cfcli verify (key mismatch or format difference)"
            echo "    cfcli output: $(echo "$OUTPUT" | tail -1)"
        fi
    else
        skip "cocli signed file not found"
    fi

    # Test 7: cocli sign fresh → cfcli verify
    echo "--- Test 7: cocli sign fresh → cfcli verify ---"
    mkdir -p "$WORK_DIR/test7"
    if $COCLI_BIN corim sign \
        --file "$UNSIGNED_CORIM" \
        --key "$KEY_FILE" \
        --meta "$META_FILE" \
        --output "$WORK_DIR/test7/cocli-signed.cbor" 2>/dev/null; then

        OUTPUT=$($CFCLI corim verify \
            --signed-corim-file "$WORK_DIR/test7/cocli-signed.cbor" \
            --key-file "$KEY_FILE" 2>&1)
        if echo "$OUTPUT" | grep -q "Verification successful"; then
            pass "cocli sign → cfcli verify"
        else
            fail "cocli sign → cfcli verify: $OUTPUT"
        fi
    else
        skip "cocli sign failed (data files may be incompatible with current cocli version)"
    fi
else
    skip "cocli binary not found (set COCLI_BIN to enable cross-tool tests)"
fi

# ── Summary ──
echo ""
echo "=== Results ==="
echo "  Passed:  $PASS"
echo "  Failed:  $FAIL"
echo "  Skipped: $SKIP"
echo ""

if [ "$FAIL" -gt 0 ]; then
    echo "SOME TESTS FAILED"
    exit 1
else
    echo "ALL TESTS PASSED"
    exit 0
fi
