#!/usr/bin/env bash
# Claim 1b: λ²=100, N=500, finite-difference grid 4001.
set -euo pipefail

source "$(dirname "$0")/claim_common.sh"
claim_init "$@"

echo "=== Claim 1b: Lemma 7.2, λ²=100, N=500, grid=4001 ==="
claim_run prolate-compare \
  --lambda-sq 100 \
  --n-modes 500 \
  --precision-digits "${PREC:-1000}" \
  --n-grid 4001
