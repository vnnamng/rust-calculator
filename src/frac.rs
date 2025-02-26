use std::fmt;

use crate::big_num::BigNum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Frac {
    numerator: BigNum,
    denominator: BigNum,
}

impl Frac {
    fn simplify(&self) -> Frac {
        let gcd = self.numerator.gcd(&self.denominator);
        let numerator = self.numerator.clone() / gcd.clone();
        let denominator = self.denominator.clone() / gcd;

        // Ensure the denominator is always positive
        let (numerator, denominator) = if denominator.is_negative() {
            (-numerator, -denominator)
        } else {
            (numerator, denominator)
        };

        Frac {
            numerator,
            denominator,
        }
    }

    fn is_simplified(&self) -> bool {
        self.numerator.gcd(&self.denominator) == BigNum::from_str("1").unwrap()
    }

    pub fn new(numerator: BigNum, denominator: BigNum) -> Self {
        if denominator.is_zero() {
            panic!("Denominator cannot be zero");
        };
        Frac {
            numerator,
            denominator,
        }
        .simplify()
    }

    pub fn inverse(&self) -> Frac {
        Frac::new(self.denominator.clone(), self.numerator.clone())
    }

    pub fn from_bignum(num: BigNum) -> Frac {
        Frac::new(num, BigNum::from_str("1").unwrap())
    }

    pub fn is_bignum(&self) -> bool {
        self.denominator == BigNum::from_str("1").unwrap() || (self.numerator.is_zero())
    }

    pub fn to_bignum(&self) -> Result<BigNum, String> {
        if self.is_bignum() {
            Ok(self.numerator.clone())
        } else {
            Err("Fraction cannot be converted to BigNum".to_string())
        }
    }
}

pub trait IntoFrac {
    fn to_frac(self) -> Frac;
}

impl IntoFrac for Frac {
    fn to_frac(self) -> Frac {
        self
    }
}

impl fmt::Display for Frac {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl FromStr for Frac {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 2 {
            return Err(
                "Invalid fraction format. Expected format: numerator/denominator".to_string(),
            );
        }

        let numerator = parts[0].parse::<BigNum>()?;
        let denominator = parts[1].parse::<BigNum>()?;

        if denominator.is_zero() {
            return Err("Denominator cannot be zero".to_string());
        }

        Ok(Frac::new(numerator, denominator))
    }
}

impl PartialEq for Frac {
    fn eq(&self, other: &Self) -> bool {
        self.numerator.clone() * other.denominator.clone()
            == self.denominator.clone() * other.numerator.clone()
            && self.is_simplified()
            && other.is_simplified()
    }
}

impl Neg for Frac {
    type Output = Frac;

    fn neg(self) -> Self::Output {
        Frac::new(-self.numerator, self.denominator)
    }
}

impl Add for Frac {
    type Output = Frac;

    fn add(self, other: Self) -> Self::Output {
        let numerator = self.numerator.clone() * other.denominator.clone()
            + self.denominator.clone() * other.numerator.clone();
        let denominator = self.denominator.clone() * other.denominator.clone();
        Frac::new(numerator, denominator)
    }
}

impl Add<BigNum> for Frac {
    type Output = Frac;

    fn add(self, other: BigNum) -> Self::Output {
        self + other.to_frac()
    }
}

impl AddAssign for Frac {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Sub for Frac {
    type Output = Frac;

    fn sub(self, other: Self) -> Self::Output {
        self + (-other)
    }
}

impl Sub<BigNum> for Frac {
    type Output = Frac;

    fn sub(self, other: BigNum) -> Self::Output {
        self - other.to_frac()
    }
}

impl SubAssign for Frac {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}

impl Mul for Frac {
    type Output = Frac;

    fn mul(self, other: Self) -> Self::Output {
        let numerator = self.numerator.clone() * other.numerator.clone();
        let denominator = self.denominator.clone() * other.denominator.clone();
        Frac::new(numerator, denominator)
    }
}

impl Mul<BigNum> for Frac {
    type Output = Frac;

    fn mul(self, other: BigNum) -> Self::Output {
        self * other.to_frac()
    }
}

impl MulAssign for Frac {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl Div for Frac {
    type Output = Frac;

    fn div(self, other: Self) -> Self::Output {
        self * other.inverse()
    }
}

impl Div<BigNum> for Frac {
    type Output = Frac;

    fn div(self, other: BigNum) -> Self::Output {
        self / other.to_frac()
    }
}

impl DivAssign for Frac {
    fn div_assign(&mut self, other: Self) {
        *self = self.clone() / other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod test_simplify {
        use super::*;

        #[test]
        fn test_simplify() {
            let frac = Frac::new(
                BigNum::from_str("2").unwrap(),
                BigNum::from_str("4").unwrap(),
            );
            assert_eq!(frac.numerator, BigNum::from_str("1").unwrap());
            assert_eq!(frac.denominator, BigNum::from_str("2").unwrap());
        }

        #[test]
        fn test_simplify_negative_numerator() {
            let frac = Frac::new(
                BigNum::from_str("-2").unwrap(),
                BigNum::from_str("4").unwrap(),
            );
            assert_eq!(frac.numerator, BigNum::from_str("-1").unwrap());
            assert_eq!(frac.denominator, BigNum::from_str("2").unwrap());
        }

        #[test]
        fn test_simplify_negative_denominator() {
            let frac = Frac::new(
                BigNum::from_str("2").unwrap(),
                BigNum::from_str("-4").unwrap(),
            );
            assert_eq!(frac.numerator, BigNum::from_str("-1").unwrap());
            assert_eq!(frac.denominator, BigNum::from_str("2").unwrap());

            let frac = Frac::new(
                BigNum::from_str("-2").unwrap(),
                BigNum::from_str("-4").unwrap(),
            );
            assert_eq!(frac.numerator, BigNum::from_str("1").unwrap());
            assert_eq!(frac.denominator, BigNum::from_str("2").unwrap());
        }
    }

    mod test_inverse {
        use super::*;

        #[test]
        fn test_inverse() {
            let frac = Frac::new(
                BigNum::from_str("2").unwrap(),
                BigNum::from_str("4").unwrap(),
            );
            let inverse = frac.inverse();
            assert_eq!(inverse.numerator, BigNum::from_str("2").unwrap());
            assert_eq!(inverse.denominator, BigNum::from_str("1").unwrap());
        }
    }

    mod test_display {
        use super::*;

        #[test]
        fn test_display() {
            let frac = Frac::new(
                BigNum::from_str("2").unwrap(),
                BigNum::from_str("4").unwrap(),
            );
            assert_eq!(frac.to_string(), "1/2");
        }
    }

    mod test_from_str {
        use super::*;

        #[test]
        fn test_from_str() {
            let frac = Frac::from_str("2/4").unwrap();
            assert_eq!(frac.numerator, BigNum::from_str("1").unwrap());
            assert_eq!(frac.denominator, BigNum::from_str("2").unwrap());
        }

        #[test]
        fn test_from_str_invalid_format() {
            let frac = Frac::from_str("2");
            assert_eq!(
                frac.err().unwrap(),
                "Invalid fraction format. Expected format: numerator/denominator"
            );
        }

        #[test]
        fn test_from_str_invalid_denominator() {
            let frac = Frac::from_str("2/0");
            assert_eq!(frac.err().unwrap(), "Denominator cannot be zero");
        }
    }

    mod test_eq {
        use super::*;

        #[test]
        fn test_eq() {
            let frac1 = Frac::new(
                BigNum::from_str("2").unwrap(),
                BigNum::from_str("4").unwrap(),
            );
            let frac2 = Frac::new(
                BigNum::from_str("1").unwrap(),
                BigNum::from_str("2").unwrap(),
            );
            assert_eq!(frac1, frac2);
        }
    }

    mod test_neg {
        use super::*;

        #[test]
        fn test_neg() {
            let frac = Frac::new(
                BigNum::from_str("2").unwrap(),
                BigNum::from_str("4").unwrap(),
            );
            let neg = -frac;
            assert_eq!(neg.numerator, BigNum::from_str("-1").unwrap());
            assert_eq!(neg.denominator, BigNum::from_str("2").unwrap());
        }
    }

    mod test_add {
        use super::*;

        #[test]
        fn test_add() {
            let frac1 = Frac::new(
                BigNum::from_str("1").unwrap(),
                BigNum::from_str("2").unwrap(),
            );
            let frac2 = Frac::new(
                BigNum::from_str("1").unwrap(),
                BigNum::from_str("3").unwrap(),
            );
            let sum = frac1 + frac2;
            assert_eq!(sum.numerator, BigNum::from_str("5").unwrap());
            assert_eq!(sum.denominator, BigNum::from_str("6").unwrap());
        }
    }

    mod test_add_assign {
        use super::*;

        #[test]
        fn test_add_assign() {
            let mut frac1 = Frac::new(
                BigNum::from_str("1").unwrap(),
                BigNum::from_str("2").unwrap(),
            );
            let frac2 = Frac::new(
                BigNum::from_str("1").unwrap(),
                BigNum::from_str("3").unwrap(),
            );
            frac1 += frac2;
            assert_eq!(frac1.numerator, BigNum::from_str("5").unwrap());
            assert_eq!(frac1.denominator, BigNum::from_str("6").unwrap());
        }
    }

    mod test_sub {
        use super::*;

        #[test]
        fn test_sub() {
            let frac1 = Frac::new(
                BigNum::from_str("1").unwrap(),
                BigNum::from_str("2").unwrap(),
            );
            let frac2 = Frac::new(
                BigNum::from_str("1").unwrap(),
                BigNum::from_str("3").unwrap(),
            );
            let diff = frac2 - frac1;
            assert_eq!(diff.numerator, BigNum::from_str("-1").unwrap());
            assert_eq!(diff.denominator, BigNum::from_str("6").unwrap());
        }
    }

    mod test_sub_assign {
        use super::*;

        #[test]
        fn test_sub_assign() {
            let mut frac1 = Frac::new(
                BigNum::from_str("1").unwrap(),
                BigNum::from_str("2").unwrap(),
            );
            let frac2 = Frac::new(
                BigNum::from_str("1").unwrap(),
                BigNum::from_str("3").unwrap(),
            );
            frac1 -= frac2;
            assert_eq!(frac1.numerator, BigNum::from_str("1").unwrap());
            assert_eq!(frac1.denominator, BigNum::from_str("6").unwrap());
        }
    }

    mod test_mul {
        use super::*;

        #[test]
        fn test_mul() {
            let frac1 = Frac::new(
                BigNum::from_str("1").unwrap(),
                BigNum::from_str("2").unwrap(),
            );
            let frac2 = Frac::new(
                BigNum::from_str("2").unwrap(),
                BigNum::from_str("3").unwrap(),
            );
            let product = frac1 * frac2;
            assert_eq!(product.numerator, BigNum::from_str("1").unwrap());
            assert_eq!(product.denominator, BigNum::from_str("3").unwrap());
        }
    }

    mod test_mul_assign {
        use super::*;

        #[test]
        fn test_mul_assign() {
            let mut frac1 = Frac::new(
                BigNum::from_str("2").unwrap(),
                BigNum::from_str("3").unwrap(),
            );
            let frac2 = Frac::new(
                BigNum::from_str("2").unwrap(),
                BigNum::from_str("36").unwrap(),
            );
            frac1 *= frac2;
            assert_eq!(frac1.numerator, BigNum::from_str("1").unwrap());
            assert_eq!(frac1.denominator, BigNum::from_str("27").unwrap());
        }
    }

    mod test_div {
        use super::*;

        #[test]
        fn test_div() {
            let frac1 = Frac::new(
                BigNum::from_str("1").unwrap(),
                BigNum::from_str("2").unwrap(),
            );
            let frac2 = Frac::new(
                BigNum::from_str("2").unwrap(),
                BigNum::from_str("3").unwrap(),
            );
            let quotient = frac1 / frac2;
            assert_eq!(quotient.numerator, BigNum::from_str("3").unwrap());
            assert_eq!(quotient.denominator, BigNum::from_str("4").unwrap());
        }
    }

    mod test_div_assign {
        use super::*;

        #[test]
        fn test_div_assign() {
            let mut frac1 = Frac::new(
                BigNum::from_str("1").unwrap(),
                BigNum::from_str("2").unwrap(),
            );
            let frac2 = Frac::new(
                BigNum::from_str("2").unwrap(),
                BigNum::from_str("3").unwrap(),
            );
            frac1 /= frac2;
            assert_eq!(frac1.numerator, BigNum::from_str("3").unwrap());
            assert_eq!(frac1.denominator, BigNum::from_str("4").unwrap());
        }
    }
}
