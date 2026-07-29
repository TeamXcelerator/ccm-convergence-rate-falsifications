#!/usr/bin/env bash
# Claim 2g: κ = N = λ = 500 at HP-1000.
#
# Seventh and final Claim 2 (Śliwiński Conjecture 4.1) configuration.
set -euo pipefail

source "$(dirname "$0")/claim_common.sh"
claim_init "$@"

PREC=${PREC:-1000}
TRIM=${TRIM:-0.2}

echo "=== Claim 2g: κ = N = λ = 500 at HP-${PREC} (long-pole, overnight) ==="
echo

claim_run sliwinski-check \
  --lambdas "500" \
  --n-values "500" \
  --precision-digits "$PREC" \
  --trim-edge-fraction "$TRIM"
