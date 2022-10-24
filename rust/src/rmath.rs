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

use log::warn;

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

const DBL_EPSILON: f64 = 2.220446049250313e-16;

// Assuming lower_tail and !log_p.
// https://github.com/wch/r-source/blob/trunk/src/nmath/dpq.h
// The lower_tail is probably meant to avoid losing precision caused by 1 - x.
// For the cdf, this is also called the "survival function" as is also implmented
// in boost and scipy according to https://github.com/statrs-dev/statrs/pull/172.
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

fn fmax2(x: f64, y: f64) -> f64 {
    return f64::max(x, y);
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
    if ncp == 0.0 {
        return pt(t, df);
    }

    if !t.is_finite() {
        if t < 0.0 {
            return 0.0;
        } else {
            return 1.0;
        }
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
    let mut rxb = df / (x + df);
    x = x / (x + df);

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
        return f64::NAN;
    } else {
        let tnc = 0.0;
        return pnt_finis(tnc, del, negdel);
    }
}

fn qt(p: f64, df: f64) -> f64 {
    let dist = StudentsT::new(0.0, 1.0, df).unwrap();
    return dist.inverse_cdf(p);
}

// Based on https://github.com/wch/r-source/blob/trunk/src/nmath/dpq.h.
fn r_q_p01_boundaries(p: f64) -> f64 {
    if p < 0.0 || p > 1.0 {
        return f64::NAN;
    }
    if p == 0.0 {
        let left = -f64::INFINITY;
        return left;
    }
    if p == 1.0 {
        let right = f64::INFINITY;
        return right;
    }
    return p;
}

fn r_dt_qiv(p: f64) -> f64 {
    return p; // since r_d_lval(p) == p
}

/// Quantile for noncentral t-distribution.
/// Based on https://github.com/wch/r-source/blob/trunk/src/nmath/qnt.c
/// Staying as close to the original code as possible to avoid bugs.
fn qnt(p: f64, df: f64, ncp: f64) -> f64 {
    let accu: f64 = 1e-13;
    let eps: f64 = 1e-11;

    if p.is_nan() || df.is_nan() || ncp.is_nan() {
        return f64::NAN;
    }

    if df <= 0.0 {
        warn!("Warning! df <= 0.0");
        return f64::NAN;
    }

    if ncp == 0.0 && df >= 1.0 {
        return qt(p, df);
    }

    let boundaries_response = r_q_p01_boundaries(p);
    if boundaries_response.is_infinite() {
        return boundaries_response;
    }

    // We skip an approximation for df == Inf here.
    // Saves implementing the noncentral quantile for the normal distribution.

    // p = R_DT_qIv(p) == p.

    // Invert pnt(.) :
    // 1. finding an upper and lower bound
    if p > 1.0 - DBL_EPSILON { return f64::INFINITY; };
    let mut pp = fmin2(1.0 - DBL_EPSILON, p * (1.0 + eps));
    let mut ux = fmax2(1.0, ncp);
    while ux < f64::MAX && pnt(ux, df, ncp) < pp {
        ux *= 2.0;
    }
    pp = p * (1.0 - eps);
    let mut lx = fmin2(-1.0, -ncp);
    while lx > -f64::MAX && pnt(lx, df, ncp) > pp {
        lx *= 2.0;
        println!("lx: {:?}, df: {:?}, ncp: {:?}", lx, df, ncp);
        println!("pnt: {}", pnt(lx, df, ncp));
        return 0.0;
    }

    // 2. interval (lx, ux) halving :
    //
    // do (1st iteration of do-while.)
    let mut nx = 0.5 * (lx + ux);
    if pnt(nx, df, ncp) > p {
        ux = nx;
    } else {
        lx = nx;
    }

    // while
    while (ux - lx) > accu * fmax2(lx.abs(), ux.abs()) {
        // do (2nd iteration of do-while.)
        nx = 0.5 * (lx + ux);
        if pnt(nx, df, ncp) > p {
            ux = nx;
        } else {
            lx = nx;
        }
    }

    return 0.5 * (lx + ux);
}

#[cfg(test)]
mod rmath_tests {
    extern crate approx;
    use approx::assert_ulps_eq;

    use super::*;

    #[test]
    fn that_pnt_is_correct() {
        assert_ulps_eq!(1.8, fmod(9.2, 3.7), max_ulps = 6);

        assert_eq!(0.9986501019684255, pnorm(3.0, 0.0, 1.0));

        // cdf(TDist(0.3), 0.2)
        assert_eq!(pt(0.2, 0.3), 0.544619617595772);

        // R> pbeta(0.3, 3, 1)
        // [1] 0.027
        assert_ulps_eq!(pbeta(0.3, 3.0, 1.0), 0.027);
        // R> pbeta(0.6, 2, 1)
        // [1] 0.36
        assert_ulps_eq!(pbeta(0.6, 2.0, 1.0), 0.36, max_ulps = 80);

        // R> pt(2.0095, 49.0, 0)
        // [1] 0.9749959
        assert_eq!(pnt(2.0095, 49.0, 0.0), 0.9749958761700477);

        // R> pt(Inf, 49.0, 2.0)
        // [1] 1
        assert_eq!(pnt(f64::INFINITY, 49.0, 2.0), 1.0);

        // R> pt(-Inf, 49.0, 2.0)
        // [1] 0
        assert_eq!(pnt(-f64::INFINITY, 49.0, 2.0), 0.0);

        // R> pt(2.0095, 5e5, 2)
        // [1] 0.5037895
        assert_eq!(pnt(2.0095, 5e5, 2.0), 0.5037818943498734);

        // julia> cdf(NoncentralT(49.0, 3.5355), 2.0095)
        // 0.0660970064371808
        // R> pt(2.0095, 49.0, 3.5355)
        // [1] 0.06609701
        let t = 2.0095;
        let df = 49.0;
        let ncp = 3.5355;
        assert_eq!(pnt(t, df, ncp), 0.0660970064372871);

        // R> pt(-4.46, 11.0, 2.23)
        // [1] 1.584881e-07
        assert_eq!(pnt(-4.46, 11.0, 2.23), 1.584881e-07);
    }

    #[test]
    fn that_qnt_is_correct() {
        // R> qt(0.13, 10)
        // [1] -1.194086
        assert_eq!(qt(0.13, 10.0), -1.194085555341413);

        // R> qt(0, 10)
        // [1] -Inf
        assert_eq!(qt(0.0, 10.0), -f64::INFINITY);

        // R> qt(NaN, 1, 2)
        // [1] NaN
        assert!(qnt(f64::NAN, 1.0, 2.0).is_nan());

        assert!(qnt(1.0, -1.0, 2.0).is_nan());

        // R> qt(0.54, Inf, 12.0)
        // [1] 12.10043
        assert_eq!(qnt(0.54, f64::INFINITY, 12.0), 12.100433720511148);

        // > qt(0.54, 11, 2.23)
        // [1] 2.40025
        assert_eq!(qnt(0.54, 11.0, 2.23), 2.40025);
    }
}
