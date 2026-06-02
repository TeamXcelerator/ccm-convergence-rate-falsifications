#!/usr/bin/env bash
# Claim 3: CCM vs naive Mellin truncation at λ²=13.
#
# Demonstrates that integral transforms of ξ_λ tested here (naive
# Mellin and ξ-weighted variants) fall short of CCM's algebraic
# accuracy by orders of magnitude. Computes ξ_λ at HP via the CCM
# construction (toolkit weil_eigvec cache when available, else
# computed + cached — no paper-local ξ files), then scans the
# critical line for zeros of Λ_λ(s) and the ξ-weighted G(s), and
# compares to Riemann zeros.
#
# Author: Ronnie Andrews, Jr. (Team Xcelerator Inc.(R))
set -euo pipefail

BIN=${BIN:-./target/release/ccm-falsifications}
PREC=${PREC:-1000}

echo "=== Claim 3: CCM vs Mellin (λ²=13) at HP-${PREC} ==="
"$BIN" mellin-compare \
  --lambda 3.6055512754639896 --n-modes 120 \
  --precision-digits "$PREC" \
  --t-min 5.0 --t-max 55.0 \
  --n-scan 5000 --n-quad 500
