use libm::expm1;
use libm::log;

use statrs::function::gamma::ln_gamma;

use crate::rmath::utils::*;

fn pnt_finis(tnc: f64, del: f64, negdel: bool) -> f64 {
    let tnc2 = tnc + pnorm(-del, 0.0, 1.0);
    let lower_tail = !negdel;
    assert!(!(tnc2 > 1.0 - 1e-10 && lower_tail));
    return r_dt_val(fmin2(tnc2, 1.0));
}

/// Cumulative probability at t of the non-central t-distribution with
/// df degrees of freedom and non-centrality parameter delta.
/// Based on https://github.com/wch/r-source/blob/trunk/src/nmath/pnt.c
pub fn pnt(t: f64, df: f64, ncp: f64) -> f64 {
    let itrmax: i32 = 1000;
    let errmax: f64 = 1.0e-12;

    assert!(0.0 < df);
    if ncp == 0.0 { return pt(t, df); }

    if !t.is_finite() {
        if t < 0.0 {
            return 0.0;
        } else {
            return 1.0;
        }
    }

    println!("foo");

    let negdel: bool;
    let tt: f64;
    let del: f64;

    if t >= 0.0 {
        negdel = false;
        tt = t;
        del = ncp;
    } else {
        if ncp > 40.0 { return R_DT_0; }
        negdel = true;
        tt = -t;
        del = -ncp;
    }

    let mut s: f64;

    if df > 4e5 || del * del > 2.0 * M_LN2 * -DBL_MIN_EXP {
        s = 1.0 / (4.0 * df);

        let x = tt * (1.0 - s);
        let mean = del;
        let sd = (1.0 + tt * tt * 2.0 * s).sqrt();
        let cumulative_density = pnorm(x, mean, sd);
        if negdel {
            return 1.0 - cumulative_density;
        } else {
            return cumulative_density;
        }
    }

    let mut x = t * t;
    let mut rxb = df / (x + df);
    x = x / (x + df);

    let lambda: f64;
    let mut p: f64;
    if x > 0.0 {
        lambda = del * del;
        p = 0.5 * (-0.5 * lambda).exp();

        // Original code warns here.
        assert!(p != 0.0);

        let mut q = M_SQRT_2DPI * p * del;
        s = 0.5 - p;

        if s < 1.0e-7 {
            s = -0.5 * expm1(-0.5 * lambda);
        }
        let mut a = 0.5;
        let b = 0.5 * df;
        rxb = f64::powf(rxb, b);
        let albeta = M_LN_SQRT_PI + ln_gamma(b) - ln_gamma(0.5 + b);
        let mut xodd = pbeta(x, a, b);
        let mut godd = 2.0 * rxb * (a * log(x) - albeta).exp();
        let mut tnc = b * x;
        let mut xeven = if tnc < DBL_EPSILON { tnc } else { 1.0 - rxb };
        let mut geven = tnc * rxb;
        tnc = p * xodd + q * xeven;

        for it in 1..itrmax {
            a += 1.0;
            xodd -= godd;
            xeven -= geven;
            godd *= x * (a + b - 1.0) / a;
            geven *= x * (a + b - 0.5) / (a + 0.5);
            p *= lambda / (2 * it) as f64;
            q *= lambda / (2 * it + 1) as f64;
            tnc += p * xodd + q * xeven;
            s -= p;
            // Warns in the original implementation.
            assert!(!(s < -1.0e-10));
            if s <= 0.0 && it > 1 {
                return pnt_finis(tnc, del, negdel);
            }
            let errbd = 2.0 * s * (xodd - godd);
            if errbd.abs() < errmax {
                return pnt_finis(tnc, del, negdel);
            }
        }
        assert!(false);
        return f64::NAN;
    } else {
        let tnc = 0.0;
        return pnt_finis(tnc, del, negdel);
    }
}


#[cfg(test)]
mod rmath_tests {
    extern crate approx;

    use super::*;

    #[test]
    fn that_pnt_is_correct() {
        // R> pt(2.0095, 49.0, 0)
        // [1] 0.9749959
        assert_eq!(pnt(2.0095, 49.0, 0.0), 0.9749958761700477);

        // R> pt(Inf, 49.0, 2.0)
        // [1] 1
        assert_eq!(pnt(f64::INFINITY, 49.0, 2.0), 1.0);

        // R> pt(-Inf, 49.0, 2.0)
        // [1] 0
        assert_eq!(pnt(-f64::INFINITY, 49.0, 2.0), 0.0);

        // R> pt(2.0095, 5e5, 2)
        // [1] 0.5037895
        assert_eq!(pnt(2.0095, 5e5, 2.0), 0.5037894861873192);

        // julia> cdf(NoncentralT(49.0, 3.5355), 2.0095)
        // 0.0660970064371808
        // R> pt(2.0095, 49.0, 3.5355)
        // [1] 0.06609701
        let t = 2.0095;
        let df = 49.0;
        let ncp = 3.5355;
        assert_eq!(pnt(t, df, ncp), 0.0660970064372871);

        // julia> cdf(NoncentralT(11, 2.23), -4.46)
        // 1.5848808498919453e-7
        // R> pt(-4.46, 11.0, 2.23)
        // [1] 1.584881e-07
        // This case occurs during qnt(0.54, 11.0, 2.23)
        assert_eq!(pnt(-4.46, 11.0, 2.23), 1.5848808498919453e-7);
    }
}
