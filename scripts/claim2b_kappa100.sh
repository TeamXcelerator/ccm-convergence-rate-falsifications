#!/usr/bin/env bash
# Claim 2b: κ = N = λ = 100 at HP-1000.
#
# Second of seven Claim 2 (Śliwiński Conjecture 4.1) configurations.
# NO τ-cache fixture for λ²=10000: matrix is built fresh.
#
# Wall-clock: ~5–10 min.
#
# Designed to run independently on its own server. The trim-20%
# metric matches the published README table.
set -euo pipefail

BIN=${BIN:-./target/release/ccm-falsifications}
PREC=${PREC:-1000}
TRIM=${TRIM:-0.2}

echo "=== Claim 2b: κ = N = λ = 100 at HP-${PREC} ==="
echo

"$BIN" sliwinski-check \
  --lambdas "100" \
  --n-values "100" \
  --precision-digits "$PREC" \
  --trim-edge-fraction "$TRIM"
