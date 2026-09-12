use anyhow::{ensure, Result};
use rug::{float::Constant, Float};
pub fn decimal(s: &str, p: u32) -> Result<Float> {
    let x = Float::with_val(p, Float::parse(s)?);
    ensure!(x.is_finite(), "expected finite decimal");
    Ok(x)
}
pub fn text(x: &Float) -> String {
    x.to_string_radix(10, None)
}
pub fn pi(p: u32) -> Float {
    Float::with_val(p, Constant::Pi)
}

/// Evaluate the full real part in the centered Fourier basis. Positive and
/// negative coefficients are both used; callers separately qualify evenness.
pub fn state_value(xi: &[Float], n: usize, x: &Float, l: &Float, p: u32) -> Float {
    let mut angle = pi(p);
    angle *= 2;
    angle *= x;
    angle /= l;
    let c = angle.cos();
    let mut previous = Float::with_val(p, 1);
    let mut current = c.clone();
    let mut total = xi[n].clone();
    for j in 1..=n {
        let mut term = xi[n + j].clone();
        term += &xi[n - j];
        term *= &current;
        if j % 2 == 1 {
            term = -term;
        }
        total += term;
        let mut next = c.clone();
        next *= 2;
        next *= &current;
        next -= &previous;
        previous = current;
        current = next;
    }
    total /= l.clone().sqrt();
    total
}
pub fn even_defect(xi: &[Float], n: usize, p: u32) -> Float {
    let mut defect = Float::with_val(p, 0);
    let mut scale = Float::with_val(p, 0);
    for x in xi {
        if x.clone().abs() > scale {
            scale = x.clone().abs();
        }
    }
    for j in 1..=n {
        let mut d = xi[n + j].clone();
        d -= &xi[n - j];
        d.abs_mut();
        if d > defect {
            defect = d;
        }
    }
    if scale > 0 {
        defect /= scale;
    }
    defect
}
pub fn secular(xi: &[Float], n: usize, t: &Float, l: &Float, p: u32) -> Option<Float> {
    let mut s = t.clone();
    s *= l;
    s /= 2;
    s /= pi(p);
    s.square_mut();
    let mut r = xi[n].clone();
    for j in 1..=n {
        let mut den = s.clone();
        den -= j * j;
        if den == 0 {
            return None;
        }
        let mut term = xi[n + j].clone();
        term *= 2;
        term *= &s;
        term /= den;
        r += term;
    }
    Some(r)
}
pub fn fourier_closed(xi: &[Float], n: usize, t: &Float, l: &Float, p: u32) -> Option<Float> {
    if t == &0 {
        let mut v = xi[n].clone();
        v *= l.clone().sqrt();
        return Some(v);
    }
    let mut phase = t.clone();
    phase *= l;
    phase /= 2;
    let mut value = phase.sin();
    value *= 2;
    value /= t;
    value /= l.clone().sqrt();
    value *= secular(xi, n, t, l, p)?;
    Some(value)
}

pub struct Fits {
    pub least_squares: Float,
    pub minimax: Float,
    pub minimax_error: Float,
    pub ls_error: Float,
    pub width: Float,
}
/// Discrete L-infinity fit on normalized samples. A finite bisection interval
/// is retained; this numerical optimization is not a continuous-norm certificate.
pub fn fits(x: &[Float], k: &[Float], p: u32) -> Result<Fits> {
    ensure!(
        !x.is_empty() && x.len() == k.len(),
        "sample lengths must agree"
    );
    let mut xx = Float::with_val(p, 0);
    let mut kk = Float::with_val(p, 0);
    for (x, k) in x.iter().zip(k) {
        let mut term = x.clone();
        term *= k;
        xx += term;
        let mut term = k.clone();
        term.square_mut();
        kk += term;
    }
    ensure!(kk > 0, "prolate samples are all zero");
    let mut ls = xx;
    ls /= kk;
    let residual = |a: &Float| {
        x.iter()
            .zip(k)
            .map(|(x, k)| {
                let mut r = k.clone();
                r *= a;
                r -= x;
                r.abs()
            })
            .fold(Float::with_val(p, 0), |a, b| if a > b { a } else { b })
    };
    let interval = |e: &Float| -> Option<(Float, Float)> {
        let mut lower: Option<Float> = None;
        let mut upper: Option<Float> = None;
        for (x, k) in x.iter().zip(k) {
            if k == &0 {
                if x.clone().abs() > *e {
                    return None;
                }
                continue;
            }
            let mut lo = x.clone();
            lo -= e;
            lo /= k;
            let mut hi = x.clone();
            hi += e;
            hi /= k;
            if lo > hi {
                std::mem::swap(&mut lo, &mut hi);
            }
            if lower.as_ref().is_none_or(|old| lo > *old) {
                lower = Some(lo);
            }
            if upper.as_ref().is_none_or(|old| hi < *old) {
                upper = Some(hi);
            }
            if lower
                .as_ref()
                .zip(upper.as_ref())
                .is_some_and(|(l, h)| l > h)
            {
                return None;
            }
        }
        lower.zip(upper)
    };
    let mut lo = Float::with_val(p, 0);
    let mut hi = residual(&Float::with_val(p, 0));
    for _ in 0..160.min(p / 2) {
        let mut mid = lo.clone();
        mid += &hi;
        mid /= 2;
        if interval(&mid).is_some() {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    let (a, b) = interval(&hi).context("no feasible minimax scalar")?;
    let mut minimax = a;
    minimax += b;
    minimax /= 2;
    let mut width = hi;
    width -= lo;
    Ok(Fits {
        least_squares: ls.clone(),
        ls_error: residual(&ls),
        minimax_error: residual(&minimax),
        minimax,
        width,
    })
}
use anyhow::Context;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimax_is_distinct_from_least_squares() {
        let x = vec![Float::with_val(256, 0), Float::with_val(256, 1)];
        let k = vec![Float::with_val(256, 1), Float::with_val(256, 2)];
        let f = fits(&x, &k, 256).unwrap();
        assert!(f.minimax_error < f.ls_error);
        let mut d = f.minimax.clone();
        d -= Float::with_val(256, 1) / 3;
        assert!(d.abs() < decimal("1e-35", 256).unwrap());
    }
    #[test]
    fn tiny_nonzero_samples_are_not_rejected_by_absolute_threshold() {
        let x = vec![Float::with_val(256, 1), Float::with_val(256, 2)];
        let k = vec![
            decimal("1e-500", 256).unwrap(),
            decimal("2e-500", 256).unwrap(),
        ];
        assert!(fits(&x, &k, 256).is_ok());
    }
    #[test]
    fn reconstruction_matches_direct_cosines() {
        let p = 256;
        let n = 4;
        let xi = (0..9)
            .map(|j| Float::with_val(p, j + 1))
            .collect::<Vec<_>>();
        let l = Float::with_val(p, 13).ln();
        let x = decimal(".71", p).unwrap();
        let mut direct = xi[n].clone();
        for j in 1..=n {
            let mut phase = pi(p);
            phase *= 2 * j;
            phase *= &x;
            phase /= &l;
            let mut term = phase.cos();
            term *= Float::with_val(p, &xi[n + j] + &xi[n - j]);
            if j % 2 == 1 {
                term = -term;
            }
            direct += term;
        }
        direct /= l.clone().sqrt();
        let mut error = state_value(&xi, n, &x, &l, p);
        error -= direct;
        assert!(error.abs() < decimal("1e-70", p).unwrap());
    }
}
