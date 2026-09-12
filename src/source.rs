use crate::{
    journal::{Check, Journal, Measurement, Status},
    Command, Common, NumericalProfile, ResearchCapture, RootAcquisition,
};
use anyhow::{ensure, Context, Result};
use serde_json::json;
use std::{collections::BTreeMap, time::Instant};
use xc_cache::{
    ArtifactCacheContext, CaptureFailure, CapturedDiagnostic, ManagedArtifactCacheSession,
};
use xc_spectral::ccm::{
    self,
    capture::{CcmCaptureLevel, CcmCapturePlan, RetainedReductionRequest},
    hp::{capture_run::RetainedCcmRun, HighPrecConfig, HighPrecResult, PortableHighPrecResult},
    CcmParams,
};

pub fn config(digits: u32, profile: NumericalProfile, count: usize) -> HighPrecConfig {
    let mut cfg = HighPrecConfig::for_decimal_digits(digits);
    cfg.n_eigenvalues = count;
    match profile {
        NumericalProfile::Paper => {
            cfg.set_parity_policy(ccm::hp::CcmParityPolicy::AdaptiveEven);
            cfg.eigenstate_solver = ccm::hp::CcmEigenstateSolver::LegacyInverseIteration;
        }
        NumericalProfile::Current => {
            cfg.set_parity_policy(ccm::hp::CcmParityPolicy::EvenSector);
            cfg.eigenstate_solver = ccm::hp::CcmEigenstateSolver::Auto;
        }
    }
    cfg
}

pub fn execute<F>(
    args: &Common,
    command: &Command,
    tuple: Option<(u64, usize, u32, usize)>,
    measure: F,
) -> Result<bool>
where
    F: FnOnce(Option<&HighPrecResult>, &ArtifactCacheContext<'_>) -> Result<Measurement>,
{
    let timer = Instant::now();
    let journal = Journal::start(
        &args.capture_output,
        &json!({"schema_version":1,"command":command,"options":args,"resolved_C_N_P_root_count":tuple,"source_policy":"reuse compatible identities; preserve historical results","reference_use":args.root_acquisition,"worker_count":rayon::current_num_threads()}),
    )?;
    let mut checks = Vec::new();
    let result = (|| -> Result<bool> {
        let managed =
            ManagedArtifactCacheSession::from_environment()?.context("managed cache required")?;
        let cache = managed.context();
        let mut run = None;
        let mut resolved = None;
        if let Some((c, n, p, count)) = tuple {
            ensure!(
                c >= 2 && (2..=8192).contains(&n) && (20..=100_000).contains(&p),
                "require C>=2, 2<=N<=8192 and 20<=precision-digits<=100000"
            );
            let params = CcmParams::from_lambda_sq_integer(c, n);
            let cfg = config(p, args.numerical_profile, count);
            let value = match args.root_acquisition {
                RootAcquisition::Independent => RetainedCcmRun::independent(
                    &params,
                    &cfg,
                    &ccm::window::ZeroTarget::FirstK { count },
                    ccm::hp::IndependentRootDiscoveryOptions::advanced(false, true),
                    &cache,
                )?,
                RootAcquisition::Seeded => {
                    let dataset = xc_zeta::zeros::bundled_dataset_identity()?;
                    let seeds = xc_zeta::zeros::bundled_first_n_strings(count)?
                        .iter()
                        .map(|s| crate::numerics::decimal(s, cfg.precision_bits))
                        .collect::<Result<Vec<_>>>()?;
                    RetainedCcmRun::seeded(&params, &cfg, 1, &seeds, &dataset, &cache)?
                }
            };
            journal.save(
                "primary.json",
                &PortableHighPrecResult::from_runtime(value.primary())?,
            )?;
            journal.save("primary-sources.json", &value.primary_sources())?;
            run = Some(value);
            resolved = Some((params, cfg));
        }
        // Measurements are durable before any supplemental capture or publication.
        let mut measurement = measure(run.as_ref().map(|r| r.primary()), &cache)?;
        if let Some(run) = &run {
            measurement.source_manifests.extend(run.primary_sources());
        }
        journal.save("measurements.json", &measurement)?;
        // Existing generic research artifacts retain Paper 2's complete data
        // without inventing unregistered artifact types or requiring log parsing.
        let receipt = xc_cache::capture_and_persist(
            &json!({"semantics":"paper2-finite-measurements-v1","implementation_digest":crate::journal::implementation_digest(),"source_identities":measurement.source_manifests.iter().map(|m|json!({"key":m.key,"content_digest":m.content_digest})).collect::<Vec<_>>(),"command":command,"resolved_C_N_P_root_count":tuple,"numerical_profile":args.numerical_profile,"root_acquisition":args.root_acquisition,"toolkit_revision":crate::TOOLKIT_REVISION}),
            vec!["paper2_measurements".into()],
            |_| {
                CapturedDiagnostic::new(&measurement, measurement.source_manifests.clone())
                    .map_err(CaptureFailure::failed)
            },
            &cache,
        )?;
        journal.save("measurement-receipt.json", &receipt.value)?;
        ensure!(
            receipt.value.receipt.is_complete(),
            "measurement artifact retention incomplete; local measurements preserved"
        );
        let valid = measurement.valid();
        checks.extend(measurement.checks);
        let capture_complete = if let (Some(run), Some((params, cfg))) = (&mut run, &resolved) {
            match capture(run, params, cfg, args, &cache, &journal, &mut checks) {
                Ok(complete) => complete,
                Err(error) => {
                    checks.push(Check::new(
                        "capture persistence",
                        "capture",
                        Status::Incomplete,
                        format!("{error:#}"),
                    ));
                    false
                }
            }
        } else {
            checks.push(Check::new(
                "CCM research capture",
                "capture",
                Status::Unsupported,
                "the naive transform has no CCM source; all transform measurements are retained",
            ));
            true
        };
        // A publication failure is recorded after measurements and receipts are saved.
        match managed.finalize_publication_inventory() {
            Ok(_) => checks.push(Check::new(
                "artifact finalization",
                "publication",
                Status::Pass,
                "managed artifact inventory finalized under configured publication policy",
            )),
            Err(error) => {
                checks.push(Check::new(
                    "artifact finalization",
                    "publication",
                    Status::Incomplete,
                    error.to_string(),
                ));
                return Ok(false);
            }
        }
        Ok(valid && (!args.require_complete_capture || capture_complete))
    })();
    let successful = match result {
        Ok(value) => value,
        Err(error) => {
            journal.save("failure.json",&json!({"error":format!("{error:#}"),"primary_saved":journal.directory.join("primary.json").exists(),"measurements_saved":journal.directory.join("measurements.json").exists()}))?;
            checks.push(Check::new(
                "required computation",
                "validation",
                Status::Incomplete,
                format!("{error:#}"),
            ));
            false
        }
    };
    let capture_status = if checks
        .iter()
        .any(|c| c.category == "capture" && matches!(c.status, Status::Incomplete | Status::Fail))
    {
        "INCOMPLETE"
    } else {
        "PASS"
    };
    let summary = json!({"schema_version":1,"run_status":if successful {"PASS"} else {"INCOMPLETE"},"capture_status":capture_status,"checks":checks,"elapsed_seconds":timer.elapsed().as_secs_f64(),"measurements_saved":journal.directory.join("measurements.json").exists(),"scope":"run integrity and finite hypothesis outcomes are separate; no asymptotic theorem is inferred"});
    journal.save("summary.json", &summary)?;
    for check in &checks {
        println!(
            "[{}] {} / {}: {}",
            check.status.label(),
            check.category,
            check.name,
            check.reason
        );
    }
    println!(
        "Run integrity: {} ({:.3}s)",
        if successful { "PASS" } else { "INCOMPLETE" },
        timer.elapsed().as_secs_f64()
    );
    println!(
        "Summary: {}",
        journal.directory.join("summary.json").display()
    );
    println!("Applicable capture completeness: {capture_status}");
    Ok(successful)
}

fn exclusions(
    params: &CcmParams,
    cfg: &HighPrecConfig,
    plan: &CcmCapturePlan,
) -> BTreeMap<String, String> {
    let mut skipped: BTreeMap<String, String> = BTreeMap::new();
    if cfg.effective_parity_policy() != ccm::hp::CcmParityPolicy::EvenSector {
        for id in ["prime_power_response", "u_flow_response"] {
            skipped.insert(
                id.into(),
                "requires an isolated even-sector primary; historical parity policy is preserved"
                    .into(),
            );
        }
        for n in &plan.prefix_checkpoint_dimensions {
            skipped.insert(format!("prefix_checkpoint_{n}"),"checkpoint eigenstate export requires an exact even-sector source; no projection or replacement is substituted".into());
        }
    }
    if params.lambda_sq_int() == 1000 && params.n_modes == 800 && cfg.precision_bits == 3386 {
        for id in [
            "prime_power_response",
            "u_flow_response",
            "prefix_ladder",
            "prefix_checkpoint_801",
            "target_distance",
            "distance_resolution",
            "target_residual_analysis",
            "deviation_decomposition",
        ] {
            skipped.insert(id.into(),"documented C=1000,N=800,HP-1000 resolution limit: eigenstate/positive prefixes and target-distance refinement are unresolved; use a separate higher-precision source".into());
        }
    }
    if skipped.contains_key("prefix_ladder") {
        let reason = skipped["prefix_ladder"].clone();
        for n in &plan.prefix_checkpoint_dimensions {
            skipped.insert(format!("prefix_checkpoint_{n}"), reason.clone());
        }
    }
    skipped
}

pub fn preflight(args: &Common, n: usize, p: u32) -> Result<()> {
    let level = match args.research_capture {
        ResearchCapture::Claim => CcmCaptureLevel::Claim,
        ResearchCapture::Research => CcmCaptureLevel::Research,
        ResearchCapture::Gap => CcmCaptureLevel::Gap,
        ResearchCapture::Maximum => CcmCaptureLevel::Maximum,
        ResearchCapture::Ultra => CcmCaptureLevel::Ultra,
    };
    let mut plan = CcmCapturePlan::resolve(level, args.research_sector_eigenpairs.min(n), n + 1)?;
    if !args.capture_prefix_checkpoints.is_empty() {
        let mut points = args.capture_prefix_checkpoints.clone();
        points.push(n + 1);
        points.sort_unstable();
        points.dedup();
        plan = plan.with_prefix_checkpoints(points)?;
    }
    if let Some(bits) = args.capture_working_precision_bits {
        plan = plan.with_prefix_working_precision(bits)?;
    }
    plan.validate_for_source_precision(config(p, args.numerical_profile, 1).precision_bits)?;
    ensure!(
        args.capture_reduction_max_dimension > 0,
        "reduction dimension budget must be positive"
    );
    Ok(())
}

fn capture(
    run: &mut RetainedCcmRun,
    params: &CcmParams,
    cfg: &HighPrecConfig,
    args: &Common,
    cache: &ArtifactCacheContext<'_>,
    journal: &Journal,
    checks: &mut Vec<Check>,
) -> Result<bool> {
    let level = match args.research_capture {
        ResearchCapture::Claim => CcmCaptureLevel::Claim,
        ResearchCapture::Research => CcmCaptureLevel::Research,
        ResearchCapture::Gap => CcmCaptureLevel::Gap,
        ResearchCapture::Maximum => CcmCaptureLevel::Maximum,
        ResearchCapture::Ultra => CcmCaptureLevel::Ultra,
    };
    let mut plan = CcmCapturePlan::resolve(
        level,
        args.research_sector_eigenpairs.min(params.n_modes),
        params.n_modes + 1,
    )?;
    if !args.capture_prefix_checkpoints.is_empty() {
        let mut checkpoints = args.capture_prefix_checkpoints.clone();
        checkpoints.push(params.n_modes + 1);
        checkpoints.sort_unstable();
        checkpoints.dedup();
        plan = plan.with_prefix_checkpoints(checkpoints)?;
    }
    if let Some(bits) = args.capture_working_precision_bits {
        plan = plan.with_prefix_working_precision(bits)?;
    }
    plan.validate_for_source_precision(cfg.precision_bits)?;
    let mut options = plan.primary_options()?;
    if let Some(distance) = &mut options.distance_capture {
        let replacement = ccm::hp::CcmDistanceCaptureOptions::default_convention(
            args.capture_grid_resolution,
            args.capture_profile_steps,
        );
        distance.rules = replacement.rules;
        distance.profile_steps = replacement.profile_steps;
    }
    let reduction = (level == CcmCaptureLevel::Ultra).then(|| RetainedReductionRequest {
        working_precision_bits: args
            .capture_working_precision_bits
            .unwrap_or(cfg.precision_bits),
        maximum_dimension: args.capture_reduction_max_dimension,
        relative_tolerance: format!(
            "1e-{}",
            (cfg.precision_bits.saturating_sub(48) as usize * 3 / 10).max(1)
        ),
    });
    let mut requested = plan
        .receipt()?
        .outcomes()
        .keys()
        .filter(|id| !id.starts_with("prefix_"))
        .cloned()
        .collect::<Vec<_>>();
    if plan.capture_prefix_analysis {
        requested.push("prefix_ladder".into());
        requested.extend(
            plan.prefix_checkpoint_dimensions
                .iter()
                .map(|d| format!("prefix_checkpoint_{d}")),
        );
    }
    if reduction.is_some() {
        requested.push("retained_reduction".into());
    }
    let mut skipped = exclusions(params, cfg, &plan);
    skipped.retain(|id, _| requested.contains(id));
    requested.retain(|id| !skipped.contains_key(id));
    let resolved = json!({"schema_version":1,"capture":plan,"retained_reduction":reduction,"excluded_diagnostics":skipped,"requested_diagnostics":requested,"primary_source_ids":run.primary_sources(),"distance_grid":args.capture_grid_resolution,"profile_steps":args.capture_profile_steps});
    journal.save("capture-plan.json", &resolved)?;
    for (id, reason) in &skipped {
        checks.push(Check::new(id, "capture", Status::Unsupported, reason));
    }
    if requested.is_empty() {
        return Ok(true);
    }
    let retained = if plan.capture_prefix_analysis
        && requested
            .iter()
            .any(|id| id.starts_with("prefix_") || id == "retained_reduction")
    {
        match run.retained_even_sources(cache) {
            Ok(v) => Some(v),
            Err(error) => {
                journal.save(
                    "retained-source-failure.json",
                    &json!({"error":format!("{error:#}")}),
                )?;
                None
            }
        }
    } else {
        None
    };
    let mut prefix: Option<xc_cache::ArtifactExecutionCacheResult<ccm::prefix::CcmPrefixAnalysis>> =
        None;
    let target_missing = xc_spectral::target::TargetProfileSpec::from_environment()
        .err()
        .map(|e| e.to_string());
    let record = xc_cache::capture_and_persist(
        &resolved,
        requested,
        |id| {
            println!("Capture {id}: started");
            if id == "retained_reduction" {
                let (matrix, _) = retained.as_ref().ok_or_else(|| CaptureFailure::Missing {
                    reason: "retained even matrix unavailable".into(),
                })?;
                let budget = reduction.as_ref().expect("requested reduction");
                if matrix.dimension() > budget.maximum_dimension {
                    return Err(CaptureFailure::Blocked {
                        reason: "retained reduction exceeds explicit dimension budget".into(),
                    });
                }
                let r = ccm::prefix::check_retained_reduction_via_cache(
                    matrix,
                    budget.working_precision_bits,
                    budget.maximum_dimension,
                    &budget.relative_tolerance,
                    cache,
                )
                .map_err(CaptureFailure::failed)?;
                let sources = r
                    .produced_manifest
                    .as_ref()
                    .or(r.reused_manifest.as_ref())
                    .map(|m| vec![m.clone()])
                    .unwrap_or_else(|| vec![matrix.manifest().clone()]);
                return CapturedDiagnostic::new(&r.value, sources).map_err(CaptureFailure::failed);
            }
            if id == "prefix_ladder" {
                let (matrix, pairs) = retained.as_ref().ok_or_else(|| CaptureFailure::Missing {
                    reason: "retained even matrix unavailable".into(),
                })?;
                let r = plan
                    .capture_retained_diagnostics(matrix, pairs, cache)
                    .map_err(CaptureFailure::failed)?
                    .ok_or_else(|| CaptureFailure::failed("prefix disabled"))?;
                let sources = r
                    .produced_manifest
                    .as_ref()
                    .or(r.reused_manifest.as_ref())
                    .map(|m| vec![m.clone()])
                    .unwrap_or_else(|| vec![matrix.manifest().clone()]);
                let diagnostic =
                    CapturedDiagnostic::new(&r.value, sources).map_err(CaptureFailure::failed)?;
                prefix = Some(r);
                return Ok(diagnostic);
            }
            if let Some(dimension) = id.strip_prefix("prefix_checkpoint_") {
                let r = prefix.as_ref().ok_or_else(|| CaptureFailure::Blocked {
                    reason: "prefix ladder unavailable".into(),
                })?;
                let checkpoint = r
                    .value
                    .checkpoints
                    .iter()
                    .find(|c| c.dimension.to_string() == dimension)
                    .ok_or_else(|| CaptureFailure::Missing {
                        reason: "requested checkpoint not reached".into(),
                    })?;
                if checkpoint.eigenpair_source.is_none() {
                    return Err(CaptureFailure::Missing {
                        reason:
                            "checkpoint eigenstate unavailable; partial export retained in ladder"
                                .into(),
                    });
                }
                let sources = r
                    .produced_manifest
                    .as_ref()
                    .or(r.reused_manifest.as_ref())
                    .map(|m| vec![m.clone()])
                    .unwrap_or_default();
                return CapturedDiagnostic::new(checkpoint, sources)
                    .map_err(CaptureFailure::failed);
            }
            if matches!(
                id,
                "target_distance"
                    | "distance_resolution"
                    | "target_residual_analysis"
                    | "deviation_decomposition"
            ) {
                if let Some(reason) = &target_missing {
                    return Err(CaptureFailure::Missing {
                        reason: format!("runtime target unavailable: {reason}"),
                    });
                }
            }
            run.capture_diagnostic(id, &options, cache)
                .map_err(CaptureFailure::failed)
        },
        cache,
    )?;
    journal.save("capture.json", &record.value)?;
    for (id, outcome) in record.value.receipt.outcomes() {
        let status = match outcome {
            xc_core::DiagnosticOutcome::Completed { .. } => Status::Pass,
            _ => Status::Incomplete,
        };
        checks.push(Check::new(
            id,
            "capture",
            status,
            if status == Status::Pass {
                "completed".into()
            } else {
                serde_json::to_string(outcome)?
            },
        ));
    }
    Ok(record.value.receipt.is_complete())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn frozen_low_precision_case_is_excluded_before_capture() {
        let p = CcmParams::from_lambda_sq_integer(1000, 800);
        let cfg = config(1000, NumericalProfile::Current, 8);
        let plan = CcmCapturePlan::ultra(8, 801).unwrap();
        let skipped = exclusions(&p, &cfg, &plan);
        assert!(skipped.contains_key("prefix_ladder"));
        assert!(skipped.contains_key("target_distance"));
        assert!(
            !exclusions(&p, &config(2000, NumericalProfile::Current, 8), &plan)
                .contains_key("prefix_ladder")
        );
    }
}
