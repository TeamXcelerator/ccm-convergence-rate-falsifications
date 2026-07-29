#!/usr/bin/env bash
# Claim 3b: λ²=100 naive Mellin table. No CCM source is needed.
set -euo pipefail

source "$(dirname "$0")/claim_common.sh"
claim_init "$@"

echo "=== Claim 3b: naive Mellin, λ²=100 ==="
claim_run mellin-compare \
  --lambda-sq 100 \
  --n-modes 500 \
  --precision-digits "${PREC:-1000}" \
  --t-min 5.0 \
  --t-max 55.0 \
  --n-scan 5000 \
  --n-quad 500 \
  --mellin-mode naive
