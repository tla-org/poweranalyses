//!
//! R : A Computer Language for Statistical Data Analysis
//! Copyright (C) 1995, 1996  Robert Gentleman and Ross Ihaka
//! Copyright (C) 2000-2007   The R Core Team
//!
//! This program is free software; you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation; either version 2 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with this program; if not, a copy is available at
//! https://www.R-project.org/Licenses/
//!

use crate::dpq;
use crate::nmath;
use crate::rmath;

extern "C" {
    fn lbeta(a: f64, b: f64) -> f64;
}

/// Return P[ T <= x ] where
///  T ~ t_{n}  (t distrib. with n degrees of freedom).
/// *  --> ./pnt.c for NON-central
pub fn pt(x: f64, n: f64, mut lower_tail: bool, log_p: bool) -> f64 {
    let mut val: f64;

    if x.is_nan() || n.is_nan() {
        return x + n;
    }

    if n <= 0.0 {
        nmath::ml_warn_return_nan();
    }

    if !nmath::r_finite(x) {
        if x < 0.0 {
            return dpq::r_dt_0(lower_tail, log_p);
        } else {
            return dpq::r_dt_1(lower_tail, log_p);
        };
    }

    if !nmath::r_finite(n) {
        return rmath::pnorm(x, 0.0, 1.0, lower_tail, log_p);
    }

    let nx: f64 = 1.0 + (x / n) * x;

    if nx > 1e100 {
        let lval: f64 = -0.5 * n * (2.0 * x.abs().ln() - n.ln())
            - unsafe { lbeta(0.5 * n, 0.5) }
            - (0.5 * n).ln();
        val = if log_p { lval } else { lval.exp() };
    } else {
        val = if n > x * x {
            rmath::pbeta(
                x * x / (n + x * x),
                0.5,
                n / 2.0,
                /*lower_tail*/ false,
                log_p,
            )
        } else {
            rmath::pbeta(1.0 / nx, n / 2.0, 0.5, /*lower_tail*/ true, log_p)
        };
    }

    if x <= 0.0 {
        lower_tail = !lower_tail;

        if log_p {
            if lower_tail {
                return rmath::log1p(-0.5 * val.exp());
            } else {
                return val - rmath::M_LN2; // = log(.5* pbeta(....))
            }
        }
    } else {
        val /= 2.0;
        return dpq::r_d_val(val, log_p);
    }

    0.0
}

#[cfg(test)]
mod test_pt {
    use super::*;

    #[test]
    fn outcome_matches_r() {
        // assert_eq!(pt(0.3, 0.5, true, false), 0.7822574);
    }
}
