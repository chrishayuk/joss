//! `log` and `exp` as the January 1965 listing computes them: routines F1
//! (pp. 44-45) and F2 (pp. 46-47) of "JOSS 666, JCS 1-9-65".
//!
//! JOHNNIAC worked in 40-bit binary words, two's complement, read either as
//! a fraction (the binary point after the sign) or, "at scale 39", as a whole
//! number. Each function below follows its routine card by card: the same
//! constants, the same order of operations, the same rounding (`MR`) or
//! truncation (`M`, `D`) at each step. The card numbers are given so the code
//! can be checked against the page.
//!
//! What the listing does not say, and is assumed here:
//! - a product `M`/`MA` leaves the high part in A (rounded down) and the low
//!   39 bits in MQ as a positive fraction, which is what card F001E540
//!   ("GET ABSOLUTE VALUE OF NEGATIVE LOG") requires;
//! - a quotient (`DS`, `D`) is cut off towards zero;
//! - the tables are what their names say: N k is 10^k, I k is k, K21 is 1/2.
//!
//! Two things follow from the routines that a correctly rounded log would
//! not do. A logarithm smaller than 1 in size is cut off at nine digits, not
//! rounded (cards F001E660-E670: `M N 9`, not `MR`), so log(2) is .693147180,
//! typed `.69314718`, where the true value rounds to .693147181. And exp
//! works from x·log10(e) with an eleven-digit constant, so e^72 comes out
//! 1.85867174·10^31 where the true value rounds to 1.85867175·10^31.

use crate::num::{Fault, Num};

const H: i128 = 1 << 39; // the unit of a fraction
const LOW: i128 = H - 1;

/// `MR`: M·MQ, rounded to a fraction.
fn mr(m: i128, mq: i128) -> i128 {
    (m * mq + (1 << 38)) >> 39
}

/// `M`: M·MQ as a double word: (A, MQ).
fn mul(m: i128, mq: i128) -> (i128, i128) {
    let p = m * mq;
    (p >> 39, p & LOW)
}

/// A double-length dividend (in units of 2^-78) over a word: the quotient in
/// units of 2^-39, cut off towards zero.
fn div(dividend: i128, m: i128) -> i128 {
    let q = dividend.abs() / m.abs();
    if (dividend < 0) != (m < 0) {
        -q
    } else {
        q
    }
}

/// A decimal constant of the listing with binary scale 00: the nearest
/// fraction. `digits` are the digits after the point.
const fn fraction(digits: i128, places: u32) -> i128 {
    let ten = 10i128.pow(places);
    (digits * H * 2 + ten) / (2 * ten)
}

fn ten(k: i32) -> i128 {
    10i128.pow(k as u32)
}

/// F1, `log(x)` for x > 0, given its nine digits `c` (10^8 ≤ c < 10^9) and the
/// power of ten `e` of the first of them.
pub fn log(c: i64, e: i32) -> Result<Num, Fault> {
    const LOWER: i128 = 565_685_425; // *91  (2E3)(10E8)(SQRT(2)/2)
    const MIDDLE: i128 = 800_000_000; // *92 (2E3)(10E8)
    const LOG2_4: i128 = fraction(17_328_679_514, 11); // *93 LOG(2)/4
    const LOG10_4: i128 = fraction(57_564_627_325, 11); // *94 LOG(10)/4

    // E080-E120: double the coefficient until it is above the lower bound,
    // counting the doublings in Q.
    let mut c = c as i128;
    let mut q = 0i128;
    while c - LOWER < 0 {
        c <<= 1;
        q += 1;
    }
    // E130-E170: Z = (c - 8·10^8)/(c + 8·10^8), Z^2.
    let z = div((c - MIDDLE) * H, c + MIDDLE);
    let z2 = mr(z, z);
    // E180-E240: T0 = (...((1/13)Z^2 + 1/11)Z^2 + ... + 1/3)Z^2.
    let mut t0 = 0;
    let mut delta = 13;
    loop {
        t0 += div(H, delta);
        t0 = mr(z2, t0);
        if delta - 4 < 0 {
            break;
        }
        delta -= 2;
    }
    // E250-E270: Z + Z·T0 is half the log of c/(8·10^8); halved again.
    let quarter = (mr(z, t0) + z) >> 1;
    // E280-E300: T5 = (3-Q)·log(2)/4, the low half of the product.
    let t5 = mul(3 - q, LOG2_4).1;
    // E310-E330: e·log(10)/4 + T5 + T0 as a double word, times 4: the
    // whole part of the log in A, the fraction in MQ.
    let p = (e as i128) * LOG10_4 + (t5 + quarter);
    let v = p << 2;
    let whole = v >> 39;
    let fract = v & LOW;
    let negative = whole < 0;
    // E340, E530-E570: the size of a negative log.
    let (whole, fract) = if !negative {
        (whole, fract)
    } else if fract == 0 {
        (-whole, 0)
    } else {
        (-whole - 1, H - fract)
    };
    let (coef, exp) = if whole == 0 {
        // E580-E680: less than one. Find the first digit that is not
        // nought, then take nine digits from there, cut off.
        if fract == 0 {
            return Ok(Num::int(0));
        }
        let mut i = 1;
        let mut mq = fract;
        loop {
            let (digit, rest) = mul(ten(1), mq);
            mq = rest;
            if digit >= 1 {
                break;
            }
            i += 1;
        }
        let low = mul(fract, ten(i - 1)).1;
        let (coef, _) = mul(low, ten(9));
        (coef, -i)
    } else {
        // E350-E474: one or more. I is the number of digits of the whole
        // part; the fraction is rounded into the rest.
        let mut i = 1;
        while whole - ten(i) >= 0 {
            i += 1;
        }
        let scale = ten(9 - i);
        let coef = whole * scale + mr(fract, scale);
        if coef - ten(9) >= 0 {
            (ten(8), i)
        } else {
            (coef, i - 1)
        }
    };
    Num::from_parts(negative, coef as i64, exp)
}

/// F2, `exp(x)` for x ≠ 0, given its sign, its nine digits and its power of
/// ten.
pub fn exp(negative: bool, c: i64, e: i32) -> Result<Num, Fault> {
    const LN10: i128 = 230_258_510; // *92 LOG(10), BASE E, TIMES 10*8 PLUS 1
    const LOG10E: i128 = fraction(43_429_448_190, 11); // *93 LOG(E), BASE 10

    let c = c as i128;
    // E060: COEF(X)/10*9.
    let t0 = div(c * H, ten(9));
    let (mut xbar, mut p4) = if e >= 0 {
        // E150-E210: too large either way.
        if e - 3 >= 0 || (e == 2 && c - LN10 >= 0) {
            return if negative {
                Ok(Num::int(0))
            } else {
                Err(Fault::Overflow)
            };
        }
        // E220-E280: x·log10(e) = P4 + f; XBAR = f·ln(10)/4.
        let t = mr(LOG10E, t0);
        let (whole, f) = mul(ten(e + 1), t);
        (div((f * H) >> 2, LOG10E), whole)
    } else {
        // E090-E140: small x. Below 10^-9, e^x is 1 to nine digits.
        if e + 9 < 0 {
            return Ok(Num::int(1));
        }
        let q = div(t0, ten(-e - 1));
        (q >> 2, 0)
    };
    if negative {
        xbar = -xbar;
        p4 = -p4;
    }
    // E310-E380: Z = 1/2 e^XBAR, by twelve terms, from the inside out.
    let mut sum = H / 2;
    for k in (1..=12).rev() {
        sum = mr(div(xbar, k), sum) + H / 2;
    }
    // E390-E430: (Z^4)(10^10)(16) = e^(4·XBAR), with ten more digits.
    let z2 = mr(sum, sum);
    let z4 = mr(z2, z2);
    let mut p6 = (z4 * ten(10)) >> 35;
    // E440-E600: round to nine digits.
    let shift = if p6 - ten(11) >= 0 {
        3
    } else if p6 - ten(10) >= 0 {
        2
    } else if p6 - ten(9) >= 0 {
        1
    } else {
        0
    };
    if shift > 0 {
        p4 += shift as i128;
        p6 = (p6 + ten(shift) / 2) / ten(shift);
        if p6 - ten(9) >= 0 {
            p6 = ten(8);
            p4 += 1;
        }
    }
    Num::from_parts(false, p6 as i64, (p4 - 2) as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn n(text: &str) -> Num {
        match text.strip_prefix('-') {
            Some(rest) => Num::numeral(rest).unwrap().neg(),
            None => Num::numeral(text).unwrap(),
        }
    }

    #[test]
    fn the_printed_logarithm_of_two() {
        // P-2922 fig. 3b and RM-4162: `log(2)` is typed .69314718.
        assert_eq!(n("2").log().unwrap(), n(".69314718"));
        assert_eq!(n(".5").log().unwrap(), n("-.69314718"));
        assert_eq!(n("10").log().unwrap(), n("2.30258509"));
        assert_eq!(n("1").log().unwrap(), n("0"));
        assert_eq!(n("2.71828183").log().unwrap(), n("1"));
        assert_eq!(n(".00001").log().unwrap(), n("-11.5129255"));
        assert_eq!(n(".7").log().unwrap(), n("-.356674943"));
    }

    #[test]
    fn the_printed_exponentials() {
        // RM-4162 p. 23: e^72 typed 1.8586717·10*31.
        assert_eq!(n("72").exp().unwrap(), n("18586717400000000000000000000000"));
        // P-2922 fig. 3f: exp(100) = 2.68811714·10*43.
        assert_eq!(n("100").exp().unwrap().mantissa(), 268811714);
        assert_eq!(n("1").exp().unwrap(), n("2.71828183"));
        assert_eq!(n(".5").exp().unwrap(), n("1.64872127"));
        assert_eq!(n("-1").exp().unwrap(), n(".367879441"));
        assert_eq!(n("0").exp().unwrap(), n("1"));
        assert_eq!(n("230.3").exp(), Err(Fault::Overflow));
        assert_eq!(n("-230.3").exp().unwrap(), n("0"));
    }
}
