use crate::dist::*;

use roots::*;

struct AlphaArgs {
    tail: i32,
    n: f64,
    power: f64,
    es: f64
}

enum TestKind {
    OneSampleTTest,
    DeviationFromZeroMultipleRegression {
        n_predictors: i32
    }
}

impl TestKind {
    fn null_distribution(&self, n: f64, _es: f64) -> Box<dyn Distribution> {
        match self {
            TestKind::OneSampleTTest => Box::new(NoncentralT{ v: n - 1.0, lambda: 0.0 }),
            TestKind::DeviationFromZeroMultipleRegression { n_predictors } => {
                Box::new(NoncentralF::new(
                    *n_predictors as f64,
                    (n as f64) - (*n_predictors as f64) - (1 as f64),
                    0.0
                ))
            }
        }
    }

    fn alternative_distribution(&self, n: f64, es: f64) -> Box<dyn Distribution> {
        match self {
            TestKind::OneSampleTTest => {
                Box::new(NoncentralT{ v: n - 1.0, lambda: n.sqrt() * es })
            },
            TestKind::DeviationFromZeroMultipleRegression { n_predictors } => {
                Box::new(NoncentralF::new(
                    *n_predictors as f64,
                    (n as f64) - (*n_predictors as f64) - (1 as f64),
                    es.powi(2) * (n as f64)
                ))
            }
        }
    }

    fn n(&self, tail: i32, alpha: f64, power: f64, es: f64) -> i64 {
        let f = | n | { self.alpha(AlphaArgs { tail, n, power, es }) - alpha };
        let mut convergency = SimpleConvergency { eps: 0.00001f64, max_iter: 40 };
        return match find_root_brent(2f64, 10000f64, &f, &mut convergency) {
            Ok(number) => number.round() as i64,
            Err(_) => -111
        };
    }

    fn alpha(&self, args: AlphaArgs) -> f64 {
        let d0 = self.null_distribution(args.n, args.es);
        let d1 = self.alternative_distribution(args.n, args.es);
        let critical_value = d1.quantile(args.power, false);
        let right_tail = d0.cdf(critical_value, false);
        return if args.tail == 1 { right_tail } else { 2.0 * right_tail };
    }

    fn power(&self, tail: i32, n: f64, alpha: f64, es: f64) -> f64 {
        let d0 = self.null_distribution(n, es);
        let d1 = self.alternative_distribution(n, es);
        let right_tail = if tail == 1 { alpha } else { alpha / 2.0 };
        let critical_value = d0.quantile(right_tail, false);
        return d1.cdf(critical_value, false);
    }

    fn es(&self, tail: i32, n: f64, alpha: f64, power: f64) -> f64 {
        let f = | es | { self.alpha(AlphaArgs { tail, n, power, es }) - alpha };
        let mut convergency = SimpleConvergency { eps: 0.00001f64, max_iter: 40 };
        return match find_root_brent(0f64, 1000f64, &f, &mut convergency) {
            Ok(number) => number,
            Err(_) => -111.0
        };
    }
}

fn round(x: f64, decimals: u32) -> f64 {
    let factor = i32::checked_pow(10, decimals);
    return match factor {
        Some(number) => (x * number as f64).round() / number as f64,
        None => x
    };
}

#[no_mangle]
pub extern fn oneSampleTTestN(tail: i32, alpha: f64, power: f64, es: f64) -> i64 {
    return TestKind::OneSampleTTest.n(tail, alpha, power, es);
}
#[no_mangle]
pub extern fn oneSampleTTestAlpha(tail: i32, n: f64, power: f64, es: f64) -> f64 {
    return round(TestKind::OneSampleTTest.alpha(AlphaArgs { tail, n, power, es }), 3);
}
#[no_mangle]
pub extern fn oneSampleTTestPower(tail: i32, n: f64, alpha: f64, es: f64) -> f64 {
    return round(TestKind::OneSampleTTest.power(tail, n, alpha, es), 3);
}
#[no_mangle]
pub extern fn oneSampleTTestES(tail: i32, n: f64, alpha: f64, power: f64) -> f64 {
    return round(TestKind::OneSampleTTest.es(tail, n, alpha, power), 3);
}

#[no_mangle]
pub extern fn deviationFromZeroMultipleRegressionN(n_predictors: i32, alpha: f64, power: f64, es: f64) -> i64 {
    let tail = 1;
    let test = TestKind::DeviationFromZeroMultipleRegression{ n_predictors };
    test.n(tail, alpha, power, es)
}
#[no_mangle]
pub extern fn deviationFromZeroMultipleRegressionAlpha(n_predictors: i32, n: f64, power: f64, es: f64) -> f64 {
    let test = TestKind::DeviationFromZeroMultipleRegression{ n_predictors };
    round(test.alpha(AlphaArgs { tail: 1, n, power, es }), 3)
}
#[no_mangle]
pub extern fn deviationFromZeroMultipleRegressionPower(n_predictors: i32, n: f64, alpha: f64, es: f64) -> f64 {
    let tail = 1;
    let test = TestKind::DeviationFromZeroMultipleRegression{ n_predictors };
    round(test.power(tail, n, alpha, es), 3)
}
#[no_mangle]
pub extern fn deviationFromZeroMultipleRegressionES(n_predictors: i32, n: f64, alpha: f64, power: f64) -> f64 {
    let tail = 1;
    let test = TestKind::DeviationFromZeroMultipleRegression{ n_predictors };
    round(test.es(tail, n, alpha, power), 3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(round(1.234, 2), 1.23);
        // Output obtained from G*Power via VirtualBox IE11 Windows 7.
        let es = 0.5;
        let alpha = 0.05;
        let power = 0.95;
        let n = 50.0;
        assert_eq!(oneSampleTTestAlpha(2, n, power, es), 0.067);
        assert_eq!(oneSampleTTestAlpha(1, n, power, es), 0.034);
        assert_eq!(oneSampleTTestPower(2, n, alpha, es), 0.934);
        assert_eq!(oneSampleTTestPower(1, n, alpha, es), 0.967);

        assert_eq!(oneSampleTTestES(2, n, alpha, power), 0.520);
        assert_eq!(oneSampleTTestES(1, n, alpha, power), 0.472);
        assert_eq!(oneSampleTTestES(1, 0.99, power, es), -111.0);
        assert_eq!(oneSampleTTestN(2, alpha, power, es), 54);
        assert_eq!(oneSampleTTestN(1, alpha, power, es), 45);
        assert_eq!(oneSampleTTestN(1, 0.99, power, es), -111);

        // assert_eq!(deviationFromZeroMultipleRegressionN(2, alpha, power, es), 35);
        let f_squared = es.sqrt();
        assert_eq!(deviationFromZeroMultipleRegressionAlpha(2, n, power, f_squared), 0.006);
    }
}

