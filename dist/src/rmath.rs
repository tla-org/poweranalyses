#![allow(non_upper_case_globals)]

pub const M_LN2: f64 = 0.693147180559945309417232121458; /* ln(2) */
pub const M_SQRT_2dPI: f64 = 0.797884560802865355879892119869; /* sqrt(2/pi) */
pub const M_LN_SQRT_PI: f64 = 0.572364942924700087071713675677; /* log(sqrt(pi)) */

extern "C" {
    fn pnorm5(x: f64, mu: f64, sigma: f64, lower_tail: i32, log_p: i32) -> f64;
}

pub fn pnorm(x: f64, mu: f64, sigma: f64, lower_tail: bool, log_p: bool) -> f64 {
    unsafe { pnorm5(x, mu, sigma, lower_tail as i32, log_p as i32) }
}
