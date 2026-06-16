# Empirical Falsification of Convergence Rate Predictions for the CCM Zeta Spectral Triple

> Two quantitative predictions about how the Connes–Consani–Moscovici
> zeta spectral triple converges to Riemann zeros are tested at high
> precision (primarily 1000 digits, MPFR/GMP). CCM Lemma 7.2 is
> falsified outright; Śliwiński's Conjecture 4.1 is empirically
> unsupported at HP-1000. The CCM construction is 10⁵⁵ more accurate
> than naive Mellin truncation, confirming its power is algebraic
> rather than analytic.

**Author:** Ronnie Andrews, Jr.  
**ORCID:** [0009-0003-9724-3104](https://orcid.org/0009-0003-9724-3104)  
**Contact:** randrewsmath@gmail.com  
**Date:** June 2026

## Headline Results

| Test | Predicted rate | HP measurement | Verdict |
|---|---|---|---|
| CCM Lemma 7.2 (prolate-wave approx) | rel error × λ² ≈ const | grows by ~1,116,000× from λ²=13 to λ²=1000 | **falsified** |
| Śliwiński Conjecture 4.1 | ε(κ)·ln(κ) → const | oscillates 2.316–3.704 (full) and 1.041–1.853 (trim-20%) over κ=50–500; no convergence to constant | **unsupported at HP-1000** |
| CCM vs naive Mellin truncation | (qualitative) | CCM is 10⁵⁵ more accurate (verified at HP-1000) | **algebraic, not analytic** |

## Key Findings

### Claim 1: CCM Lemma 7.2 is empirically falsified

At HP-1000, the quantity `rel × λ²` (which Lemma 7.2 predicts is bounded
by a constant) grows by ~1,116,000× across λ²∈{13, 100, 1000}. At
λ²=1000 the prolate-wave residual is 99.9% of ξ_λ — the guess has
essentially no correlation with the actual eigenvector.

### Claim 2: Śliwiński Conjecture 4.1 is empirically unsupported

In the κ=N=λ regime at HP-1000 across κ∈{50–500}, ε(κ)·lnκ ranges
from 2.316 to 3.704 (full-spectrum) and 1.041 to 1.853 (trim-20%),
with no convergence to a constant — values oscillate without settling,
and the trimmed metric grows 1.6× from κ=50 to κ=500. Theorem 3.1
(Śliwiński's lower bound) holds at every configuration with margin
4.2× to 14.8×. The aggregate metrics are dominated by high-index
eigenvalues at the matrix's truncation boundary, which contribute O(1)
error regardless of κ; the first eigenvalues converge super-exponentially
(160 digits at κ=50, 608 digits at κ=200, saturating HP-1000 at κ=400).

### Claim 3: CCM is 10⁵⁵ more accurate than naive Mellin truncation

Integral transforms of ξ_λ (naive Mellin and ξ-weighted variants) fall
short of CCM's algebraic accuracy by orders of magnitude. The
construction's power is in the eigenvalue problem + rational-function
zeros, not in any integral transform.

## Reproduction

### Requirements

- Rust toolchain (stable)
- Linux/WSL/macOS
- System libraries: `sudo apt install build-essential m4 libgmp-dev libmpfr-dev libmpc-dev`

### Build

```bash
cargo build --release --features hp
```

### Reproduce all claims

```bash
bash scripts/retest_all_claims.sh
```

Or run individual claims:

```bash
bash scripts/claim1_lemma72_falsification.sh     # Lemma 7.2 (~1 hr)
bash scripts/claim2_sliwinski_conjecture.sh      # Conjecture 4.1 (~1 day)
bash scripts/claim3_mellin_vs_ccm.sh             # Mellin comparison (~1 hr)
```

Claim 2 is split into 7 per-κ sub-scripts for parallel reproduction:

```bash
bash scripts/claim2a_kappa50.sh      # κ=50  (minutes)
bash scripts/claim2b_kappa100.sh     # κ=100 (minutes)
bash scripts/claim2c_kappa150.sh     # κ=150 (~30 min)
bash scripts/claim2d_kappa200.sh     # κ=200 (~1 hr)
bash scripts/claim2e_kappa300.sh     # κ=300 (~2 hr)
bash scripts/claim2f_kappa400.sh     # κ=400 (~4 hr)
bash scripts/claim2g_kappa500.sh     # κ=500 (many hours)
```

## Cache Infrastructure

Caches (GL nodes, τ-matrices, Weil eigenvectors, prolate eigenvalues)
are hosted in dedicated public repositories and fetched automatically
on demand via DynamicFetch — no manual download required:

- [xcelerator-gl-cache](https://github.com/TeamXcelerator/xcelerator-gl-cache)
- [xcelerator-tau-cache](https://github.com/TeamXcelerator/xcelerator-tau-cache)
- [xcelerator-weil-eigvec-cache](https://github.com/TeamXcelerator/xcelerator-weil-eigvec-cache)
- [xcelerator-prolate-eigvals-cache](https://github.com/TeamXcelerator/xcelerator-prolate-eigvals-cache)

All configurations reported in the paper have their cache fixtures in
these repositories. No configuration requires fresh cold compute to
reproduce — the dominant cost on a fresh clone is LU factorization and
Newton refinement on the cached matrices.

## Architecture

This repository contains the paper-specific CLI harness and
reproduction scripts. The core mathematical library is the
[Xcelerator Toolkit](https://github.com/TeamXcelerator/xcelerator-toolkit)
(pinned to a tagged release), pulled automatically by Cargo during
build. No manual cloning of the toolkit is required.

## References

1. Connes, A., Consani, C., Moscovici, H. (2025). *Zeta Spectral
   Triples*. arXiv:2511.22755.
2. Śliwiński, D. (2026). *Spectral Analysis of the D_log^(λ,N)
   Operators*. arXiv:2601.12133.
3. Andrews, R. Jr. (2026). *Independent Reproduction and Convergence
   Analysis of the CCM Zeta Spectral Triple*.
   GitHub: TeamXcelerator/ccm-reproduction-and-convergence.

## Citation

```
Andrews, R. Jr. (2026). Empirical Falsification of Convergence Rate
Predictions for the CCM Zeta Spectral Triple. GitHub:
TeamXcelerator/ccm-convergence-rate-falsifications.
```

## License

See [LICENSE](LICENSE). Source-available for verification and study.
Not licensed for modification, redistribution, or commercial use.

## Trademarks

"Team Xcelerator Inc." is a registered trademark of Team Xcelerator Inc.
All other trademarks are the property of their respective owners.
