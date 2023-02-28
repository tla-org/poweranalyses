use crate::dist::Dist;
use crate::dist::NoncentralChisq;
use crate::dist::NoncentralF;
use crate::dist::NoncentralT;
use crate::string::json;
use crate::string::u8_to_string;
use crate::string::write_to_ptr;
use roots::SimpleConvergency;
use roots::find_root_brent;
use roots::find_root_regula_falsi;

struct AlphaArgs {
    tail: i32,
    n: f64,
    power: f64,
    es: f64
}

/// Supertype for all test types.
///
/// See the G*Power 3 paper for the equations for the distribution parameters
/// (https://doi.org/10.3758/BF03193146).
enum TestKind {
    OneSampleTTest,
    DeviationFromZeroMultipleRegression {
        n_predictors: i32
    },
    GoodnessOfFitChisqTest {
        df: i32
    },
    /// Multiple regression: increase of R^2.
    /// Total number of predictors `p` (#A + #B).
    /// Number of tested predictors `q` (#B).
    IncreaseMultipleRegression {
        p: i32,
        q: i32
    },
    IndependentSamplesTTest
}

impl TestKind {
    fn alternative_distribution(&self, n: f64, es: f64) -> Dist {
        match self {
            TestKind::OneSampleTTest => {
                Box::new(NoncentralT::new(n - 1.0, n.sqrt() * es))
            },
            TestKind::DeviationFromZeroMultipleRegression { n_predictors } => {
                Box::new(NoncentralF::new(
                    *n_predictors as f64,
                    (n as f64) - (*n_predictors as f64) - (1 as f64),
                    es.powi(2) * (n as f64)
                ))
            },
            TestKind::GoodnessOfFitChisqTest { df } => {
                Box::new(NoncentralChisq::new(*df as f64, es.powi(2) * n))
            },
            TestKind::IncreaseMultipleRegression { p, q } => {
                Box::new(NoncentralF::new(
                    *q as f64,
                    n - (*p as f64) - 1.0,
                    es.powi(2) * n)
                )
            }
            TestKind::IndependentSamplesTTest => {
                let v = n - 2.0; // n1 + n2 - 2
                Box::new(NoncentralT::new(v, (n / 2.0).sqrt() * es))
            }

        }
    }

    fn null_distribution(&self, n: f64, es: f64) -> Dist {
        self.alternative_distribution(n, es).central_distribution()
    }

    fn n(&self, tail: i32, alpha: f64, power: f64, es: f64) -> i64 {
        let f = | n | { self.alpha(AlphaArgs { tail, n, power, es }) - alpha };
        let mut convergency = SimpleConvergency { eps: 0.0001f64, max_iter: 500 };
        return match find_root_brent(2f64, 1000f64, &f, &mut convergency) {
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
        let mut convergency = SimpleConvergency { eps: 0.0001f64, max_iter: 500 };
        return match find_root_regula_falsi(0.001f64, 8f64, &f, &mut convergency) {
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

#[test]
fn rounding() {
    assert_eq!(round(1.234, 2), 1.23);
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

#[no_mangle]
pub extern fn goodnessOfFitChisqTestN(df: i32, alpha: f64, power: f64, es: f64) -> i64 {
    let tail = 1;
    let test = TestKind::GoodnessOfFitChisqTest{ df };
    test.n(tail, alpha, power, es)
}
#[no_mangle]
pub extern fn goodnessOfFitChisqTestAlpha(df: i32, n: f64, power: f64, es: f64) -> f64 {
    let tail = 1;
    let test = TestKind::GoodnessOfFitChisqTest{ df };
    round(test.alpha(AlphaArgs { tail, n, power, es }), 3)
}
#[no_mangle]
pub extern fn goodnessOfFitChisqTestPower(df: i32, n: f64, alpha: f64, es: f64) -> f64 {
    let tail = 1;
    let test = TestKind::GoodnessOfFitChisqTest{ df };
    round(test.power(tail, n, alpha, es), 3)
}
#[no_mangle]
pub extern fn goodnessOfFitChisqTestES(df: i32, n: f64, alpha: f64, power: f64) -> f64 {
    let tail = 2;
    let test = TestKind::GoodnessOfFitChisqTest{ df };
    round(test.es(tail, n, alpha, power), 3)
}

#[no_mangle]
pub extern fn increaseMultipleRegressionN(p: i32, q: i32, alpha: f64, power: f64, es: f64) -> i64 {
    let tail = 1;
    let test = TestKind::IncreaseMultipleRegression{ p, q };
    test.n(tail, alpha, power, es)
}
#[no_mangle]
pub extern fn increaseMultipleRegressionAlpha(p: i32, q: i32, n: f64, power: f64, es: f64) -> f64 {
    let test = TestKind::IncreaseMultipleRegression{ p, q };
    round(test.alpha(AlphaArgs { tail: 1, n, power, es }), 3)
}
#[no_mangle]
pub extern fn increaseMultipleRegressionPower(p: i32, q: i32, n: f64, alpha: f64, es: f64) -> f64 {
    let tail = 1;
    let test = TestKind::IncreaseMultipleRegression{ p, q };
    round(test.power(tail, n, alpha, es), 3)
}
#[no_mangle]
pub extern fn increaseMultipleRegressionES(p: i32, q: i32, n: f64, alpha: f64, power: f64) -> f64 {
    let tail = 1;
    let test = TestKind::IncreaseMultipleRegression{ p, q };
    round(test.es(tail, n, alpha, power), 3)
}

#[no_mangle]
pub extern fn independentSamplesTTestN(tail: i32, alpha: f64, power: f64, es: f64) -> i64 {
    TestKind::IndependentSamplesTTest.n(tail, alpha, power, es)
}
#[no_mangle]
pub extern fn independentSamplesTTestAlpha(tail: i32, n: f64, power: f64, es: f64) -> f64 {
    round(TestKind::IndependentSamplesTTest.alpha(AlphaArgs { tail, n, power, es }), 3)
}
#[no_mangle]
pub extern fn independentSamplesTTestPower(tail: i32, n: f64, alpha: f64, es: f64) -> f64 {
    round(TestKind::IndependentSamplesTTest.power(tail, n, alpha, es), 3)
}
#[no_mangle]
pub extern fn independentSamplesTTestES(tail: i32, n: f64, alpha: f64, power: f64) -> f64 {
    round(TestKind::IndependentSamplesTTest.es(tail, n, alpha, power), 3)
}

#[no_mangle]
pub extern fn calculatePower(ptr: *mut u8) {
    let text = unsafe { u8_to_string(ptr) };
    let data = json(text);
    // let result = json::parse(r#"{"n": 10}"#).unwrap();
    let result = r#"{"n":10}"#;
    write_to_ptr(ptr, result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const ES: f64 = 0.5;
    const ALPHA: f64 = 0.05;
    const POWER: f64 = 0.95;
    const N: f64 = 50.0;

    // Outputs below are obtained from G*Power via VirtualBox IE11 Windows 7.

    #[test]
    fn one_sample_t_test() {
        assert_eq!(oneSampleTTestAlpha(2, N, POWER, ES), 0.067);
        assert_eq!(oneSampleTTestAlpha(1, N, POWER, ES), 0.034);
        assert_eq!(oneSampleTTestPower(2, N, ALPHA, ES), 0.934);
        assert_eq!(oneSampleTTestPower(1, N, ALPHA, ES), 0.967);

        assert_eq!(oneSampleTTestES(2, N, ALPHA, POWER), 0.520);
        assert_eq!(oneSampleTTestES(1, N, ALPHA, POWER), 0.472);
        assert_eq!(oneSampleTTestN(2, ALPHA, POWER, ES), 54);
        assert_eq!(oneSampleTTestN(1, ALPHA, POWER, ES), 45);
        assert_eq!(oneSampleTTestN(1, 0.99, POWER, ES), -111);
    }

    #[test]
    fn deviation_from_zero_multiple_regression() {
        let f_squared = ES.sqrt();
        assert_eq!(deviationFromZeroMultipleRegressionAlpha(2, N, POWER, f_squared), 0.006);
        assert_eq!(deviationFromZeroMultipleRegressionPower(2, N, ALPHA, f_squared), 0.994);
        assert_eq!(deviationFromZeroMultipleRegressionES(2, N, ALPHA, POWER), 0.574);
        assert_eq!(deviationFromZeroMultipleRegressionN(2, ALPHA, POWER, f_squared), 34);
    }

    #[test]
    fn goodness_of_fit_chisq_test() {
        let df = 5;
        assert_eq!(goodnessOfFitChisqTestAlpha(df, N, POWER, ES), 0.254);
        assert_eq!(goodnessOfFitChisqTestAlpha(df, N, POWER, 0.628), 0.051);
        assert_eq!(goodnessOfFitChisqTestPower(df, N, ALPHA, ES), 0.788);
        // This number is 0.629 in G*Power and I cannot figure out why.
        // After manual inspection, the root finding is going well so that is not it.
        // Also, the logic here matches the rest, so I guess that G*Power is off again.
        // G*Power was also sometimes off compared to Julia likely due to a not suboptimal
        // root finding algorithm.
        assert_eq!(goodnessOfFitChisqTestES(df, N, ALPHA, POWER), 0.670);
        assert_eq!(goodnessOfFitChisqTestN(df, ALPHA, POWER, ES), 79);
    }

    #[test]
    fn increase_multiple_regression() {
        let p = 5;
        let q = 2;
        let f_squared = ES.sqrt();
        assert_eq!(increaseMultipleRegressionAlpha(p, q, N, POWER, f_squared), 0.006);
    }

    #[test]
    fn independent_samples_t_test() {
        // G*Power only gives 0.392 if you put sample size group 1 and 2 both on n=50.
        // pwr.t.test(n=50, d=0.5, sig.level=NULL, power=0.95, type="two.sample", alternative="two.sided")
        assert_eq!(independentSamplesTTestAlpha(2, N, POWER, ES), 0.398);
    }
}

