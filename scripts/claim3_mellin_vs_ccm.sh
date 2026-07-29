#!/usr/bin/env bash
# Claim 3 wrapper: reproduce the λ²=13 weighted table and λ²=100 naive table.
set -euo pipefail

for script in \
  scripts/claim3a_lambda13_weighted.sh \
  scripts/claim3b_lambda100_naive.sh; do
  bash "$script" "$@"
  echo
done
