use std::fmt;

use crate::big_num::BigNum;
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

        Ok(Frac {
            numerator,
            denominator,
        })
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
