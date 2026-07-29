#!/usr/bin/env bash
# Claim 2d: κ = N = λ = 200 at HP-1000.
#
# Fourth of seven Claim 2 (Śliwiński Conjecture 4.1) configurations.
set -euo pipefail

source "$(dirname "$0")/claim_common.sh"
claim_init "$@"

PREC=${PREC:-1000}
TRIM=${TRIM:-0.2}

echo "=== Claim 2d: κ = N = λ = 200 at HP-${PREC} ==="
echo

claim_run sliwinski-check \
  --lambdas "200" \
  --n-values "200" \
  --precision-digits "$PREC" \
  --trim-edge-fraction "$TRIM"
