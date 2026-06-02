#!/usr/bin/env bash
# Full HP-1000 retest cycle for all 3 Paper B claims.
#
# Each claim computes ξ_λ on demand via the CCM construction; the
# toolkit manages the weil_eigvec cache transparently (no paper-local
# ξ files, no save-xi precondition step). Each claim can also be run
# individually via its own script:
#   scripts/claim1_lemma72_falsification.sh  (CCM Lemma 7.2 falsification)
#   scripts/claim2_sliwinski_conjecture.sh   (Śliwiński Conjecture 4.1)
#   scripts/claim3_mellin_vs_ccm.sh          (CCM vs naive Mellin)
#
# Author: Ronnie Andrews, Jr. (Team Xcelerator Inc.(R))
set -euo pipefail

echo "=== Paper B full retest cycle at HP-1000 ==="
echo "Toolkit: $(grep '^xc-spectral' Cargo.toml)"
echo "Started: $(date)"
echo

run_step() {
  local name=$1
  local script=$2
  echo "================================================================"
  echo "  $name"
  echo "================================================================"
  echo "Started: $(date)"
  bash "$script"
  echo "Finished: $(date)"
  echo
}

run_step "claim1_lemma72_falsification  (CCM Lemma 7.2)" scripts/claim1_lemma72_falsification.sh
run_step "claim2_sliwinski_conjecture   (Śliwiński 4.1)" scripts/claim2_sliwinski_conjecture.sh
run_step "claim3_mellin_vs_ccm          (CCM vs Mellin)" scripts/claim3_mellin_vs_ccm.sh

echo
echo "=== All claims complete ==="
