#!/usr/bin/env bash
# Bounded independent source window, before a full N-root request.
set -euo pipefail
source "$(dirname "$0")/claim_common.sh"
claim_init "$@"
claim_run sliwinski-check --lambda-squares 13 --n-values 120 \
  --precision-digits "${PREC:-1000}" --root-count 20 --trim-edge-fraction 0
