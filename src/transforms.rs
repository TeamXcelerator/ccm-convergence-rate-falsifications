use crate::{
    journal::{Measurement, Status},
    numerics::{decimal, even_defect, fourier_closed, state_value, text},
    Command, MellinMode,
};
use anyhow::{ensure, Result};
use rayon::prelude::*;
use rug::Float;
use serde_json::json;
use xc_cache::{ArtifactCacheContext, ArtifactManifest};
use xc_spectral::ccm::hp::HighPrecResult;

struct Quadrature {
    x: Vec<Float>,
    naive: Vec<Float>,
    weighted: Option<Vec<Float>>,
    fourier: Option<Vec<Float>>,
    manifest: ArtifactManifest,
    p: u32,
}
impl Quadrature {
    fn new(
        c: u64,
        order: usize,
        p: u32,
        primary: Option<&HighPrecResult>,
        cache: &ArtifactCacheContext<'_>,
    ) -> Result<Self> {
        let borrowed = ArtifactCacheContext {
            resolver: cache.resolver,
            reference_resolver: cache.reference_resolver,
            acceptance: cache.acceptance,
            ordered_overlays: cache.ordered_overlays.clone(),
            mode: cache.mode,
            write_on_miss: cache.write_on_miss,
            write_visibility: cache.write_visibility,
            requested_assurance: cache.requested_assurance,
            certification_failure_policy: cache.certification_failure_policy,
            production_sink: cache.production_sink,
        };
        let rule = xc_numerics::quadrature::gauss_legendre_nodes_via_cache(order, p, borrowed)?;
        let l = Float::with_val(p, c).ln();
        let mut half = l.clone();
        half /= 2;
        let mut xs = Vec::new();
        let mut naive = Vec::new();
        let mut weighted = primary.map(|_| Vec::new());
        let mut fourier = primary.map(|_| Vec::new());
        for (node, weight) in rule.nodes.iter().zip(&rule.weights) {
            let mut x = node.clone();
            x *= &half;
            let u = x.clone().exp();
            let exp_minus_u = (-u.clone()).exp();
            let mut den = exp_minus_u.clone();
            den += 1;
            den.square_mut();
            let mut a = x.clone();
            a *= 3;
            a /= 2;
            a.exp_mut();
            a *= exp_minus_u;
            a /= den;
            a *= weight;
            a *= &half;
            if let Some(primary) = primary {
                let n = (primary.xi.len() - 1) / 2;
                let f = state_value(&primary.xi, n, &x, &l, p);
                let mut w = a.clone();
                w *= &f;
                weighted.as_mut().unwrap().push(w);
                let mut w = f;
                w *= weight;
                w *= &half;
                fourier.as_mut().unwrap().push(w);
            }
            xs.push(x);
            naive.push(a);
        }
        Ok(Self {
            x: xs,
            naive,
            weighted,
            fourier,
            manifest: rule.artifact_manifest,
            p,
        })
    }
    fn evaluate(&self, t: &Float, weights: &[Float]) -> (Float, Float) {
        let mut re = Float::with_val(self.p, 0);
        let mut im = Float::with_val(self.p, 0);
        for (x, a) in self.x.iter().zip(weights) {
            let mut phase = t.clone();
            phase *= x;
            let (mut sin, mut cos) = phase.sin_cos(Float::new(self.p));
            cos *= a;
            sin *= a;
            re += cos;
            im += sin;
        }
        (re, im)
    }
    fn norm(&self, weights: &[Float]) -> Float {
        let mut n = Float::with_val(self.p, 0);
        for a in weights {
            n += a.clone().abs();
        }
        n
    }
}

/// This deliberately returns real-part crossings, never "complex zeros".
fn crossings<F>(f: F, lo: &Float, hi: &Float, steps: usize, iterations: u32) -> Vec<Float>
where
    F: Fn(&Float) -> Float + Sync,
{
    let p = lo.prec();
    let mut spacing = hi.clone();
    spacing -= lo;
    spacing /= steps;
    let point = |i: usize| {
        let mut t = spacing.clone();
        t *= i;
        t += lo;
        t
    };
    let values = (0..=steps)
        .into_par_iter()
        .map(|i| f(&point(i)))
        .collect::<Vec<_>>();
    let mut roots = Vec::new();
    for i in 0..=steps {
        if values[i] == 0 {
            roots.push(point(i));
        }
        if i == 0
            || values[i] == 0
            || values[i - 1] == 0
            || values[i].is_sign_negative() == values[i - 1].is_sign_negative()
        {
            continue;
        }
        let mut a = point(i - 1);
        let mut b = point(i);
        let sign = values[i - 1].is_sign_negative();
        for _ in 0..iterations {
            let mut mid = a.clone();
            mid += &b;
            mid /= 2;
            let value = f(&mid);
            if value == 0 {
                a = mid.clone();
                b = mid;
                break;
            }
            if value.is_sign_negative() == sign {
                a = mid;
            } else {
                b = mid;
            }
        }
        let mut root = Float::with_val(p, a);
        root += b;
        root /= 2;
        roots.push(root);
    }
    roots.sort_by(|a, b| a.partial_cmp(b).unwrap());
    roots.dedup();
    roots
}

pub fn run(
    primary: Option<&HighPrecResult>,
    command: &Command,
    cache: &ArtifactCacheContext<'_>,
) -> Result<Measurement> {
    let Command::MellinCompare {
        lambda_sq,
        n_modes,
        precision_digits,
        t_min,
        t_max,
        n_scan,
        n_quad,
        mellin_mode,
        scan_digits,
        residual_tolerance,
    } = command
    else {
        unreachable!()
    };
    ensure!(
        *lambda_sq >= 2 && t_min.is_finite() && t_max.is_finite() && t_min < t_max && *t_min >= 0.0,
        "invalid transform range"
    );
    ensure!(
        *n_scan > 0
            && *n_quad >= 8
            && *n_quad <= 100_000
            && *scan_digits > 0
            && scan_digits <= precision_digits,
        "invalid transform resolution"
    );
    let p =
        xc_spectral::ccm::hp::HighPrecConfig::for_decimal_digits(*precision_digits).precision_bits;
    let tolerance = decimal(residual_tolerance, p)?;
    ensure!(
        tolerance > 0 && tolerance < 1,
        "residual tolerance must be between zero and one"
    );
    let lo = Float::with_val(p, *t_min);
    let hi = Float::with_val(p, *t_max);
    let coarse = Quadrature::new(*lambda_sq, *n_quad, p, primary, cache)?;
    let fine = Quadrature::new(*lambda_sq, 2 * n_quad, p, primary, cache)?;
    let iterations = (*scan_digits as f64 * std::f64::consts::LOG2_10).ceil() as u32 + 8;
    let mut all = Vec::new();
    let mut all_refined = true;
    let mut any_crossings = false;
    let mut failures = 0;
    let kinds = if *mellin_mode == MellinMode::Both {
        vec!["naive", "weighted"]
    } else {
        vec!["naive"]
    };
    for kind in kinds {
        let weights = if kind == "naive" {
            &coarse.naive
        } else {
            coarse.weighted.as_ref().expect("weighted source")
        };
        let fine_weights = if kind == "naive" {
            &fine.naive
        } else {
            fine.weighted.as_ref().expect("weighted source")
        };
        let norm = fine.norm(fine_weights);
        ensure!(norm > 0, "transform has zero sampled weight");
        let roots = crossings(
            |t| coarse.evaluate(t, weights).0,
            &lo,
            &hi,
            *n_scan,
            iterations,
        );
        any_crossings |= !roots.is_empty();
        let mut rows = Vec::new();
        for (i, t) in roots.iter().enumerate() {
            let a = coarse.evaluate(t, weights);
            let b = fine.evaluate(t, fine_weights);
            let mut dre = b.0.clone();
            dre -= &a.0;
            let mut dim = b.1.clone();
            dim -= &a.1;
            let mut delta = if dre.clone().abs() > dim.clone().abs() {
                dre.abs()
            } else {
                dim.abs()
            };
            delta /= &norm;
            let mut re = b.0.clone().abs();
            re /= &norm;
            let mut im = b.1.clone().abs();
            im /= &norm;
            let refined = delta <= tolerance;
            all_refined &= refined;
            let mut separation = delta.clone();
            separation *= 2;
            separation += &tolerance;
            let rejected = im > separation || re > separation;
            if refined && rejected {
                failures += 1;
            }
            rows.push(json!({"crossing_index":i+1,"t":text(t),"real_coarse":text(&a.0),"imaginary_coarse":text(&a.1),"real_refined":text(&b.0),"imaginary_refined":text(&b.1),"normalized_real_residual":text(&re),"normalized_imaginary_residual":text(&im),"normalized_quadrature_difference":text(&delta),"quadrature_gate":if refined {"PASS"}else{"INCOMPLETE"},"complex_zero_residual_gate":if !refined {"INCOMPLETE"}else if rejected {"FAIL"}else{"PASS"},"meaning":"real-part crossing; small complex residual is a necessary numerical test, not a root certificate"}));
        }
        // Reference-centric nearest matches are explicitly allowed to collide;
        // collisions are exposed rather than presented as unique spectral indexing.
        let refs = xc_zeta::zeros::bundled_first_n_strings(1000)?;
        let mut matches = Vec::new();
        let mut used = std::collections::BTreeSet::new();
        for (k, s) in refs.iter().enumerate() {
            let reference = decimal(s, p)?;
            if reference < lo || reference > hi {
                continue;
            }
            let nearest = roots
                .iter()
                .enumerate()
                .map(|(i, t)| {
                    let mut e = t.clone();
                    e -= &reference;
                    (i, e.abs())
                })
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            if let Some((i, error)) = nearest {
                let duplicate = !used.insert(i);
                matches.push(json!({"reference_k":k+1,"reference":text(&reference),"crossing_index":i+1,"absolute_distance":text(&error),"duplicate_crossing_assignment":duplicate,"indexing":"nearest-crossing diagnostic only"}));
            } else {
                matches.push(json!({"reference_k":k+1,"crossing_index":null,"qualification":"no observed crossing"}));
            }
        }
        all.push(json!({"transform":kind,"crossing_count":rows.len(),"crossings":rows,"nearest_reference_diagnostics":matches}));
    }
    let mut fourier_rows = Vec::new();
    let mut fourier_ok = true;
    if let Some(primary) = primary {
        let l = Float::with_val(p, *lambda_sq).ln();
        let n = (primary.xi.len() - 1) / 2;
        let weights = fine.fourier.as_ref().unwrap();
        let norm = fine.norm(weights);
        for (i, root) in primary.eigenvalues_pos.iter().enumerate() {
            let Some(t) = root.value() else {
                continue;
            };
            let Some(closed) = fourier_closed(&primary.xi, n, t, &l, p) else {
                continue;
            };
            let integral = fine.evaluate(t, weights);
            let mut error = integral.0.clone();
            error -= &closed;
            error.abs_mut();
            if norm > 0 {
                error /= &norm;
            }
            let mut odd = integral.1.clone().abs();
            if norm > 0 {
                odd /= &norm;
            }
            let passed = error <= tolerance && odd <= tolerance;
            fourier_ok &= passed;
            fourier_rows.push(json!({"root_index":i+1,"t":text(t),"fourier_real":text(&integral.0),"fourier_imaginary":text(&integral.1),"secular_closed_form":text(&closed),"normalized_discrepancy":text(&error),"gate":if passed {"PASS"}else{"INCOMPLETE"}}));
        }
    }
    let mut m = Measurement::new(
        "complex transform residuals at real-part crossings",
        json!({"C":lambda_sq,"N":n_modes,"precision_bits":p,"quadrature_coordinate":"log-u","coarse_order":n_quad,"refined_order":2*n_quad,"scan_intervals":n_scan,"scan_digits":scan_digits,"bisection_iterations":iterations,"residual_tolerance":residual_tolerance,"transform_results":all,"exact_fourier_secular_control":fourier_rows,"root_count_assurance":"sampled real-part crossings; no completeness certificate"}),
    );
    m.source_manifests = vec![coarse.manifest, fine.manifest];
    m.check(
        "observed comparison crossings",
        "validation",
        if any_crossings {
            Status::Pass
        } else {
            Status::Incomplete
        },
        "all observed crossings and unmatched reference points are retained",
    );
    m.check(
        "quadrature refinement",
        "validation",
        if all_refined {
            Status::Pass
        } else {
            Status::Incomplete
        },
        "complex values compared at GL orders Q and 2Q, normalized by the absolute weight integral",
    );
    m.check("reported crossings are complex zeros","hypothesis",if !all_refined {Status::Unassessed}else if failures>0 {Status::Fail}else {Status::Unassessed},format!("{failures} crossing(s) fail the complex residual gate; a passing residual alone is not a zero certificate"));
    if let Some(primary) = primary {
        let defect = even_defect(&primary.xi, *n_modes, p);
        m.check(
            "even source",
            "validation",
            if defect < decimal("1e-15", p)? {
                Status::Pass
            } else {
                Status::Incomplete
            },
            format!("relative coefficient defect {}", text(&defect)),
        );
        m.check("exact Fourier/secular control","validation",if fourier_ok && !fourier_rows.is_empty() {Status::Pass}else{Status::Incomplete},"same finite even state, independent quadrature versus analytic rational representation");
    }
    Ok(m)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_scan_node_is_retained_once() {
        let roots = crossings(
            |t| {
                let mut x = t.clone();
                x -= 0.5;
                x
            },
            &Float::with_val(128, 0),
            &Float::with_val(128, 1),
            8,
            80,
        );
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0], 0.5);
    }
    #[test]
    fn crossing_is_not_a_complex_zero() {
        let roots = crossings(
            |t| {
                let mut x = t.clone();
                x -= 0.375;
                x
            },
            &Float::with_val(128, 0),
            &Float::with_val(128, 1),
            7,
            80,
        );
        assert_eq!(roots.len(), 1);
        // The complex function (t-.375)+i has this real crossing and no zero.
        let imaginary = Float::with_val(128, 1);
        assert!(imaginary > decimal("1e-20", 128).unwrap());
    }
}
