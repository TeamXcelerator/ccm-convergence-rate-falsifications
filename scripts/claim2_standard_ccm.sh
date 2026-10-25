#!/usr/bin/env bash
# Claim 2 standard CCM regime: reproduce the three N >> λ table rows.
# The shared default covers C=1000. Use P=1000 explicitly for the audited
# individual C=13/C=100 rows; see scripts/README.md for their exact commands.
set -euo pipefail

source "$(dirname "$0")/claim_common.sh"
claim_init "$@"

echo "=== Claim 2 standard CCM regime at HP-${PREC:-2000} ==="
claim_run sliwinski-check \
  --lambda-squares "13,100,1000" \
  --n-values "120,500,800" \
  --precision-digits "${PREC:-2000}" \
  --trim-edge-fraction 0
