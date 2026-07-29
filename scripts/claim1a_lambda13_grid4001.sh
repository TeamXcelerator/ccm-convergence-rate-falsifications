#!/usr/bin/env bash
# Claim 1a: λ²=13, N=120, finite-difference grid 4001.
set -euo pipefail

source "$(dirname "$0")/claim_common.sh"
claim_init "$@"

echo "=== Claim 1a: Lemma 7.2, λ²=13, N=120, grid=4001 ==="
claim_run prolate-compare \
  --lambda-sq 13 \
  --n-modes 120 \
  --precision-digits "${PREC:-1000}" \
  --n-grid 4001
