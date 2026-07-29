#!/usr/bin/env bash
# Claim 2e: κ = N = λ = 300 at HP-1000.
#
# Fifth of seven Claim 2 (Śliwiński Conjecture 4.1) configurations.
set -euo pipefail

source "$(dirname "$0")/claim_common.sh"
claim_init "$@"

PREC=${PREC:-1000}
TRIM=${TRIM:-0.2}

echo "=== Claim 2e: κ = N = λ = 300 at HP-${PREC} ==="
echo

claim_run sliwinski-check \
  --lambdas "300" \
  --n-values "300" \
  --precision-digits "$PREC" \
  --trim-edge-fraction "$TRIM"
