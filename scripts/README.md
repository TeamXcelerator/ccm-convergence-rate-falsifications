# Individual convergence experiments

[Repository overview](../README.md) · [Reproduction guide](../docs/RETESTING.md) ·
[Evidence and summaries](../docs/RESEARCH_EVIDENCE.md) · [Validation](../docs/VALIDATION.md)

Run each script separately. Every launcher locates this repository, uses its
own `ccm-falsifications` executable and creates a fresh invocation directory.
It checks the exact source/lock digest, Toolkit pin and complete-root
capability, including for a supplied `BIN`.

`C = lambda^2`; N is the positive Fourier cutoff, so the full matrix dimension
is 2N+1 and the even-sector dimension is N+1. P denotes decimal digits.
Historical filenames containing `lambda13`, `lambda100` or `lambda1000` refer
to C, whereas the `kappa` scripts use lambda=N and therefore C=N^2.

## Choose one script

| Script | Default configuration | Measurement |
|---|---|---|
| [probe_mellin_c13_naive.sh](probe_mellin_c13_naive.sh) | C=13, P=100, t=14..15 | Small naive-transform control; no CCM source |
| [probe_indexed_c13.sh](probe_indexed_c13.sh) | C=13, N=120, P=1000, 20 roots | Bounded independent root control |
| [claim1a_lambda13_grid4001.sh](claim1a_lambda13_grid4001.sh) | C=13, N=120, grid=4001 | Weil/prolate comparison |
| [claim1b_lambda100_grid4001.sh](claim1b_lambda100_grid4001.sh) | C=100, N=500, grid=4001 | Weil/prolate comparison |
| [claim1c_lambda100_grid8001.sh](claim1c_lambda100_grid8001.sh) | C=100, N=500, grid=8001 | Prolate grid refinement |
| [claim1d_lambda1000_grid8001.sh](claim1d_lambda1000_grid8001.sh) | C=1000, N=800, grid=8001 | Source-resolution stress test; review precision first |
| [claim2a_kappa50.sh](claim2a_kappa50.sh) | C=2500, N=50 | All 50 indexed roots |
| [claim2b_kappa100.sh](claim2b_kappa100.sh) | C=10000, N=100 | All 100 indexed roots |
| [claim2c_kappa150.sh](claim2c_kappa150.sh) | C=22500, N=150 | All 150 indexed roots |
| [claim2d_kappa200.sh](claim2d_kappa200.sh) | C=40000, N=200 | All 200 indexed roots |
| [claim2e_kappa300.sh](claim2e_kappa300.sh) | C=90000, N=300 | All 300 indexed roots |
| [claim2f_kappa400.sh](claim2f_kappa400.sh) | C=160000, N=400 | All 400 indexed roots |
| [claim2g_kappa500.sh](claim2g_kappa500.sh) | C=250000, N=500 | All 500 indexed roots |
| [claim2_standard_ccm.sh](claim2_standard_ccm.sh) | (C,N)=(13,120),(100,500),(1000,800) | Historical three-row list; narrow as below |
| [claim3a_lambda13_weighted.sh](claim3a_lambda13_weighted.sh) | C=13, N=120, t=5..55 | Naive and weighted complex residuals; Fourier control |
| [claim3b_lambda100_naive.sh](claim3b_lambda100_naive.sh) | C=100, t=5..55 | Naive complex residuals; no CCM source |

All claim scripts default to P=1000. The seven `kappa` scripts also retain a
trimmed summary with `TRIM=0.2`, alongside the full requested-window evidence;
`TRIM=0` disables trimming. A trimmed result cannot substitute for an all-N
hypothesis. Script arguments override defaults; see the
[override table](../docs/RETESTING.md#common-overrides-and-direct-cli).

Claim numbers identify the manuscript experiments. Their interpretation is
qualified in the [evidence guide](../docs/RESEARCH_EVIDENCE.md). There is no
automatic all-claims launcher. To run just one row of the standard list:

```bash
bash scripts/claim2_standard_ccm.sh --lambda-squares 13 --n-values 120
```

For a separate higher-precision source-resolution comparison:

```bash
bash scripts/claim1d_lambda1000_grid8001.sh --precision-digits 2000
```

This is an expensive job; start with the small controls and review their
results before expanding the campaign. Target-dependent Ultra diagnostics
require the [on-disk target setup](../docs/RETESTING.md#directories-and-local-cache).

## Shared infrastructure

| File | Purpose |
|---|---|
| [claim_common.sh](claim_common.sh) | Build, validate the executable, create a unique invocation, log and summarize |
| [check_binary.py](check_binary.py) | Verify an executable against the current source, lockfile and complete-root capability |
| [summarize_runs.py](summarize_runs.py) | Reassess saved journals without recomputing or overwriting evidence |

The scripts use this repository's executable, configuration and journals.
