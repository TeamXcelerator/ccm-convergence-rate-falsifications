# Empirical Falsification of Convergence Rate Predictions for the CCM Zeta Spectral Triple

Two quantitative predictions about how the Connes–Consani–Moscovici
zeta spectral triple converges to Riemann zeros are tested at high
precision. CCM Lemma 7.2 is falsified empirically, Śliwiński's
Conjecture 4.1 is unsupported over the tested range, and the CCM
construction substantially outperforms the tested Mellin truncations.

**Xcelerator Toolkit:** v0.13.3

**Author:** Ronnie Andrews, Jr.

**ORCID:** [0009-0003-9724-3104](https://orcid.org/0009-0003-9724-3104)

**Contact:** randrewsmath@gmail.com

**Issues:** [GitHub issue tracker](https://github.com/TeamXcelerator/ccm-convergence-rate-falsifications/issues)

## Citation

```text
Andrews, R. Jr. (2026). Empirical Falsification of Convergence Rate
Predictions for the CCM Zeta Spectral Triple. GitHub:
TeamXcelerator/ccm-convergence-rate-falsifications.
```

## Headline results

| Test | Predicted rate | HP measurement | Verdict |
|---|---|---|---|
| CCM Lemma 7.2 | relative error × λ² approximately constant | grows by about 1,116,000× from λ²=13 to λ²=1000 | **falsified** |
| Śliwiński Conjecture 4.1 | ε(κ)·ln(κ) approaches a constant | no convergence to a constant over κ=50–500 | **unsupported over the tested range** |
| CCM versus naive Mellin truncation | qualitative comparison | CCM is about 10⁵⁵ more accurate at λ²=13 | **CCM advantage is not recovered by the tested transforms** |

The numerical tables and interpretation are in [paper.tex](paper.tex).

## Requirements

- Rust stable
- Linux, WSL, or macOS for the HP build
- GMP/MPFR build prerequisites

On Ubuntu:

```bash
sudo apt update
sudo apt install -y build-essential m4 libgmp-dev libmpfr-dev libmpc-dev
```

## Build

```bash
cargo build --release --features hp --locked
```

The claim scripts perform the same locked build automatically when `BIN`
is not supplied. A prebuilt binary can be selected with:

```bash
BIN=/path/to/ccm-falsifications bash scripts/claim1a_lambda13_grid4001.sh
```

## Numerical profiles

Every claim defaults to:

```text
--numerical-profile paper
```

The profiles are:

- `paper`: retains the paper's original full-space adaptive-even and legacy
  inverse-iteration route, while using Toolkit v0.13.3's current 64-guard-bit
  precision contract. HP-1000 therefore runs at 3386 bits.
- `current`: uses the optimized even-sector and Auto eigenstate route.

Select the current route without editing a script:

```bash
bash scripts/claim1a_lambda13_grid4001.sh --numerical-profile current
```

The original paper tables were produced with Toolkit v0.12.1's 16 guard bits
(3338 working bits at HP-1000). Modern reruns deliberately use the stronger
v0.13.3 precision contract. Comparisons should therefore be numerical rather
than byte-for-byte.

Parity policy, eigenstate algorithm, and working precision participate in
artifact identity. The two profiles cannot silently reuse or overwrite an
incompatible eigenstate, although mathematically compatible lower-level
artifacts may still be shared.

## Claim scripts

Each table row has an independently runnable script.

### Claim 1: prolate-wave approximation

```bash
bash scripts/claim1a_lambda13_grid4001.sh
bash scripts/claim1b_lambda100_grid4001.sh
bash scripts/claim1c_lambda100_grid8001.sh
bash scripts/claim1d_lambda1000_grid8001.sh
```

Run all four:

```bash
bash scripts/claim1_lemma72_falsification.sh
```

### Claim 2: Śliwiński rate

The standard CCM regime table:

```bash
bash scripts/claim2_standard_ccm.sh
```

The κ=N=λ sweep:

```bash
bash scripts/claim2a_kappa50.sh
bash scripts/claim2b_kappa100.sh
bash scripts/claim2c_kappa150.sh
bash scripts/claim2d_kappa200.sh
bash scripts/claim2e_kappa300.sh
bash scripts/claim2f_kappa400.sh
bash scripts/claim2g_kappa500.sh
```

Run the standard table and complete κ sweep:

```bash
bash scripts/claim2_sliwinski_conjecture.sh
```

### Claim 3: Mellin comparisons

```bash
# λ²=13: naive and ξ-weighted transforms
bash scripts/claim3a_lambda13_weighted.sh

# λ²=100: naive transform table
bash scripts/claim3b_lambda100_naive.sh
```

Run both:

```bash
bash scripts/claim3_mellin_vs_ccm.sh
```

Run every paper claim:

```bash
bash scripts/retest_all_claims.sh
```

## Reference zeros

The runtime uses the toolkit-owned
`xc-zeta/data/zeta_zeros_1000x2500.json` dataset. Its 2,500-digit ordinates
were computed with Arb interval arithmetic, and the leading 1,000 digits were
independently checked against Odlyzko. The dataset identity and SHA-256 digest
are attached to explicitly seeded CCM artifacts.

No paper-local zero file is required.

## Cache infrastructure

Xcelerator Toolkit v0.13.3 automatically manages reusable quadrature,
CCM components, matrices, factorizations, eigenstates, roots, evidence, and
prolate artifacts. An ordinary user can run a claim without cache environment
variables; the toolkit uses the local workstation cache and public fabric on
the standard reuse path.

Authorized research runs may opt into private reads or publication using the
toolkit's standard environment controls. Cache policy is not embedded in the
claim scripts, so the mathematical command remains the same on a workstation
or remote server.

## Architecture

This repository contains the paper-specific command-line harness and claim
scripts. The mathematical implementation comes from the
[Xcelerator Toolkit](https://github.com/TeamXcelerator/xcelerator-toolkit),
pinned by both the v0.13.3 tag and the exact commit in `Cargo.lock`.

Claims 1 and 3 request only the finite CCM source because they do not need a
root-refinement pass. Claim 2 explicitly refines indexed bundled reference
zeros because its statistic compares the kth CCM value to the kth reference
zero.

## References

1. Connes, A., Consani, C., Moscovici (2025). *Zeta Spectral Triples*.
   arXiv:2511.22755.
2. Śliwiński, D. (2026). *Spectral Analysis of the D_log^(λ,N)
   Operators*. arXiv:2601.12133.
3. Andrews, R. Jr. (2026). *Independent Reproduction and Convergence
   Analysis of the CCM Zeta Spectral Triple*.

## License

See [LICENSE](LICENSE). Source-available for verification and study.
Not licensed for modification, redistribution, or commercial use.

## Trademarks

"Team Xcelerator Inc." is a registered trademark of Team Xcelerator Inc.
All other trademarks are the property of their respective owners.
