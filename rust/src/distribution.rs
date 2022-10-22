trait Distribution {
    // See Wikipedia for the definitions.
    fn pdf(&self, x: f64) -> f64;
}

const PI: f64 = 3.141592653589793;

struct Normal {
    mu: f64,
    sigma: f64
}

impl Distribution for Normal {
    fn pdf(&self, x: f64) -> f64 {
        let frac: f64 = 1.0 / (self.sigma * (2.0 * PI).sqrt());
        let expable: f64 = -0.5 * ((x - self.mu) / self.sigma).powf(2.0);
        return frac * expable.exp();
    }
}

#[cfg(test)]
mod distributions {
    use super::*;

    #[test]
    fn it_gives_pdf() {
        let normal = Normal{mu: 1.0, sigma: 2.0};
        assert_eq!(normal.pdf(2.2), 0.16661230144589984);
    }
}
