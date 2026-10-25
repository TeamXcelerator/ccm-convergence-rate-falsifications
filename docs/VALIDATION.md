# Software validation

[Overview](../README.md) · [Reproduction guide](RETESTING.md) · [Results and scope](RESULTS.md)

Software checks test the implementation and reporting contracts. They are
separate from the mathematical interpretation of a manuscript result and from
audit of the stored inputs used by an earlier run.

## Run locally

Launcher and journal checks need Bash and Python 3. They use synthetic fixtures;
they do not start a numerical experiment or publish artifacts:

```bash
python3 -B -m unittest discover -s tests -p 'test_*.py' -v
bash tests/test_claim_scripts.sh
```

For Rust checks, use the dependencies in the [reproduction guide](RETESTING.md):

```bash
cargo test --locked
cargo test --release --locked --features arb
cargo clippy --locked --all-targets -- -D warnings
cargo clippy --release --locked --all-targets --features arb -- -D warnings
```

The default-feature tests also run on Windows. The high-precision tests require
Linux or WSL with FLINT 3 or later. A successful default-feature run does not
qualify the high-precision path. These are local commands; no hosted CI run is
required.

## What is checked

- Executables must match the current source and lockfile digest, Toolkit
  revision, and complete-root capability. A shared version label is insufficient.
- Journal reassessment rejects missing or malformed evidence and preserves
  available failure information. It distinguishes a negative finite hypothesis
  from an execution failure.
- Launcher regressions cover help, configuration-only validation, explicit
  publication policy, invalid output arguments, unique journals, and propagation
  of computation or evidence failures.
- Rust unit and integration tests cover argument policies, numerical helpers,
  and the feature-specific experiment interfaces. Review individual test scope;
  a small test is not a complete manuscript reproduction.

## Current documentation and launcher review

The November 3, 2026 public-repository cleanup preserves the numerical Rust
sources, manuscript, and machine-readable table data. Its checks target the
shell interface and documentation. In particular, `--validate-only` now exits
after configuration validation instead of trying to summarize a nonexistent
computation journal, and `--help` works after other arguments.

The review passed 42 wrapper invocation checks, eight launcher regression cases,
16 existing executable-contract and journal-summary tests, and 55 local
documentation link checks. Manuscript, numerical-source, and table-data hashes
were unchanged. No full numerical campaign was repeated for this cleanup.

Historical numerical and GPU qualification records remain bound to their exact
source revisions and inputs. This editorial review does not rerun or extend
those qualifications. The [results guide](RESULTS.md) describes the completed
experiments and their remaining mathematical limitations.

## v2.1 release checks

The November 4, 2026 release check passed 28 Rust tests with the `arb` feature,
formatting, strict Clippy, and a locked optimized build under Rust 1.98.1.
The resulting executable passed the source/lockfile identity check and admitted
configuration-only validation for all 16 script wrappers. The 16 helper tests
and eight launcher regression cases also passed. The eight-page PDF was rebuilt
with resolved citations and references and visually checked on every page.

These are bounded local release checks. No full claim campaign or new GPU
performance qualification was run for this release.
