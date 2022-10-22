/// Functions here are based on math functions from base R.

const DBL_EPSILON: f64 = 2.220446049250313e-16; // 2^-52
const M_LOG10_2: f64 = 0.3010299956639812; // log(10, 2)
const XMAX: f64 = 2.5327372760800758e305;

/// https://github.com/wch/r-source/blob/trunk/src/nmath/lgamma.c
fn lgammafn(x: f64) -> f64 {
    let sgn: i32 = 1;
    
    if x == f64::NAN {
        return x;
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
