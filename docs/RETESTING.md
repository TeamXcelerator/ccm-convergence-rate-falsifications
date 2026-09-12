# Retesting Paper 2

The v2.1.0 harness pins Toolkit v0.15.0 at
`545041c192cb5a8b78a7dd34c53f93c8bd40681a`. The manuscript remains the
historical v2.0 text pending author review. Harness validation is not a
completed rerun of the manuscript tables.

## Start with individual experiments

```bash
cargo build --release --features arb --locked
bash scripts/probe_mellin_c13_naive.sh
bash scripts/probe_indexed_c13.sh
```

Run these separately and inspect each summary before selecting a larger job.
The first probe evaluates the naive transform around its first reported
real-part crossing; it requires no CCM matrix. The second requests 20
independently discovered movable roots at C=13, N=120, P=1000 and retains
Ultra diagnostics. It does not establish a certified full spectral count.

`C=lambda^2`; `N` is the positive Fourier cutoff (full matrix dimension
`2N+1`, even-sector dimension `N+1`); `P` in scripts denotes decimal digits.
Journals additionally record the resolved MPFR precision in bits.

| Script | Configuration | Question |
|---|---|---|
| `claim1a_lambda13_grid4001.sh` | C=13, N=120, grid=4001 | Weil/prolate sampled approximation |
| `claim1b_lambda100_grid4001.sh` | C=100, N=500, grid=4001 | Cutoff dependence |
| `claim1c_lambda100_grid8001.sh` | Same C,N, grid=8001 | Prolate discretization sensitivity |
| `claim1d_lambda1000_grid8001.sh` | C=1000, N=800, grid=8001 | Source-resolution stress test; raise P first |
| `claim2a_kappa50.sh` through `claim2g_kappa500.sh` | lambda=N=50,100,150,200,300,400,500 | Full-window maximum/mean errors and tail structure |
| `claim2_standard_ccm.sh` | C=13,100,1000; N=120,500,800 | Historical three-row configuration list |
| `claim3a_lambda13_weighted.sh` | C=13, N=120, t=5..55 | Naive and weighted complex residuals; Fourier control |
| `claim3b_lambda100_naive.sh` | C=100 | Naive complex residuals |

The former combined claim-group and `retest_all_claims.sh` launchers were
removed. The historical `claim2_standard_ccm.sh` list can be narrowed to one
configuration, for example:

```bash
bash scripts/claim2_standard_ccm.sh --lambda-squares 13 --n-values 120
```

## Complete spectral windows

Independent spectral claims now isolate all positive movable roots of the
exact retained even point source, including roots above its largest pole.
The original pole-span scanner could return an incomplete window even when
the missing roots existed. It remains available to other Toolkit callers;
Paper 2's individual spectral scripts select complete-range acquisition.
No reference zeros enter this acquisition. Adaptive root-refinement precision
handles cancellation without changing the matrix/eigenstate precision.

The ordinary launcher builds the `arb` feature. On Ubuntu/WSL, install its
native dependency once before running a claim:

```bash
sudo apt-get update
sudo apt-get install -y libflint-dev pkg-config
```

FLINT 3 or newer is required. A build without Arb rejects the independent
spectral request before matrix work. Insufficient real-root counts, repeated
roots, non-even sources and unsuccessful refinement remain explicit failures
or incomplete results; they are never filled using reference ordinates.
Complete-range root artifacts have distinct identities. Compatible matrices,
eigenstates and source-only diagnostics remain reusable. Existing journals
and incomplete root artifacts are preserved; run the usual individual script
to produce a new qualified result from the retained source.

## Controlled comparisons

Script arguments override defaults. Keep C, N, P and observables fixed when
comparing numerical profiles. The `paper` profile selects adaptive-even
parity and the legacy solver **within the new toolkit**; it does not reproduce
the old toolkit implementation bit for bit. Root acquisition is a separate
choice:

```bash
bash scripts/probe_indexed_c13.sh --numerical-profile paper --root-acquisition seeded
bash scripts/probe_indexed_c13.sh --numerical-profile current --root-acquisition seeded
bash scripts/probe_indexed_c13.sh --numerical-profile current --root-acquisition independent
```

Seeded labels are reference indices. Independent discovery uses no reference
zeros for acquisition; reference values enter the subsequent comparison.
Missing, unconverged, duplicate, nonpositive or unordered roots prevent a
full-window aggregate verdict. Available rows are still saved. A partial
root request cannot assess a claim about all N roots.

For prolate comparisons, vary finite-difference grid, comparison sample count,
N and precision independently. The least-squares and discrete minimax fits
are different observables and are both retained. Sampled minimax error is
not a certified continuous supremum norm. The toolkit interface exposes
prolate eigenvalues and the sampled E-transform, but not the individual
h0/h4 eigenvectors or residual certificates. A future direct Lemma 7.2
experiment would require those additional observables and the lemma's exact
normalization; the current comparison does not test that lemma.

## Evidence and status

Each invocation creates a new directory beneath `claim-runs` (or
`CLAIM_RUN_ROOT`). No old journal is overwritten. A run contains:

- `request.json` and `build.json`: exact inputs, root policy, toolkit pin,
  source implementation digest and dependency lockfile;
- `primary.json` and `primary-sources.json` when a CCM source is used;
- `measurements.json`: all sampled pairs or per-index roots/errors, complex
  residuals, quadrature checks, source manifests and finite hypothesis checks;
- `measurement-receipt.json`: managed retention of those measurements using
  the existing `research_capture_receipt` artifact type;
- `capture-plan.json` and `capture.json` for supplemental diagnostics;
- `summary.json`, and `failure.json` when a required operation fails.

Measurements are written before supplemental capture and publication.
Compatible artifacts are reused; new semantic identities are recomputed
without rewriting historical evidence. The implementation digest normalizes
line endings and binds all Rust source files, Cargo.toml and Cargo.lock.
It distinguishes harness changes even when the toolkit pin is unchanged.

`PASS` and `FAIL` on **hypothesis** rows report finite numerical comparisons.
A negative hypothesis can be a successfully completed experiment.
`INCOMPLETE` means necessary evidence or an applicable diagnostic is missing
or unresolved. `UNASSESSED` means no conclusion is justified from this run.
`UNSUPPORTED` identifies a diagnostic excluded by its documented domain.
Run integrity fails for invalid required measurements or failed publication.
`--require-complete-capture` additionally requires every applicable requested
diagnostic to complete. Without it, an incomplete supplemental diagnostic
remains prominently reported and does not discard the primary measurements.

Reassess without rerunning:

```bash
python3 scripts/summarize_runs.py /path/to/claim-invocation
```

## Ultra applicability

Individual scripts default to Ultra: sector spectra and selected eigenpairs,
evenness, root conditioning, distance/profile/resolution and target-residual
analyses, deviation decomposition, prime-power and u-flow responses, prefix
ladder/checkpoints and retained-reduction checks, as supported by the source.
Use `--research-capture claim` for measurements alone. A naive-transform-only
run has no CCM source and retains its complex measurements and both quadrature
rules; CCM diagnostics are inapplicable.

Target-dependent diagnostics require an on-disk specification selected by
`XC_TARGET_SPEC_FILE`. A missing or invalid target is recorded, with its
reason, without erasing the primary result. Historical adaptive-even sources
exclude the even-sector-only response and checkpoint-eigenstate exports.
At C=1000, N=800, P=1000, the known source-resolution limitation excludes
prime/u responses, prefix exports and target refinement; the prolate source
comparison itself is marked incomplete. A distinct higher-precision run is
needed to determine whether the behavior survives. Grid and distance-profile
data remain available where supported.

Root interval certification and sector-gap proof artifacts are separate
assurance tasks, not inferred from Ultra capture or high decimal precision.
No interval-certified ordinal or asymptotic theorem is claimed by this harness.

## Computational changes

Fourier reconstruction uses a cosine recurrence. Mellin quadrature nodes,
weights, amplitudes and Weil samples are computed once per quadrature order;
frequency evaluations reuse them. Independent grid evaluations use Rayon.
Set `RAYON_NUM_THREADS` to control workers; the resolved count and elapsed time
are recorded. No unmeasured speedup factor is claimed.
