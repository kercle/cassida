use std::{
    cmp::Ordering,
    fmt::{Debug, Display},
};

use crate::{alg::gcd, integer::BigInteger};

#[derive(Clone)]
pub struct BigRational {
    numerator: BigInteger,
    denominator: BigInteger,
}

impl BigRational {
    pub fn new(mut numerator: BigInteger, mut denominator: BigInteger) -> Result<Self, String> {
        if denominator.is_zero() {
            return Err("Denominator cannot be zero".to_string());
        }

        if denominator.is_negative() {
            numerator.flip_sign();
            denominator.flip_sign();
        }

        let mut r = Self {
            numerator,
            denominator,
        };

        r.reduce();
        Ok(r)
    }

    pub fn from_big_integer(value: BigInteger) -> Self {
        BigRational::new(value, BigInteger::one()).unwrap()
    }

    pub fn from_decimal_str(value: &str) -> Result<Self, String> {
        let comma_index = value.find('.');
        let value_without_dot = value.replacen('.', "", 1);

        let mut denominator = String::from("1");
        if let Some(index) = comma_index {
            denominator.push_str(&"0".repeat(value.len() - index - 1));
        }

        let numerator = BigInteger::from_str_radix(&value_without_dot, 10)?;
        let denominator = BigInteger::from_str_radix(&denominator, 10)?;

        Self::new(numerator, denominator)
    }

    pub fn from_f64(value: f64) -> Self {
        let s = value.to_string();

        Self::from_decimal_str(&s).expect("Failed to convert f64 to Rational")
    }

    pub fn to_f64(&self) -> Option<f64> {
        let numerator_f64 = self.numerator.to_i64()? as f64;
        let denominator_f64 = self.denominator.to_i64()? as f64;

        Some(numerator_f64 / denominator_f64)
    }

    pub fn zero() -> Self {
        Self {
            numerator: BigInteger::from_u64(0),
            denominator: BigInteger::from_u64(1),
        }
    }

    pub fn one() -> Self {
        Self {
            numerator: BigInteger::from_u64(1),
            denominator: BigInteger::from_u64(1),
        }
    }

    pub fn take_numerator(self) -> BigInteger {
        self.numerator
    }

    pub fn take_denominator(self) -> BigInteger {
        self.denominator
    }

    pub fn numerator(&self) -> &BigInteger {
        &self.numerator
    }

    pub fn as_pair(self) -> (BigInteger, BigInteger) {
        (self.numerator, self.denominator)
    }

    pub fn denominator(&self) -> &BigInteger {
        &self.denominator
    }

    pub fn is_integer(&self) -> bool {
        self.denominator.is_one()
    }

    pub fn is_zero(&self) -> bool {
        self.numerator.is_zero()
    }

    pub fn is_one(&self) -> bool {
        self.numerator == self.denominator
    }

    pub fn is_minus_one(&self) -> bool {
        self.numerator.abs() == self.denominator.abs()
            && self.numerator.sign() != self.denominator.sign()
    }

    pub fn is_negative(&self) -> bool {
        self.numerator.sign() != self.denominator.sign()
    }

    pub fn is_positive(&self) -> bool {
        self.numerator.sign() == self.denominator.sign()
    }

    pub fn reduce(&mut self) {
        let q = gcd(self.numerator.clone(), self.denominator.clone());

        self.numerator = (&self.numerator / &q).unwrap_or(self.numerator.clone());
        self.denominator = (&self.denominator / &q).unwrap_or(self.denominator.clone());
    }

    pub fn abs(&self) -> Self {
        Self {
            numerator: self.numerator.abs(),
            denominator: self.denominator.abs(),
        }
    }

    pub fn flip_sign(&mut self) {
        if self.denominator.is_negative() {
            self.denominator.flip_sign();
            self.numerator.flip_sign();
        }

        self.numerator.flip_sign();
    }

    pub fn add(&self, other: &BigRational) -> BigRational {
        let nf1 = self.numerator() * other.denominator();
        let nf2 = other.numerator() * self.denominator();
        let df = self.denominator() * other.denominator();

        let num = nf1 + nf2;
        let den = df;

        BigRational::new(num, den).unwrap()
    }

    pub fn neg(&self) -> BigRational {
        let mut ret = self.clone();
        ret.numerator.flip_sign();
        ret
    }

    pub fn sub(&self, other: &BigRational) -> BigRational {
        self.add(&other.neg())
    }

    pub fn mul(&self, other: &BigRational) -> BigRational {
        BigRational::new(
            self.numerator() * other.numerator(),
            self.denominator() * other.denominator(),
        )
        .unwrap()
    }

    pub fn div(&self, other: &BigRational) -> Option<BigRational> {
        if other.is_zero() {
            return None;
        }

        Some(
            BigRational::new(
                self.denominator() * other.denominator(),
                self.numerator() * other.numerator(),
            )
            .unwrap(),
        )
    }

    pub fn pow(&self, exp: &BigRational) -> Result<Self, String> {
        if !exp.denominator().is_one() {
            todo!("Implement non-trivial exponentiation of rationals: {self:?}^{exp:?}");
        }

        let exp = exp.numerator();

        if exp.is_one() {
            return Ok(self.clone());
        } else if exp.is_zero() {
            if self.is_zero() {
                return Err("Base and exponent are zero.".to_string());
            } else {
                return Ok(Self::one());
            }
        }

        let neg_exp = exp.is_negative();
        let exp = exp.abs();

        let a = self.numerator.pow(&exp)?;
        let b = self.denominator.pow(&exp)?;

        if neg_exp {
            Self::new(b, a)
        } else {
            Self::new(a, b)
        }
    }

    pub fn to_hex_string(&self) -> String {
        format!(
            "{}:{}",
            self.numerator.to_hex_string(),
            self.denominator.to_hex_string()
        )
    }
}

impl PartialEq for BigRational {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd for BigRational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigRational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let v = &self.numerator * &other.denominator - &other.numerator * &self.denominator;
        if v.is_zero() {
            Ordering::Equal
        } else if v.is_negative() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl Eq for BigRational {}

impl Display for BigRational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.denominator.is_one() {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

impl Debug for BigRational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::integer::BigInteger;

    #[test]
    fn test_rational_creation() {
        let r = BigRational::new(BigInteger::from_u64(1), BigInteger::from_u64(2));
        assert!(r.is_ok());
    }

    #[test]
    fn test_rational_zero_denominator() {
        let r = BigRational::new(BigInteger::from_u64(1), BigInteger::from_u64(0));
        assert!(r.is_err());
    }

    #[test]
    fn test_from_decimal_str() {
        let r = BigRational::from_decimal_str("3.14").unwrap();
        assert_eq!(r.numerator, BigInteger::from_u64(157));
        assert_eq!(r.denominator, BigInteger::from_u64(50));
    }

    #[test]
    fn test_from_f64() {
        let r = BigRational::from_f64(2.53447e-5);
        assert_eq!(r.numerator, BigInteger::from_u64(253447));
        assert_eq!(r.denominator, BigInteger::from_u64(10000000000));
    }
}
