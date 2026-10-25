#!/usr/bin/env bash
# First, inexpensive check: no CCM matrix or eigensolve.
set -euo pipefail
source "$(dirname "$0")/claim_common.sh"
claim_init "$@"
claim_run mellin-compare --lambda-sq 13 --precision-digits "${PREC:-100}" \
  --t-min 14 --t-max 15 --n-scan 128 --n-quad 256 --mellin-mode naive \
  --scan-digits 30 --residual-tolerance 1e-20
