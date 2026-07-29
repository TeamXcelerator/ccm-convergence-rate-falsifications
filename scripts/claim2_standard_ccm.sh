#!/usr/bin/env bash
# Claim 2 standard CCM regime: reproduce the three N >> λ table rows.
set -euo pipefail

source "$(dirname "$0")/claim_common.sh"
claim_init "$@"

echo "=== Claim 2 standard CCM regime at HP-${PREC:-1000} ==="
claim_run sliwinski-check \
  --lambdas "3.605551275463989,10,31.622776601683793" \
  --n-values "120,500,800" \
  --precision-digits "${PREC:-1000}" \
  --trim-edge-fraction 0
