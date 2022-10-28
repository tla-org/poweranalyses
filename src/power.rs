use crate::dist::*;

use roots::*;

trait StatisticalTest {
    fn null_distribution(&self, es: f64, n: f64) -> Box<dyn Distribution>;
    fn alternative_distribution(&self, es: f64, n: f64) -> Box<dyn Distribution>;
    fn tail(&self) -> i32;
    fn power(&self, es: f64, alpha: f64, n: f64) -> f64 {
        let d0 = self.null_distribution(es, n);
        let d1 = self.alternative_distribution(es, n);
        let right_tail = if self.tail() == 1 { alpha } else { alpha / 2.0 };
        let critical_value = d0.quantile(right_tail, false);
        let beta = d1.cdf(critical_value, false);
        return beta;
    }
    fn alpha(&self, es: f64, power: f64, n: f64) -> f64 {
        let d0 = self.null_distribution(es, n);
        let d1 = self.alternative_distribution(es, n);
        let critical_value = d1.quantile(power, false);
        let right_tail = d0.cdf(critical_value, false);
        return if self.tail() == 1 { right_tail } else { 2.0 * right_tail };
    }
    fn es(&self, alpha: f64, power: f64, n: f64) -> f64 {
        let f = | es | { self.alpha(es, power, n) - alpha };
        let mut convergency = SimpleConvergency { eps: 1e-15f64, max_iter: 100 };
        return find_root_brent(0f64, 1000f64, &f, &mut convergency).unwrap();
    }
    fn n(&self, alpha: f64, power: f64, es: f64) -> f64 {
        let f = | n | { self.alpha(es, power, n) - alpha };
        let mut convergency = SimpleConvergency { eps: 1e-15f64, max_iter: 100 };
        return find_root_brent(-1000f64, 1000f64, &f, &mut convergency).unwrap();
    }
}

struct OneSampleTTest {
    tail: i32
}

impl OneSampleTTest {
    pub fn new(tail: i32) -> Self {
        return Self{ tail };
    }
}

impl StatisticalTest for OneSampleTTest {
    fn null_distribution(&self, es: f64, n: f64) -> Box<dyn Distribution> {
        return Box::new(NoncentralT{ v: n - 1.0, lambda: 0.0 })
    }
    fn alternative_distribution(&self, es: f64, n: f64) -> Box<dyn Distribution> {
        return Box::new(NoncentralT{ v: n - 1.0, lambda: n.sqrt() * es });
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
        assert_eq!(OneSampleTTest::new(2).alpha(es, power, n), 0.06731683009994659);
        assert_eq!(OneSampleTTest::new(1).alpha(es, power, n), 0.03365841504997329);
        assert_eq!(OneSampleTTest::new(2).power(es, alpha, n), 0.9338975528614741);
        assert_eq!(OneSampleTTest::new(1).power(es, alpha, n), 0.9672067458263426);

        assert_eq!(OneSampleTTest::new(2).es(alpha, power, n), 0.5201211596125199);
        assert_eq!(OneSampleTTest::new(1).es(alpha, power, n), 0.4718256927232961);
        // assert_eq!(OneSampleTTest::new(2).n(alpha, power, es), 0.5201211596125199);
    }
}

