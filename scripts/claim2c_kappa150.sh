#!/usr/bin/env bash
# Claim 2c: κ = N = λ = 150 at HP-1000.
#
# Third of seven Claim 2 (Śliwiński Conjecture 4.1) configurations.
# NO τ-cache fixture for λ²=22500: matrix is built fresh.
#
# Wall-clock: ~15–20 min.
#
# Designed to run independently on its own server. The trim-20%
# metric matches the published README table.
set -euo pipefail

BIN=${BIN:-./target/release/ccm-falsifications}
PREC=${PREC:-1000}
TRIM=${TRIM:-0.2}

echo "=== Claim 2c: κ = N = λ = 150 at HP-${PREC} ==="
echo

"$BIN" sliwinski-check \
  --lambdas "150" \
  --n-values "150" \
  --precision-digits "$PREC" \
  --trim-edge-fraction "$TRIM"
