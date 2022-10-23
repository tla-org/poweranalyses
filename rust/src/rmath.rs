/// Functions here are based on math functions from base R.
/// The benefits of porting are that it:
/// - is more fun than to fiddle with C includes,
/// - is an opportunity for learning,
/// - is more flexible when building to different platforms.

const DBL_EPSILON: f64 = 2.220446049250313e-16; // 2^-52
const M_LOG10_2: f64 = 0.3010299956639812; // log(10, 2)
const XMAX: f64 = 2.5327372760800758e305;

fn fmod(a: f64, b: f64) -> f64 {
    return a % b;
}

/// https://github.com/wch/r-source/blob/trunk/src/nmath/lgamma.c
fn lgammafn(x: f64) -> f64 {
    let mut sgn: i32 = 1;

    if x == f64::NAN {
        return x;
    }

    if x < 0. && fmod(f64::floor(-x), 2.) == 0. {
        sgn = -1;
    }

    1.0
}

fn pbeta(x: f64, a: f64, b: f64) -> f64 {
    1.0
}

fn pnorm(x: f64) -> f64 {
    1.0
}

fn pnt(t: f64, df: f64, ncp: f64, lower_tail: i32, log_p: i32) -> f64 {
    1.0
}

#[cfg(test)]
mod rmath_tests {
    extern crate approx;

    use super::*;

    #[test]
    fn it_gives_remainder() {
        approx::assert_ulps_eq!(1.8, fmod(9.2, 3.7), max_ulps = 6);
    }
}
