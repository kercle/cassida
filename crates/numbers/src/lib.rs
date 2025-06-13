use std::fmt;

use crate::integer::BigInteger;

pub mod ops;
pub mod alg;

pub mod integer;
pub mod rational;

#[derive(Debug, Clone, PartialEq)]
pub enum RealScalar {
    Integer(integer::BigInteger),
    Rational(rational::Rational),
    Pi,
    EulerNumber,
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

    pub fn from_f64(_value: f64) -> Self {
        todo!()
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        if let Ok(i) = BigInteger::from_str_radix(s, 10) {
            Ok(RealScalar::Integer(i))
        } else if let Ok(r) = rational::Rational::from_decimal_str(s) {
            Ok(RealScalar::Rational(r))
        } else {
            Err(format!("Invalid real scalar: {}", s))
        }
    }

    pub fn is_zero(&self) -> bool {
        match self {
            RealScalar::Integer(i) => i.is_zero(),
            RealScalar::Rational(r) => r.is_zero(),
            RealScalar::Pi | RealScalar::EulerNumber => false,
        }
    }

    pub fn is_one(&self) -> bool {
        match self {
            RealScalar::Integer(i) => i.is_one(),
            RealScalar::Rational(r) => r.is_one(),
            RealScalar::Pi | RealScalar::EulerNumber => false,
        }
    }
}

impl fmt::Display for RealScalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RealScalar::Integer(i) => write!(f, "{}", i),
            RealScalar::Rational(r) => write!(f, "{}/{}", r.numerator(), r.denominator()),
            RealScalar::Pi => write!(f, "π"),
            RealScalar::EulerNumber => write!(f, "e"),
        }
    }
}
