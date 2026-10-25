//! Independent finite-grid reference preparation. Operational scheduling only:
//! the ordinary measurement route still authenticates the complete artifacts.
use crate::Command;
use anyhow::{ensure, Result};
use xc_cache::ArtifactCacheContext;
use xc_core::{HpRuntimeMode, RunResources};

pub(super) fn with_reference<T, F>(
    command: &Command,
    precision: u32,
    cache: &ArtifactCacheContext<'_>,
    acquire: F,
) -> Result<T>
where
    F: FnOnce() -> Result<T>,
{
    let Command::ProlateCompare {
        lambda_sq,
        n_grid,
        n_sample,
        ..
    } = command
    else {
        return acquire();
    };
    let enabled = match std::env::var("XC_PROLATE_PREFETCH") {
        Ok(v) if v == "0" => false,
        Ok(v) if v == "1" => true,
        Ok(_) => anyhow::bail!("XC_PROLATE_PREFETCH must be 0 or 1"),
        Err(std::env::VarError::NotPresent) => true,
        Err(e) => return Err(e.into()),
    };
    let policy = xc_numerics::hp_runtime::active_policy().unwrap_or_default();
    if !enabled
        || rayon::current_num_threads() < 2
        || policy.mode != HpRuntimeMode::FullParallel
        || cache.mode != xc_cache::ArtifactExecutionCacheMode::PreferReuse
        || !cache.write_on_miss
    {
        return acquire();
    }
    let Some(parent) = RunResources::current() else {
        return acquire();
    };
    // A dedicated worker makes progress even if foreground Rayon workers must
    // wait for this reservation. The child never uses the parent's ledger.
    let pool = match rayon::ThreadPoolBuilder::new().num_threads(1).build() {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Reference overlap unavailable ({e}); using sequential stages");
            return acquire();
        }
    };
    let exponents = (rug::float::exp_min(), rug::float::exp_max());
    let lambda = rug::Float::with_val(precision, *lambda_sq).sqrt();
    let plan = pool.install(|| {
        ensure!(
            exponents == (rug::float::exp_min(), rug::float::exp_max()),
            "reference worker exponent range differs"
        );
        xc_spectral::prolate::hp::plan_finite_dirichlet(
            &lambda,
            *n_grid,
            *n_sample,
            precision,
            parent.maximum_bytes(),
        )
    });
    let bytes = match plan {
        Ok(plan) => plan.estimated_peak_bytes.checked_add(
            65536
                + 4 * (u64::from(precision)
                    .saturating_mul(2)
                    .saturating_add(64)
                    .div_ceil(4)
                    + 64),
        ),
        Err(_) => None,
    };
    let Some((bytes, reservation)) = bytes.and_then(|bytes| {
        parent
            .try_reserve_independent(bytes)
            .ok()
            .map(|r| (bytes, r))
    }) else {
        println!("Stage independent reference: sequential (joint workspace unavailable)");
        return acquire();
    };
    let child = RunResources::with_worker_limit(bytes, 1)?;
    println!("Stage independent reference: started (one dedicated worker, {bytes} reserved bytes)");
    std::thread::scope(|scope| {
        let child_ref = &child;
        let task = scope.spawn(move || {
            let _reservation = reservation;
            pool.install(|| {
                let _scope = child_ref.enter();
                xc_numerics::hp_runtime::run_hp_with_policy(&policy, || {
                    // Drop all result allocations before releasing the parent
                    // reservation. Measurement subsequently reuses the cache.
                    xc_spectral::prolate::hp::compute_k_lambda_finite_dirichlet_via_cache(
                        &lambda, *n_grid, *n_sample, precision, cache,
                    )
                    .map(drop)
                })
                .map_err(anyhow::Error::from)
                .and_then(std::convert::identity)
            })
        });
        struct CancelOnDrop<'a>(&'a RunResources);
        impl Drop for CancelOnDrop<'_> {
            fn drop(&mut self) {
                self.0.cancel();
            }
        }
        let cancel_on_unwind = CancelOnDrop(&child);
        let primary = acquire();
        if primary.is_err() {
            child.cancel();
        }
        let prepared = task.join();
        drop(cancel_on_unwind);
        match prepared {
            Ok(Ok(())) => println!("Stage independent reference: completed; artifacts ready for measurement"),
            Ok(Err(e)) => eprintln!("Stage independent reference: {e:#}; required measurement retains its ordinary validation path"),
            Err(_) => eprintln!("Stage independent reference: worker panicked; required measurement retains its ordinary validation path"),
        }
        // Primary failures always win; background preparation never changes a
        // failed primary into a successful experiment.
        primary
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::BTreeMap,
        sync::{Condvar, Mutex},
        time::Duration,
    };
    use xc_cache::{ArtifactProductionSink, CacheError, ContentDigest, ProducedArtifactRecord};

    #[derive(Default)]
    struct Sink {
        records: Mutex<Vec<ProducedArtifactRecord>>,
        ready: Condvar,
    }
    impl ArtifactProductionSink for Sink {
        fn record(&self, record: ProducedArtifactRecord) -> Result<(), CacheError> {
            self.records.lock().unwrap().push(record);
            self.ready.notify_all();
            Ok(())
        }
        fn record_assurance(
            &self,
            _: xc_cache::ArtifactAssuranceAttestation,
        ) -> Result<(), CacheError> {
            Ok(())
        }
        fn record_assurance_requirement(
            &self,
            _: xc_cache::ArtifactAssuranceRequirement,
        ) -> Result<(), CacheError> {
            Ok(())
        }
        fn record_evidence(&self, _: &str, payload: &[u8]) -> Result<ContentDigest, CacheError> {
            Ok(ContentDigest::sha256(payload))
        }
    }
    fn records(sink: &Sink) -> BTreeMap<String, (serde_json::Value, Vec<u8>)> {
        sink.records
            .lock()
            .unwrap()
            .iter()
            .map(|r| {
                let mut manifest = serde_json::to_value(&r.manifest).unwrap();
                // Independently created envelopes naturally have different wall
                // clocks. Payload bytes, keys, dependencies and all other fields
                // must agree exactly.
                manifest
                    .as_object_mut()
                    .unwrap()
                    .remove("created_unix_seconds");
                (
                    r.semantic_key.artifact_kind.clone(),
                    (manifest, r.payload.clone()),
                )
            })
            .collect()
    }
    #[test]
    fn overlap_preserves_complete_reference_artifacts_and_primary_errors() {
        use xc_cache::{
            ArtifactExecutionCacheMode, CacheLayer, CachePolicy, CacheResolver, CacheVisibility,
            FilesystemCacheStore,
        };
        let root = std::env::temp_dir().join(format!(
            "prolate-overlap-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&root).unwrap();
        struct Cleanup(std::path::PathBuf);
        impl Drop for Cleanup {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }
        let _cleanup = Cleanup(root.clone());
        let policy = CachePolicy {
            current_toolkit_version: xc_cache::ToolkitVersion::parse("0.17.0").unwrap(),
            minimum_quality: xc_cache::CacheQuality::Validated,
            accepted_schema_versions: vec![1],
            allow_deprecated: false,
            allow_quarantined: false,
            allowed_visibilities: vec![CacheVisibility::Local],
        };
        let command = Command::ProlateCompare {
            lambda_sq: 13,
            n_modes: 24,
            precision_digits: 30,
            n_grid: 33,
            n_sample: 7,
            max_scaled_residual: None,
        };
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(2)
            .build()
            .unwrap();
        let mut expected = None;
        for overlap in [false, true] {
            let resolver = CacheResolver::new(vec![CacheLayer {
                precedence: 0,
                store: Box::new(FilesystemCacheStore::new(
                    "fixture",
                    root.join(if overlap { "overlap" } else { "sequential" }),
                    true,
                    CacheVisibility::Local,
                )),
            }]);
            let sink = Sink::default();
            let cache = ArtifactCacheContext {
                resolver: Some(&resolver),
                reference_resolver: None,
                acceptance: Some(&policy),
                ordered_overlays: vec!["fixture".into()],
                mode: ArtifactExecutionCacheMode::PreferReuse,
                write_on_miss: true,
                write_visibility: CacheVisibility::Local,
                requested_assurance: xc_core::AssuranceLevel::Computed,
                certification_failure_policy:
                    xc_cache::CertificationFailurePolicy::RetainComputedFailRun,
                production_sink: Some(&sink),
            };
            pool.install(|| {
                let resources = RunResources::new(64 << 20)
                    .unwrap()
                    .waiting_for_independent();
                let _scope = resources.enter();
                if overlap {
                    let result = with_reference(&command, 160, &cache, || {
                        // No reference can finish here if preparation runs only
                        // after the primary closure: this checks actual overlap.
                        let (guard, timeout) = sink
                            .ready
                            .wait_timeout_while(
                                sink.records.lock().unwrap(),
                                Duration::from_secs(30),
                                |r| r.len() < 3,
                            )
                            .unwrap();
                        assert!(!timeout.timed_out() && guard.len() == 3);
                        Ok(42)
                    })
                    .unwrap();
                    assert_eq!(result, 42);
                    assert_eq!(resources.usage().working_bytes, 0);
                }
                let lambda = rug::Float::with_val(160, 13).sqrt();
                xc_spectral::prolate::hp::compute_k_lambda_finite_dirichlet_via_cache(
                    &lambda, 33, 7, 160, &cache,
                )
                .unwrap();
                let actual = records(&sink);
                assert_eq!(actual.len(), 3); // spectrum, modes including windows, samples
                if let Some(expected) = &expected {
                    assert_eq!(&actual, expected);
                } else {
                    expected = Some(actual);
                }
                if overlap {
                    let error = with_reference::<(), _>(&command, 160, &cache, || {
                        anyhow::bail!("primary refused")
                    })
                    .unwrap_err();
                    assert_eq!(error.to_string(), "primary refused");
                    assert_eq!(resources.usage().working_bytes, 0);
                    let tight = RunResources::new(500_000)
                        .unwrap()
                        .waiting_for_independent();
                    let _tight_scope = tight.enter();
                    with_reference(&command, 160, &cache, || {
                        assert_eq!(tight.usage().working_bytes, 0);
                        Ok(())
                    })
                    .unwrap();
                }
            });
        }
    }
}
