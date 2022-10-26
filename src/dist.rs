pub trait Distribution {
    // See Wikipedia for the definitions.
    fn cdf(&self, x: f64) -> f64;
}

const PI: f64 = 3.141592653589793;

struct NoncentralT {
    v: f64,
    mu: f64
}

impl Distribution for NoncentralT {
    fn cdf(&self, _x: f64) -> f64 {
        return 1.0;
    }
}

#[cfg(test)]
mod distributions {
    // use super::*;

    #[test]
    fn it_gives_cdf() {
        // assert_eq!(NoncentralT{v: 0.5, mu: 0.4}.cdf(0.3), 0.4226402426934749);
    }
}
