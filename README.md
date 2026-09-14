# Empirical Falsification of Convergence Rate Predictions for the Connes–Consani–Moscovici Zeta Spectral Triple

Reproducible finite tests of Weil/prolate approximation, indexed root errors,
and complex Mellin-transform residuals for the Connes–Consani–Moscovici (CCM)
construction. The accompanying software provides independently runnable
experiments and records their numerical evidence.

**Author:** Ronnie Andrews, Jr.<br>
**ORCID:** [0009-0003-9724-3104](https://orcid.org/0009-0003-9724-3104)<br>
**Contact:** [randrewsmath@gmail.com](mailto:randrewsmath@gmail.com)<br>
**Software release:** v2.1 (Xcelerator Toolkit v0.15.0)<br>
**Manuscript:** v2.0

The software release provides individual experiment scripts, Ultra research
capture and explicit result summaries. [Cargo.toml](Cargo.toml) and
[Cargo.lock](Cargo.lock) pin Toolkit revision
`5d50b5b862b075a43e7c2c9ccedd29b568a32808`. The manuscript and software have
separate version histories; software qualification is not a completed rerun
or a revision of the manuscript's conclusions.

## Read the paper and evidence

| I want to... | Open |
|---|---|
| Read the manuscript | [PDF](paper.pdf) · [LaTeX source](paper.tex) |
| Choose one experiment | [Individual script catalog](scripts/README.md) |
| Set up and run a reproduction | [Reproduction guide](docs/RETESTING.md) |
| Interpret results and retained artifacts | [Research evidence guide](docs/RESEARCH_EVIDENCE.md) |
| Check software validation | [Validation report](docs/VALIDATION.md) · [Machine-readable record](docs/validation/v2.1.json) |

## Run one experiment

Use Rust 1.98 or later, Python 3, Git and Bash. The high-precision numerical
experiments run on Linux or WSL. Ubuntu 24.04 supplies the required FLINT 3
package:

```bash
sudo apt-get update
sudo apt-get install -y build-essential m4 libgmp-dev libmpfr-dev libmpc-dev libflint-dev pkg-config
```

From a checkout of this repository, build the locked high-precision executable
and start with the small transform control:

```bash
cargo build --release --features arb --locked
bash scripts/probe_mellin_c13_naive.sh
```

Before a CCM run with target-dependent diagnostics, select the target
specification **file on disk**:

```bash
export XC_TARGET_SPEC_FILE=/absolute/path/to/runtime-target.json
test -r "$XC_TARGET_SPEC_FILE"
```

Then run the bounded indexed-root control separately:

```bash
bash scripts/probe_indexed_c13.sh
```

The first control requires no CCM matrix or target. The second requests 20
independently discovered movable roots at C=13, N=120, P=1000. All individual
scripts default to the current even-sector solver, independent root discovery
and Ultra capture. The [setup and override instructions](docs/RETESTING.md)
explain directories, the target file, precision, and selecting a larger claim.

Every invocation writes a fresh directory and prints its location. Scripts
build automatically unless `BIN` selects an existing executable; the launcher
checks that executable against the exact source, lockfile, Toolkit pin and
complete-root capability before computation.

## Read the summary

The terminal distinguishes **run integrity**, **finite hypothesis outcomes**
and **capture completeness**. This illustrative excerpt shows how a negative
finite hypothesis can still be a valid completed experiment:

```text
[PASS] run integrity: run-...
[FAIL] hypothesis: <finite comparison> — <measured reason>
Overall run integrity: PASS
```

| Status | Meaning |
|---|---|
| PASS | The named finite check or required operation passed. |
| FAIL | The named hypothesis failed; read its category and measured reason. This alone is not an execution failure. |
| INCOMPLETE | Required evidence or an applicable diagnostic is missing, invalid or unresolved. |
| UNASSESSED | The run does not decide the stated question. |
| UNSUPPORTED | A diagnostic is outside the source's supported domain; its reason is recorded. |

Use `--require-complete-capture` to make incomplete applicable capture fail the
invocation. Without that option, supplemental incompleteness is still reported
and the primary measurements remain available. Review both the run summary and
the capture status before treating the evidence as complete.

Reassess the invocation directory printed by the script without recomputing:

```bash
python3 scripts/summarize_runs.py /absolute/path/to/claim-invocation
```

## What is measured

| Experiment | Retained evidence |
|---|---|
| Weil/prolate approximation | Sample pairs, least-squares and discrete minimax fits, normalization, prolate eigenvalues, grid/precision and source identities |
| Indexed root errors | Every requested index, missing rows, absolute and relative errors, means, maxima, log products, quantiles, cumulative error and accuracy prefixes |
| Transform comparison | Real-part crossings with both complex components, quadrature refinement, reference assignments and a finite Fourier/secular control where applicable |
| Ultra capture | Applicable sector, response, distance, conditioning, prefix and retained-reduction diagnostics bound to the same CCM source |

Ultra includes the prefix ladder and a full even-sector checkpoint at dimension
N+1 when supported. Additional checkpoints and working precision are selectable.
The [evidence guide](docs/RESEARCH_EVIDENCE.md) explains the retained files,
capture controls and exclusions, including configurations limited by source
resolution. High precision and finite checks alone do not establish interval
certification or an asymptotic theorem.

Compatible cached inputs are reused; unavailable inputs are computed locally
under their current identities. Existing artifacts and journals are preserved.
Remote cache access and artifact publication are optional. See the
[local cache setup](docs/RETESTING.md#directories-and-local-cache).

## Local validation

```bash
python3 -m unittest discover -s tests -p 'test_*.py' -v
cargo test --locked
cargo test --release --locked --features arb
cargo clippy --locked --all-targets -- -D warnings
cargo clippy --release --locked --all-targets --features arb -- -D warnings
```

Default Rust tests also run on Windows. See the
[validation report](docs/VALIDATION.md) for checked configurations and scope.

## Paper and citation

Cite the manuscript edition actually used. When reporting a new experiment,
record the software revision, Toolkit pin, configuration and run identity as
well; a software release number does not identify a new manuscript edition.

```bibtex
@misc{andrews2026ccmfalsifications,
  author = {Andrews, Ronnie Jr.},
  title = {Empirical Falsification of Convergence Rate Predictions for the Connes--Consani--Moscovici Zeta Spectral Triple},
  year = {2026},
  note = {Manuscript version 2.0},
  url = {https://github.com/TeamXcelerator/ccm-convergence-rate-falsifications}
}
```

## License

See [LICENSE](LICENSE). Source-available for verification and study.
Not licensed for modification, redistribution, or commercial use.
The included manuscript files are licensed under CC BY-NC-ND 4.0.

"Team Xcelerator Inc." is a registered trademark of Team Xcelerator Inc.
