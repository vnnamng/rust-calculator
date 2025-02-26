use std::cmp::Ordering;
use std::fmt;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use std::str::FromStr;
use std::string::ToString;

use crate::frac::{Frac, IntoFrac};
#[derive(Clone, Debug)]
pub struct BigNum {
    sign: bool,   // true = positive, false = negative
    num: Vec<u8>, // Stores digits in most significant to least significant order
}

impl BigNum {
    fn new() -> BigNum {
        BigNum {
            sign: true,
            num: vec![0],
        }
    }

    pub fn from(num: Vec<u8>, sign: bool) -> BigNum {
        if BigNum::is_num_zero(&num) {
            return BigNum::zero();
        }
        BigNum {
            sign,
            num: BigNum::remove_leading_zeros(num),
        }
    }

    pub fn abs(&self) -> BigNum {
        BigNum {
            sign: true,
            num: self.num.clone(),
        }
    }

    fn set_sign(&mut self, sign: bool) {
        self.sign = sign;
    }

    fn remove_leading_zeros(num: Vec<u8>) -> Vec<u8> {
        num.into_iter().skip_while(|n| *n == 0).collect()
    }

    pub fn zero() -> BigNum {
        BigNum::new()
    }

    fn is_num_zero(num: &Vec<u8>) -> bool {
        num.iter().all(|&n| n == 0)
    }
    pub fn is_zero(&self) -> bool {
        BigNum::is_num_zero(&self.num)
    }

    pub fn is_negative(&self) -> bool {
        self.sign == false
    }

    pub fn negate(&self) -> BigNum {
        BigNum {
            sign: !self.sign,
            num: self.num.clone(),
        }
    }

    pub fn gcd(&self, other: &BigNum) -> Result<BigNum, String> {
        // GCD of 2 zeroes is undefined, so return an error
        if self.is_zero() && other.is_zero() {
            return Err("GCD of 2 zeroes is undefined".to_string());
        }
        // GCD of a number and 0 is the number itself
        if self.is_zero() {
            return Ok(other.abs());
        }

        if other.is_zero() {
            return Ok(self.abs());
        }

        let mut a = self.abs();
        let mut b = other.abs();
        while !b.is_zero() {
            let temp = b.clone();
            b = a.clone() % b;
            a = temp;
        }
        Ok(a)
    }

    fn one() -> BigNum {
        BigNum::from(vec![1], true)
    }
}

// Implementing Display for BigNum
impl fmt::Display for BigNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.sign && !self.is_zero() {
            write!(f, "-")?;
        }
        for &n in &self.num {
            write!(f, "{}", n)?;
        }
        Ok(())
    }
}

impl FromStr for BigNum {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().peekable();
        let mut sign = true;
        if let Some(&c) = chars.peek() {
            if c == '-' {
                sign = false;
                chars.next();
            } else if c == '+' {
                chars.next();
            }
        }

        let digits: Vec<u8> = chars
            .map(|c| {
                c.to_digit(10)
                    .ok_or_else(|| format!("Invalid character: {}", c))
            })
            .collect::<Result<Vec<u32>, _>>()?
            .into_iter()
            .map(|d| d as u8)
            .collect();

        if digits.is_empty() {
            return Err("Invalid number format".to_string());
        }

        Ok(BigNum { sign, num: digits })
    }
}

impl IntoFrac for BigNum {
    fn to_frac(self) -> Frac {
        Frac::new(self, BigNum::one())
    }
}

impl Neg for BigNum {
    type Output = BigNum;

    fn neg(self) -> BigNum {
        self.negate()
    }
}

impl Add for BigNum {
    type Output = BigNum;

    fn add(self: BigNum, other: BigNum) -> BigNum {
        let mut num1 = self.num.clone();
        let mut num2 = other.num.clone();

        let max_len = num1.len().max(num2.len());
        while num1.len() < max_len {
            num1.insert(0, 0);
        }
        while num2.len() < max_len {
            num2.insert(0, 0);
        }
        if self.sign == other.sign {
            let mut result = vec![0; max_len + 1];
            let mut carry = 0;

            for i in (0..max_len).rev() {
                let sum = num1[i] + num2[i] + carry;
                result[i + 1] = sum % 10;
                carry = sum / 10;
            }
            result[0] = carry;

            while result.len() > 1 && result[0] == 0 {
                result.remove(0);
            }

            return BigNum {
                sign: self.sign,
                num: result,
            };
        } else {
            let (larger, smaller, result_sign) = if num1 > num2 {
                (num1, num2, self.sign)
            } else {
                (num2, num1, other.sign)
            };

            let mut result = vec![0; max_len];
            let mut borrow = 0;

            for i in (0..max_len).rev() {
                let mut diff = larger[i] as i8 - smaller[i] as i8 - borrow;
                if diff < 0 {
                    diff += 10;
                    borrow = 1;
                } else {
                    borrow = 0;
                }
                result[i] = diff as u8;
            }

            while result.len() > 1 && result[0] == 0 {
                result.remove(0);
            }

            return BigNum {
                sign: result_sign,
                num: result,
            };
        }
    }
}

impl Add<Frac> for BigNum {
    type Output = Frac;

    fn add(self: BigNum, other: Frac) -> Frac {
        self.to_frac() + other
    }
}

impl AddAssign for BigNum {
    fn add_assign(&mut self, other: BigNum) {
        *self = self.clone() + other;
    }
}

impl PartialEq for BigNum {
    fn eq(&self, other: &BigNum) -> bool {
        self.sign == other.sign && self.num == other.num
    }
}

impl PartialOrd for BigNum {
    fn partial_cmp(&self, other: &BigNum) -> Option<Ordering> {
        if self.sign && !other.sign {
            return Some(Ordering::Greater);
        }
        if !self.sign && other.sign {
            return Some(Ordering::Less);
        }
        if self.num.len() > other.num.len() {
            return Some(if self.sign {
                Ordering::Greater
            } else {
                Ordering::Less
            });
        }
        if self.num.len() < other.num.len() {
            return Some(if self.sign {
                Ordering::Less
            } else {
                Ordering::Greater
            });
        }
        for (&n1, &n2) in self.num.iter().zip(other.num.iter()) {
            if n1 > n2 {
                return Some(if self.sign {
                    Ordering::Greater
                } else {
                    Ordering::Less
                });
            }
            if n1 < n2 {
                return Some(if self.sign {
                    Ordering::Less
                } else {
                    Ordering::Greater
                });
            }
        }
        Some(Ordering::Equal)
    }
}

impl Sub for BigNum {
    type Output = BigNum;

    fn sub(self: BigNum, other: BigNum) -> BigNum {
        self + (-other)
    }
}

impl Sub<Frac> for BigNum {
    type Output = Frac;

    fn sub(self: BigNum, other: Frac) -> Frac {
        self.to_frac() - other
    }
}

impl SubAssign for BigNum {
    fn sub_assign(&mut self, other: BigNum) {
        *self = self.clone() - other;
    }
}

impl Mul for BigNum {
    type Output = BigNum;

    fn mul(self: BigNum, other: BigNum) -> BigNum {
        let mut result = BigNum::zero();
        for (i, &n) in other.num.iter().rev().enumerate() {
            let mut temp = vec![0; i];
            let mut carry = 0;
            for &m in self.num.iter().rev() {
                let product = n * m + carry;
                temp.insert(0, product % 10);
                carry = product / 10;
            }
            if carry > 0 {
                temp.insert(0, carry);
            }
            result += BigNum::from(temp, true);
        }
        if self.sign != other.sign {
            result.negate()
        } else {
            result
        }
    }
}

impl Mul<Frac> for BigNum {
    type Output = Frac;

    fn mul(self: BigNum, other: Frac) -> Frac {
        self.to_frac() * other
    }
}

impl MulAssign for BigNum {
    fn mul_assign(&mut self, other: BigNum) {
        *self = self.clone() * other;
    }
}

impl Div for BigNum {
    type Output = Self;

    // The quotient has the same sign as the dividend multiplied by the divisor
    #[inline]
    fn div(self, other: Self) -> Self::Output {
        if other.is_zero() {
            panic!("Division by zero");
        }
        if self.is_zero() {
            return BigNum::zero();
        }
        let self_sign: bool = self.sign;
        let other_sign = other.sign;
        let other = other.abs();
        let self_abs = self.abs();
        let mut result = BigNum::zero();
        let mut remainder = BigNum::zero();
        for &n in &self_abs.num {
            remainder = remainder * BigNum::from(vec![10], true) + BigNum::from(vec![n], true);
            let mut count = BigNum::zero();
            while &remainder >= &other {
                remainder -= other.clone();
                count += BigNum::from(vec![1], true);
            }
            result = result * BigNum::from(vec![10], true) + count;
        }
        if self_sign != other_sign && !result.is_zero() {
            result.set_sign(false);
        } else {
            result.set_sign(true);
        }
        result
    }
}

impl Div<Frac> for BigNum {
    type Output = Frac;

    fn div(self: BigNum, other: Frac) -> Frac {
        self.to_frac() / other
    }
}

impl DivAssign for BigNum {
    fn div_assign(&mut self, other: BigNum) {
        *self = self.clone() / other;
    }
}

impl Rem for BigNum {
    type Output = BigNum;

    // The remainder has the same sign as the dividend
    fn rem(self: BigNum, other: BigNum) -> BigNum {
        let self_sign: bool = self.sign;
        let mut reminder = self.clone() - (self / other.clone()) * other;
        reminder.set_sign(self_sign);
        reminder
    }
}

impl RemAssign for BigNum {
    fn rem_assign(&mut self, other: BigNum) {
        *self = self.clone() % other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_neg {
        use super::*;

        #[test]
        fn test_negate_positive() {
            let num = BigNum::from(vec![1, 2, 3], true);
            let expected = BigNum::from(vec![1, 2, 3], false);
            assert_eq!(num.negate(), expected);
            assert_eq!(-num, expected);
        }

        #[test]
        fn test_negate_negative() {
            let num = BigNum::from(vec![1, 2, 3], false);
            let expected = BigNum::from(vec![1, 2, 3], true);
            assert_eq!(num.negate(), expected);
            assert_eq!(-num, expected);
        }
    }

    mod test_add {
        use super::*;

        #[test]
        fn test_add_positive_positive() {
            let num1 = BigNum::from(vec![1, 2, 3], true);
            let num2 = BigNum::from(vec![4, 5, 6], true);
            let expected = BigNum::from(vec![5, 7, 9], true);
            assert_eq!(num1 + num2, expected);
        }

        #[test]
        fn test_add_positive_negative() {
            let num1 = BigNum::from(vec![1, 2, 3], true);
            let num2 = BigNum::from(vec![4, 5, 6], false);
            let expected = BigNum::from(vec![3, 3, 3], false);
            assert_eq!(num1 + num2, expected);

            let num3 = BigNum::from(vec![6, 7, 8], true);
            let num4 = BigNum::from(vec![4, 5, 6], false);
            let expected = BigNum::from(vec![2, 2, 2], true);
            assert_eq!(num3 + num4, expected);
        }

        #[test]
        fn test_add_negative_positive() {
            let num1 = BigNum::from(vec![1, 2, 3], false);
            let num2 = BigNum::from(vec![4, 5, 6], true);
            let expected = BigNum::from(vec![3, 3, 3], true);
            assert_eq!(num1 + num2, expected);

            let num3 = BigNum::from(vec![6, 7, 8], false);
            let num4 = BigNum::from(vec![4, 5, 6], true);
            let expected = BigNum::from(vec![2, 2, 2], false);
            assert_eq!(num3 + num4, expected);
        }

        #[test]
        fn test_add_negative_negative() {
            let num1 = BigNum::from(vec![1, 2, 3], false);
            let num2 = BigNum::from(vec![4, 5, 6], false);
            let expected = BigNum::from(vec![5, 7, 9], false);
            assert_eq!(num1 + num2, expected);
        }
    }

    mod test_add_assign {
        use super::*;

        #[test]
        fn test_add_assign_positive_positive() {
            let mut num1 = BigNum::from(vec![1, 2, 3], true);
            let num2 = BigNum::from(vec![4, 5, 6], true);
            let expected = BigNum::from(vec![5, 7, 9], true);
            num1 += num2;
            assert_eq!(num1, expected);
        }

        #[test]
        fn test_add_assign_positive_negative() {
            let mut num1 = BigNum::from(vec![1, 2, 3], true);
            let num2 = BigNum::from(vec![4, 5, 6], false);
            let expected = BigNum::from(vec![3, 3, 3], false);
            num1 += num2;
            assert_eq!(num1, expected);

            let mut num3 = BigNum::from(vec![6, 7, 8], true);
            let num4 = BigNum::from(vec![4, 5, 6], false);
            let expected = BigNum::from(vec![2, 2, 2], true);
            num3 += num4;
            assert_eq!(num3, expected);
        }

        #[test]
        fn test_add_assign_negative_positive() {
            let mut num1 = BigNum::from(vec![1, 2, 3], false);
            let num2 = BigNum::from(vec![4, 5, 6], true);
            let expected = BigNum::from(vec![3, 3, 3], true);
            num1 += num2;
            assert_eq!(num1, expected);

            let mut num3 = BigNum::from(vec![6, 7, 8], false);
            let num4 = BigNum::from(vec![4, 5, 6], true);
            let expected = BigNum::from(vec![2, 2, 2], false);
            num3 += num4;
            assert_eq!(num3, expected);
        }

        #[test]
        fn test_add_assign_negative_negative() {
            let mut num1 = BigNum::from(vec![1, 2, 3], false);
            let num2 = BigNum::from(vec![4, 5, 6], false);
            let expected = BigNum::from(vec![5, 7, 9], false);
            num1 += num2;
            assert_eq!(num1, expected);
        }
    }

    mod test_mul {
        use super::*;

        #[test]
        fn test_mul_positive_positive() {
            let num1 = BigNum::from(vec![1, 2, 3], true);
            let num2 = BigNum::from(vec![4, 5, 6], true);
            let expected = BigNum::from(vec![5, 6, 0, 8, 8], true);
            assert_eq!(num1 * num2, expected);
        }

        #[test]
        fn test_mul_positive_negative() {
            let num1 = BigNum::from(vec![1, 2, 3], true);
            let num2 = BigNum::from(vec![4, 5, 6], false);
            let expected = BigNum::from(vec![5, 6, 0, 8, 8], false);
            assert_eq!(num1 * num2, expected);

            let num3 = BigNum::from(vec![6, 7, 8], true);
            let num4 = BigNum::from(vec![4, 5, 6], false);
            let expected = BigNum::from(vec![3, 0, 9, 1, 6, 8], false);
            assert_eq!(num3 * num4, expected);
        }

        #[test]
        fn test_mul_negative_positive() {
            let num1 = BigNum::from(vec![1, 2, 3], false);
            let num2 = BigNum::from(vec![4, 5, 6], true);
            let expected = BigNum::from(vec![5, 6, 0, 8, 8], false);
            assert_eq!(num1 * num2, expected);

            let num3 = BigNum::from(vec![6, 7, 8], false);
            let num4 = BigNum::from(vec![4, 5, 6], true);
            let expected = BigNum::from(vec![3, 0, 9, 1, 6, 8], false);
            assert_eq!(num3 * num4, expected);
        }

        #[test]
        fn test_mul_negative_negative() {
            let num1 = BigNum::from(vec![1, 2, 3], false);
            let num2 = BigNum::from(vec![4, 5, 6], false);
            let expected = BigNum::from(vec![5, 6, 0, 8, 8], true);
            assert_eq!(num1 * num2, expected);
        }

        #[test]
        fn test_mul_zero() {
            let num1 = BigNum::from(vec![1, 2, 3], true);
            let num2 = BigNum::zero();
            let expected = BigNum::zero();
            assert_eq!(num1 * num2, expected);
        }
    }

    mod test_eq_ord {
        use super::*;

        #[test]
        fn test_eq() {
            let num1 = BigNum::from(vec![1, 2, 3], true);
            let num2 = BigNum::from(vec![1, 2, 3], true);
            assert_eq!(num1, num2);
        }

        #[test]
        fn test_ne() {
            let num1 = BigNum::from(vec![1, 2, 3], true);
            let num2 = BigNum::from(vec![1, 2, 4], true);
            assert_ne!(num1, num2);
        }

        #[test]
        fn test_lt() {
            let num1 = BigNum::from(vec![1, 2, 3], true);
            let num2 = BigNum::from(vec![1, 2, 4], true);
            assert!(num1 < num2);
        }

        #[test]
        fn test_le() {
            let num1 = BigNum::from(vec![1, 2, 3], true);
            let num2 = BigNum::from(vec![1, 2, 4], true);
            assert!(num1 <= num2);
            let num3 = BigNum::from(vec![1, 2, 3], true);
            let num4 = BigNum::from(vec![1, 2, 3], true);
            assert!(num3 <= num4);
        }

        #[test]
        fn test_gt() {
            let num1 = BigNum::from(vec![1, 2, 4], true);
            let num2 = BigNum::from(vec![1, 2, 3], true);
            assert!(num1 > num2);
        }

        #[test]
        fn test_ge() {
            let num1 = BigNum::from(vec![1, 2, 4], true);
            let num2 = BigNum::from(vec![1, 2, 3], true);
            assert!(num1 >= num2);
            let num3 = BigNum::from(vec![1, 2, 3], true);
            let num4 = BigNum::from(vec![1, 2, 3], true);
            assert!(num3 >= num4);
        }
    }

    mod test_div {
        use super::*;
        #[test]
        fn test_div_positive_positive() {
            let num1 = BigNum::from(vec![1, 2, 3], true);
            let num2 = BigNum::from(vec![4, 5], true);
            let expected = BigNum::from(vec![2], true);
            assert_eq!(num1 / num2, expected);
        }

        #[test]
        fn test_div_negative_positive() {
            let num1 = BigNum::from(vec![1, 2, 3], false);
            let num2 = BigNum::from(vec![4, 5], true);
            let expected = BigNum::from(vec![2], false);
            assert_eq!(num1 / num2, expected);
        }

        #[test]
        fn test_div_positive_negative() {
            let num1 = BigNum::from(vec![1, 2, 3], true);
            let num2 = BigNum::from(vec![4, 5], false);
            let expected = BigNum::from(vec![2], false);
            assert_eq!(num1 / num2, expected);
        }

        #[test]
        fn test_div_negative_negative() {
            let num1 = BigNum::from(vec![1, 2, 3], false);
            let num2 = BigNum::from(vec![4, 5], false);
            let expected = BigNum::from(vec![2], true);
            assert_eq!(num1 / num2, expected);
        }

        #[test]
        #[should_panic]
        fn test_div_by_zero() {
            let num1 = BigNum::from(vec![1, 2, 3], true);
            let num2 = BigNum::zero();

            let _ = num1 / num2;
        }
    }

    mod test_rem {
        use super::*;
        #[test]
        fn test_rem_positive_positive() {
            let num1 = BigNum::from(vec![5], true);
            let num2 = BigNum::from(vec![3], true);
            let expected = BigNum::from(vec![2], true);
            assert_eq!(num1 % num2, expected);
        }

        #[test]
        fn test_rem_negative_positive() {
            let num1 = BigNum::from(vec![1, 2, 3], false);
            let num2 = BigNum::from(vec![4, 5], true);
            let expected = BigNum::from(vec![3, 3], false);
            assert_eq!(num1 % num2, expected);
        }

        #[test]
        fn test_rem_positive_negative() {
            let num1 = BigNum::from(vec![1, 2, 3], true);
            let num2 = BigNum::from(vec![4, 5], false);
            let expected = BigNum::from(vec![3, 3], true);
            assert_eq!(num1 % num2, expected);
        }

        #[test]
        fn test_rem_negative_negative() {
            let num1 = BigNum::from(vec![1, 2, 3], false);
            let num2 = BigNum::from(vec![4, 5], false);
            let expected = BigNum::from(vec![3, 3], false);
            assert_eq!(num1 % num2, expected);
        }

        #[test]
        #[should_panic]
        fn test_rem_by_zero() {
            let num1 = BigNum::from(vec![1, 2, 3], true);
            let num2 = BigNum::zero();

            let _ = num1 % num2;
        }
    }

    mod test_gcd {
        use super::*;

        #[test]
        fn test_gcd_normal() {
            let num1 = BigNum::from(vec![1, 2, 3], true);
            let num2 = BigNum::from(vec![6, 0], true);
            let expected = BigNum::from(vec![3], true);
            assert_eq!(num1.gcd(&num2).unwrap(), expected);
        }

        #[test]
        #[should_panic]
        fn test_gcd_zero() {
            let num1 = BigNum::from(vec![1, 2, 3], true);
            let num2 = BigNum::zero();

            let _ = num1.gcd(&num2);
        }

        #[test]
        fn test_gcd_negative() {
            let num1 = BigNum::from(vec![1, 2, 3], false);
            let num2 = BigNum::from(vec![6, 0], true);
            let expected = BigNum::from(vec![3], true);
            assert_eq!(num1.gcd(&num2).unwrap(), expected);
        }

        #[test]
        fn test_gcd_coprime() {
            let num1 = BigNum::from(vec![1, 0], true);
            let num2 = BigNum::from(vec![3], true);
            let expected = BigNum::from(vec![1], true);
            assert_eq!(num1.gcd(&num2).unwrap(), expected);

            let num3 = BigNum::from(vec![1, 0], false);
            let num4 = BigNum::from(vec![3], true);
            let expected = BigNum::from(vec![1], true);
            assert_eq!(num3.gcd(&num4).unwrap(), expected);
        }
    }
}
