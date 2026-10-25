#!/usr/bin/env bash
# Claim 2f: κ = N = λ = 400 at the manuscript's HP-2000 setting.
#
# Sixth of seven Claim 2 (Śliwiński Conjecture 4.1) configurations.
set -euo pipefail

source "$(dirname "$0")/claim_common.sh"
claim_init "$@"

PREC=${PREC:-2000}
TRIM=${TRIM:-0.2}

echo "=== Claim 2f: κ = N = λ = 400 at HP-${PREC} ==="
echo

claim_run sliwinski-check \
  --lambda-squares "160000" \
  --n-values "400" \
  --precision-digits "$PREC" \
  --trim-edge-fraction "$TRIM"
