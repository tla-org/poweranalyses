mod dist;
mod power;
mod string;
mod interface;

use crate::dist::*;

#[no_mangle]
pub extern fn add_ten(x: i32) -> i32 {
    x + 10
}

#[no_mangle]
pub extern fn some_r() -> f64 {
    return NoncentralT::new(0.5, 0.4).cdf(0.3, true);
}

pub fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_ten() {
        assert_eq!(12, add_ten(2));
    }
}
