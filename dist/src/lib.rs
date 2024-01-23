mod dpq;
mod nmath;
mod pnt;
mod pt;
mod rmath;

extern "C" {
    fn qnt(p: f64, df: f64, ncp: f64, lower_tail: i32, log_p: i32) -> f64;

    fn pnf(x: f64, df1: f64, df2: f64, ncp: f64, lower_tail: i32, log_p: i32) -> f64;
    fn qnf(p: f64, df1: f64, df2: f64, ncp: f64, lower_tail: i32, log_p: i32) -> f64;

    fn pnchisq(x: f64, df: f64, ncp: f64, lower_tail: i32, log_p: i32) -> f64;
    fn qnchisq(p: f64, df: f64, ncp: f64, lower_tail: i32, log_p: i32) -> f64;
}

pub trait Distribution {
    fn cdf(&self, x: f64, lower_tail: bool) -> f64;
    fn quantile(&self, x: f64, lower_tail: bool) -> f64;
    fn central_distribution(&self) -> Box<dyn Distribution>;
}

pub type Dist = Box<dyn Distribution>;

/// Implements the noncentral t-distribution with `v` degrees of freedom and
/// noncentrality parameter `lambda`.
#[derive(Clone)]
pub struct NoncentralT {
    v: f64,
    lambda: f64,
}

impl NoncentralT {
    pub fn new(v: f64, lambda: f64) -> Self {
        // println!("NoncentralT with v1: {v} and lambda: {lambda}");
        Self { v, lambda }
    }
}

impl Distribution for NoncentralT {
    fn cdf(&self, x: f64, lower_tail: bool) -> f64 {
        pnt::pnt(x, self.v, self.lambda, lower_tail, false)
    }
    fn quantile(&self, x: f64, lower_tail: bool) -> f64 {
        unsafe { qnt(x, self.v, self.lambda, lower_tail as i32, 0) }
    }
    fn central_distribution(&self) -> Dist {
        let mut clone = self.clone();
        clone.lambda = 0.0;
        Box::new(clone)
    }
}

/// Implements the noncentral F-distribution with `v1` and `v2` degrees of freedom and
/// noncentrality parameter `lambda`.
#[derive(Clone)]
pub struct NoncentralF {
    v1: f64,
    v2: f64,
    lambda: f64,
}

/// Returns a number lower bounded at 0.
///
/// By default, nmath returns NaN for negative distribution parameters.
/// Returning a number instead of a NaN makes it easier for root finding to find
/// a solution.
fn ensure_positive_non_zero(x: f64) -> f64 {
    if x <= 0.0 {
        1e-10f64
    } else {
        x
    }
}

impl NoncentralF {
    pub fn new(v1: f64, v2: f64, lambda: f64) -> Self {
        // println!("NoncentralF with v1: {v1}, v2: {v2}, and lambda: {lambda}");
        Self {
            v1: ensure_positive_non_zero(v1),
            v2: ensure_positive_non_zero(v2),
            lambda,
        }
    }
}

impl Distribution for NoncentralF {
    fn cdf(&self, x: f64, lower_tail: bool) -> f64 {
        unsafe { pnf(x, self.v1, self.v2, self.lambda, lower_tail as i32, 0) }
    }
    fn quantile(&self, x: f64, lower_tail: bool) -> f64 {
        unsafe { qnf(x, self.v1, self.v2, self.lambda, lower_tail as i32, 0) }
    }
    fn central_distribution(&self) -> Dist {
        let mut clone = self.clone();
        clone.lambda = 0.0;
        Box::new(clone)
    }
}

/// Implements the noncentral Chi-squared distribution with `v` degrees of freedom and
/// noncentrality parameter `lambda`.
#[derive(Clone)]
pub struct NoncentralChisq {
    v: f64,
    lambda: f64,
}

impl NoncentralChisq {
    pub fn new(v: f64, lambda: f64) -> Self {
        // println!("Noncentral Chisq with v: {v} and lambda: {lambda}");
        Self {
            v: ensure_positive_non_zero(v),
            lambda,
        }
    }
}

impl Distribution for NoncentralChisq {
    fn cdf(&self, x: f64, lower_tail: bool) -> f64 {
        unsafe { pnchisq(x, self.v, self.lambda, lower_tail as i32, 0) }
    }
    fn quantile(&self, x: f64, lower_tail: bool) -> f64 {
        unsafe { qnchisq(x, self.v, self.lambda, lower_tail as i32, 0) }
    }
    fn central_distribution(&self) -> Dist {
        let mut clone = self.clone();
        clone.lambda = 0.0;
        Box::new(clone)
    }
}

#[cfg(test)]
mod distributions {
    use super::*;

    #[test]
    fn outcome_matches_distributions_jl() {
        assert_eq!(
            NoncentralT {
                v: 0.5,
                lambda: 0.4
            }
            .cdf(0.3, true),
            0.4226402426934749
        );
        assert_eq!(
            NoncentralT {
                v: 0.5,
                lambda: 0.4
            }
            .quantile(0.3, true),
            -0.1924780204059502
        );

        assert_eq!(
            NoncentralF {
                v1: 0.4,
                v2: 0.3,
                lambda: 0.2
            }
            .cdf(0.1, true),
            0.2685519910190277
        );
        assert_eq!(
            NoncentralF {
                v1: 0.4,
                v2: 0.3,
                lambda: 0.2
            }
            .quantile(0.1, true),
            0.000702279780334189
        );
    }
}
