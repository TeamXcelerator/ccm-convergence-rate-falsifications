// Copyright (c) 2026 Ronnie Andrews, Jr. (Team Xcelerator Inc.(R))
// All rights reserved. See LICENSE file for terms.

//! Integration tests for the ccm-falsifications binary.
//!
//! Lightweight structural tests run on every platform. HP numerical smoke
//! tests are enabled with `--features hp` and run on Linux/WSL.

use xc_spectral::ccm::CcmParams;

/// The toolkit-owned reference table replaces paper-local zero fixtures.
#[test]
fn bundled_reference_zeros_are_available() {
    let strings = xc_zeta::zeros::bundled_first_n_strings(200).unwrap();
    assert_eq!(strings.len(), 200);
    assert!(
        strings[0].starts_with("14.134725"),
        "first zero should start with 14.134725, got {:?}",
        &strings[0][..30]
    );
    let median_len = {
        let mut lens: Vec<usize> = strings.iter().map(|s| s.len()).collect();
        lens.sort();
        lens[lens.len() / 2]
    };
    assert!(
        median_len > 2_000,
        "expected 2,500-digit bundled zeros, got median length {}",
        median_len
    );
    let identity = xc_zeta::zeros::bundled_dataset_identity().unwrap();
    assert!(identity.validate());
    assert_eq!(identity.record_count, 1_000);
    assert_eq!(identity.decimal_digits, 2_500);
}

/// The independently runnable wrappers cover every paper table row.
#[test]
fn claim_script_inventory_is_complete() {
    for relative in [
        "scripts/claim1a_lambda13_grid4001.sh",
        "scripts/claim1b_lambda100_grid4001.sh",
        "scripts/claim1c_lambda100_grid8001.sh",
        "scripts/claim1d_lambda1000_grid8001.sh",
        "scripts/claim2_standard_ccm.sh",
        "scripts/claim2a_kappa50.sh",
        "scripts/claim2g_kappa500.sh",
        "scripts/claim3a_lambda13_weighted.sh",
        "scripts/claim3b_lambda100_naive.sh",
    ] {
        assert!(
            std::path::Path::new(relative).is_file(),
            "missing {relative}"
        );
    }
}

/// HP parsing of bundled strings retains the requested MPFR precision.
#[cfg(feature = "hp")]
#[test]
fn bundled_reference_zeros_load_hp_1000() {
    let prec_bits: u32 = 3386;
    let strings = xc_zeta::zeros::bundled_first_n_strings(5).unwrap();
    let zeros = strings
        .iter()
        .map(|value| {
            rug::Float::with_val(
                prec_bits,
                rug::Float::parse(value).expect("bundled zero parses"),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(zeros.len(), 5);
    assert_eq!(zeros[0].prec(), prec_bits);
    let lo = rug::Float::with_val(prec_bits, rug::Float::parse("14.13").unwrap());
    let hi = rug::Float::with_val(prec_bits, rug::Float::parse("14.14").unwrap());
    assert!(
        zeros[0] > lo && zeros[0] < hi,
        "first zero out of expected range"
    );
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
#[cfg(feature = "hp")]
#[test]
fn prolate_runs_at_small_lambda_hp() {
    let prec_bits: u32 = 256;
    // λ = √13 in HP. Parse from a decimal string so there's no f64
    // round-trip (the literal 3.605551275463989_f64 would discard the
    // tail of √13 — irrelevant at 256 bits but principle matters).
    let lambda = rug::Float::with_val(prec_bits, rug::Float::parse("3.605551275463989").unwrap());
    let result = xc_spectral::prolate::hp::compute_k_lambda(
        &lambda,
        201,
        32,
        prec_bits,
        xc_numerics::quadrature::CacheMode::Off,
    )
    .unwrap();
    assert_eq!(result.k_values.len(), 32);
    // h_0 eigenvalue should be positive in HP.
    let zero_hp = rug::Float::with_val(prec_bits, 0);
    assert!(
        result.eigenvalue_0 > zero_hp,
        "h_0 eigenvalue should be positive"
    );
}
