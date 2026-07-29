#!/usr/bin/env bash
# Claim 1 wrapper: reproduce all four rows, including the grid-convergence pair.
set -euo pipefail

for script in \
  scripts/claim1a_lambda13_grid4001.sh \
  scripts/claim1b_lambda100_grid4001.sh \
  scripts/claim1c_lambda100_grid8001.sh \
  scripts/claim1d_lambda1000_grid8001.sh; do
  bash "$script" "$@"
  echo
done
