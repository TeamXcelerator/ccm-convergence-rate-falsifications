# Convergence Claims for the CCM Construction: An Empirical Reassessment

Three reproducible experiments on the Connes–Consani–Moscovici construction:
Weil/prolate approximation, spectral errors motivated by Śliwiński's conjecture,
and auxiliary Mellin transforms.

**Author:** Ronnie Andrews, Jr. · [ORCID](https://orcid.org/0009-0003-9724-3104) ·
[randrewsmath@gmail.com](mailto:randrewsmath@gmail.com)<br>
**Manuscript:** v2.1, November 3, 2026<br>
**Software:** v2.1, using Xcelerator Toolkit v0.17.0

The revised manuscript reports close finite Weil/prolate agreement, evidence
against uniform inverse-logarithmic maximum-error decay in the tested spectral
range, and a fixed-cutoff obstruction for the auxiliary Mellin transforms.
These conclusions have distinct scopes; the finite comparisons do not settle
an asymptotic limit. The draft corrects the earlier paper's interpretation of
CCM Lemma 7.2 and its use of mean error to evaluate a maximum-error conjecture.

## Read and reproduce

| Resource | Contents |
|---|---|
| [Manuscript PDF](paper.pdf) · [LaTeX source](paper.tex) | Experiments, derivations, results, and qualifications |
| [Script catalog](scripts/README.md) | One script per experiment and exact configurations |
| [Reproduction guide](docs/RETESTING.md) | Dependencies, inputs, resource settings, and output files |
| [Results and scope](docs/RESULTS.md) | Completed configurations and interpretation of the evidence |
| [Numerical table data](docs/manuscript-results.json) | Retained values, source bindings, and verification scope |
| [Software validation](docs/VALIDATION.md) | Local checks and their limits |

## Start with a small control

Use Bash, Git, Python 3, and stable Rust 1.98 or later. Numerical computation is
implemented in Rust; Python handles executable verification and journal
summaries. High-precision experiments require Linux or WSL and FLINT 3 or later.
For Ubuntu 24.04:

```bash
sudo apt-get update
sudo apt-get install -y build-essential m4 libgmp-dev libmpfr-dev libmpc-dev libflint-dev pkg-config python3
cargo build --release --features arb --locked
bash scripts/probe_mellin_c13_naive.sh
```

This control needs no CCM matrix or external target. A GPU is optional.
The exact Toolkit revision and dependencies are pinned in
[Cargo.toml](Cargo.toml) and [Cargo.lock](Cargo.lock).

Inspect an individual experiment before starting it:

```bash
bash scripts/claim1a_lambda13_grid4001.sh --help
bash scripts/claim1a_lambda13_grid4001.sh --validate-only
```

Help requires no build. Configuration validation may build the executable, but
starts no numerical acquisition and creates no run journal. It does not certify
that resources or external inputs will suffice for the full calculation.

The scripts default to **Ultra** diagnostic capture, with sector-gap
certificates off. Some diagnostics require external target and prepared inputs;
see the [input guide](docs/RETESTING.md#external-inputs).
Every computation keeps a fresh journal. Remote publication is off unless
explicitly requested.

## Understand the result

Read **run integrity**, **finite hypothesis outcomes**, and **capture
completeness** separately. A negative finite hypothesis can be a valid completed
experiment. An acquired artifact may still contain a conditional bound or an
unresolved numerical row. Neither high precision nor a successful invocation
alone establishes an asymptotic theorem.

Reassess saved journals without rerunning the calculation:

```bash
python3 scripts/summarize_runs.py /absolute/path/to/claim-invocation
```

The [results guide](docs/RESULTS.md) defines the status categories and explains
which conclusions the current evidence supports.

## Citation

Cite the manuscript edition used. For a reproduction, also retain the software
commit, Toolkit revision, configuration, and run identity. The `v2.1` Git tag
identifies this repository edition; a Zenodo deposit is maintained separately.

```bibtex
@misc{andrews2026ccmreassessment,
  author = {Andrews, Ronnie Jr.},
  title = {Convergence Claims for the CCM Construction: An Empirical Reassessment},
  year = {2026},
  note = {Version 2.1, November 3, 2026},
  url = {https://github.com/TeamXcelerator/ccm-convergence-rate-falsifications}
}
```

## License

See [LICENSE](LICENSE). Source-available for verification and study.
Not licensed for modification, redistribution, or commercial use.
The included manuscript files are licensed under CC BY-NC-ND 4.0.

"Team Xcelerator Inc." is a registered trademark of Team Xcelerator Inc.
