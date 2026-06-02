#!/usr/bin/env bash
# Claim 2d: κ = N = λ = 200 at HP-1000.
#
# Fourth of seven Claim 2 (Śliwiński Conjecture 4.1) configurations.
# NO τ-cache fixture for λ²=40000: matrix is built fresh.
#
# Wall-clock: ~30–40 min.
#
# Designed to run independently on its own server. The trim-20%
# metric matches the published README table.
set -euo pipefail

BIN=${BIN:-./target/release/ccm-falsifications}
PREC=${PREC:-1000}
TRIM=${TRIM:-0.2}

echo "=== Claim 2d: κ = N = λ = 200 at HP-${PREC} ==="
echo

"$BIN" sliwinski-check \
  --lambdas "200" \
  --n-values "200" \
  --precision-digits "$PREC" \
  --trim-edge-fraction "$TRIM"
