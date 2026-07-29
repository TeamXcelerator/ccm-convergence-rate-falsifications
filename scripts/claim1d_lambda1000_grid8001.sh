#!/usr/bin/env bash
# Claim 1d: λ²=1000, N=800, finite-difference grid 8001.
set -euo pipefail

source "$(dirname "$0")/claim_common.sh"
claim_init "$@"

echo "=== Claim 1d: Lemma 7.2, λ²=1000, N=800, grid=8001 ==="
claim_run prolate-compare \
  --lambda-sq 1000 \
  --n-modes 800 \
  --precision-digits "${PREC:-1000}" \
  --n-grid 8001
