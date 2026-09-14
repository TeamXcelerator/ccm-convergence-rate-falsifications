# Reproducing the convergence experiments

[Repository overview](../README.md) · [Individual scripts](../scripts/README.md) ·
[Research evidence](RESEARCH_EVIDENCE.md) · [Validation](VALIDATION.md)

The v2.1.0 harness pins Toolkit v0.15.0 at
`5d50b5b862b075a43e7c2c9ccedd29b568a32808`. The manuscript remains the
v2.0 edition. Software validation is not a
completed rerun of the manuscript tables.

## Start with individual experiments

```bash
cargo build --release --features arb --locked
bash scripts/probe_mellin_c13_naive.sh
```

After configuring the target file below, run the CCM control separately:

```bash
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

## Directories and local cache

Start in a checkout of this repository. The following optional setup keeps
journals and reusable data together and selects local computation/reuse:

```bash
mkdir -p .xcelerator-local/claim-runs .xcelerator-cache
export CLAIM_RUN_ROOT="$PWD/.xcelerator-local/claim-runs"
export XC_CACHE_ROOT="$PWD/.xcelerator-cache"
export XC_CACHE_REMOTE=none
export XC_PUBLISH_TARGET=none
export XC_PUBLISH_EXECUTE=false
```

These local directories are ignored by Git. Without `CLAIM_RUN_ROOT`, the
launcher creates its journals under `claim-runs/`. An explicit
`--capture-output /absolute/path/to/runs` selects a different parent. Each
invocation still gets a unique subdirectory.

For target-dependent capture, point to the JSON specification file defining
the target for your experiment:

```bash
export XC_TARGET_SPEC_FILE=/absolute/path/to/runtime-target.json
test -r "$XC_TARGET_SPEC_FILE"
python3 -m json.tool "$XC_TARGET_SPEC_FILE" > /dev/null
```

Replace the example path with an existing file. The JSON check verifies syntax;
the Toolkit validates the target schema and numerical definition during use.
The file is an experiment input, not a directory or an inline formula.
Do not substitute an arbitrary target when comparing an existing distance
measurement. Missing or invalid target input affects the corresponding
diagnostics and is reported in the capture outcomes.

## Common overrides and direct CLI

| Setting | Selection |
|---|---|
| Primary decimal precision | `PREC=2000 bash scripts/claim2f_kappa400.sh`, or `--precision-digits 2000` |
| Worker pool | `RAYON_NUM_THREADS=8` before the command |
| Existing executable | `BIN=/absolute/path/to/ccm-falsifications`; the source/lock contract must pass |
| Script capture default | `RESEARCH_CAPTURE=ultra`, or explicit `--research-capture ultra` |
| Numerical profile | `NUMERICAL_PROFILE=current`, or `--numerical-profile current` |
| Root acquisition | `ROOT_ACQUISITION=independent`, or `--root-acquisition independent` |

Command-line overrides take precedence. Use one individual script at a time;
raising P or N can substantially increase work. A running executable keeps its
original revision even if another checkout is updated.

For direct CLI use, request capture explicitly (the executable defaults to
`claim`, whereas the scripts default to `ultra`):

```bash
./target/release/ccm-falsifications --help
./target/release/ccm-falsifications sliwinski-check --help
./target/release/ccm-falsifications sliwinski-check \
  --lambda-squares 13 --n-values 120 --precision-digits 1000 --root-count 20 \
  --research-capture ultra --capture-output .xcelerator-local/direct-runs
```

Direct execution creates `run-*` journals under the specified parent. Pass
that parent to `summarize_runs.py` to obtain an aggregate assessment.

## Complete spectral windows

Independent spectral claims now isolate all positive movable roots of the
exact retained even point source, including roots above its largest pole.
The original pole-span scanner could return an incomplete window even when
the missing roots existed. It remains available to other Toolkit callers;
The individual spectral scripts select complete-range acquisition.
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
`XC_TARGET_SPEC_FILE`; see [directory and input setup](#directories-and-local-cache).
A missing or invalid target is recorded, with its reason, without erasing the primary result. Historical adaptive-even sources
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

Response capture uses the v0.15.0 performance amendment: fixed root quantities
are prepared once, independent events and root batches run in parallel, and
fresh output uses its completed producer checks plus an exact process-local
payload seal. Cached responses retain numerical replay. Large response phases
report event progress. Artifact formats and identities are unchanged; existing
results need no flush or repair. A running executable continues to use its
original revision. Complete-root isolation before root-window reuse remains a
separate acquisition cost. See the Toolkit
[performance and validation guide](https://github.com/TeamXcelerator/xcelerator-toolkit/blob/v0.15.0/docs/CCM_RESPONSE_PERFORMANCE.md).

Fourier reconstruction uses a cosine recurrence. Mellin quadrature nodes,
weights, amplitudes and Weil samples are computed once per quadrature order;
frequency evaluations reuse them. Independent grid evaluations use Rayon.
Set `RAYON_NUM_THREADS` to control workers; the resolved count and elapsed time
are recorded. No unmeasured speedup factor is claimed.

## Publication progress

The pinned Toolkit reports batch sizes, existing bytes reused, elapsed push
time and progress heartbeats during long Git operations. Its artifact push
policy avoids delta searches on already-compressed archives. See the
[Toolkit publication guide](https://github.com/TeamXcelerator/xcelerator-toolkit/blob/v0.15.0/docs/PUBLICATION_PERFORMANCE.md)
for measured scope and recovery behavior. This transport amendment does not
require recalculating completed experiments or clearing their caches. Preserve
run journals and staging if publication is interrupted. An already-running
executable continues using its original implementation until it exits.

Large encoded artifacts can exceed the default workstation transfer ceiling. Set `XC_RESOURCE_POLICY_FILE` to an explicit resource policy appropriate to the machine; see the Toolkit [publication recovery guide](https://github.com/TeamXcelerator/xcelerator-toolkit/blob/v0.15.0/docs/PUBLICATION_RECOVERY.md). If packaging fails after local retention, recover the exact saved artifact without repeating the numerical claim. Preserve the original capture receipt and record recovery separately.
