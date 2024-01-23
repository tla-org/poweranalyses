//!
//! Mathlib : A C Library of Special Functions
//! Copyright (C) 1998-2015 The R Core Team
//! based on AS243 (C) 1989 Royal Statistical Society
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
use crate::pt;
use crate::rmath;

extern "C" {
    fn pt(x: f64, n: f64, lower_tail: i32, log_p: i32) -> f64;
    fn lgammafn(x: f64) -> f64;
}

fn finis(mut tnc: f64, del: f64, negdel: bool, mut lower_tail: bool) -> f64 {
    tnc += rmath::pnorm(-del, 0.0, 1.0, /*lower*/ true, /*log_p*/ false);
    lower_tail = lower_tail != negdel;
    if tnc > 1.0 - 1e-10 && lower_tail {
        println!("precision problem in pnt");
    }
    dpq::r_dt_val(f64::min(tnc, 1.0), lower_tail, /*log_p*/ false)
}

/// Algorithm AS 243  Lenth,R.V. (1989). Appl. Statist., Vol.38, 185-189.
/// ----------------
/// Cumulative probability at t of the non-central t-distribution
/// with df degrees of freedom (may be fractional) and non-centrality
/// parameter delta.
pub fn pnt(t: f64, df: f64, ncp: f64, lower_tail: bool, log_p: bool) -> f64 {
    let albeta: f64;
    let mut a: f64;
    let b: f64;
    let del: f64;
    let mut errbd: f64;
    let lambda: f64;
    let mut rxb: f64;
    let tt: f64;
    let mut x: f64;
    // Long doubles can most likely be represented by f64 without problems:
    // https://en.wikipedia.org/wiki/Long_double
    let mut geven: f64;
    let mut godd: f64;
    let mut p: f64;
    let mut q: f64;
    let mut s: f64;
    let mut tnc: f64;
    let mut xeven: f64;
    let mut xodd: f64;

    let negdel: bool;

    let itrmax: i32 = 1000;
    let errmax: f64 = 1e-12;

    if df <= 0.0 {
        nmath::ml_warn_return_nan();
    }
    if ncp == 0.0 {
        return pt::pt(t, df, lower_tail, log_p);
        // return unsafe { pt(t, df, lower_tail as i32, log_p as i32) };
    }

    if !nmath::r_finite(t) {
        return if t < 0.0 {
            dpq::r_dt_0(lower_tail, log_p)
        } else {
            dpq::r_dt_1(lower_tail, log_p)
        };
    }

    if t >= 0.0 {
        negdel = false;
        tt = t;
        del = ncp;
    } else {
        if ncp > 40.0 && (!log_p || !lower_tail) {
            return dpq::r_dt_0(lower_tail, log_p);
        }
        negdel = true;
        tt = -t;
        del = -ncp;
    }

    if df > 4e5 || del * del > 2.0 * rmath::M_LN2 * -(f64::MIN_EXP as f64) {
        s = 1.0 / (4.0 * df);
        return rmath::pnorm(
            tt * (1.0 - s),
            del,
            (1.0 + tt * tt * 2.0 * s).sqrt(),
            lower_tail != negdel,
            log_p,
        );
    }

    /* initialize twin series */
    /* Guenther, J. (1978). Statist. Computn. Simuln. vol.6, 199. */

    x = t * t;
    rxb = df / (x + df);
    x = x / (x + df);

    if x > 0.0 {
        lambda = del * del;
        p = 0.5 * (-0.5 * lambda).exp();
        if p == 0.0 {
            println!("Underflow in pnt; |ncp| too large");
            return dpq::r_dt_0(lower_tail, log_p);
        }
        q = rmath::M_SQRT_2dPI * p * del;
        s = 0.5 - p;
        if s < 1e-7 {
            s = -0.5 * (-0.5 * lambda).exp_m1();
        }
        a = 0.5;
        b = 0.5 * df;
        rxb = rxb.powf(b);
        albeta = rmath::M_LN_SQRT_PI + unsafe { lgammafn(b) - lgammafn(0.5 + b) };
        xodd = rmath::pbeta(x, a, b, /*lower*/ true, /*log_p*/ false);
        godd = 2. * rxb * (a * x.ln() - albeta).exp();
        tnc = b * x;
        xeven = if tnc < f64::EPSILON { tnc } else { 1. - rxb };
        geven = tnc * rxb;
        tnc = p * xodd + q * xeven;

        for it in 1..=itrmax {
            a += 1.0;
            xodd -= godd;
            xeven -= geven;
            godd *= x * (a + b - 1.0) / a;
            geven *= x * (a + b - 0.5) / (a + 0.5);
            p *= lambda / (2 * it) as f64;
            q *= lambda / (2 * it + 1) as f64;
            tnc += p * xodd + q * xeven;
            s -= p;
            if s < -1e-10 {
                println!("precision problem in pnt");
                return finis(tnc, del, negdel, lower_tail);
            }
            errbd = 2. * s * (xodd - godd);
            if errbd.abs() < errmax {
                return finis(tnc, del, negdel, lower_tail);
            }
            println!("pnt didn't converge");
        }
    } else {
        tnc = 0.0;
    }

    finis(tnc, del, negdel, lower_tail)
}
