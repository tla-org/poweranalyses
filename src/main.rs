mod dist;
mod power;
mod string;

use crate::dist::*;
use crate::string::u8_to_string;
use crate::string::json;

#[no_mangle]
pub extern fn add_ten(x: i32) -> i32 {
    x + 10
}

#[no_mangle]
pub extern fn some_r() -> f64 {
    return NoncentralT::new(0.5, 0.4).cdf(0.3, true);
}

#[no_mangle]
pub extern fn foo(ptr: *mut u8) {
    let text = unsafe { u8_to_string(ptr) };
    println!("text: {text}");
    let data = json(text);
    println!("data: {data:?}");
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
