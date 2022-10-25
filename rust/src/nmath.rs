extern {
    fn dnt(x: f64, df: f64, ncp: f64, give_log: i32) -> f64;
}

#[no_mangle]
pub(crate) extern fn foobar() -> f64 {
    unsafe {
        dnt(1.0, 2.0, 1.0, 0)
    }
}
