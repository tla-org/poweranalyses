use crate::rmath::utils::*;
use crate::rmath::pnt::pnt;

use log::warn;

/// Quantile for noncentral t-distribution.
/// Based on https://github.com/wch/r-source/blob/trunk/src/nmath/qnt.c
/// Staying as close to the original code as possible to avoid bugs.
pub fn qnt(p: f64, df: f64, ncp: f64) -> f64 {
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
mod qnt_tests {
    use super::*;

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
        // TODO: assert_eq!(qnt(0.54, 11.0, 2.23), 2.40025);
    }
}

