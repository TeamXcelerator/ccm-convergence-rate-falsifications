#!/usr/bin/env bash
# Claim 2 wrapper: standard CCM regime followed by κ=N=λ sweep.
set -euo pipefail

for script in \
  scripts/claim2_standard_ccm.sh \
  scripts/claim2a_kappa50.sh \
  scripts/claim2b_kappa100.sh \
  scripts/claim2c_kappa150.sh \
  scripts/claim2d_kappa200.sh \
  scripts/claim2e_kappa300.sh \
  scripts/claim2f_kappa400.sh \
  scripts/claim2g_kappa500.sh; do
  bash "$script" "$@"
  echo
done
