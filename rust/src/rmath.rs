/// Functions here are based on math functions from base R.
/// The benefits of porting are that it:
/// - is more fun than to fiddle with C includes,
/// - is an opportunity for learning,
/// - is more flexible when building to different platforms.

use statrs::distribution::Normal;
use statrs::distribution::ContinuousCDF;
use statrs::function::gamma::ln_gamma;
use statrs::function::beta::beta_inc;

const DBL_EPSILON: f64 = 2.220446049250313e-16; // 2^-52
const M_LOG10_2: f64 = 0.3010299956639812; // log(10, 2)
const XMAX: f64 = 2.5327372760800758e305;

fn fmod(a: f64, b: f64) -> f64 {
    return a % b;
}

fn lgammafn(x: f64) -> f64 {
    return ln_gamma(x);
}

/// Incomplete beta function.
fn pbeta(x: f64, a: f64, b: f64) -> f64 {
    // Not completely sure, maybe checked_beta_inc or something.
    return beta_inc(x, a, b);
}

/// Normal (cumulative) distribution function.
fn pnorm(x: f64) -> f64 {
    let unit_normal = Normal::new(0.0, 1.0).unwrap();
    return unit_normal.cdf(x);
}

fn pnt(t: f64, df: f64, ncp: f64, lower_tail: i32, log_p: i32) -> f64 {
    1.0
}

#[cfg(test)]
mod rmath_tests {
    extern crate approx;
    use approx::assert_ulps_eq;

    use super::*;

    #[test]
    fn equalities() {
        assert_ulps_eq!(1.8, fmod(9.2, 3.7), max_ulps = 6);

        assert_eq!(0.9986501019684255, pnorm(3.));
    }
}
