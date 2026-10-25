# Individual experiment scripts

[Overview](../README.md) · [Reproduction guide](../docs/RETESTING.md) ·
[Results and scope](../docs/RESULTS.md)

Each script runs independently, verifies its executable, and retains a fresh
journal. C denotes lambda squared, N the positive Fourier cutoff, and P decimal
precision. The full matrix has dimension 2N+1; its even sector has dimension N+1.
Filenames containing `lambda13`, `lambda100`, or `lambda1000` refer to C.
The seven `kappa` scripts use lambda=N, so C=N squared.

## Configurations

| Script | Default configuration | Measurement |
|---|---|---|
| [probe_mellin_c13_naive.sh](probe_mellin_c13_naive.sh) | C=13, P=100, t=14..15 | Small naive-transform control; no CCM source |
| [probe_indexed_c13.sh](probe_indexed_c13.sh) | C=13, N=120, P=1000, 20 roots | Bounded independent root control |
| [claim1a_lambda13_grid4001.sh](claim1a_lambda13_grid4001.sh) | C=13, N=120, grid=4001, P=1000 | Weil/prolate comparison |
| [claim1b_lambda100_grid4001.sh](claim1b_lambda100_grid4001.sh) | C=100, N=500, grid=4001, P=1000 | Weil/prolate comparison |
| [claim1c_lambda100_grid8001.sh](claim1c_lambda100_grid8001.sh) | C=100, N=500, grid=8001, P=1000 | Prolate grid refinement |
| [claim1d_lambda1000_grid8001.sh](claim1d_lambda1000_grid8001.sh) | C=1000, N=800, grid=8001, P=2000 | Weil/prolate comparison |
| [claim2a_kappa50.sh](claim2a_kappa50.sh) | C=2500, N=50, P=1000 | All 50 indexed roots |
| [claim2b_kappa100.sh](claim2b_kappa100.sh) | C=10000, N=100, P=1000 | All 100 indexed roots |
| [claim2c_kappa150.sh](claim2c_kappa150.sh) | C=22500, N=150, P=1000 | All 150 indexed roots |
| [claim2d_kappa200.sh](claim2d_kappa200.sh) | C=40000, N=200, P=1000 | All 200 indexed roots |
| [claim2e_kappa300.sh](claim2e_kappa300.sh) | C=90000, N=300, P=1000 | All 300 indexed roots |
| [claim2f_kappa400.sh](claim2f_kappa400.sh) | C=160000, N=400, P=2000 | All 400 indexed roots |
| [claim2g_kappa500.sh](claim2g_kappa500.sh) | C=250000, N=500, P=1500 | All 500 indexed roots |
| [claim2_standard_ccm.sh](claim2_standard_ccm.sh) | (C,N)=(13,120),(100,500),(1000,800), shared P=2000 | Standard configurations; individual row settings below |
| [claim3a_lambda13_weighted.sh](claim3a_lambda13_weighted.sh) | C=13, N=120, P=1000, t=5..55 | Naive and weighted complex residuals; Fourier control |
| [claim3b_lambda100_naive.sh](claim3b_lambda100_naive.sh) | C=100, P=1000, t=5..55 | Naive complex residuals; no CCM source |

The 13 named experiments and three standard configurations have retained
completion evidence. The two probes are additional controls, not replacements
for those experiments. See [results and qualifications](../docs/RESULTS.md).

Reproduce the completed standard rows with their individual precisions:

```bash
bash scripts/claim2_standard_ccm.sh --lambda-squares 13 --n-values 120 --precision-digits 1000
bash scripts/claim2_standard_ccm.sh --lambda-squares 100 --n-values 500 --precision-digits 1000
bash scripts/claim2_standard_ccm.sh --lambda-squares 1000 --n-values 800 --precision-digits 2000
```

The kappa scripts retain a trimmed summary with `TRIM=0.2` alongside all requested
roots. `TRIM=0` disables trimming. A trimmed result does not replace a full-window
hypothesis. Script arguments override defaults; consult the
[override table](../docs/RETESTING.md#common-overrides-and-direct-cli).

## Before a long run

```bash
bash scripts/claim2f_kappa400.sh --help
bash scripts/claim2f_kappa400.sh --validate-only
```

Help does not build. Validation may build and checks the configuration without
starting a computation or creating a journal. Run scripts individually; there
is no automatic all-claims launcher. Primary-only reproduction and the inputs
needed for complete Ultra capture are explained in the
[reproduction guide](../docs/RETESTING.md#external-inputs).

Publication is off by default. With `--publish`, claim 1a targets both public
and private artifact repositories; all other scripts target private repositories,
subject to artifact visibility rules and configured access.

## Shared helpers

| File | Purpose |
|---|---|
| [claim_common.sh](claim_common.sh) | Build, verify the executable, create a unique invocation, log, and summarize |
| [check_binary.py](check_binary.py) | Verify source/lockfile identity, Toolkit revision, and complete-root capability |
| [summarize_runs.py](summarize_runs.py) | Reassess saved journals without recomputing or overwriting evidence |

The numerical calculations are implemented in Rust. These helpers preserve the
relationship between the executable, configuration, and reported result.
