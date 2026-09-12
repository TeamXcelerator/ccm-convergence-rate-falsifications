# Individual Paper 2 experiments

Run scripts from any directory. Every script locates this repository, uses
its own `ccm-falsifications` executable and writes a fresh run directory.
None calls another paper's runner or modifies another paper's checkout.

| Group | Individual scripts |
|---|---|
| Initial controls | `probe_mellin_c13_naive.sh`, `probe_indexed_c13.sh` |
| Claim 1: Weil/prolate approximation | `claim1a_lambda13_grid4001.sh`, `claim1b_lambda100_grid4001.sh`, `claim1c_lambda100_grid8001.sh`, `claim1d_lambda1000_grid8001.sh` |
| Claim 2: indexed spectral errors | `claim2a_kappa50.sh`, `claim2b_kappa100.sh`, `claim2c_kappa150.sh`, `claim2d_kappa200.sh`, `claim2e_kappa300.sh`, `claim2f_kappa400.sh`, `claim2g_kappa500.sh` |
| Claim 2: standard-cutoff comparison | `claim2_standard_ccm.sh`; narrow to one C,N pair using the documented overrides |
| Claim 3: transform residuals | `claim3a_lambda13_weighted.sh`, `claim3b_lambda100_naive.sh` |
| Shared infrastructure | `claim_common.sh` builds, runs, logs and summarizes; `summarize_runs.py` reassesses saved evidence |

The [retest guide](../docs/RETESTING.md) explains exact configurations,
precision, capture applicability and verdicts. Claim numbers identify the
historical experiments, not an endorsement of their manuscript interpretation.
There is no automatic all-claims launcher.

Examples:

```bash
bash scripts/claim2a_kappa50.sh
bash scripts/claim2_standard_ccm.sh --lambda-squares 13 --n-values 120
bash scripts/claim1d_lambda1000_grid8001.sh --precision-digits 2000
```

The final example is an expensive resolution discriminator. Start with the
small controls and review their evidence before expanding the campaign.
