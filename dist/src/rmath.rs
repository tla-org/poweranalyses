#![allow(non_upper_case_globals)]

extern "C" {
    fn Rlog1p(x: f64) -> f64;
    fn pnorm5(x: f64, mu: f64, sigma: f64, lower_tail: i32, log_p: i32) -> f64;
}

pub fn log1p(x: f64) -> f64 {
    unsafe { Rlog1p(x) }
}

pub const M_LN2: f64 = std::f64::consts::LN_2;
pub const M_SQRT_2dPI: f64 = 0.797_884_560_802_865_4; /* sqrt(2/pi) */
pub const M_LN_SQRT_PI: f64 = 0.572_364_942_924_700_1; /* log(sqrt(pi)) */

pub fn pnorm(x: f64, mu: f64, sigma: f64, lower_tail: bool, log_p: bool) -> f64 {
    unsafe { pnorm5(x, mu, sigma, lower_tail as i32, log_p as i32) }
}

pub fn pbeta(x: f64, a: f64, b: f64, lower_tail: bool, log_p: bool) -> f64 {
    extern "C" {
        fn pbeta(x: f64, a: f64, b: f64, lower_tail: i32, log_p: i32) -> f64;
    }
    unsafe { pbeta(x, a, b, lower_tail as i32, log_p as i32) }
}

#[cfg(test)]
mod test_rmath {
    use super::*;

    #[test]
    fn outcome_matches_r() {
        assert_eq!(log1p(0.2), 0.18232155679395465);
        assert_eq!(log1p(0.95), 0.6678293725756554);
    }
}
