use crate::dist::*;

trait StatisticalTest {
    fn null_distribution(&self) -> Box<Distribution>;
}

struct IndependentSamplesTTest {
    tail: i32,
    v: f64,
    lambda: f64
}

impl IndependentSamplesTTest {
    pub fn new(tail: i32, n: i32, es: f64) -> Self {
        return Self{ tail, v: (n - 2) as f64, lambda: (n as f64 / 2.0).sqrt() * es};
    }
}

impl StatisticalTest for IndependentSamplesTTest {
    fn null_distribution(&self) -> Box<dyn Distribution> {
        return Box::new(NoncentralT{ v: self.v, lambda: 0.0 });
    }
}
