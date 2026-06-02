#!/usr/bin/env bash
# Claim 2g: κ = N = λ = 500 at HP-1000.
#
# Seventh and final Claim 2 (Śliwiński Conjecture 4.1) configuration.
# This is the long-pole: the published README data shows a sharp
# upturn in ε×lnλ at κ=500 (4.1× jump over κ=400 in the full metric;
# 2.3× in the trimmed metric) — driving the headline finding that
# the conjectural limit is unsupported.
#
# NO τ-cache fixture for λ²=250000: matrix is built fresh.
#
# Wall-clock: many hours; intended as an overnight run on a
# dedicated server. The trim-20% metric matches the published
# README table.
set -euo pipefail

BIN=${BIN:-./target/release/ccm-falsifications}
PREC=${PREC:-1000}
TRIM=${TRIM:-0.2}

echo "=== Claim 2g: κ = N = λ = 500 at HP-${PREC} (long-pole, overnight) ==="
echo

"$BIN" sliwinski-check \
  --lambdas "500" \
  --n-values "500" \
  --precision-digits "$PREC" \
  --trim-edge-fraction "$TRIM"
