use crate::big_num::BigNum;
use crate::frac::{Frac, IntoFrac};

use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(BigNum),
    Frac(Frac),
}

impl Value {
    pub fn simplify(self) -> Self {
        match self {
            Value::Number(num) => Value::Number(num),
            Value::Frac(frac) => match frac.to_bignum() {
                Ok(num) => Value::Number(num),
                Err(_) => Value::Frac(frac),
            },
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Value::Number(num) => num.is_zero(),
            Value::Frac(frac) => frac.is_zero(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(num) => write!(f, "{}", num),
            Value::Frac(frac) => write!(f, "{}", frac),
        }
    }
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            Value::Number(num) => Value::Number(-num),
            Value::Frac(frac) => Value::Frac(-frac),
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left + right),
            (Value::Frac(left), Value::Frac(right)) => Value::Frac(left + right),
            (Value::Number(num), Value::Frac(frac)) => Value::Frac(frac + num),
            (Value::Frac(frac), Value::Number(num)) => Value::Frac(frac + num),
        }
        .simplify()
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left - right),
            (Value::Frac(left), Value::Frac(right)) => Value::Frac(left - right),
            (Value::Number(num), Value::Frac(frac)) => Value::Frac(frac - num),
            (Value::Frac(frac), Value::Number(num)) => Value::Frac(frac - num),
        }
        .simplify()
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left * right),
            (Value::Frac(left), Value::Frac(right)) => Value::Frac(left * right),
            (Value::Number(num), Value::Frac(frac)) => Value::Frac(frac * num),
            (Value::Frac(frac), Value::Number(num)) => Value::Frac(frac * num),
        }
        .simplify()
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Value::Number(left), Value::Number(right)) => {
                if left.clone() % right.clone() == BigNum::zero() {
                    Value::Number(left / right)
                } else {
                    Value::Frac(Frac::new(left, right))
                }
            }
            (Value::Frac(left), Value::Frac(right)) => Value::Frac(left / right),
            (Value::Number(num), Value::Frac(frac)) => Value::Frac(frac / num),
            (Value::Frac(frac), Value::Number(num)) => Value::Frac(frac / num),
        }
        .simplify()
    }
}

impl FromStr for Value {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(num) = BigNum::from_str(s) {
            Ok(Value::Number(num).simplify())
        } else if let Ok(frac) = Frac::from_str(s) {
            Ok(Value::Frac(frac).simplify())
        } else {
            Err(())
        }
    }
}
