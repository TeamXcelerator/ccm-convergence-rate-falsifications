//! Source-bound finite CCM experiments; historical manuscript conclusions are reviewed separately.
use anyhow::{ensure, Result};
use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;
use std::path::PathBuf;
#[cfg(feature = "hp")]
mod experiments;
#[cfg(any(feature = "hp", test))]
mod journal;
#[cfg(feature = "hp")]
mod numerics;
#[cfg(feature = "hp")]
mod source;
#[cfg(feature = "hp")]
mod transforms;
const TOOLKIT_REVISION: &str = "5d50b5b862b075a43e7c2c9ccedd29b568a32808";

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum, Serialize)]
#[serde(rename_all = "kebab-case")]
enum NumericalProfile {
    Paper,
    Current,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum, Serialize)]
#[serde(rename_all = "kebab-case")]
enum RootAcquisition {
    Independent,
    Seeded,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum, Serialize)]
#[serde(rename_all = "kebab-case")]
enum ResearchCapture {
    Claim,
    Research,
    Gap,
    Maximum,
    Ultra,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum, Serialize)]
#[serde(rename_all = "kebab-case")]
enum MellinMode {
    Naive,
    Both,
}

#[derive(Clone, Debug, clap::Args, Serialize)]
struct Common {
    /// Optimized even-sector/Auto policy, or historical adaptive-even/legacy solver.
    #[arg(long, global = true, value_enum, default_value = "current")]
    numerical_profile: NumericalProfile,
    /// Reference-free discovery or explicitly reference-seeded refinement.
    #[arg(long, global = true, value_enum, default_value = "independent")]
    root_acquisition: RootAcquisition,
    /// Capture volume; individual scripts default to Ultra.
    #[arg(long, global = true, value_enum, default_value = "claim")]
    research_capture: ResearchCapture,
    #[arg(long, global = true, default_value_t = 8)]
    research_sector_eigenpairs: usize,
    /// Parent of unique run journals; prior runs are never replaced.
    #[arg(long, global = true, default_value = "claim-runs")]
    capture_output: PathBuf,
    #[arg(long, global = true, value_delimiter = ',')]
    capture_prefix_checkpoints: Vec<usize>,
    #[arg(long, global = true)]
    capture_working_precision_bits: Option<u32>,
    #[arg(long, global = true, default_value_t = 4000)]
    capture_grid_resolution: usize,
    #[arg(long, global = true, default_value_t = 1000)]
    capture_profile_steps: usize,
    #[arg(long, global = true, default_value_t = 8193)]
    capture_reduction_max_dimension: usize,
    /// Fail the run when applicable supplemental capture is incomplete.
    #[arg(long, global = true)]
    require_complete_capture: bool,
}

#[derive(Clone, Debug, Subcommand, Serialize)]
#[serde(tag = "experiment", rename_all = "kebab-case")]
enum Command {
    /// Finite Weil/prolate approximation. This does not test CCM Lemma 7.2.
    ProlateCompare {
        #[arg(long, default_value_t = 13)]
        lambda_sq: u64,
        #[arg(long, default_value_t = 120)]
        n_modes: usize,
        #[arg(long, default_value_t = 1000)]
        precision_digits: u32,
        /// Odd number of interior finite-difference points; no silent adjustment.
        #[arg(long, default_value_t = 4001)]
        n_grid: usize,
        #[arg(long, default_value_t = 1024)]
        n_sample: usize,
        /// Optional finite hypothesis C*relative sampled minimax residual <= this value.
        #[arg(long)]
        max_scaled_residual: Option<String>,
    },
    /// Per-index errors, means, maxima, logarithmic diagnostics and accuracy budgets.
    SliwinskiCheck {
        /// Exact integer C=lambda^2 values, paired with n-values.
        #[arg(long, conflicts_with = "lambdas")]
        lambda_squares: Option<String>,
        /// Compatibility input: each decimal must identify an integer squared cutoff.
        #[arg(long, conflicts_with = "lambda_squares")]
        lambdas: Option<String>,
        #[arg(long, default_value = "50,100,150,200")]
        n_values: String,
        #[arg(long, default_value_t = 1000)]
        precision_digits: u32,
        #[arg(long, default_value_t = 0.0)]
        trim_edge_fraction: f64,
        /// Score a bounded prefix rather than all N roots; recorded as a different window.
        #[arg(long)]
        root_count: Option<usize>,
    },
    /// Complex residuals at real-part crossings, with quadrature refinement.
    MellinCompare {
        #[arg(long, default_value_t = 13)]
        lambda_sq: u64,
        #[arg(long, default_value_t = 120)]
        n_modes: usize,
        #[arg(long, default_value_t = 1000)]
        precision_digits: u32,
        #[arg(long, default_value_t = 5.0)]
        t_min: f64,
        #[arg(long, default_value_t = 55.0)]
        t_max: f64,
        #[arg(long, default_value_t = 5000)]
        n_scan: usize,
        #[arg(long, default_value_t = 500)]
        n_quad: usize,
        #[arg(long, default_value = "both", value_enum)]
        mellin_mode: MellinMode,
        /// Crossing localization depth, separate from arithmetic precision.
        #[arg(long, default_value_t = 30)]
        scan_digits: u32,
        /// Normalized residual/refinement gate, not an interval certificate.
        #[arg(long, default_value = "1e-20")]
        residual_tolerance: String,
    },
}
#[derive(Debug, Parser)]
#[command(
    name = "ccm-falsifications",
    version,
    args_override_self = true,
    about = "Source-bound finite CCM convergence experiments"
)]
struct Cli {
    #[command(flatten)]
    common: Common,
    #[command(subcommand)]
    command: Command,
}

#[cfg(any(feature = "hp", test))]
fn parse_integers(value: &str, minimum: u64) -> Result<Vec<u64>> {
    let values = value
        .split(',')
        .map(|s| s.trim().parse::<u64>().map_err(anyhow::Error::from))
        .collect::<Result<Vec<_>>>()?;
    ensure!(
        !values.is_empty() && values.iter().all(|&n| n >= minimum),
        "invalid integer configuration list"
    );
    Ok(values)
}
#[cfg(any(feature = "hp", test))]
fn configurations(
    cutoffs: &Option<String>,
    lambdas: &Option<String>,
    modes: &str,
) -> Result<Vec<(u64, usize)>> {
    let cs = if let Some(values) = lambdas {
        values
            .split(',')
            .map(|s| {
                let x = s.trim().parse::<f64>()?;
                ensure!(
                    x.is_finite() && x > 1.0 && x * x <= (1_u64 << 53) as f64,
                    "invalid lambda"
                );
                let c = (x * x).round();
                ensure!(
                    (x * x - c).abs() <= 16.0 * f64::EPSILON * c,
                    "lambda must identify an integer C; use --lambda-squares"
                );
                Ok(c as u64)
            })
            .collect::<Result<Vec<_>>>()?
    } else {
        parse_integers(cutoffs.as_deref().unwrap_or("2500,10000,22500,40000"), 2)?
    };
    let ns = parse_integers(modes, 2)?;
    ensure!(
        cs.len() == ns.len(),
        "cutoff and N lists must have equal lengths"
    );
    cs.into_iter()
        .zip(ns)
        .map(|(c, n)| {
            ensure!(c >= 2 && n <= 8192, "C must be >=2 and N in 2..8192");
            Ok((c, n as usize))
        })
        .collect()
}
fn main() -> Result<()> {
    // A side-effect-free build handshake lets launchers reject stale binaries
    // even when a release was amended without changing its version number.
    let arguments = std::env::args_os().collect::<Vec<_>>();
    if arguments.len() == 2 && arguments[1] == "--build-info" {
        #[cfg(feature = "hp")]
        let digest = Some(journal::implementation_digest().0);
        #[cfg(not(feature = "hp"))]
        let digest: Option<String> = None;
        println!(
            "{}",
            serde_json::json!({"schema_version":1,"version":env!("CARGO_PKG_VERSION"),"toolkit_revision":TOOLKIT_REVISION,"implementation_digest":digest,"complete_positive_roots":cfg!(feature="arb")})
        );
        return Ok(());
    }
    let cli = Cli::parse();
    validate(&cli)?;
    #[cfg(not(feature = "hp"))]
    {
        let _ = cli;
        anyhow::bail!("numerical experiments require --features hp; build with cargo build --release --features hp --locked");
    }
    #[cfg(feature = "hp")]
    {
        // Fail before cache initialization or matrix work if the build cannot
        // honor ordinary full-range independent spectral requests.
        ensure!(
            cfg!(feature = "arb") || cli.common.root_acquisition != RootAcquisition::Independent
                || !matches!(cli.command, Command::SliwinskiCheck { .. }),
            "independent spectral claims require complete-range discovery; build with cargo build --release --features arb --locked (FLINT development library required)"
        );
        let mut successful = true;
        match &cli.command {
            Command::SliwinskiCheck {
                lambda_squares,
                lambdas,
                n_values,
                precision_digits,
                trim_edge_fraction,
                root_count,
            } => {
                ensure!(
                    trim_edge_fraction.is_finite() && (0.0..1.0).contains(trim_edge_fraction),
                    "trim fraction must be in [0,1)"
                );
                for (c, n) in configurations(lambda_squares, lambdas, n_values)? {
                    let count = root_count.unwrap_or(n);
                    ensure!(
                        count > 0 && count <= n && count <= 1000,
                        "root-count must be in 1..min(N,1000)"
                    );
                    successful &= source::execute(
                        &cli.common,
                        &cli.command,
                        Some((c, n, *precision_digits, count)),
                        |primary, cache| {
                            experiments::spectral(
                                primary.expect("CCM source"),
                                c,
                                n,
                                count,
                                *trim_edge_fraction,
                                &cli.common,
                                cache,
                            )
                        },
                    )?;
                }
            }
            Command::ProlateCompare {
                lambda_sq,
                n_modes,
                precision_digits,
                ..
            } => {
                successful = source::execute(
                    &cli.common,
                    &cli.command,
                    Some((*lambda_sq, *n_modes, *precision_digits, 8.min(*n_modes))),
                    |primary, cache| {
                        experiments::prolate(primary.expect("CCM source"), &cli.command, cache)
                    },
                )?;
            }
            Command::MellinCompare {
                lambda_sq,
                n_modes,
                precision_digits,
                mellin_mode,
                ..
            } => {
                let config = (*mellin_mode == MellinMode::Both).then_some((
                    *lambda_sq,
                    *n_modes,
                    *precision_digits,
                    8.min(*n_modes),
                ));
                successful =
                    source::execute(&cli.common, &cli.command, config, |primary, cache| {
                        transforms::run(primary, &cli.command, cache)
                    })?;
            }
        }
        ensure!(successful,"one or more runs have incomplete/invalid required evidence; measurements and reasons are retained");
        Ok(())
    }
}
fn validate(cli: &Cli) -> Result<()> {
    ensure!(
        cli.common.research_sector_eigenpairs >= 2
            && cli.common.capture_grid_resolution >= 8
            && cli.common.capture_profile_steps >= 2,
        "invalid capture dimensions"
    );
    match &cli.command {
        Command::ProlateCompare {
            lambda_sq,
            n_modes,
            precision_digits,
            n_grid,
            n_sample,
            ..
        } => {
            ensure!(
                *lambda_sq >= 2
                    && (2..=8192).contains(n_modes)
                    && (20..=100_000).contains(precision_digits),
                "invalid C,N or precision"
            );
            ensure!(
                *n_grid >= 17 && *n_grid < u32::MAX as usize && n_grid % 2 == 1 && *n_sample >= 3,
                "require odd n-grid>=17 and n-sample>=3"
            );
        }
        Command::MellinCompare {
            lambda_sq,
            n_modes,
            precision_digits,
            t_min,
            t_max,
            n_scan,
            n_quad,
            scan_digits,
            ..
        } => {
            ensure!(
                *lambda_sq >= 2
                    && (2..=8192).contains(n_modes)
                    && (20..=100_000).contains(precision_digits),
                "invalid C,N or precision"
            );
            ensure!(
                t_min.is_finite() && t_max.is_finite() && t_min < t_max && *t_min >= 0.0,
                "invalid scan range"
            );
            ensure!(
                *n_scan > 0
                    && (8..=100_000).contains(n_quad)
                    && *scan_digits > 0
                    && scan_digits <= precision_digits,
                "invalid scan or quadrature resolution"
            );
        }
        Command::SliwinskiCheck {
            precision_digits,
            trim_edge_fraction,
            ..
        } => {
            ensure!(
                (20..=100_000).contains(precision_digits)
                    && trim_edge_fraction.is_finite()
                    && (0.0..1.0).contains(trim_edge_fraction),
                "invalid precision or trim fraction"
            );
        }
    }
    #[cfg(feature = "hp")]
    match &cli.command {
        Command::ProlateCompare {
            n_modes,
            precision_digits,
            max_scaled_residual,
            ..
        } => {
            source::preflight(&cli.common, *n_modes, *precision_digits)?;
            if let Some(s) = max_scaled_residual {
                ensure!(
                    numerics::decimal(s, 128)? >= 0,
                    "max-scaled-residual must be nonnegative"
                );
            }
        }
        Command::MellinCompare {
            n_modes,
            precision_digits,
            residual_tolerance,
            mellin_mode,
            ..
        } => {
            let t = numerics::decimal(residual_tolerance, 128)?;
            ensure!(
                t > 0 && t < 1,
                "residual tolerance must be between zero and one"
            );
            if *mellin_mode == MellinMode::Both {
                source::preflight(&cli.common, *n_modes, *precision_digits)?;
            }
        }
        Command::SliwinskiCheck {
            lambda_squares,
            lambdas,
            n_values,
            precision_digits,
            root_count,
            ..
        } => {
            for (_, n) in configurations(lambda_squares, lambdas, n_values)? {
                let count = root_count.unwrap_or(n);
                ensure!(
                    count > 0 && count <= n && count <= 1000,
                    "root-count must be in 1..min(N,1000)"
                );
                source::preflight(&cli.common, n, *precision_digits)?;
            }
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalid_lists_are_never_silently_repaired() {
        assert!(configurations(&None, &Some("50,bad,100".into()), "50,100").is_err());
        assert!(configurations(&Some("2500,10000".into()), &None, "50,bad,100").is_err());
        assert!(configurations(&Some("2500,10000".into()), &None, "50").is_err());
        assert!(configurations(&None, &Some("3.7".into()), "50").is_err());
        assert_eq!(
            configurations(&None, &Some("3.605551275463989,10".into()), "120,500").unwrap(),
            vec![(13, 120), (100, 500)]
        );
    }
    #[test]
    fn defaults_use_current_independent_route() {
        let cli = Cli::try_parse_from(["ccm-falsifications", "prolate-compare"]).unwrap();
        assert_eq!(cli.common.numerical_profile, NumericalProfile::Current);
        assert_eq!(cli.common.root_acquisition, RootAcquisition::Independent);
    }
    #[test]
    fn capture_and_historical_route_are_selectable() {
        let cli = Cli::try_parse_from([
            "ccm-falsifications",
            "sliwinski-check",
            "--research-capture",
            "ultra",
            "--root-acquisition",
            "seeded",
            "--numerical-profile",
            "paper",
        ])
        .unwrap();
        assert_eq!(cli.common.research_capture, ResearchCapture::Ultra);
        assert_eq!(cli.common.root_acquisition, RootAcquisition::Seeded);
    }
}
