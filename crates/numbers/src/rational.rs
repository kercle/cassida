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

    pub fn from_big_integer(value: &BigInteger) -> Self {
        BigRational::new(value.clone(), BigInteger::one()).unwrap()
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

    pub fn numerator(&self) -> &BigInteger {
        &self.numerator
    }

    pub fn denominator(&self) -> &BigInteger {
        &self.denominator
    }

    pub fn is_zero(&self) -> bool {
        self.numerator.is_zero()
    }

    pub fn is_one(&self) -> bool {
        self.numerator == self.denominator
    }

    pub fn reduce(&mut self) {
        let q = gcd(self.numerator.clone(), self.denominator.clone());

        self.numerator = (&self.numerator / &q).unwrap_or(self.numerator.clone());
        self.denominator = (&self.denominator / &q).unwrap_or(self.denominator.clone());
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
