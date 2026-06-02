#!/usr/bin/env bash
# Claim 1: CCM Lemma 7.2 falsification sweep across λ² ∈ {13, 100, 1000}.
#
# For each config, computes ξ_λ at HP via the CCM construction (the
# toolkit fetches the smallest Weil eigenvector from its residual-
# validated weil_eigvec cache when available, else computes + caches
# it — no paper-local ξ files), builds the prolate-wave educated guess
# k_λ via finite-difference diagonalization of PW_λ, and computes the
# relative L∞ error ‖ξ_λ - c·k_λ‖_∞ / ‖ξ_λ‖_∞. CCM Lemma 7.2 predicts
# `rel × λ²` to be a constant C across λ.
#
# Author: Ronnie Andrews, Jr. (Team Xcelerator Inc.(R))
set -euo pipefail

BIN=${BIN:-./target/release/ccm-falsifications}
PREC=${PREC:-1000}

echo "=== Claim 1: CCM Lemma 7.2 sweep at HP-${PREC} ==="
echo "Conjecture: rel L∞ error × λ² ≈ const"
echo

# config = "lambda:n_modes:n_grid"
#   λ²=13   → λ=√13,   N=120
#   λ²=100  → λ=10,    N=500
#   λ²=1000 → λ=√1000, N=800
for cfg in \
  "3.6055512754639896:120:4001" \
  "10.0:500:8001" \
  "31.622776601683793:800:8001"; do
  lambda=${cfg%%:*}
  rest=${cfg#*:}
  n_modes=${rest%%:*}
  n_grid=${rest##*:}
  echo "--- λ=${lambda} (N=${n_modes}, N_grid=${n_grid}) ---"
  "$BIN" prolate-compare \
    --lambda "$lambda" --n-modes "$n_modes" \
    --precision-digits "$PREC" --n-grid "$n_grid"
  echo
done
