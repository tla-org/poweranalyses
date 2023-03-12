use dist::Dist;
use dist::NoncentralChisq;
use dist::NoncentralF;
use dist::NoncentralT;
use roots::SimpleConvergency;
use roots::find_root_brent;
use roots::find_root_regula_falsi;
use serde_json::Value;

/// Supertype for all test types.
///
/// See the G*Power 3 paper for the equations for the distribution parameters
/// (https://doi.org/10.3758/BF03193146).
pub enum TestKind {
    OneSampleTTest,
    DeviationFromZeroMultipleRegression {
        n_predictors: i64
    },
    GoodnessOfFitChisqTest {
        df: i64
    },
    /// Multiple regression: increase of R^2.
    /// Total number of predictors `p` (#A + #B).
    /// Number of tested predictors `q` (#B).
    IncreaseMultipleRegression {
        p: i64,
        q: i64
    },
    IndependentSamplesTTest
}

#[derive(Clone, Debug)]
pub enum Tail {
    OneSided,
    TwoSided,
}

impl Tail {
    pub fn from_json(value: &Value) -> Option<Tail> {
        let tail: &Value = match value.get("tail") {
            Some(tail) => tail,
            None => return None,
        };
        let tail: i64 = tail.as_i64().unwrap();
        match tail {
            1 => Some(Tail::OneSided),
            2 => Some(Tail::TwoSided),
            _ => None,
        }
    }
}

impl TestKind {
    pub fn from_str(text: &str, data: &Value) -> Result<TestKind, String> {
        match text {
            "oneSampleTTest" => Ok(TestKind::OneSampleTTest),
            "deviationFromZeroMultipleRegression" => {
                let n_predictors = data["nPredictors"].as_i64().ok_or("nPredictors is not an integer")?;
                Ok(TestKind::DeviationFromZeroMultipleRegression{ n_predictors})
            },
            "goodnessOfFitChisqTest" => {
                let df = data["df"].as_i64().ok_or("df is not an integer")?;
                Ok(TestKind::GoodnessOfFitChisqTest{ df })
            },
            "increaseMultipleRegression" => {
                let p = data["p"].as_i64().ok_or("p is not an integer")?;
                let q = data["q"].as_i64().ok_or("q is not an integer")?;
                Ok(TestKind::IncreaseMultipleRegression{ p, q })
            },
            "independentSamplesTTest" => Ok(TestKind::IndependentSamplesTTest),
            _ => Err(format!("Unknown test: {}", text)),
        }
    }

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

    pub fn n(&self, tail: Tail, alpha: f64, power: f64, es: f64) -> i64 {
        let f = | n | { self.alpha(tail.clone(), n, power, es) - alpha };
        let mut convergency = SimpleConvergency { eps: 0.0001f64, max_iter: 500 };
        return match find_root_brent(2f64, 1000f64, &f, &mut convergency) {
            Ok(number) => number.round() as i64,
            Err(_) => -111
        };
    }

    pub fn alpha(&self, tail: Tail, n: f64, power: f64, es: f64) -> f64 {
        let d0 = self.null_distribution(n, es);
        let d1 = self.alternative_distribution(n, es);
        let critical_value = d1.quantile(power, false);
        let right_tail = d0.cdf(critical_value, false);
        match tail {
            Tail::OneSided => right_tail,
            Tail::TwoSided => 2.0 * right_tail,
        }
    }

    pub fn power(&self, tail: Tail, n: f64, alpha: f64, es: f64) -> f64 {
        let d0 = self.null_distribution(n, es);
        let d1 = self.alternative_distribution(n, es);
        let right_tail = match tail {
            Tail::OneSided => alpha,
            Tail::TwoSided => alpha / 2.0,
        };
        let critical_value = d0.quantile(right_tail, false);
        return d1.cdf(critical_value, false);
    }

    pub fn es(&self, tail: Tail, n: f64, alpha: f64, power: f64) -> f64 {
        let f = | es | { self.alpha(tail.clone(), n, power, es) - alpha };
        let mut convergency = SimpleConvergency { eps: 0.0001f64, max_iter: 500 };
        return match find_root_regula_falsi(0.001f64, 8f64, &f, &mut convergency) {
            Ok(number) => number,
            Err(_) => -111.0
        };
    }
}
