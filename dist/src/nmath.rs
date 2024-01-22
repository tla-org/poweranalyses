pub fn ml_warn_return_nan() -> f64 {
    println!("argument out of domain");
    f64::NAN
}

pub fn r_finite(x: f64) -> bool {
    x.is_finite()
}
