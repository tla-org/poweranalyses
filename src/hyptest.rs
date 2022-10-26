use crate::dist::Distribution;

trait HypothesisTest {
    fn dist(&self) -> dyn Distribution;
}

struct OneSampleTTest {
    /// 1 for a one sample t-test and 2 for a two sample t-test.
    tail: i32
}

impl OneSampleTTest {
    pub fn new(tail: i32) -> OneSampleTTest {
        assert!(tail == 1 || tail == 2, "OneSampleTTest::new called with incorrect tail");
        return Self{ tail };
    }
}
