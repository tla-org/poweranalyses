use crate::dist::*;

trait StatisticalTest {
    fn null_distribution(&self) -> Box<dyn Distribution>;
    fn alternative_distribution(&self) -> Box<dyn Distribution>;
    fn tail(&self) -> i32;
    fn power(&self, alpha: f64) -> f64 {
        let d0 = self.null_distribution();
        let d1 = self.alternative_distribution();
        let right_tail = if self.tail() == 1 { alpha } else { alpha / 2.0 };
        let critical_value = d0.quantile(right_tail, false);
        let beta = d1.cdf(critical_value, false);
        return beta;
    }
    fn alpha(&self, power: f64) -> f64 {
        let d0 = self.null_distribution();
        let d1 = self.alternative_distribution();
        let critical_value = d1.quantile(power, false);
        let right_tail = d0.cdf(critical_value, false);
        return if self.tail() == 1 { right_tail } else { 2.0 * right_tail };
    }
}

struct OneSampleTTest {
    tail: i32,
    n: f64,
    es: f64,
    v: f64,
    lambda: f64
}

impl OneSampleTTest {
    pub fn new(tail: i32, n: f64, es: f64) -> Self {
        return Self{ tail, n, es, v: n - 1.0, lambda: n.sqrt() * es };
    }
}

impl StatisticalTest for OneSampleTTest {
    fn null_distribution(&self) -> Box<dyn Distribution> {
        return Box::new(NoncentralT{ v: self.v, lambda: 0.0 });
    }
    fn alternative_distribution(&self) -> Box<dyn Distribution> {
        return Box::new(NoncentralT{ v: self.v, lambda: self.lambda });
    }
    fn tail(&self) -> i32 {
        return self.tail;
    }
}

struct IndependentSamplesTTest {
    tail: i32,
    n: f64,
    es: f64,
    v: f64,
    lambda: f64
}

impl IndependentSamplesTTest {
    pub fn new(tail: i32, n: f64, es: f64) -> Self {
        return Self{ tail, n, es, v: n - 2.0, lambda: (n / 2.0).sqrt() * es };
    }
}

impl StatisticalTest for IndependentSamplesTTest {
    fn null_distribution(&self) -> Box<dyn Distribution> {
        return Box::new(NoncentralT{ v: self.v, lambda: 0.0 });
    }
    fn alternative_distribution(&self) -> Box<dyn Distribution> {
        return Box::new(NoncentralT{ v: self.v, lambda: self.lambda });
    }
    fn tail(&self) -> i32 {
        return self.tail;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let es = 0.5;
        let alpha = 0.05;
        let power = 0.95;
        let n = 50.0;
        assert_eq!(OneSampleTTest::new(1, n, es).power(alpha), 0.9672067458263426);
        assert_eq!(OneSampleTTest::new(2, n, es).power(alpha), 0.9338975528614741);
        assert_eq!(OneSampleTTest::new(1, n, es).alpha(power), 0.03365841504997329);
        assert_eq!(OneSampleTTest::new(2, n, es).alpha(power), 0.06731683009994659);
    }
}

