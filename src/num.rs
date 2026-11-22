//! JOSS numbers: nine significant decimal digits and a power of ten.
//!
//! "JOSS represents all numbers internally in scientific notation--nine
//! decimal digits of significance and a base-ten scale factor with an integer
//! exponent in the range -99 through +99. ... Addition, subtraction,
//! multiplication, division, and square root are carried out by JOSS to give
//! true results rounded to nine significant decimal digits (except on overflow
//! which yields an error message, or on underflow for which zero is
//! substituted)." (Shaw, P-2922, August 1964, p. 25)
//!
//! So a tenth is a tenth here, and adding it ten times gives exactly 1.

use std::cmp::Ordering;

use crate::routines;

/// Significant digits carried.
pub const DIGITS: u32 = 9;
/// The largest and smallest power of ten of the leading digit.
pub const MAX_EXP: i32 = 99;
pub const MIN_EXP: i32 = -99;

const UNIT: i64 = 100_000_000; // 10^8: the smallest nine-digit mantissa

/// What can go wrong in arithmetic. The session turns these into messages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fault {
    /// The result is too large to hold: its exponent would pass 99.
    Overflow,
    ZeroDivisor,
    NegativeSqrt,
    /// log of zero or of a negative number.
    BadLog,
    /// A negative number to a power that is not a whole number.
    NegativeBase,
    /// Zero to a negative power.
    ZeroToNegative,
    /// sin or cos of a number too large for the answer to mean anything.
    BadAngle,
}

/// `m` × 10^(`e` − 8). Either `m` is 0 (and `e` is 0), or 10^8 ≤ |m| < 10^9,
/// so `m` is the nine digits and `e` the power of ten of the first of them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Num {
    m: i64,
    e: i32,
}

pub const ZERO: Num = Num { m: 0, e: 0 };
pub const ONE: Num = Num { m: UNIT, e: 0 };

fn pow10(n: u32) -> i128 {
    10i128.pow(n)
}

fn digits(mut v: i128) -> u32 {
    let mut n = 0;
    while v > 0 {
        v /= 10;
        n += 1;
    }
    n
}

/// The number `v` × 10^`exp`, rounded to nine digits. A half rounds away from
/// zero ("rounded in the conventional way").
fn normal(negative: bool, mut v: i128, mut exp: i32) -> Result<Num, Fault> {
    if v == 0 {
        return Ok(ZERO);
    }
    let n = digits(v);
    if n > DIGITS {
        let drop = n - DIGITS;
        let div = pow10(drop);
        let rest = v % div;
        v /= div;
        if rest * 2 >= div {
            v += 1;
        }
        exp += drop as i32;
        if v == pow10(DIGITS) {
            v /= 10;
            exp += 1;
        }
    } else if n < DIGITS {
        v *= pow10(DIGITS - n);
        exp -= (DIGITS - n) as i32;
    }
    let e = exp + (DIGITS as i32 - 1);
    if e > MAX_EXP {
        return Err(Fault::Overflow);
    }
    if e < MIN_EXP {
        return Ok(ZERO);
    }
    Ok(Num {
        m: if negative { -(v as i64) } else { v as i64 },
        e,
    })
}

fn isqrt(n: u128) -> u128 {
    if n < 2 {
        return n;
    }
    let mut x = (n as f64).sqrt() as u128;
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

// These are named for what they are. They are not the operators of
// `std::ops` because each can fail, and says how.
#[allow(clippy::should_implement_trait)]
impl Num {
    pub fn int(n: i64) -> Num {
        normal(n < 0, (n as i128).abs(), 0).unwrap_or(ZERO)
    }

    /// How many significant digits a numeral has: noughts in front and
    /// noughts at the end do not count. "Please limit numbers to 9
    /// significant digits."
    pub fn significant(text: &str) -> usize {
        let digits: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
        digits.trim_start_matches('0').trim_end_matches('0').len()
    }

    /// A numeral as typed: digits with at most one point, no sign and no
    /// exponent. More than nine digits are rounded.
    pub fn numeral(text: &str) -> Option<Num> {
        let mut v: i128 = 0;
        let mut exp = 0i32;
        let mut seen_point = false;
        let mut any = false;
        let mut extra = 0i32; // digits past the 30th, which cannot matter
        for c in text.chars() {
            match c {
                '0'..='9' => {
                    any = true;
                    if digits(v) < 30 {
                        v = v * 10 + (c as u8 - b'0') as i128;
                        if seen_point {
                            exp -= 1;
                        }
                    } else if !seen_point {
                        extra += 1;
                    }
                }
                '.' if !seen_point => seen_point = true,
                _ => return None,
            }
        }
        if !any {
            return None;
        }
        normal(false, v, exp + extra).ok()
    }

    pub fn is_zero(self) -> bool {
        self.m == 0
    }

    pub fn is_negative(self) -> bool {
        self.m < 0
    }

    /// −1, 0 or 1.
    pub fn sign(self) -> i32 {
        self.m.signum() as i32
    }

    /// The nine digits, as a number from 100000000 to 999999999 (or 0).
    pub fn mantissa(self) -> i64 {
        self.m.abs()
    }

    /// The power of ten of the first digit: 2 for 123.456.
    pub fn exponent(self) -> i32 {
        self.e
    }

    pub fn neg(self) -> Num {
        Num { m: -self.m, e: self.e }
    }

    pub fn abs(self) -> Num {
        Num {
            m: self.m.abs(),
            e: self.e,
        }
    }

    pub fn add(self, other: Num) -> Result<Num, Fault> {
        if self.m == 0 {
            return Ok(other);
        }
        if other.m == 0 {
            return Ok(self);
        }
        let (hi, lo) = if self.e >= other.e {
            (self, other)
        } else {
            (other, self)
        };
        let gap = (hi.e - lo.e) as u32;
        if gap > 2 * DIGITS {
            return Ok(hi);
        }
        let v = hi.m as i128 * pow10(gap) + lo.m as i128;
        normal(v < 0, v.abs(), lo.e - 8)
    }

    pub fn sub(self, other: Num) -> Result<Num, Fault> {
        self.add(other.neg())
    }

    pub fn mul(self, other: Num) -> Result<Num, Fault> {
        let v = self.m as i128 * other.m as i128;
        normal(v < 0, v.abs(), self.e + other.e - 16)
    }

    pub fn div(self, other: Num) -> Result<Num, Fault> {
        if other.m == 0 {
            return Err(Fault::ZeroDivisor);
        }
        // Twelve extra digits: the quotient is cut off below them, which
        // cannot change which way the ninth digit rounds.
        let v = (self.m as i128).abs() * pow10(12) / (other.m as i128).abs();
        normal((self.m < 0) != (other.m < 0), v, self.e - other.e - 12)
    }

    pub fn sqrt(self) -> Result<Num, Fault> {
        if self.m < 0 {
            return Err(Fault::NegativeSqrt);
        }
        if self.m == 0 {
            return Ok(ZERO);
        }
        // self = m × 10^(e−8). Scale m so that the power left over is even.
        let mut exp = self.e - 8;
        let mut shift = 12;
        if (exp - shift).rem_euclid(2) != 0 {
            shift += 1;
        }
        exp -= shift;
        let root = isqrt(self.m as u128 * pow10(shift as u32) as u128);
        normal(false, root as i128, exp / 2)
    }

    pub fn is_integer(self) -> bool {
        self.m == 0 || self.e >= 8 || (self.e >= 0 && self.m % 10i64.pow((8 - self.e) as u32) == 0)
    }

    /// The whole number this is, if it is one that fits.
    pub fn to_i64(self) -> Option<i64> {
        if !self.is_integer() {
            return None;
        }
        if self.m == 0 {
            return Some(0);
        }
        if self.e > 17 {
            return None;
        }
        Some(if self.e >= 8 {
            self.m * 10i64.pow((self.e - 8) as u32)
        } else {
            self.m / 10i64.pow((8 - self.e) as u32)
        })
    }

    /// ip: the integer part, with the sign. ip(−3.5) is −3.
    pub fn ip(self) -> Num {
        if self.m == 0 || self.e >= 8 {
            return self;
        }
        if self.e < 0 {
            return ZERO;
        }
        let cut = 10i64.pow((8 - self.e) as u32);
        Num {
            m: self.m / cut * cut,
            e: self.e,
        }
    }

    /// fp: the fraction part, with the sign. ip(x) + fp(x) is x.
    pub fn fp(self) -> Num {
        self.sub(self.ip()).unwrap_or(ZERO)
    }

    /// dp: the digit part. dp(123.456) is 1.23456.
    pub fn dp(self) -> Num {
        Num { m: self.m, e: 0 }
    }

    /// xp: the exponent part. xp(123.456) is 2.
    pub fn xp(self) -> Num {
        Num::int(self.e as i64)
    }

    pub fn sgn(self) -> Num {
        Num::int(self.sign() as i64)
    }

    pub fn to_f64(self) -> f64 {
        if self.m == 0 {
            return 0.0;
        }
        format!("{}e{}", self.m, self.e - 8).parse().unwrap_or(0.0)
    }

    /// The nearest nine-digit number to `x`.
    pub fn from_f64(x: f64) -> Result<Num, Fault> {
        if x == 0.0 {
            return Ok(ZERO);
        }
        if !x.is_finite() {
            return Err(Fault::Overflow);
        }
        let text = format!("{:.8e}", x.abs());
        let (digits, exp) = text.split_once('e').unwrap_or((&text, "0"));
        let v: i128 = digits.replace('.', "").parse().unwrap_or(0);
        let exp: i32 = exp.parse().unwrap_or(0);
        normal(x < 0.0, v, exp - 8)
    }

    /// Whether this is 1 followed by noughts, or a tenth, a hundredth...
    fn is_power_of_ten(self) -> bool {
        self.m == UNIT
    }

    /// `self` to the power `b`, written `a*b` in JOSS.
    ///
    /// "The general exponential routine to compute a*b, for example, factors
    /// out error situations and the special cases of b = 0, a = 0, b = 1,
    /// b an integer and a an integer power of 10, b = .5, b = -.5, and b an
    /// integer with 2 ≤ b ≤ 29, before resorting to exp[b·log(a)]."
    /// (P-2922, p. 25)
    ///
    /// The order and the methods are those of routine =35 of the January
    /// 1965 listing (pp. 166-168). In particular a whole power below 30 is
    /// done by repeated squaring (cards =035E816-852): the product starts at
    /// 1, and for each binary digit of |b| from the lowest, if it is 1 the
    /// product is multiplied by the current power, then the power is squared;
    /// each multiplication is rounded to nine digits. For a negative b the
    /// first power is 1/a, rounded.
    pub fn pow(self, b: Num) -> Result<Num, Fault> {
        let a = self;
        // Anything to the power 0 is 1, zero included: the listing's
        // comment on the routine is "DOES B=0 ... RESULT IS ONE".
        if b.m == 0 {
            return Ok(ONE);
        }
        if a.m == 0 {
            return if b.m > 0 { Ok(ZERO) } else { Err(Fault::ZeroToNegative) };
        }
        let integral = b.is_integer();
        if a.m < 0 && !integral {
            return Err(Fault::NegativeBase);
        }
        // =035E290-360: a negative number to an odd power is negative. A
        // whole b of ten digits or more ends in a nought, so is even.
        let whole = |n: Num| -> Option<i128> {
            (n.e <= 20).then(|| {
                let m = n.m.abs() as i128;
                if n.e >= 8 {
                    m * 10i128.pow((n.e - 8) as u32)
                } else {
                    m / 10i128.pow((8 - n.e) as u32)
                }
            })
        };
        let odd = integral && b.e <= 8 && whole(b).is_some_and(|w| w % 2 == 1);
        let sign = |n: Num| if a.m < 0 && odd { n.neg() } else { n };
        let a_abs = a.abs();
        // =035E370-390: |a| = 1.
        if a_abs == ONE {
            return Ok(sign(ONE));
        }
        if integral {
            // =035E700-795: a power of ten to a whole power.
            if a_abs.is_power_of_ten() {
                // Past 10^21 the answer is out of range whatever a is.
                let size = whole(b).unwrap_or(i128::MAX / 1000);
                let e = a.e as i128 * if b.m < 0 { -size } else { size };
                if e > MAX_EXP as i128 {
                    return Err(Fault::Overflow);
                }
                if e < MIN_EXP as i128 {
                    return Ok(ZERO);
                }
                return Ok(sign(Num { m: UNIT, e: e as i32 }));
            }
            // =035E800-852: |b| below 30, by repeated squaring.
            if let Some(whole) = whole(b).filter(|w| *w < 30) {
                let mut power = if b.m < 0 { ONE.div(a_abs)? } else { a_abs };
                let mut product = ONE;
                let mut rest = whole;
                loop {
                    if rest & 1 == 1 {
                        product = product.mul(power)?;
                    }
                    rest >>= 1;
                    if rest == 0 {
                        break;
                    }
                    power = power.mul(power)?;
                }
                return Ok(sign(product));
            }
        } else {
            // =035E440-504: b = 1/2 and b = -1/2.
            let half = Num { m: 5 * UNIT, e: -1 };
            if b.abs() == half {
                let root = a.sqrt()?;
                return if b.m > 0 { Ok(root) } else { ONE.div(root) };
            }
        }
        // =035E530-690: exp[b·log(a)], each of the three done to nine
        // digits.
        let magnitude = b.mul(a_abs.log()?)?.exp()?;
        Ok(sign(magnitude))
    }

    /// The natural logarithm: routine F1 of the January 1965 listing.
    pub fn log(self) -> Result<Num, Fault> {
        if self.m <= 0 {
            return Err(Fault::BadLog);
        }
        routines::log(self.m, self.e)
    }

    /// e to the power: routine F2 of the January 1965 listing.
    pub fn exp(self) -> Result<Num, Fault> {
        if self.m == 0 {
            return Ok(ONE);
        }
        routines::exp(self.m < 0, self.m.abs(), self.e)
    }

    /// The number `coef` × 10^(`e` − 8), rounded to nine digits; overflow is
    /// a fault and underflow is zero.
    pub(crate) fn from_parts(negative: bool, coef: i64, e: i32) -> Result<Num, Fault> {
        normal(negative, coef as i128, e - 8)
    }

    pub fn sin(self) -> Result<Num, Fault> {
        self.angle(f64::sin)
    }

    pub fn cos(self) -> Result<Num, Fault> {
        self.angle(f64::cos)
    }

    fn angle(self, f: fn(f64) -> f64) -> Result<Num, Fault> {
        // With nine digits, an angle of 100 or more radians is known to
        // fewer and fewer places of a turn.
        if self.m != 0 && self.e >= 2 {
            return Err(Fault::BadAngle);
        }
        Num::from_f64(f(self.to_f64()))
    }

    /// arg(x, y): the angle of the point [x, y], from −π to π.
    ///
    /// arg(0,0) is 0: routine F10 of the January 1965 listing (pp. 56-57)
    /// tests y = 0 first and sets the angle to zero (card F010E110) before it
    /// looks at x, and the quadrant step that follows leaves a zero angle
    /// alone when x is not negative.
    pub fn arg(x: Num, y: Num) -> Result<Num, Fault> {
        if x.m == 0 && y.m == 0 {
            return Ok(ZERO);
        }
        Num::from_f64(y.to_f64().atan2(x.to_f64()))
    }

    /// The digits of the number as written without an exponent: the digits
    /// before the point and the digits after it, with no needless noughts.
    /// 123.456 gives ("123", "456"); .001 gives ("", "001"); 5 gives ("5", "").
    pub fn fixed_digits(self) -> (String, String) {
        if self.m == 0 {
            return ("0".into(), String::new());
        }
        let nine = self.m.abs().to_string();
        let (whole, fraction) = if self.e >= 8 {
            (format!("{}{}", nine, "0".repeat((self.e - 8) as usize)), String::new())
        } else if self.e >= 0 {
            let at = (self.e + 1) as usize;
            (nine[..at].to_string(), nine[at..].to_string())
        } else {
            (String::new(), format!("{}{}", "0".repeat((-self.e - 1) as usize), nine))
        };
        (whole, fraction.trim_end_matches('0').to_string())
    }

    /// This number rounded to `places` digits after the decimal point.
    pub fn round_places(self, places: i32) -> Num {
        if self.m == 0 {
            return self;
        }
        // The digit worth 10^(−places) is digit number (e + places) counting
        // the first as 0. Keep that many + 1 digits.
        let keep = self.e + places + 1;
        if keep >= DIGITS as i32 {
            return self;
        }
        if keep < 0 {
            return ZERO;
        }
        let div = 10i64.pow(DIGITS - keep as u32);
        let mut v = self.m.abs() / div;
        if (self.m.abs() % div) * 2 >= div {
            v += 1;
        }
        normal(self.m < 0, v as i128, self.e - 8 + (DIGITS as i32 - keep)).unwrap_or(self)
    }

    /// This number rounded to `n` significant digits (1 to 9).
    pub fn round_digits(self, n: u32) -> Num {
        if self.m == 0 || n >= DIGITS {
            return self;
        }
        self.round_places(n as i32 - 1 - self.e)
    }
}

impl PartialOrd for Num {
    fn partial_cmp(&self, other: &Num) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Num {
    fn cmp(&self, other: &Num) -> Ordering {
        match (self.sign(), other.sign()) {
            (a, b) if a != b => a.cmp(&b),
            (0, _) => Ordering::Equal,
            (1, _) => self.e.cmp(&other.e).then(self.m.cmp(&other.m)),
            _ => other.e.cmp(&self.e).then(self.m.cmp(&other.m)),
        }
    }
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
    fn a_tenth_ten_times_is_one() {
        // "The user may ask JOSS to add .1 ten times and the result is indeed 1." (P-3149, p. 8)
        let mut sum = ZERO;
        for _ in 0..10 {
            sum = sum.add(n(".1")).unwrap();
        }
        assert_eq!(sum, ONE);
    }

    #[test]
    fn results_are_rounded_to_nine_digits() {
        assert_eq!(ONE.div(n("3")).unwrap(), n(".333333333"));
        assert_eq!(n("2").div(n("3")).unwrap(), n(".666666667"));
        assert_eq!(
            n("123456789").mul(n("10")).unwrap().add(n("5")).unwrap(),
            n("1234567900")
        );
        assert_eq!(n("999999999").add(n(".5")).unwrap(), n("1000000000"));
        assert_eq!(n("2").sqrt().unwrap(), n("1.41421356"));
        assert_eq!(n("3").sqrt().unwrap(), n("1.73205081"));
        assert_eq!(n("4").sqrt().unwrap(), n("2"));
        assert_eq!(n(".25").sqrt().unwrap(), n(".5"));
    }

    #[test]
    fn parts_of_a_number() {
        let y = n("123.456");
        assert_eq!(y.ip(), n("123"));
        assert_eq!(y.fp(), n(".456"));
        assert_eq!(y.dp(), n("1.23456"));
        assert_eq!(y.xp(), n("2"));
        assert_eq!(n("-3.5").ip(), n("-3"));
        assert_eq!(n("-3.5").fp(), n("-.5"));
        assert_eq!(n("-3.5").sgn(), n("-1"));
    }

    #[test]
    fn overflow_is_a_fault_and_underflow_is_zero() {
        let big = Num { m: UNIT, e: 99 };
        assert_eq!(big.mul(n("10")), Err(Fault::Overflow));
        let small = Num { m: UNIT, e: -99 };
        assert_eq!(small.div(n("10")), Ok(ZERO));
        assert_eq!(ONE.div(ZERO), Err(Fault::ZeroDivisor));
        assert_eq!(n("-1").sqrt(), Err(Fault::NegativeSqrt));
    }

    #[test]
    fn powers() {
        assert_eq!(n("3").pow(n("2")).unwrap(), n("9"));
        assert_eq!(n("10").pow(n("2")).unwrap(), n("100"));
        assert_eq!(n("10").pow(n("-3")).unwrap(), n(".001"));
        assert_eq!(n("2").pow(n(".5")).unwrap(), n("1.41421356"));
        assert_eq!(n("-2").pow(n("3")).unwrap(), n("-8"));
        assert_eq!(n("2").pow(n("-2")).unwrap(), n(".25"));
        assert_eq!(n("-8").pow(n(".5")), Err(Fault::NegativeBase));
        assert_eq!(ZERO.pow(ZERO), Ok(ONE));
        assert_eq!(ZERO.pow(n("-1")), Err(Fault::ZeroToNegative));
    }

    #[test]
    fn powers_follow_the_listing() {
        // Routine =35 of the January 1965 listing. Whole powers below 30 by
        // repeated squaring, each product rounded: not the correctly rounded
        // 15.4772823.
        assert_eq!(n("1.23456789").pow(n("13")).unwrap(), n("15.4772825"));
        // A power of ten to a large whole power: overflow, or underflow to
        // zero, without the exponent arithmetic itself overflowing.
        let big = Num { m: UNIT, e: 99 };
        let small = Num { m: UNIT, e: -99 };
        let huge = Num { m: UNIT, e: 17 };
        assert_eq!(big.pow(huge), Err(Fault::Overflow));
        assert_eq!(small.pow(huge), Ok(ZERO));
        assert_eq!(big.pow(Num { m: UNIT, e: 99 }), Err(Fault::Overflow));
        // A whole power too large for an i64 is still whole, and even.
        assert_eq!(n("-1").pow(Num { m: UNIT, e: 18 }), Ok(ONE));
        assert_eq!(n("-1").pow(n("3")), Ok(n("-1")));
        assert_eq!(n("-2").pow(Num { m: UNIT, e: 18 }), Err(Fault::Overflow));
        assert_eq!(n("2").pow(n("-3")).unwrap(), n(".125"));
        assert_eq!(n("-2").pow(n("-3")).unwrap(), n("-.125"));
        assert_eq!(n("3").pow(n("-.5")).unwrap(), n(".577350268"));
    }

    #[test]
    fn arg_of_the_origin_is_zero() {
        // Routine F10 tests y = 0 before x (card F010E110).
        assert_eq!(Num::arg(ZERO, ZERO), Ok(ZERO));
    }

    #[test]
    fn the_functions_hit_their_magic_values() {
        // Figure 3b of P-2922.
        let e = n("2.71828183");
        assert_eq!(ONE.log().unwrap(), ZERO);
        // The listing cuts a log below 1 off at nine digits: see routines.rs.
        assert_eq!(n("2").log().unwrap(), n(".69314718"));
        assert_eq!(e.log().unwrap(), ONE);
        assert_eq!(ZERO.exp().unwrap(), ONE);
        assert_eq!(n(".5").exp().unwrap(), n("1.64872127"));
        assert_eq!(ONE.exp().unwrap(), e);
        assert_eq!(n(".5").sin().unwrap(), n(".479425539"));
        assert_eq!(n(".5").cos().unwrap(), n(".877582562"));
        assert_eq!(Num::arg(ONE, ZERO).unwrap(), ZERO);
        assert_eq!(Num::arg(ZERO, ONE).unwrap(), n("1.57079633"));
        assert_eq!(Num::arg(n("-1"), ZERO).unwrap(), n("3.14159265"));
    }

    #[test]
    fn ordering() {
        assert!(n("-2") < n("-1"));
        assert!(n("-1") < ZERO);
        assert!(n(".5") < n("5"));
        assert!(n("100") > n("99.9999999"));
        assert!(n("-100") < n("-99.9999999"));
    }

    #[test]
    fn rounding_for_output() {
        assert_eq!(n("1.41421356").round_places(3), n("1.414"));
        assert_eq!(n("2.5").round_places(0), n("3"));
        assert_eq!(n("-2.5").round_places(0), n("-3"));
        assert_eq!(n(".0004").round_places(3), ZERO);
        assert_eq!(n(".0005").round_places(3), n(".001"));
        assert_eq!(n("9.9996").round_places(3), n("10"));
        assert_eq!(n("123456").round_digits(2), n("120000"));
        assert_eq!(n("123.456").fixed_digits(), ("123".to_string(), "456".to_string()));
        assert_eq!(n(".001").fixed_digits(), (String::new(), "001".to_string()));
        assert_eq!(n("1500").fixed_digits(), ("1500".to_string(), String::new()));
    }
}
