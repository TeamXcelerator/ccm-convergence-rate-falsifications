use crate::{
    journal::{Measurement, Status},
    numerics::{decimal, even_defect, fits, state_value, text},
    Command, Common, RootAcquisition,
};
use anyhow::{ensure, Result};
use rayon::prelude::*;
use rug::Float;
use serde_json::json;
use xc_cache::ArtifactCacheContext;
use xc_spectral::ccm::hp::HighPrecResult;

pub fn prolate(
    primary: &HighPrecResult,
    command: &Command,
    cache: &ArtifactCacheContext<'_>,
) -> Result<Measurement> {
    let Command::ProlateCompare {
        lambda_sq,
        n_modes,
        n_grid,
        n_sample,
        max_scaled_residual,
        ..
    } = command
    else {
        unreachable!()
    };
    ensure!(
        *n_grid >= 17 && n_grid % 2 == 1 && *n_sample >= 3,
        "require odd n-grid>=17 and n-sample>=3"
    );
    let p = primary.precision_bits;
    let lambda = Float::with_val(p, *lambda_sq).sqrt();
    let l = Float::with_val(p, *lambda_sq).ln();
    // Observe the exact prolate spectrum dependency while preserving the managed sink.
    struct Observer<'a> {
        parent: Option<&'a dyn xc_cache::ArtifactProductionSink>,
        manifests: std::sync::Mutex<Vec<xc_cache::ArtifactManifest>>,
    }
    impl xc_cache::ArtifactProductionSink for Observer<'_> {
        fn record_assurance_requirement(
            &self,
            value: xc_cache::ArtifactAssuranceRequirement,
        ) -> std::result::Result<(), xc_cache::CacheError> {
            if let Some(parent) = self.parent {
                parent.record_assurance_requirement(value)?;
            }
            Ok(())
        }
        fn record_assurance(
            &self,
            value: xc_cache::ArtifactAssuranceAttestation,
        ) -> std::result::Result<(), xc_cache::CacheError> {
            if let Some(parent) = self.parent {
                parent.record_assurance(value)?;
            }
            Ok(())
        }
        fn record_evidence(
            &self,
            kind: &str,
            payload: &[u8],
        ) -> std::result::Result<xc_cache::ContentDigest, xc_cache::CacheError> {
            if let Some(parent) = self.parent {
                parent.record_evidence(kind, payload)
            } else {
                Ok(xc_cache::ContentDigest::sha256(payload))
            }
        }
        fn record(
            &self,
            artifact: xc_cache::ProducedArtifactRecord,
        ) -> std::result::Result<(), xc_cache::CacheError> {
            self.manifests
                .lock()
                .expect("manifest observer")
                .push(artifact.manifest.clone());
            if let Some(parent) = self.parent {
                parent.record(artifact)?;
            }
            Ok(())
        }
    }
    let observer = Observer {
        parent: cache.production_sink,
        manifests: std::sync::Mutex::new(vec![]),
    };
    let observed = ArtifactCacheContext {
        resolver: cache.resolver,
        reference_resolver: cache.reference_resolver,
        acceptance: cache.acceptance,
        ordered_overlays: cache.ordered_overlays.clone(),
        mode: cache.mode,
        write_on_miss: cache.write_on_miss,
        write_visibility: cache.write_visibility,
        requested_assurance: cache.requested_assurance,
        certification_failure_policy: cache.certification_failure_policy,
        production_sink: Some(&observer),
    };
    let result = xc_spectral::prolate::hp::compute_k_lambda_via_cache(
        &lambda, *n_grid, *n_sample, p, &observed,
    )?;
    let values = result
        .u_grid
        .par_iter()
        .map(|u| state_value(&primary.xi, *n_modes, &u.clone().ln(), &l, p))
        .collect::<Vec<_>>();
    let maximum = |v: &[Float]| {
        v.iter()
            .map(|x| x.clone().abs())
            .fold(Float::with_val(p, 0), |a, b| if a > b { a } else { b })
    };
    let xscale = maximum(&values);
    let kscale = maximum(&result.k_values);
    ensure!(
        xscale > 0 && kscale > 0,
        "comparison requires nonzero finite sample vectors"
    );
    let xs = values
        .iter()
        .map(|x| {
            let mut v = x.clone();
            v /= &xscale;
            v
        })
        .collect::<Vec<_>>();
    let ks = result
        .k_values
        .iter()
        .map(|x| {
            let mut v = x.clone();
            v /= &kscale;
            v
        })
        .collect::<Vec<_>>();
    let fit = fits(&xs, &ks, p)?;
    let original_scalar = |a: &Float| {
        let mut v = a.clone();
        v *= &xscale;
        v /= &kscale;
        text(&v)
    };
    let mut scaled = fit.minimax_error.clone();
    scaled *= *lambda_sq;
    let defect = even_defect(&primary.xi, *n_modes, p);
    let samples = result
        .u_grid
        .iter()
        .zip(&values)
        .zip(&result.k_values)
        .map(|((u, x), k)| json!({"u":text(u),"xi":text(x),"k_lambda":text(k)}))
        .collect::<Vec<_>>();
    let mut m = Measurement::new(
        "sampled Weil/prolate approximation",
        json!({"C":lambda_sq,"N":n_modes,"precision_bits":p,"n_grid":n_grid,"n_sample":n_sample,"least_squares":{"scalar":original_scalar(&fit.least_squares),"relative_linf":text(&fit.ls_error)},"discrete_minimax":{"scalar":original_scalar(&fit.minimax),"relative_linf":text(&fit.minimax_error),"scaled_by_C":text(&scaled),"optimization_error_interval_width":text(&fit.width),"assurance":"numerical discrete optimization; not a continuous norm certificate"},"sample_xi_norm":text(&xscale),"sample_k_norm":text(&kscale),"prolate_eigenvalue_0":text(&result.eigenvalue_0),"prolate_eigenvalue_4":text(&result.eigenvalue_4),"combination_c0":text(&result.c_0),"combination_c4":text(&result.c_4),"source_evenness_defect":text(&defect),"samples":samples}),
    );
    m.source_manifests = observer.manifests.into_inner().expect("manifest observer");
    ensure!(
        !m.source_manifests.is_empty(),
        "prolate source identity missing"
    );
    m.check(
        "complete sample grid",
        "validation",
        Status::Pass,
        format!(
            "retained all {} function pairs; small amplitudes were normalized before fitting",
            samples.len()
        ),
    );
    m.check(
        "even source",
        "validation",
        if defect < decimal("1e-15", p)? {
            Status::Pass
        } else {
            Status::Incomplete
        },
        format!("relative coefficient evenness defect {}", text(&defect)),
    );
    if *lambda_sq == 1000 && *n_modes == 800 && p == 3386 {
        m.check("historical source resolution","validation",Status::Incomplete,"this known HP-1000 source requires a higher-precision comparison before a structural interpretation");
    }
    m.check("CCM Lemma 7.2","hypothesis",Status::Unsupported,"the lemma concerns prolate/Hermite convergence; this experiment tests the proposed Weil/prolate approximation");
    if let Some(bound) = max_scaled_residual {
        let b = decimal(bound, p)?;
        ensure!(b >= 0, "max-scaled-residual must be nonnegative");
        m.check(
            "specified finite scaled-residual gate",
            "hypothesis",
            if scaled <= b {
                Status::Pass
            } else {
                Status::Fail
            },
            format!(
                "observed C*relative sampled minimax residual {}; specified bound {}",
                text(&scaled),
                text(&b)
            ),
        );
    } else {
        m.check(
            "asymptotic rate",
            "hypothesis",
            Status::Unassessed,
            "no finite O-constant or asymptotic onset is inferred from this row",
        );
    }
    Ok(m)
}

fn stats(errors: &[Float], ln_lambda: &Float, p: u32) -> serde_json::Value {
    if errors.is_empty() {
        return serde_json::Value::Null;
    }
    let mut sorted = errors.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).expect("finite errors"));
    let mut sum = Float::with_val(p, 0);
    for x in errors {
        sum += x;
    }
    let mut mean = sum.clone();
    mean /= errors.len();
    let maximum = sorted.last().unwrap();
    let mut mean_log = mean.clone();
    mean_log *= ln_lambda;
    let mut product = maximum.clone();
    let mut maxima = Vec::new();
    for alpha in 1..=3 {
        product *= ln_lambda;
        maxima.push(json!({"alpha":alpha,"value":text(&product)}));
    }
    json!({"count":errors.len(),"sum_absolute_error":text(&sum),"mean_absolute_error":text(&mean),"maximum_absolute_error":text(maximum),"median_absolute_error":text(&sorted[(sorted.len()-1)/2]),"p90_absolute_error":text(&sorted[((sorted.len()-1)*9)/10]),"p99_absolute_error":text(&sorted[((sorted.len()-1)*99)/100]),"mean_times_log_lambda":text(&mean_log),"maximum_times_log_powers":maxima})
}

pub fn spectral(
    primary: &HighPrecResult,
    c: u64,
    n: usize,
    count: usize,
    trim: f64,
    args: &Common,
    _cache: &ArtifactCacheContext<'_>,
) -> Result<Measurement> {
    let p = primary.precision_bits;
    let references = xc_zeta::zeros::bundled_first_n_strings(count)?
        .iter()
        .map(|s| decimal(s, p))
        .collect::<Result<Vec<_>>>()?;
    let complete = primary.eigenvalues_pos.len() == count && primary.first_positive_root_index == 1;
    let finite = primary
        .eigenvalues_pos
        .iter()
        .all(|r| r.value().is_some_and(|x| x.is_finite() && x > &0));
    let converged = primary.eigenvalues_pos.iter().all(|r| r.is_converged());
    let ordered = primary
        .eigenvalues_pos
        .windows(2)
        .all(|w| w[0].value().zip(w[1].value()).is_some_and(|(a, b)| a < b));
    let mut rows = Vec::new();
    let mut errors = Vec::new();
    let mut relatives = Vec::new();
    let ceiling =
        (((p.saturating_sub(64)) as f64 * std::f64::consts::LOG10_2).floor() as usize).min(2500);
    for (index, reference) in references.iter().enumerate() {
        let root = primary.eigenvalues_pos.get(index);
        if let Some(value) = root.and_then(|r| r.value()).filter(|v| v.is_finite()) {
            let mut error = value.clone();
            error -= reference;
            error.abs_mut();
            let mut relative = error.clone();
            relative /= reference;
            let digits = if relative == 0 {
                None
            } else {
                Some(text(&(-relative.clone().log10())))
            };
            rows.push(json!({"k":index+1,"index_kind":if args.root_acquisition==RootAcquisition::Independent {"source_discovered_finite_window"}else{"reference_seed_index"},"value":text(value),"reference":text(reference),"absolute_error":text(&error),"relative_error":text(&relative),"relative_digits":digits,"comparison_precision_ceiling":ceiling,"equal_at_comparison_precision":relative==0,"converged":root.is_some_and(|r|r.is_converged())}));
            errors.push(error);
            relatives.push(Some(relative));
        } else {
            rows.push(json!({"k":index+1,"value":null,"qualification":"missing_or_failed"}));
            relatives.push(None);
        }
    }
    let mut ln_lambda = Float::with_val(p, c).ln();
    ln_lambda /= 2;
    let qualified = complete && finite && converged && ordered;
    let kept = ((count as f64 * (1.0 - trim)).floor() as usize).max(1);
    let full = if qualified {
        stats(&errors, &ln_lambda, p)
    } else {
        serde_json::Value::Null
    };
    let interior = if qualified && kept < count {
        stats(&errors[..kept], &ln_lambda, p)
    } else {
        serde_json::Value::Null
    };
    let budgets=[1,5,10,25,50,100,200,500,1000].into_iter().filter(|&d|d<=ceiling).map(|digits|->Result<_>{
        let threshold=decimal(&format!("1e-{digits}"),p)?;
        let prefix=relatives.iter().enumerate().take_while(|(i,e)|e.as_ref().is_some_and(|e|e<=&threshold) && primary.eigenvalues_pos.get(*i).is_some_and(|r|r.is_converged())).count();
        Ok(json!({"relative_digits":digits,"contiguous_prefix":prefix,"qualified_source_window":qualified && args.root_acquisition==RootAcquisition::Independent}))
    }).collect::<Result<Vec<_>>>()?;
    let cumulative = if qualified {
        let mut total = Float::with_val(p, 0);
        for e in &errors {
            total += e;
        }
        let mut cumulative = Float::with_val(p, 0);
        errors
            .iter()
            .enumerate()
            .map(|(i, e)| {
                cumulative += e;
                let fraction = if total > 0 {
                    let mut x = cumulative.clone();
                    x /= &total;
                    Some(text(&x))
                } else {
                    None
                };
                json!({"through_k":i+1,"error_sum":text(&cumulative),"fraction_of_total":fraction})
            })
            .collect::<Vec<_>>()
    } else {
        vec![]
    };
    let mut m = Measurement::new(
        "indexed finite-window spectral errors",
        json!({"C":c,"N":n,"requested_root_count":count,"returned_root_count":primary.eigenvalues_pos.len(),"precision_bits":p,"root_acquisition":args.root_acquisition,"reference_dataset":xc_zeta::zeros::bundled_dataset_identity()?,"full_window":full,"trim_fraction":trim,"trimmed_prefix":interior,"observed_subset":if qualified {serde_json::Value::Null}else{stats(&errors,&ln_lambda,p)},"roots":rows,"accuracy_prefixes":budgets,"cumulative_error":cumulative,"spectral_scope":"finite movable-root window; not a certificate of the full determinant spectral ordinal"}),
    );
    for (name, ok) in [
        ("requested root count", complete),
        ("finite positive roots", finite),
        ("converged roots", converged),
        ("strict unique ordering", ordered),
    ] {
        m.check(
            name,
            "validation",
            if ok { Status::Pass } else { Status::Incomplete },
            if ok {
                "check satisfied"
            } else {
                "aggregate claim statistics withheld; inspect retained root rows"
            },
        );
    }
    m.values["root_discovery_policy"] =
        json!(if args.root_acquisition == RootAcquisition::Independent {
            "complete_positive_even_point_numerator_flint_arb_adaptive_refinement_v1"
        } else {
            "explicit_reference_seeded_refinement"
        });
    m.check(
        "independent ordinal assurance",
        "hypothesis",
        Status::Unassessed,
        if args.root_acquisition == RootAcquisition::Independent {
            "all positive movable-root seeds isolated from the exact even point-source numerator; this is not a full Fourier/determinant ordinal certificate"
        } else {
            "reference-seeded labels do not establish counted spectral ordinals"
        },
    );
    let mut bound = Float::with_val(p, 1);
    bound /= 4;
    bound /= &ln_lambda;
    if qualified && count == n && args.root_acquisition == RootAcquisition::Independent {
        let mut mean = Float::with_val(p, 0);
        for e in &errors {
            mean += e;
        }
        mean /= count;
        m.check("mean lower-bound numerical test","hypothesis",if mean>=bound {Status::Pass}else {Status::Fail},format!("finite-window mean compared with 1/(4 ln lambda)={}; this does not validate the source proof",text(&bound)));
    } else {
        m.check(
            "mean lower-bound numerical test",
            "hypothesis",
            Status::Unassessed,
            "requires a complete independently acquired N-root comparison window",
        );
    }
    m.check("inverse-log asymptotic conjecture","hypothesis",Status::Unassessed,"maximum-error log products are retained; no asymptotic conclusion is assigned from a finite row");
    Ok(m)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maximum_log_products_use_maximum_not_mean() {
        let errors = vec![Float::with_val(128, 0), Float::with_val(128, 4)];
        let s = stats(&errors, &Float::with_val(128, 2), 128);
        assert_eq!(
            decimal(s["mean_times_log_lambda"].as_str().unwrap(), 128).unwrap(),
            4
        );
        assert_eq!(
            decimal(
                s["maximum_times_log_powers"][0]["value"].as_str().unwrap(),
                128
            )
            .unwrap(),
            8
        );
    }
}
