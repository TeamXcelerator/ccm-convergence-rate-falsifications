#!/usr/bin/env bash
# Claim 2e: κ = N = λ = 300 at HP-1000.
#
# Fifth of seven Claim 2 (Śliwiński Conjecture 4.1) configurations.
# NO τ-cache fixture for λ²=90000: matrix is built fresh.
#
# Wall-clock: ~1–2 hr (the κ scaling is roughly cubic).
#
# Designed to run independently on its own server. The trim-20%
# metric matches the published README table.
set -euo pipefail

BIN=${BIN:-./target/release/ccm-falsifications}
PREC=${PREC:-1000}
TRIM=${TRIM:-0.2}

echo "=== Claim 2e: κ = N = λ = 300 at HP-${PREC} ==="
echo

"$BIN" sliwinski-check \
  --lambdas "300" \
  --n-values "300" \
  --precision-digits "$PREC" \
  --trim-edge-fraction "$TRIM"
