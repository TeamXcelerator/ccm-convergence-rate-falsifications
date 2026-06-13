// Copyright (c) 2026 Ronnie Andrews, Jr. (Team Xcelerator Inc.(R))
// All rights reserved. See LICENSE file for terms.

//! Integration tests for the ccm-falsifications binary.
//!
//! HP-only by design — Paper B's claims are HP-1000 results, so the
//! tests exercise the same HP code paths the binary actually uses on
//! Vast. The f64 tier is not tested here because it is not used by
//! any published claim.
//!
//! All tests below require `--features hp` at build time. On Windows
//! (where rug doesn't link with MSVC) the suite is empty by design;
//! HP testing happens on Vast where the publication runs also occur.
//!
//! Run with: `cargo test --release --features hp`

#![cfg(feature = "hp")]

use std::path::Path;
use xc_spectral::ccm::CcmParams;

/// Reference zeros file should exist and load as HP decimal strings.
/// Uses the toolkit's HP loader (no f64 round-trip).
#[test]
fn reference_zeros_loadable_hp() {
    let path = Path::new("data/zeta_zeros.json");
    assert!(path.exists(), "data/zeta_zeros.json must exist");
    let strings = xc_zeta::zeros::first_n_strings(path, 200).unwrap();
    assert_eq!(strings.len(), 200);
    // First zero is ~14.1347... — check the leading digits as a string.
    assert!(strings[0].starts_with("14.134725"),
        "first zero should start with 14.134725, got {:?}", &strings[0][..30]);
    // Every entry should be a long decimal (HP-1000d zeros are ~1000 chars).
    let median_len = {
        let mut lens: Vec<usize> = strings.iter().map(|s| s.len()).collect();
        lens.sort();
        lens[lens.len() / 2]
    };
    assert!(median_len > 100,
        "expected HP zeros (>100 chars), got median length {}", median_len);
}

/// HP zeros loader at HP-1000 precision returns HP Floats with the
/// requested precision and a sane first-zero magnitude.
#[test]
fn reference_zeros_load_hp_1000() {
    let path = Path::new("data/zeta_zeros.json");
    // HP-1000 ≈ 3322 bits.
    let prec_bits: u32 = 3322;
    let zeros = xc_zeta::zeros::first_n_hp(path, 5, prec_bits).unwrap();
    assert_eq!(zeros.len(), 5);
    assert_eq!(zeros[0].prec(), prec_bits);
    // First zero is in (14.13, 14.14). Bounds parsed from decimal
    // strings so there's no f64 round-trip — strict HP-everywhere.
    let lo = rug::Float::with_val(
        prec_bits,
        rug::Float::parse("14.13").unwrap(),
    );
    let hi = rug::Float::with_val(
        prec_bits,
        rug::Float::parse("14.14").unwrap(),
    );
    assert!(zeros[0] > lo && zeros[0] < hi,
        "first zero out of expected range");
}

/// CcmParams for the Sliwiński regime (κ = N = λ = 50). Pure metadata
/// construction; no f64 in the math path.
#[test]
fn sliwinski_regime_params() {
    // κ = N = λ = 50 means λ²=2500
    let params = CcmParams::from_lambda_sq_integer(2500, 50);
    assert!((params.lambda_squared() - 2500.0).abs() < 1e-12);
    assert_eq!(params.matrix_size(), 101);
}

/// HP prolate compute_k_lambda should run at small λ without panicking.
/// This mirrors the cmd_prolate_compare HP path that Claim 1 exercises
/// at HP-1000 on Vast.
#[test]
fn prolate_runs_at_small_lambda_hp() {
    let prec_bits: u32 = 256;
    // λ = √13 in HP. Parse from a decimal string so there's no f64
    // round-trip (the literal 3.605551275463989_f64 would discard the
    // tail of √13 — irrelevant at 256 bits but principle matters).
    let lambda = rug::Float::with_val(
        prec_bits,
        rug::Float::parse("3.605551275463989").unwrap(),
    );
    let result = xc_spectral::prolate::hp::compute_k_lambda(
        &lambda, 201, 32, prec_bits,
        xc_numerics::quadrature::CacheMode::Off,
    ).unwrap();
    assert_eq!(result.k_values.len(), 32);
    // h_0 eigenvalue should be positive in HP.
    let zero_hp = rug::Float::with_val(prec_bits, 0);
    assert!(result.eigenvalue_0 > zero_hp,
        "h_0 eigenvalue should be positive");
}
