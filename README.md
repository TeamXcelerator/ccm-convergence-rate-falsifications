# CCM convergence experiments — Paper 2

Reproducible finite tests of Weil/prolate approximation, indexed root errors,
and complex Mellin-transform residuals for the Connes–Consani–Moscovici
construction.

**Harness v2.1.0 · Xcelerator Toolkit v0.15.0**, pinned to
`545041c192cb5a8b78a7dd34c53f93c8bd40681a` through Cargo.toml and Cargo.lock.

The historical [manuscript source](paper.tex) and [PDF](paper.pdf) remain
available. Their substantive interpretations are under author review; this
harness update is not a new manuscript edition. In particular, the existing
Weil/prolate comparison does **not** test CCM Lemma 7.2. See the
[proposed manuscript corrections](docs/MANUSCRIPT_REVIEW.md).

## Run one experiment

Use Rust 1.98 or later and Linux/WSL for the MPFR/GMP high-precision tier.
On Ubuntu, install `build-essential m4 libgmp-dev libmpfr-dev libmpc-dev libflint-dev pkg-config`
(FLINT 3 or newer).

```bash
cargo build --release --features arb --locked
bash scripts/probe_mellin_c13_naive.sh
```

Then run the bounded CCM root-window control separately:

```bash
bash scripts/probe_indexed_c13.sh
```

Scripts default to the current even-sector solver, independent root discovery,
and Ultra research capture. All configurations, measurements and status reasons
are retained in new run directories. A supplied `BIN` skips rebuilding; the
launcher checks the exact source/lock digest, toolkit pin and full-range root
capability. Missing FLINT prerequisites are reported before compilation.
Target-dependent diagnostics require an
on-disk target specification through `XC_TARGET_SPEC_FILE`.

The [retest guide](docs/RETESTING.md) lists every individual claim, controlled
historical comparisons, input overrides, capture applicability and how to
reassess a saved run without recomputing it.

## What is measured

| Experiment | Retained evidence |
|---|---|
| Weil/prolate approximation | Complete sample pairs, least-squares and discrete minimax fits, normalization, prolate eigenvalues, cutoff/grid/precision and source identities |
| Indexed root errors | Every requested index including missing rows, absolute/relative errors, mean and maximum, log products, quantiles, cumulative error and accuracy prefixes |
| Transform comparison | Real-part crossings with both complex components, quadrature refinement, unmatched/colliding reference assignments and an exact finite Fourier/secular control |
| Supplemental Ultra capture | Applicable sector, response, distance, conditioning, prefix and retained-reduction diagnostics tied to the same CCM source |

Finite hypothesis results use **PASS/FAIL**. Missing or unresolved evidence is
**INCOMPLETE**, questions the run cannot decide are **UNASSESSED**, and
inapplicable diagnostics are **UNSUPPORTED** with reasons. A negative
hypothesis can be a successful experiment. Run integrity and capture
completeness are reported separately from mathematical conclusions.

New measurements use the toolkit's existing research-receipt artifact format.
Compatible cached inputs can be reused; unavailable inputs are computed
locally under their current identities. Historical artifacts and run journals
are preserved. Arithmetic precision and finite numerical checks do not by
themselves establish interval certification or an asymptotic theorem.

## Local validation

```bash
python3 -m unittest discover -s tests -p 'test_*.py' -v
cargo test --locked
cargo test --locked --features arb
cargo clippy --locked --all-targets --features arb -- -D warnings
```

Default tests run on Windows too; numerical experiment execution requires HP.
No GitHub Actions execution is required.

**Author:** Ronnie Andrews, Jr. ·
[ORCID 0009-0003-9724-3104](https://orcid.org/0009-0003-9724-3104) ·
[Contact](mailto:randrewsmath@gmail.com)

License terms are defined in [LICENSE](LICENSE). Cite the manuscript edition
actually used and record the harness revision, toolkit pin and run identity
when reporting new numerical measurements.
