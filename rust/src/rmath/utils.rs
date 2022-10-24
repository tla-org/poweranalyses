use statrs::distribution::Normal;
use statrs::distribution::ContinuousCDF;
use statrs::function::gamma::ln_gamma;
use statrs::function::beta::beta_reg;
use statrs::distribution::StudentsT;

pub const M_SQRT_2DPI: f64 = 0.7978845608028654; // sqrt(2 / π
pub const M_LN_SQRT_PI: f64 = 0.5723649429247001; // log(π) / 2

pub fn fmod(a: f64, b: f64) -> f64 {
    return a % b;
}

pub fn lgammafn(x: f64) -> f64 {
    return ln_gamma(x);
}

/// Incomplete beta function.
pub fn pbeta(x: f64, a: f64, b: f64) -> f64 {
    return beta_reg(a, b, x);
}

/// Normal (cumulative) distribution function.
pub fn pnorm(x: f64, mean: f64, sd: f64) -> f64 {
    let unit_normal = Normal::new(mean, sd).unwrap();
    return unit_normal.cdf(x);
}

/// Cumulative probability of the t-distribution.
/// The tail and log_p are omitted because they are probably not necessary.
/// Julia doesn't have them either.
pub fn pt(x: f64, df: f64) -> f64 {
    let dist = StudentsT::new(0.0, 1.0, df).unwrap();
    return dist.cdf(x);
}

pub const DBL_EPSILON: f64 = 2.220446049250313e-16;

// Assuming lower_tail and !log_p.
// https://github.com/wch/r-source/blob/trunk/src/nmath/dpq.h
// The lower_tail is probably meant to avoid losing precision caused by 1 - x.
// For the cdf, this is also called the "survival function" as is also implmented
// in boost and scipy according to https://github.com/statrs-dev/statrs/pull/172.
pub const R_DT_0: f64 = 0.0;
pub const R_DT_1: f64 = 1.0;
pub const M_LN2: f64 = 0.693147180559945309417232121458; // ln(2)
pub const DBL_MIN_EXP: f64 = -1022.0;

// Assuming lower_tail and !log_p.
pub fn r_dt_val(x: f64) -> f64 {
    return x;
}

pub fn fmin2(x: f64, y: f64) -> f64 {
    return f64::min(x, y);
}

pub fn fmax2(x: f64, y: f64) -> f64 {
    return f64::max(x, y);
}

pub fn qt(p: f64, df: f64) -> f64 {
    let dist = StudentsT::new(0.0, 1.0, df).unwrap();
    return dist.inverse_cdf(p);
}

// Based on https://github.com/wch/r-source/blob/trunk/src/nmath/dpq.h.
pub fn r_q_p01_boundaries(p: f64) -> f64 {
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

pub fn r_dt_qiv(p: f64) -> f64 {
    return p; // since r_d_lval(p) == p
}

#[cfg(test)]
mod rmath_utils_tests {
    extern crate approx;
    use approx::assert_ulps_eq;

    use super::*;

    #[test]
    fn that_utils_are_correct() {
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
    }
}
