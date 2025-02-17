use std::fmt;

pub struct BigNum {
    sign: bool,   // true = positive, false = negative
    num: Vec<u8>, // Stores digits in most significant to least significant order
}

impl BigNum {
    /// Creates a new BigNum initialized to zero.
    pub fn new() -> BigNum {
        BigNum {
            sign: true,
            num: vec![0],
        }
    }

    /// Converts a string representation into a BigNum.
    pub fn from_str(s: &str) -> BigNum {
        let mut sign = true;
        let mut num: Vec<u8> = Vec::new();
        let mut i = 0;
        if s.chars().nth(0).unwrap() == '-' {
            sign = false;
            i = 1;
        }
        for c in s.chars().skip(i) {
            num.push(c.to_digit(10).unwrap() as u8);
        }
        BigNum { sign, num }
    }

    /// Converts the BigNum into a string representation.
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        if !self.sign {
            s.push('-');
        }
        for n in &self.num {
            s.push_str(&n.to_string());
        }
        s
    }

    /// Negates the number (changes its sign).
    pub fn negate(&self) -> BigNum {
        BigNum {
            sign: !self.sign,
            num: self.num.clone(),
        }
    }

    /// Subtracts `other` from `self` (self - other).
    pub fn minus(&self, other: &BigNum) -> BigNum {
        self.add(&other.negate())
    }

    /// Adds two BigNums using ten’s complement.
    pub fn add(&self, other: &BigNum) -> BigNum {
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

    pub fn multiply(&self, other: &BigNum) -> BigNum {
        let num1 = self.num.clone();
        let num2 = other.num.clone();

        let mut result = vec![0; num1.len() + num2.len()];

        for i in (0..num1.len()).rev() {
            let mut carry = 0;
            for j in (0..num2.len()).rev() {
                let product = num1[i] * num2[j] + result[i + j + 1] + carry;
                result[i + j + 1] = product % 10;
                carry = product / 10;
            }
            result[i] += carry;
        }

        while result.len() > 1 && result[0] == 0 {
            result.remove(0);
        }

        BigNum {
            sign: self.sign == other.sign,
            num: result,
        }
    }
}

// Implementing Display for BigNum
impl fmt::Display for BigNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.sign {
            write!(f, "-")?;
        }
        for &n in &self.num {
            write!(f, "{}", n)?;
        }
        Ok(())
    }
}
