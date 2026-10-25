# Reproducing the experiments

[Overview](../README.md) · [Script catalog](../scripts/README.md) · [Results and scope](RESULTS.md)

Run one experiment at a time and retain its configuration, executable identity,
measurements, and summary. The scripts preserve previous runs and check that
the executable matches the current source and lockfile.

## Dependencies and build

Use Linux or WSL, Bash, Git, Python 3, and stable Rust 1.98 or later. On Ubuntu
24.04, install the high-precision dependencies and build:

```bash
sudo apt-get update
sudo apt-get install -y build-essential m4 libgmp-dev libmpfr-dev libmpc-dev libflint-dev pkg-config python3
cargo build --release --locked --features arb --bin ccm-falsifications
```

FLINT 3 or later supplies the Arb support required by complete root discovery.
The `arb` feature also enables MPFR/GMP high precision. Ordinary default-feature
Rust tests are useful software checks, but do not substitute for this build.
Numerical work runs in Rust; the Python helpers verify executable identity and
summarize journals.

The scripts build automatically unless `BIN` selects a prebuilt executable:

```bash
export BIN="$PWD/target/release/ccm-falsifications"
bash scripts/probe_mellin_c13_naive.sh
```

A stale or incompatible `BIN` is rejected before computation. Unset it to let
the script rebuild. Keep [Cargo.toml](../Cargo.toml) and
[Cargo.lock](../Cargo.lock) together when reproducing a revision.

## Directories and local cache

For a local run without remote cache access or publication:

```bash
mkdir -p .xcelerator-local/claim-runs .xcelerator-cache
export CLAIM_RUN_ROOT="$PWD/.xcelerator-local/claim-runs"
export XC_CACHE_ROOT="$PWD/.xcelerator-cache"
export XC_CACHE_MODE=reuse
export XC_CACHE_REMOTE=none
export XC_PUBLISH_TARGET=none
export XC_PUBLISH_EXECUTE=false
```

Compatible cache entries are reused by identity. Missing sources are computed
within the configured resource limits. Every invocation receives a new directory,
including when `--capture-output` supplies the parent. Existing evidence is not
overwritten. The wrapper prints the final location.

## Choose a configuration

The [script catalog](../scripts/README.md) lists all experiments and the exact
commands for the three standard rows. C denotes lambda squared; N is the positive
Fourier cutoff, giving a full dimension of 2N+1 and an even dimension of N+1.
Precision arguments are decimal digits.

Start with the small naive-transform control. Before a larger run, inspect help
and validate its configuration:

```bash
bash scripts/claim1a_lambda13_grid4001.sh --help
bash scripts/claim1a_lambda13_grid4001.sh --validate-only
```

Help starts no build. Validation may build and verify the executable, then exits
before numerical acquisition and journal creation. It checks configuration,
not completion of external preparation or a guarantee of sufficient resources.

Scripts default to the `current` numerical profile, independent root acquisition,
and Ultra capture. Sector-gap certificates are off. Cross-configuration
comparison diagnostics remain deferred until compatible configurations are
available. The `paper` profile selects the historical numerical route within
the current implementation; it is not an old release binary.

## External inputs

The small naive-only control requires no external target. Some CCM diagnostics
need a target specification and source-dependent prepared inputs that are not
bundled with this repository. Select an existing compatible target explicitly:

```bash
export XC_TARGET_SPEC_FILE=/absolute/path/to/runtime-target.json
test -r "$XC_TARGET_SPEC_FILE"
```

A target filename alone is not evidence of compatibility: its definition,
precision, and source bindings must pass the preparation and capture checks.
The external preparer interface is implemented in
[src/preparation.rs](../src/preparation.rs). A configured `XC_RESEARCH_PREPARER`
requires a target specification; prepared data must join to the acquired source.
Private preparation software and operational credentials are not public
reproduction dependencies for the primary experiment.

To run primary measurements with the lighter capture profile, without claiming
complete target-dependent Ultra evidence:

```bash
unset XC_TARGET_SPEC_FILE XC_RESEARCH_PREPARER
bash scripts/probe_indexed_c13.sh --research-capture claim
```

For complete Ultra evidence, supply the required inputs and inspect both
preparation status and every applicable capture outcome. The
[results guide](RESULTS.md) explains acquired artifacts whose numerical scope
remains qualified.

## Common overrides and direct CLI

| Setting | Effect |
|---|---|
| `PREC`, or `--precision-digits` | Requested decimal precision; use the catalog values to match a manuscript row |
| `RAYON_NUM_THREADS` | Numerical worker limit |
| `CARGO_BUILD_JOBS` | Build concurrency |
| `XC_CLAIM_MEMORY_GIB` | Working-memory admission budget; wrapper default is 32 GiB |
| `XC_CLAIM_STREAM_SCALAR_BYTES` | Cumulative scalar-text budget; default 256 GiB, not a resident-memory estimate |
| `CLAIM_RUN_ROOT`, or `--capture-output` | Parent for a new invocation directory |
| `RESEARCH_CAPTURE`, or `--research-capture` | Diagnostic profile; wrapper default `ultra` |
| `ROOT_ACQUISITION`, or `--root-acquisition` | Source root route; wrapper default `independent` |
| `NUMERICAL_PROFILE`, or `--numerical-profile` | Numerical profile; wrapper default `current` |
| `TRIM` | Upper-window trimming fraction in the seven kappa scripts; default 0.2 |

Explicit script arguments override numerical defaults. For example:

```bash
RAYON_NUM_THREADS=8 CARGO_BUILD_JOBS=4 XC_CLAIM_MEMORY_GIB=32 \
  bash scripts/claim1a_lambda13_grid4001.sh --precision-digits 1000
```

The example sets limits, not a promise that every diagnostic fits them. Larger
C, N, precision, and root windows can require substantially more work. A resource
refusal must remain visible in the result; reducing precision changes the
experiment and should be recorded as a different configuration.

The executable also provides direct help:

```bash
"$BIN" --help
"$BIN" prolate-compare --help
"$BIN" sliwinski-check --help
"$BIN" mellin-compare --help
```

The optional `--parallel-gl-roots` route requires its experimental Toolkit build
feature and native-Linux qualification; it is unsupported on WSL. Ordinary
scripts do not enable it. GPU acceleration is optional and subject to the
Toolkit's runtime admission and exact-product checks.

## Numerical interpretation

Independent root acquisition does not use reference zeta zeros as seeds. Those
references enter the subsequent error comparison. The indexed experiments retain
the full requested movable-root window; the trimmed summary is additional and
cannot substitute for a full-window maximum.

The prolate experiment uses a finite Dirichlet discretization and sampled
profiles. Its grid refinement tests a finite numerical approximation, not the
continuum statement in CCM Lemma 7.2. In the Mellin experiment, a real-part
crossing is not a complex zero: both components must meet the residual test.

## Read or reassess a run

Read the printed run integrity, capture completeness, and individual finite
checks. To reassess saved evidence without recomputing:

```bash
python3 scripts/summarize_runs.py /absolute/path/to/claim-invocation
```

An optional `--output /path/to/new-assessment.json` creates a new summary and
refuses to overwrite one. An execution failure, logging failure, or invalid
journal causes the wrapper to return nonzero. A negative finite hypothesis
alone does not. Use `--require-complete-capture` when all applicable diagnostic
acquisitions must also succeed.

## Optional artifact publication

Publication is off by default, including when inherited environment settings
would otherwise enable it. `--publish` explicitly enables the configured policy:
claim 1a targets both public and private repositories; every other script targets
private repositories. Toolkit artifact visibility rules still apply.

Publication requires destination access, credentials, and a configured staging
root. A public reader can reproduce measurements locally without those private
services. Do not put tokens in a journal or command transcript.

If computation finished but publication failed, retain staging and use the
executable's `publish-staged --help` interface to retry publication. Review its
inventory and completion report separately from the mathematical result; a
successful push does not resolve a numerical qualification.
