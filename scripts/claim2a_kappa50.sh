#!/usr/bin/env bash
# Claim 2a: κ = N = λ = 50 at HP-1000.
#
# First of seven Claim 2 (Śliwiński Conjecture 4.1) configurations.
set -euo pipefail

source "$(dirname "$0")/claim_common.sh"
claim_init "$@"

PREC=${PREC:-1000}
TRIM=${TRIM:-0.2}

echo "=== Claim 2a: κ = N = λ = 50 at HP-${PREC} ==="
echo

claim_run sliwinski-check \
  --lambdas "50" \
  --n-values "50" \
  --precision-digits "$PREC" \
  --trim-edge-fraction "$TRIM"
