extern {
    fn pnt(t: f64, df: f64, ncp: f64, lower_tail: i32, log_p: i32) -> f64;
    fn qnt(p: f64, df: f64, ncp: f64, lower_tail: i32, log_p: i32) -> f64;
}


pub trait Distribution {
    fn cdf(&self, x: f64, lower_tail: bool) -> f64;
    fn quantile(&self, x: f64, lower_tail: bool) -> f64;
}

/// Implements the noncentral t-distribution with `v` degrees of freedom and
/// noncentrality parameter `lambda`.
pub struct NoncentralT {
    v: f64,
    lambda: f64
}

impl NoncentralT {
    pub fn new(v: f64, lambda: f64) -> Self {
        return Self{ v, lambda };
    }
}

impl Distribution for NoncentralT {
    fn cdf(&self, x: f64, lower_tail: bool) -> f64 {
        return unsafe { pnt(x, self.v, self.lambda, lower_tail as i32, 0) };
    }

    fn quantile(&self, x: f64, lower_tail: bool) -> f64 {
        return unsafe { qnt(x, self.v, self.lambda, lower_tail as i32, 0) };
    }
}

#[cfg(test)]
mod distributions {
    use super::*;

    #[test]
    fn it_gives_cdf() {
        // Compared against Distributions.jl.
        assert_eq!(NoncentralT{v: 0.5, lambda: 0.4}.cdf(0.3, true), 0.4226402426934749);
        assert_eq!(NoncentralT{v: 0.5, lambda: 0.4}.quantile(0.3, true), -0.1924780204059502);

    }
}
