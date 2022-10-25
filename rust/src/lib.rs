#![allow(dead_code)]

mod dist;
mod nmath;

#[no_mangle]
fn add_ten(x: i32) -> i32 {
    x + 14
}

#[cfg(test)]
mod lib {
    use super::*;

    #[test]
    fn it_adds_ten() {
        assert_eq!(12, add_ten(2));

        foobar();
    }
}
