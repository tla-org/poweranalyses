mod distribution;

#[no_mangle]
pub extern "C" fn add_ten(x: i32) -> i32 {
    x + 10
}

#[cfg(test)]
mod lib {
    use super::*;

    #[test]
    fn it_adds_ten() {
        assert_eq!(12, add_ten(2));
    }
}
