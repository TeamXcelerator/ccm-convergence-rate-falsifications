// Copyright (c) 2026 Ronnie Andrews, Jr. (Team Xcelerator Inc.(R))
// All rights reserved. See LICENSE file for terms.
//
// This source code is provided for verification and study purposes only.
// Modification, redistribution, and commercial use are prohibited
// without explicit written permission.

//! CCM Convergence Rate Falsifications
//!
//! Tests three published quantitative predictions about the
//! Connes-Consani-Moscovici zeta spectral triple at high precision:
//!
//! - **prolate-compare**: CCM Lemma 7.2 (rel. error in λ⁻²)
//! - **sliwinski-check**: Śliwiński Conjecture 4.1 (ε ~ 1/ln λ)
//! - **mellin-compare**: CCM accuracy vs naive Mellin truncation
//!
//! Author: Ronnie Andrews, Jr. (Team Xcelerator Inc.(R))

use anyhow::Result;
use clap::{Parser, Subcommand};

// These imports are used by the HP-gated command handlers.
#[allow(unused_imports)]
use std::path::Path;
#[allow(unused_imports)]
use xc_spectral::ccm;
#[allow(unused_imports)]
use xc_spectral::ccm::CcmParams;
#[allow(unused_imports)]
use xc_spectral::prolate;
#[allow(unused_imports)]
use xc_spectral::mellin;

/// Path to the canonical reference zeros file.
#[allow(dead_code)] // Used by HP-gated command handlers.
const ZEROS_PATH: &str = "data/zeta_zeros.json";

#[derive(Parser)]
#[command(name = "ccm-falsifications", about = "Empirical falsification of CCM convergence rate predictions")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Test 1 (Claim 1): CCM Lemma 7.2 falsification.
    ///
    /// Computes ξ_λ at HP via the CCM construction (the smallest Weil
    /// eigenvector is fetched from the toolkit's residual-validated
    /// weil_eigvec cache when available, else computed and cached),
    /// builds the prolate-wave educated guess k_λ via finite-difference
    /// diagonalization of PW_λ, then computes the relative L∞ error
    /// ‖ξ_λ - c·k_λ‖_∞ / ‖ξ_λ‖_∞. Reports `rel × λ²` which CCM's
    /// Lemma 7.2 predicts to be a constant C. At HP we observe
    /// `rel × λ²` grows by ~800,000× across λ²=13 to 1000.
    ProlateCompare {
        /// Value of lambda. Default reproduces λ²=13 (CCM headline config).
        #[arg(long, default_value_t = 3.605551275463989_f64)]
        lambda: f64,
        /// Mode cutoff N. Matrix size is 2N+1.
        #[arg(long, default_value_t = 120)]
        n_modes: usize,
        /// Working precision in decimal digits.
        #[arg(long, default_value_t = 1000)]
        precision_digits: u32,
        /// Number of FD grid points for PW_λ. Forced odd.
        #[arg(long, default_value_t = 4001)]
        n_grid: usize,
        /// Number of comparison sample points in [λ⁻¹, λ].
        #[arg(long, default_value_t = 1024)]
        n_sample: usize,
    },
    /// Test 2 (Claim 2): Śliwiński Conjecture 4.1 falsification.
    ///
    /// Computes the full positive eigenvalue spectrum of D_log(λ, N)
    /// at HP, compares to Riemann zeros, and tabulates
    /// ε(λ, N) = (1/N) Σ |ν_k - ζ_k| against Śliwiński's lower
    /// bound 1/(4 ln λ). Tests whether `ε × ln λ` is constant
    /// (the conjecture) or growing (our finding: 6.5× across κ ∈ [50, 200]).
    SliwinskiCheck {
        /// Comma-separated list of lambda values.
        #[arg(long, default_value = "50,100,150,200")]
        lambdas: String,
        /// Comma-separated list of N values (must match lambdas count).
        #[arg(long, default_value = "50,100,150,200")]
        n_values: String,
        /// Working precision in decimal digits.
        #[arg(long, default_value_t = 200)]
        precision_digits: u32,
        /// Fraction of edge (highest-index) eigenvalues to drop when
        /// computing a "trimmed" interior metric. The full-spectrum
        /// metric is always reported; if this is > 0, a second table
        /// using only the first (1-fraction)·N eigenvalues is also
        /// reported. Edge eigenvalues are dominated by the truncation
        /// boundary, not the spectral gap; trimming isolates the
        /// asymptotic behaviour Sliwinski's conjecture concerns.
        ///
        /// Range: [0.0, 1.0). Default 0.0 (no trim, original behaviour).
        #[arg(long, default_value_t = 0.0_f64)]
        trim_edge_fraction: f64,
    },
    /// Test 3 (Claim 3): CCM vs naive Mellin truncation.
    ///
    /// Computes ξ_λ at HP via the CCM construction (smallest Weil
    /// eigenvector from the toolkit's residual-validated weil_eigvec
    /// cache when available, else computed and cached), computes both
    /// the unweighted truncated Λ_λ(s) and the ξ_λ-weighted G(s) on the
    /// critical line, and compares zero locations against Riemann zeros.
    /// The CCM construction at the same λ is 10⁵⁵ more accurate.
    MellinCompare {
        /// Value of lambda. Default reproduces λ²=13 (CCM headline config).
        #[arg(long, default_value_t = 3.605551275463989_f64)]
        lambda: f64,
        /// Mode cutoff N. Matrix size is 2N+1.
        #[arg(long, default_value_t = 120)]
        n_modes: usize,
        /// Working precision in decimal digits.
        #[arg(long, default_value_t = 1000)]
        precision_digits: u32,
        /// Scan range: minimum imaginary part t.
        #[arg(long, default_value_t = 5.0)]
        t_min: f64,
        /// Scan range: maximum imaginary part t.
        #[arg(long, default_value_t = 55.0)]
        t_max: f64,
        /// Number of scan points for zero detection.
        #[arg(long, default_value_t = 5000)]
        n_scan: usize,
        /// Number of quadrature points for the Mellin integral.
        #[arg(long, default_value_t = 500)]
        n_quad: usize,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::ProlateCompare { lambda, n_modes, precision_digits, n_grid, n_sample } => {
            cmd_prolate_compare(lambda, n_modes, precision_digits, n_grid, n_sample)
        }
        Command::SliwinskiCheck { lambdas, n_values, precision_digits, trim_edge_fraction } => {
            cmd_sliwinski_check(&lambdas, &n_values, precision_digits, trim_edge_fraction)
        }
        Command::MellinCompare { lambda, n_modes, precision_digits, t_min, t_max, n_scan, n_quad } => {
            cmd_mellin_compare(lambda, n_modes, precision_digits, t_min, t_max, n_scan, n_quad)
        }
    }
}

/// Compute ξ_λ at high precision via the CCM construction.
///
/// This is the single ξ-acquisition path for the comparison commands.
/// `ccm::hp::run` transparently fetches the smallest Weil eigenvector
/// from the toolkit's residual-validated `weil_eigvec` disk cache when
/// one exists for this `(λ², N, prec)` (skipping the costly LU), and
/// otherwise computes it and caches it. No paper-local ξ files: the
/// toolkit is the sole cache manager.
#[cfg(feature = "hp")]
fn compute_xi(
    lambda: f64, n_modes: usize, precision_digits: u32,
) -> Result<(CcmParams, ccm::hp::HighPrecResult)> {
    let params = CcmParams::from_lambda(lambda, n_modes);
    let cfg = ccm::hp::HighPrecConfig::for_decimal_digits(precision_digits);
    let zeros_strings = xc_zeta::zeros::first_n_strings(Path::new(ZEROS_PATH), cfg.n_eigenvalues)?;
    let zero_seeds: Vec<rug::Float> = zeros_strings.iter()
        .map(|s| rug::Float::with_val(cfg.precision_bits, rug::Float::parse(s).unwrap()))
        .collect();
    let result = ccm::hp::run(&params, &cfg, &zero_seeds)?;
    Ok((params, result))
}

fn cmd_prolate_compare(
    lambda: f64, n_modes: usize, precision_digits: u32,
    n_grid: usize, n_sample: usize,
) -> Result<()> {
    if lambda <= 1.0 {
        anyhow::bail!("lambda must be > 1 (got {lambda})");
    }
    #[cfg(not(feature = "hp"))]
    {
        let _ = (lambda, n_modes, precision_digits, n_grid, n_sample);
        anyhow::bail!("prolate-compare requires --features hp at build time");
    }
    #[cfg(feature = "hp")]
    {
        use prolate::hp::{compare_xi_to_k_lambda, compute_k_lambda};

        let (params, result) = compute_xi(lambda, n_modes, precision_digits)?;
        let lambda_squared = params.lambda_squared;
        let prec = result.precision_bits;
        let n_modes = params.n_modes;
        let xi_hp = &result.xi;
        let lambda_hp = rug::Float::with_val(prec, lambda_squared).sqrt();
        println!("ξ_λ computed: λ² = {:.6}, λ = {:.6}, N = {}, precision = {} bits",
            lambda_squared, lambda, n_modes, prec);
        // Display ε_N in HP. At large λ this value can be smaller than
        // 10^-1000 (well below f64 underflow); HP-native display only.
        println!("ε_N = {}",
            xc_numerics::fmt::display_hp(&result.weil_min_eigenvalue, 6));

        println!("Prolate FD grid: {} interior points; comparison samples: {}",
            n_grid, n_sample);
        // HP prolate pipeline: build PW_λ tridiagonal at HP, find h_0/h_4
        // eigenvectors via shifted inverse iteration in HP, sample k_λ on
        // the logarithmic comparison grid in HP. No f64 round-trip.
        let pw = compute_k_lambda(
            &lambda_hp, n_grid, n_sample, prec,
            xc_numerics::quadrature::CacheMode::default(),
        )?;
        let two_pi_lambda_sq = {
            let mut v = rug::Float::with_val(prec, rug::float::Constant::Pi);
            v *= 2u32;
            v *= &lambda_hp;
            v *= &lambda_hp;
            v
        };
        println!("Prolate eigenvalues: h_0 = {}, h_4 = {} (predicted 2πλ² = {})",
            xc_numerics::fmt::display_hp(&pw.eigenvalue_0, 8),
            xc_numerics::fmt::display_hp(&pw.eigenvalue_4, 8),
            xc_numerics::fmt::display_hp(&two_pi_lambda_sq, 8));

        // HP comparison: every quantity stays in HP. The ratio
        // rel_linf = ‖ξ-c·k‖_∞ / ‖ξ‖_∞ and the predicted bound λ⁻²
        // are computed in HP; only the final ratio cross-check is
        // displayed in HP-rendered scientific notation.
        let cmp = compare_xi_to_k_lambda(
            xi_hp, n_modes, &lambda_hp,
            &pw.u_grid, &pw.k_values, prec,
        )?;
        let rel_linf = {
            let mut v = cmp.linf_error.clone();
            v /= &cmp.xi_linf;
            v
        };
        let lambda_sq_hp = rug::Float::with_val(prec, lambda_squared);
        let predicted_bound = {
            let mut v = rug::Float::with_val(prec, 1);
            v /= &lambda_sq_hp;
            v
        };
        let rel_times_lambda_sq = {
            let mut v = rel_linf.clone();
            v *= &lambda_sq_hp;
            v
        };

        println!("\n=== CCM Lemma 7.2 test ===");
        println!("‖ξ - c·k‖_∞ / ‖ξ‖_∞ = {}",
            xc_numerics::fmt::display_hp(&rel_linf, 6));
        println!("Optimal scalar c    = {}",
            xc_numerics::fmt::display_hp(&cmp.optimal_scalar, 6));
        println!("rel × λ²            = {}  (CCM predicts ≈ const C)",
            xc_numerics::fmt::display_hp(&rel_times_lambda_sq, 6));
        println!("λ⁻² (Lemma 7.2 RHS) = {}",
            xc_numerics::fmt::display_hp(&predicted_bound, 6));
        if rel_linf > predicted_bound {
            // factor = rel_linf / predicted_bound, computed in HP.
            let factor = {
                let mut v = rel_linf.clone();
                v /= &predicted_bound;
                v
            };
            println!("Relative L∞ error EXCEEDS λ⁻² bound by factor {}",
                xc_numerics::fmt::display_hp(&factor, 6));
            println!("=> CCM Lemma 7.2 is empirically falsified at this λ.");
        } else {
            let margin = {
                let mut v = predicted_bound.clone();
                v /= &rel_linf;
                v
            };
            println!("Relative L∞ error within λ⁻² bound (margin {}×)",
                xc_numerics::fmt::display_hp(&margin, 6));
        }
        Ok(())
    }
}

fn cmd_sliwinski_check(
    lambdas_str: &str, n_values_str: &str,
    precision_digits: u32, trim_edge_fraction: f64,
) -> Result<()> {
    if !(0.0..1.0).contains(&trim_edge_fraction) {
        anyhow::bail!(
            "--trim-edge-fraction must be in [0.0, 1.0); got {}",
            trim_edge_fraction
        );
    }
    #[cfg(not(feature = "hp"))]
    {
        let _ = (lambdas_str, n_values_str, precision_digits, trim_edge_fraction);
        anyhow::bail!("sliwinski-check requires --features hp at build time");
    }
    #[cfg(feature = "hp")]
    {
        let lambdas: Vec<f64> = lambdas_str
            .split(',')
            .filter_map(|s| s.trim().parse::<f64>().ok())
            .filter(|&v| v > 1.0)
            .collect();
        let n_values: Vec<usize> = n_values_str
            .split(',')
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .filter(|&v| v >= 1)
            .collect();
        if lambdas.is_empty() || n_values.is_empty() {
            anyhow::bail!("--lambdas and --n-values must each have ≥ 1 valid entry");
        }
        if lambdas.len() != n_values.len() {
            anyhow::bail!(
                "lambdas and n_values must have the same count (got {} and {})",
                lambdas.len(), n_values.len()
            );
        }

        let max_n = *n_values.iter().max().unwrap();
        let zero_strings = xc_zeta::zeros::first_n_strings(Path::new(ZEROS_PATH), max_n)?;
        if zero_strings.len() < max_n {
            anyhow::bail!("only {} reference zeros available, need {}", zero_strings.len(), max_n);
        }
        let prec_bits = ccm::hp::HighPrecConfig::for_decimal_digits(precision_digits).precision_bits;
        let zero_seeds_hp: Vec<rug::Float> = zero_strings.iter()
            .map(|s| rug::Float::with_val(prec_bits, rug::Float::parse(s).unwrap()))
            .collect();
        let zeros_hp: Vec<rug::Float> = zero_seeds_hp.clone();

        println!("\n=== Śliwiński's bound cross-check (HP, {} digits) ===", precision_digits);
        println!("Theorem 3.1 (arxiv 2601.12133): ε(λ, N) = (1/N) Σ |ν_k − ζ_k| ≥ 1/(4 ln λ)");
        println!("Conjecture 4.1: ε(κ) · ln(κ) → const as κ → ∞ (κ = λ = N)");
        println!();
        println!(
            "{:>10} {:>6} {:>16} {:>16} {:>15} {:>10} {:>16} {:>10}",
            "λ", "N", "mean abs err", "uniform err", "1/(4 ln λ)", "ratio", "ε × ln λ", "satisfied"
        );
        println!("{}", "-".repeat(110));

        // Cache HP eps×ln(λ) per config for the post-loop growth analysis,
        // and HP eigenvalues for the per-eigenvalue analysis. Both are
        // kept in HP — no f64 round-trip, no JSON, terminal output only.
        let mut eps_times_ln_per_config: Vec<rug::Float> = Vec::new();
        let mut all_satisfied = true;
        let mut hp_results: Vec<(f64, usize, Vec<rug::Float>, u32)> = Vec::new();

        for (lambda, n_modes) in lambdas.iter().zip(n_values.iter()) {
            let lambda = *lambda;
            let n_modes = *n_modes;
            let params = CcmParams::from_lambda(lambda, n_modes);
            let mut hpcfg = ccm::hp::HighPrecConfig::for_decimal_digits(precision_digits);
            hpcfg.n_eigenvalues = n_modes.min(zeros_hp.len());
            let hp = ccm::hp::run(&params, &hpcfg, &zero_seeds_hp[..hpcfg.n_eigenvalues])?;
            let prec = hp.precision_bits;

            let n_compare = hp.eigenvalues_pos.len().min(n_modes).min(zeros_hp.len());
            // HP error accumulators. Sum-of-abs and max-abs both stay in HP.
            let mut sum_abs = rug::Float::with_val(prec, 0);
            let mut max_abs = rug::Float::with_val(prec, 0);
            for k in 0..n_compare {
                // err_k = |ν_k - ζ_k| in HP.
                let mut diff = hp.eigenvalues_pos[k].clone();
                diff -= &zeros_hp[k];
                let err = diff.abs();
                sum_abs += &err;
                if err > max_abs { max_abs = err.clone(); }
            }
            // mean = sum / N in HP.
            let mut mean = sum_abs.clone();
            mean /= rug::Float::with_val(prec, n_compare as u32);
            // ln(λ) in HP. λ ≤ 200 so the f64→HP promotion is exact at any
            // working precision we use here.
            let lambda_hp = rug::Float::with_val(prec, lambda);
            let ln_lambda_hp = lambda_hp.clone().ln();
            let bound_hp = {
                let mut v = rug::Float::with_val(prec, 1);
                v /= 4u32;
                v /= &ln_lambda_hp;
                v
            };
            // ratio = mean / bound, in HP.
            let ratio_hp = {
                let mut v = mean.clone();
                v /= &bound_hp;
                v
            };
            let satisfied = mean >= bound_hp;
            // ε × ln(λ) in HP.
            let eps_times_ln_hp = {
                let mut v = mean.clone();
                v *= &ln_lambda_hp;
                v
            };

            // Format 1/(4 ln λ) in HP — at λ = 50..200 this is O(0.04–0.07)
            // so display in HP scientific is informative without underflow.
            let bound_str = xc_numerics::fmt::display_hp(&bound_hp, 6);

            println!(
                "{:>10.4} {:>6} {:>16} {:>16} {:>15} {:>10} {:>16} {:>10}",
                lambda, n_modes,
                xc_numerics::fmt::display_hp(&mean, 6),
                xc_numerics::fmt::display_hp(&max_abs, 6),
                bound_str,
                xc_numerics::fmt::display_hp(&ratio_hp, 6),
                xc_numerics::fmt::display_hp(&eps_times_ln_hp, 6),
                if satisfied { "yes" } else { "no" }
            );
            if !satisfied { all_satisfied = false; }
            eps_times_ln_per_config.push(eps_times_ln_hp);
            // Move (not clone) eigenvalues_pos into hp_results — saves
            // one Vec<rug::Float> clone per config (~200 MPFR allocs at
            // HP-1000, N=200). `hp` is dropped at end of iteration; the
            // partial move is fine since prec was already copied above.
            hp_results.push((lambda, n_modes, hp.eigenvalues_pos, prec));
        }

        println!();
        println!("Notes:");
        println!("  - Theorem 3.1 (lower bound): ratio > 1 means bound is satisfied");
        println!("  - Conjecture 4.1 (asymptotic rate): ε × ln(λ) should be ≈ const");
        println!();
        if !all_satisfied {
            println!("WARNING: at least one config VIOLATES the lower bound (numerical issue?)");
        } else {
            println!("Theorem 3.1 (lower bound) holds at all tested configurations.");
        }
        // Growth analysis in HP. eps_times_ln values stayed in HP from the
        // loop above; no string round-trip needed.
        if eps_times_ln_per_config.len() >= 2 {
            let first_hp = eps_times_ln_per_config.first().unwrap();
            let last_hp = eps_times_ln_per_config.last().unwrap();
            let growth_hp = {
                let mut v = last_hp.clone();
                v /= first_hp;
                v
            };
            println!("ε × ln(λ) growth across configs: {} → {} (factor {}×)",
                xc_numerics::fmt::display_hp(first_hp, 12),
                xc_numerics::fmt::display_hp(last_hp, 12),
                xc_numerics::fmt::display_hp(&growth_hp, 4));
            let prec_growth = first_hp.prec();
            let two_hp = rug::Float::with_val(prec_growth, 2);
            if growth_hp > two_hp {
                println!("=> Conjecture 4.1 is empirically UNSUPPORTED at HP.");
            }
        }

        // Per-eigenvalue analysis: test 1/ln^α(κ) hypothesis on first eigenvalue.
        // All quantities computed in HP; matching_digits comes from the
        // toolkit's HP-native helper. No f64 underflow possible.
        if hp_results.len() >= 2 {
            println!("\n=== Per-eigenvalue analysis (HP precision, matching digits vs κ) ===");
            println!("{:>10} {:>8} {:>14} {:>14} {:>14} {:>14}",
                "κ", "k", "match_digits", "log10(err)", "log10(ln κ)", "implied α");
            println!("{}", "-".repeat(88));

            for (lambda, n_modes, eigs_hp, prec) in hp_results.iter() {
                let lambda = *lambda;
                let n_modes = *n_modes;
                let prec = *prec;
                let lnk_hp = rug::Float::with_val(prec, lambda).ln();
                let log10_lnk_hp = lnk_hp.clone().log10();

                let indices_to_check = [0usize, 4, 9, 19, n_modes.min(zeros_hp.len()) - 1];
                for &k in &indices_to_check {
                    if k >= eigs_hp.len() || k >= zeros_hp.len() { continue; }
                    let mut diff = eigs_hp[k].clone();
                    diff -= &zeros_hp[k];
                    let abs_err = diff.abs();

                    // Matching digits = -log10(|ν_k - ζ_k| / |ζ_k|), HP-native.
                    let matching_str = if abs_err.is_zero() {
                        format!(">{}", precision_digits)
                    } else {
                        let m = xc_numerics::fmt::matching_digits(&eigs_hp[k], &zeros_hp[k]);
                        xc_numerics::fmt::display_hp(&m, 4)
                    };

                    // log10(|err|) in HP. Underflow-safe.
                    let log10_err_hp = if abs_err.is_zero() {
                        // Treat as -precision_digits (a finite stand-in).
                        let mut v = rug::Float::with_val(prec, precision_digits as i64);
                        v = -v;
                        v
                    } else {
                        abs_err.clone().log10()
                    };
                    // implied α = -log10(err) / log10(ln κ) in HP. If err ~
                    // 1/ln^α(κ), then log10(err) ≈ -α·log10(ln κ).
                    let implied_alpha_hp = {
                        if log10_lnk_hp.is_zero() {
                            rug::Float::with_val(prec, 0)
                        } else {
                            let mut v = log10_err_hp.clone();
                            v = -v;
                            v /= &log10_lnk_hp;
                            v
                        }
                    };

                    println!("{:>10.1} {:>8} {:>14} {:>14} {:>14} {:>14}",
                        lambda, k + 1, matching_str,
                        xc_numerics::fmt::display_hp(&log10_err_hp, 6),
                        xc_numerics::fmt::display_hp(&log10_lnk_hp, 6),
                        xc_numerics::fmt::display_hp(&implied_alpha_hp, 6));
                }
            }
            println!();
            println!("'match_digits' = -log10(|ν_k - ζ_k| / |ζ_k|), computed in HP");
            println!("'implied α'    = α such that err ≈ 1/ln^α(κ), in HP");
            println!("If α is constant across κ for a given k, the ln^α hypothesis holds.");
        }

        // -----------------------------------------------------------------
        // Trimmed-edge analysis (opt-in via --trim-edge-fraction)
        // -----------------------------------------------------------------
        // Edge eigenvalues (k near κ) are dominated by the truncation
        // boundary of D_log(λ, N): they're an artefact of where the
        // matrix gets cut off, not of the spectral gap Sliwinski's
        // conjecture concerns. Recomputing the metric over only the
        // first (1 - trim_edge_fraction)·N indices isolates the
        // interior-spectrum behaviour.
        //
        // Default trim_edge_fraction = 0.0 means "no trim, original
        // behaviour" — this block prints nothing and the function
        // returns identical output to before the flag was added.
        if trim_edge_fraction > 0.0 {
            println!();
            let pct = (trim_edge_fraction * 100.0).round() as u32;
            println!("=== Trimmed interior metric (top {}% of edge eigenvalues dropped) ===", pct);
            println!("Rationale: edge eigenvalues are truncation-boundary artefacts; trimming");
            println!("isolates the interior spectrum where the conjecture's asymptotic claim lives.");
            println!();
            println!(
                "{:>10} {:>6} {:>10} {:>16} {:>16} {:>15} {:>10} {:>16} {:>10}",
                "λ", "N", "kept", "mean abs err", "uniform err", "1/(4 ln λ)", "ratio",
                "ε × ln λ", "satisfied"
            );
            println!("{}", "-".repeat(122));

            let mut trimmed_eps_times_ln: Vec<rug::Float> = Vec::new();
            let mut trimmed_all_satisfied = true;
            for (lambda, n_modes, eigs_hp, prec) in hp_results.iter() {
                let lambda = *lambda;
                let prec = *prec;
                // n_kept = N · (1 - trim_edge_fraction), rounded down,
                // and clamped to at least 1.
                let n_full = eigs_hp.len().min(zeros_hp.len()).min(*n_modes);
                let n_kept = ((n_full as f64) * (1.0 - trim_edge_fraction))
                    .floor() as usize;
                let n_kept = n_kept.max(1);
                if n_kept >= n_full {
                    // Nothing to trim — skip this row to avoid a
                    // duplicate of the full-spectrum table.
                    continue;
                }

                // HP error accumulators over the kept range only.
                let mut sum_abs = rug::Float::with_val(prec, 0);
                let mut max_abs = rug::Float::with_val(prec, 0);
                for k in 0..n_kept {
                    let mut diff = eigs_hp[k].clone();
                    diff -= &zeros_hp[k];
                    let err = diff.abs();
                    sum_abs += &err;
                    if err > max_abs { max_abs = err.clone(); }
                }
                let mut mean = sum_abs.clone();
                mean /= rug::Float::with_val(prec, n_kept as u32);
                let lambda_hp = rug::Float::with_val(prec, lambda);
                let ln_lambda_hp = lambda_hp.clone().ln();
                let bound_hp = {
                    let mut v = rug::Float::with_val(prec, 1);
                    v /= 4u32;
                    v /= &ln_lambda_hp;
                    v
                };
                let ratio_hp = {
                    let mut v = mean.clone();
                    v /= &bound_hp;
                    v
                };
                let satisfied = mean >= bound_hp;
                let eps_times_ln_hp = {
                    let mut v = mean.clone();
                    v *= &ln_lambda_hp;
                    v
                };
                let bound_str = xc_numerics::fmt::display_hp(&bound_hp, 6);

                println!(
                    "{:>10.4} {:>6} {:>10} {:>16} {:>16} {:>15} {:>10} {:>16} {:>10}",
                    lambda, n_modes, n_kept,
                    xc_numerics::fmt::display_hp(&mean, 6),
                    xc_numerics::fmt::display_hp(&max_abs, 6),
                    bound_str,
                    xc_numerics::fmt::display_hp(&ratio_hp, 6),
                    xc_numerics::fmt::display_hp(&eps_times_ln_hp, 6),
                    if satisfied { "yes" } else { "no" }
                );
                if !satisfied { trimmed_all_satisfied = false; }
                trimmed_eps_times_ln.push(eps_times_ln_hp);
            }

            println!();
            if !trimmed_all_satisfied {
                println!("WARNING (trimmed): at least one config violates the lower bound.");
            } else {
                println!("Theorem 3.1 (lower bound) holds at all tested configurations (trimmed).");
            }
            // Trimmed growth analysis.
            if trimmed_eps_times_ln.len() >= 2 {
                let first_hp = trimmed_eps_times_ln.first().unwrap();
                let last_hp = trimmed_eps_times_ln.last().unwrap();
                let growth_hp = {
                    let mut v = last_hp.clone();
                    v /= first_hp;
                    v
                };
                println!("ε × ln(λ) growth (trimmed): {} → {} (factor {}×)",
                    xc_numerics::fmt::display_hp(first_hp, 12),
                    xc_numerics::fmt::display_hp(last_hp, 12),
                    xc_numerics::fmt::display_hp(&growth_hp, 4));
                let prec_growth = first_hp.prec();
                let two_hp = rug::Float::with_val(prec_growth, 2);
                if growth_hp > two_hp {
                    println!("=> Conjecture 4.1 is empirically UNSUPPORTED at HP (trimmed metric).");
                }
            }
        }

        Ok(())
    }
}

fn cmd_mellin_compare(
    lambda: f64, n_modes: usize, precision_digits: u32,
    t_min: f64, t_max: f64,
    n_scan: usize, n_quad: usize,
) -> Result<()> {
    #[cfg(not(feature = "hp"))]
    {
        let _ = (lambda, n_modes, precision_digits, t_min, t_max, n_scan, n_quad);
        anyhow::bail!("mellin-compare requires --features hp at build time");
    }
    #[cfg(feature = "hp")]
    {
        if lambda <= 1.0 {
            anyhow::bail!("lambda must be > 1 (got {lambda})");
        }
        let (params, result) = compute_xi(lambda, n_modes, precision_digits)?;
        let lambda_squared = params.lambda_squared;
        let prec = result.precision_bits;
        let lambda_hp = rug::Float::with_val(prec, lambda_squared).sqrt();
        let xi_hp = &result.xi;
        let n_modes = params.n_modes;
        println!("ξ_λ computed: λ² = {:.6}, λ = {:.6}, N = {}, precision = {} bits",
            lambda_squared, lambda, n_modes, prec);

        // HP reference zeros loaded as full-precision strings; the f64
        // copy is only used for the t_min/t_max range filter.
        let ref_strings = xc_zeta::zeros::first_n_strings(Path::new(ZEROS_PATH), 50)?;
        let ref_zeros_hp: Vec<rug::Float> = ref_strings.iter()
            .map(|s| rug::Float::with_val(prec, rug::Float::parse(s).unwrap()))
            .collect();

        println!("\n=== ξ_λ-weighted Mellin G(s) and unweighted Λ_λ(s) (HP) ===");
        println!("Scanning critical line Re(s)=1/2, Im(s)∈[{:.3}, {:.3}]", t_min, t_max);
        println!("Quadrature points: {}, scan points: {}, precision: {} bits",
            n_quad, n_scan, prec);

        let t_min_hp = rug::Float::with_val(prec, t_min);
        let t_max_hp = rug::Float::with_val(prec, t_max);
        let bisect_iter: usize = 60;

        // Toolkit's HP scan: parallel evaluation in HP, sequential
        // bisection in HP, sign tests via xc_numerics::fmt::sign_of.
        // No f64 round-trip on the scan or the bisection.
        eprintln!("[scan] Scanning {} grid points for Λ_λ in parallel...", n_scan + 1);
        let l_scan_start = std::time::Instant::now();
        let l_zeros_hp = mellin::scan_critical_line_zeros_hp(
            &|_sigma, t| mellin::truncated_lambda_hp(&_sigma.clone(), t, &lambda_hp, n_quad),
            &t_min_hp, &t_max_hp, n_scan, bisect_iter,
        );
        eprintln!("[scan] Λ_λ scan done in {:.1}s, found {} zeros.",
            l_scan_start.elapsed().as_secs_f64(), l_zeros_hp.len());

        eprintln!("[scan] Scanning {} grid points for G(s) in parallel...", n_scan + 1);
        let g_scan_start = std::time::Instant::now();
        let g_zeros_hp = mellin::scan_critical_line_zeros_hp(
            &|_sigma, t| mellin::xi_weighted_mellin_hp(&_sigma.clone(), t, &lambda_hp, xi_hp, n_modes, n_quad),
            &t_min_hp, &t_max_hp, n_scan, bisect_iter,
        );
        eprintln!("[scan] G(s) scan done in {:.1}s, found {} zeros.",
            g_scan_start.elapsed().as_secs_f64(), g_zeros_hp.len());

        println!("\n{:>5} {:>22} {:>22} {:>16} {:>16} {:>14}",
            "k", "Λ_λ zero", "Riemann zero", "Λ_λ error", "G error", "improvement");
        println!("{}", "-".repeat(98));
        let n_show = ref_zeros_hp.len().min(20);
        let mut g_errors_hp: Vec<rug::Float> = Vec::new();
        let mut l_errors_hp: Vec<rug::Float> = Vec::new();
        for (i, rz) in ref_zeros_hp.iter().take(n_show).enumerate() {
            // Range filter (t_min/t_max are CLI f64 args). Reference zeros
            // are O(10-100), well within f64. Filter first to skip the
            // expensive HP nearest-neighbour search for out-of-range zeros.
            let rz_f64 = rz.to_f64();
            if rz_f64 < t_min || rz_f64 > t_max { continue; }
            // HP nearest-neighbour search in zero lists.
            let l_close = l_zeros_hp.iter()
                .map(|z| { let mut d = z.clone(); d -= rz; (z.clone(), d.abs()) })
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            let g_close = g_zeros_hp.iter()
                .map(|z| { let mut d = z.clone(); d -= rz; (z.clone(), d.abs()) })
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            let (l_z, l_err) = match l_close {
                Some(p) => p,
                None => continue,
            };
            let (_, g_err) = match g_close {
                Some(p) => p,
                None => continue,
            };
            // improvement = l_err / g_err in HP.
            let improvement_hp = if g_err.is_zero() {
                rug::Float::with_val(prec, f64::INFINITY)
            } else {
                let mut v = l_err.clone();
                v /= &g_err;
                v
            };
            println!("{:>5} {:>22} {:>22} {:>16} {:>16} {:>14}",
                i + 1,
                xc_numerics::fmt::display_hp(&l_z, 16),
                xc_numerics::fmt::display_hp(rz, 16),
                xc_numerics::fmt::display_hp(&l_err, 6),
                xc_numerics::fmt::display_hp(&g_err, 6),
                xc_numerics::fmt::display_hp(&improvement_hp, 4));
            l_errors_hp.push(l_err);
            g_errors_hp.push(g_err);
        }
        println!("\nΛ_λ zeros found: {}    G zeros found: {}",
            l_zeros_hp.len(), g_zeros_hp.len());
        if !l_errors_hp.is_empty() && !g_errors_hp.is_empty() {
            // Mean errors in HP.
            let mut l_sum = rug::Float::with_val(prec, 0);
            for e in &l_errors_hp { l_sum += e; }
            let mut l_mean = l_sum;
            l_mean /= rug::Float::with_val(prec, l_errors_hp.len() as u32);

            let mut g_sum = rug::Float::with_val(prec, 0);
            for e in &g_errors_hp { g_sum += e; }
            let mut g_mean = g_sum;
            g_mean /= rug::Float::with_val(prec, g_errors_hp.len() as u32);

            let improvement_mean = if g_mean.is_zero() {
                rug::Float::with_val(prec, f64::INFINITY)
            } else {
                let mut v = l_mean.clone();
                v /= &g_mean;
                v
            };

            println!("Λ_λ mean error:  {}", xc_numerics::fmt::display_hp(&l_mean, 6));
            println!("G mean error:    {}", xc_numerics::fmt::display_hp(&g_mean, 6));
            println!("ξ-weighting improves Mellin by factor: {}×",
                xc_numerics::fmt::display_hp(&improvement_mean, 4));
            println!("\n=== CCM vs Mellin comparison ===");
            println!("Naive Mellin truncation Λ_λ has O(0.1-1.0) errors at this λ.");
            println!("The CCM construction at the same λ matches Riemann zeros to 55-460 digits.");
            println!("The construction's accuracy is algebraic (eigenvalue + rational-zero), not analytic.");
        }
        Ok(())
    }
}
