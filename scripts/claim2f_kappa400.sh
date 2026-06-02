#!/usr/bin/env bash
# Claim 2f: κ = N = λ = 400 at HP-1000.
#
# Sixth of seven Claim 2 (Śliwiński Conjecture 4.1) configurations.
# NO τ-cache fixture for λ²=160000: matrix is built fresh.
#
# Wall-clock: ~3–5 hr.
#
# Designed to run independently on its own server; do NOT bundle
# with other claims since this may run for several hours. The
# trim-20% metric matches the published README table.
set -euo pipefail

BIN=${BIN:-./target/release/ccm-falsifications}
PREC=${PREC:-1000}
TRIM=${TRIM:-0.2}

echo "=== Claim 2f: κ = N = λ = 400 at HP-${PREC} ==="
echo

"$BIN" sliwinski-check \
  --lambdas "400" \
  --n-values "400" \
  --precision-digits "$PREC" \
  --trim-edge-fraction "$TRIM"
