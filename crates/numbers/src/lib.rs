use std::{cmp, fmt, str::FromStr};

use crate::integer::BigInteger;

pub mod alg;
pub mod ops;

pub mod integer;
pub mod rational;

#[derive(Debug, Clone, PartialEq)]
pub enum RealScalar {
    Integer(integer::BigInteger),
    Rational(rational::Rational),
}

pub enum Scalar {
    Real(RealScalar),
    Complex(RealScalar, RealScalar),
}

impl RealScalar {
    pub fn zero() -> Self {
        RealScalar::Integer(BigInteger::from_u64(0))
    }

    pub fn one() -> Self {
        RealScalar::Integer(BigInteger::from_u64(1))
    }

    pub fn minus_one() -> Self {
        RealScalar::Integer(BigInteger::from_i64(-1))
    }

    pub fn from_f64(_value: f64) -> Self {
        todo!()
    }

    pub fn is_zero(&self) -> bool {
        match self {
            RealScalar::Integer(i) => i.is_zero(),
            RealScalar::Rational(r) => r.is_zero(),
        }
    }

    pub fn is_one(&self) -> bool {
        match self {
            RealScalar::Integer(i) => i.is_one(),
            RealScalar::Rational(r) => r.is_one(),
        }
    }

    pub fn to_hex_string(&self) -> String {
        match self {
            RealScalar::Integer(i) => i.to_hex_string(),
            RealScalar::Rational(r) => r.to_hex_string(),
        }
    }
}

impl FromStr for RealScalar {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(i) = BigInteger::from_str_radix(s, 10) {
            Ok(RealScalar::Integer(i))
        } else if let Ok(r) = rational::Rational::from_decimal_str(s) {
            Ok(RealScalar::Rational(r))
        } else {
            Err(format!("Invalid real scalar: {}", s))
        }
    }
}

impl cmp::PartialOrd for RealScalar {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match (self, other) {
            (RealScalar::Integer(i1), RealScalar::Integer(i2)) => i1.partial_cmp(i2),
            (RealScalar::Rational(_r1), RealScalar::Rational(_r2)) => {
                todo!("Implement comparison for Rational")
            }
            _ => None, // Different types cannot be compared
        }
    }
}

impl fmt::Display for RealScalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RealScalar::Integer(i) => write!(f, "{}", i),
            RealScalar::Rational(r) => write!(f, "{}/{}", r.numerator(), r.denominator()),
        }
    }
}
