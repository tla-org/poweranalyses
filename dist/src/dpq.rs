#![allow(non_snake_case)]

use crate::rmath;

fn r_d__0(log_p: bool) -> f64 {
    if log_p {
        f64::NEG_INFINITY
    } else {
        0.0
    }
}

fn r_d__1(log_p: bool) -> f64 {
    if log_p {
        0.0
    } else {
        1.0
    }
}

pub fn r_dt_0(lower_tail: bool, log_p: bool) -> f64 {
    if lower_tail {
        r_d__0(log_p)
    } else {
        r_d__1(log_p)
    }
}

pub fn r_dt_1(lower_tail: bool, log_p: bool) -> f64 {
    if lower_tail {
        r_d__1(log_p)
    } else {
        r_d__0(log_p)
    }
}

pub fn r_d_val(x: f64, log_p: bool) -> f64 {
    if log_p {
        x.ln()
    } else {
        x
    }
}

fn r_d_clog(p: f64, log_p: bool) -> f64 {
    if log_p {
        rmath::log1p(-p)
    } else {
        0.5 - p + 0.5
    }
}

pub fn r_dt_val(x: f64, lower_tail: bool, log_p: bool) -> f64 {
    if lower_tail {
        r_d_val(x, log_p)
    } else {
        r_d_clog(x, log_p)
    }
}
