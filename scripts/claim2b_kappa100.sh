#!/usr/bin/env bash
# Claim 2b: κ = N = λ = 100 at HP-1000.
#
# Second of seven Claim 2 (Śliwiński Conjecture 4.1) configurations.
set -euo pipefail

source "$(dirname "$0")/claim_common.sh"
claim_init "$@"

PREC=${PREC:-1000}
TRIM=${TRIM:-0.2}

echo "=== Claim 2b: κ = N = λ = 100 at HP-${PREC} ==="
echo

claim_run sliwinski-check \
  --lambdas "100" \
  --n-values "100" \
  --precision-digits "$PREC" \
  --trim-edge-fraction "$TRIM"
