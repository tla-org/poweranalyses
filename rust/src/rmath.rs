/// Functions here are based on math functions from base R.
/// The benefits of porting are that it is:
/// - more fun than to fiddle with C includes,
/// - is an opportunity for learning,
/// - is more flexible when building to different platforms.
///
/// The reason that these functions are not often distributed is
/// that noncentral distributions are mostly useful for power analyses.

use libm::expm1;
use libm::log;

use statrs::distribution::Normal;
use statrs::distribution::ContinuousCDF;
use statrs::function::gamma::ln_gamma;
use statrs::function::beta::beta_reg;
use statrs::distribution::StudentsT;

const M_SQRT_2DPI: f64 = 0.7978845608028654; // sqrt(2 / π
const M_LN_SQRT_PI: f64 = 0.5723649429247001; // log(π) / 2

fn fmod(a: f64, b: f64) -> f64 {
    return a % b;
}

fn lgammafn(x: f64) -> f64 {
    return ln_gamma(x);
}

/// Incomplete beta function.
fn pbeta(x: f64, a: f64, b: f64) -> f64 {
    return beta_reg(a, b, x);
}

/// Normal (cumulative) distribution function.
fn pnorm(x: f64, mean: f64, sd: f64) -> f64 {
    let unit_normal = Normal::new(mean, sd).unwrap();
    return unit_normal.cdf(x);
}

/// Cumulative probability of the t-distribution.
/// The tail and log_p are omitted because they are probably not necessary.
/// Julia doesn't have them either.
fn pt(x: f64, df: f64) -> f64 {
    let dist = StudentsT::new(0.0, 1.0, df).unwrap();
    return dist.cdf(x);
}

fn isfinite(x: f64) -> bool {
    x != f64::NAN && x != f64::INFINITY && x != -f64::INFINITY
}

const DBL_EPSILON: f64 = 2.220446049250313e-16;

// Assuming lower_tail and !log_p.
// https://github.com/wch/r-source/blob/trunk/src/nmath/dpq.h
const R_DT_0: f64 = 0.0;
const R_DT_1: f64 = 1.0;
const M_LN2: f64 = 0.693147180559945309417232121458; // ln(2)
const DBL_MIN_EXP: f64 = -1022.0;

// Assuming lower_tail and !log_p.
fn r_dt_val(x: f64) -> f64 {
    return x;
}

fn fmin2(x: f64, y: f64) -> f64 {
    return f64::min(x, y);
}

fn pnt_finis(tnc: f64, del: f64, negdel: bool) -> f64 {
    let tnc2 = tnc + pnorm(-del, 0.0, 1.0);
    let lower_tail = !negdel;
    assert!(!(tnc2 > 1.0 - 1e-10 && lower_tail));
    return r_dt_val(fmin2(tnc2, 1.0));
}

/// Cumulative probability at t of the non-central t-distribution with
/// df degrees of freedom and non-centrality parameter delta.
/// Based on https://github.com/wch/r-source/blob/trunk/src/nmath/pnt.c
fn pnt(t: f64, df: f64, ncp: f64) -> f64 {
    let itrmax: i32 = 1000;
    let errmax: f64 = 1.0e-12;

    assert!(0.0 < df);
    if ncp == 0.0 { return pt(t, df); }
    if !isfinite(t) {
        if t < 0.0 { return 0.0 } else { return 1.0 }
    }

    let negdel: bool;
    let tt: f64;
    let del: f64;

    if t >= 0.0 {
        negdel = false;
        tt = t;
        del = ncp;
    } else {
        negdel = true;
        tt = -t;
        del = -ncp;
    }

    let mut s: f64;

    if df > 4e5 || del * del > 2.0 * M_LN2 * -DBL_MIN_EXP {
        s = 1.0 / (4.0 * df);

        let x = tt * (1.0 - s);
        let mean = del;
        let sd = 1.0 + (tt * tt * 2.0 * s).sqrt();
        let cumulative_density = pnorm(x, mean, sd);
        if negdel {
            return 1.0 - cumulative_density;
        } else {
            return cumulative_density;
        }
    }

    let mut x = t * t;
    x = x / (x + df);
    let mut rxb = df / (x + df);

    let lambda: f64;
    let mut p: f64;
    if x > 0.0 {
        lambda = del * del;
        p = 0.5 * (-0.5 * lambda).exp();

        // Original code warns here.
        assert!(p != 0.0);

        let mut q = M_SQRT_2DPI * p * del;
        s = 0.5 - p;

        if s < 1.0e-7 {
            s = -0.5 * expm1(-0.5 * lambda);
        }
        let mut a = 0.5;
        let b = 0.5 * df;
        rxb = f64::powf(rxb, b);
        let albeta = M_LN_SQRT_PI + ln_gamma(b) - ln_gamma(0.5 + b);
        let mut xodd = pbeta(x, a, b);
        let mut godd = 2.0 * rxb * (a * log(x) - albeta).exp();
        let mut tnc = b * x;
        let mut xeven = if tnc < DBL_EPSILON { tnc } else { 1.0 - rxb };
        let mut geven = tnc * rxb;
        tnc = p * xodd + q * xeven;

        for it in 1..itrmax {
            a += 1.0;
            xodd -= godd;
            xeven -= geven;
            godd *= x * (a + b - 1.0) / a;
            geven *= x * (a + b - 0.5) / (a + 0.5);
            p *= lambda / (2 * it) as f64;
            q *= lambda / (2 * it + 1) as f64;
            tnc += p * xodd + q * xeven;
            s -= p;
            // Warns in the original implementation.
            assert!(!(s < -1.0e-10));
            if s <= 0.0 && it > 1 {
                return pnt_finis(tnc, del, negdel);
            }
            let errbd = 2.0 * s * (xodd - godd);
            if errbd.abs() < errmax {
                return pnt_finis(tnc, del, negdel);
            }
        }
        assert!(false);
    }

    1.
}

#[cfg(test)]
mod rmath_tests {
    extern crate approx;
    use approx::assert_ulps_eq;

    use super::*;

    #[test]
    fn equalities() {
        assert_ulps_eq!(1.8, fmod(9.2, 3.7), max_ulps = 6);

        assert_eq!(0.9986501019684255, pnorm(3.0, 0.0, 1.0));

        assert!(!isfinite(f64::INFINITY));

        // cdf(TDist(0.3), 0.2)
        assert_eq!(pt(0.2, 0.3), 0.544619617595772);

        // R> pbeta(0.3, 3, 1)
        // [1] 0.027
        assert_ulps_eq!(pbeta(0.3, 3.0, 1.0), 0.027);
        // R> pbeta(0.6, 2, 1)
        // [1] 0.36
        assert_ulps_eq!(pbeta(0.6, 2.0, 1.0), 0.36, max_ulps = 80);

        // julia> cdf(NoncentralT(49.0, 3.5355), 2.0095)
        // 0.0660970064371808
        // R> pt(2.0095, 49.0, 3.5355)
        // [1] 0.06609701
        let t = 2.0095;
        let df = 49.0;
        let ncp = 3.5355;
        assert_eq!(pnt(t, df, ncp), 0.0660970064371808);
    }
}
