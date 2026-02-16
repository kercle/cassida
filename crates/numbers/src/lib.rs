use std::{cmp::Ordering, fmt, str::FromStr};

use crate::{integer::BigInteger, rational::BigRational};

pub mod alg;
pub mod ops;

pub mod integer;
pub mod rational;

#[derive(Debug, Clone, PartialEq)]
pub enum RealScalar {
    Integer(integer::BigInteger),
    Rational(rational::BigRational),
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
        } else if let Ok(r) = rational::BigRational::from_decimal_str(s) {
            Ok(RealScalar::Rational(r))
        } else {
            Err(format!("Invalid real scalar: {}", s))
        }
    }
}

impl PartialOrd for RealScalar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RealScalar {
    fn cmp(&self, other: &Self) -> Ordering {
        use RealScalar::*;
        match (self, other) {
            (Integer(i1), Integer(i2)) => i1.cmp(i2),
            (Rational(r1), Rational(r2)) => r1.cmp(r2),
            (Integer(i1), Rational(r2)) => BigRational::from_big_integer(i1).cmp(r2),
            (Rational(r1), Integer(i2)) => r1.cmp(&BigRational::from_big_integer(i2)),
        }
    }
}

impl Eq for RealScalar {}

impl fmt::Display for RealScalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RealScalar::Integer(i) => write!(f, "{}", i),
            RealScalar::Rational(r) => write!(f, "{}/{}", r.numerator(), r.denominator()),
        }
    }
}
