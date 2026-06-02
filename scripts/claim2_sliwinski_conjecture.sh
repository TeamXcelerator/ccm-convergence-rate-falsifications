#!/usr/bin/env bash
# Claim 2: Śliwiński Conjecture 4.1 sweep across κ ∈ {50..500}
# at HP-1000 (publication standard). Trim-20% metric matches the
# published README table.
#
# This is the single-server wrapper: runs all seven sub-scripts
# sequentially. Wall-clock for the full sweep is dominated by the
# last two configs (κ=400 ~3-5 hr, κ=500 many hours), so a single
# server runs for the better part of a day.
#
# For parallel multi-server reproductions, run the seven
# claim2{a..g}_kappa*.sh sub-scripts independently on separate
# servers. The cache directory is per-cwd and the configs share
# no τ-cache or GL-cache hits, so concurrent runs do not collide.
set -euo pipefail

bash scripts/claim2a_kappa50.sh
bash scripts/claim2b_kappa100.sh
bash scripts/claim2c_kappa150.sh
bash scripts/claim2d_kappa200.sh
bash scripts/claim2e_kappa300.sh
bash scripts/claim2f_kappa400.sh
bash scripts/claim2g_kappa500.sh
