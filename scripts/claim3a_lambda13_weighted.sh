#!/usr/bin/env bash
# Claim 3a: λ²=13 naive and ξ-weighted Mellin comparison.
set -euo pipefail

source "$(dirname "$0")/claim_common.sh"
claim_init "$@"

echo "=== Claim 3a: CCM vs Mellin, λ²=13, weighted and unweighted ==="
claim_run mellin-compare \
  --lambda-sq 13 \
  --n-modes 120 \
  --precision-digits "${PREC:-1000}" \
  --t-min 5.0 \
  --t-max 55.0 \
  --n-scan 5000 \
  --n-quad 500 \
  --mellin-mode both
