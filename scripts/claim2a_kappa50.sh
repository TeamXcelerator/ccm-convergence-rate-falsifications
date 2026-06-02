#!/usr/bin/env bash
# Claim 2a: κ = N = λ = 50 at HP-1000.
#
# First of seven Claim 2 (Śliwiński Conjecture 4.1) configurations.
# Tests the conjectural limit form ε(κ)·ln(κ) → const at the smallest
# κ in the published sweep. NO τ-cache fixture for λ²=2500: the
# matrix is built fresh.
#
# Wall-clock: a few minutes (small matrix, full HP eigenspectrum).
#
# Designed to run independently on its own server so all seven Claim 2
# configs can run in parallel. The trim-20% metric matches the
# published README table (data/Section 4 of the paper).
set -euo pipefail

BIN=${BIN:-./target/release/ccm-falsifications}
PREC=${PREC:-1000}
TRIM=${TRIM:-0.2}

echo "=== Claim 2a: κ = N = λ = 50 at HP-${PREC} ==="
echo

"$BIN" sliwinski-check \
  --lambdas "50" \
  --n-values "50" \
  --precision-digits "$PREC" \
  --trim-edge-fraction "$TRIM"
