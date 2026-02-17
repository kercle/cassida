use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    str::FromStr,
};

use crate::{integer::BigInteger, rational::BigRational};

pub mod alg;
pub mod ops;

pub mod integer;
pub mod rational;

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Integer(integer::BigInteger),
    Rational(rational::BigRational),
}

impl Number {
    pub fn from_i64(value: i64) -> Self {
        Self::Integer(BigInteger::from_i64(value))
    }

    pub fn from_f64(_value: f64) -> Self {
        todo!()
    }

    pub fn zero() -> Self {
        Self::from_i64(0)
    }

    pub fn one() -> Self {
        Self::from_i64(1)
    }

    pub fn minus_one() -> Self {
        Self::from_i64(-1)
    }

    pub fn is_zero(&self) -> bool {
        match self {
            Self::Integer(i) => i.is_zero(),
            Self::Rational(r) => r.is_zero(),
        }
    }

    pub fn is_one(&self) -> bool {
        match self {
            Self::Integer(i) => i.is_one(),
            Self::Rational(r) => r.is_one(),
        }
    }

    pub fn to_rational(self) -> Result<BigRational, String> {
        use Number::*;
        match self {
            Integer(v) => Ok(BigRational::from_big_integer(v)),
            Rational(v) => Ok(v),
        }
    }

    pub fn pow(&self, exp: &Number) -> Result<Self, String> {
        let base = self.clone().to_rational()?;
        let exp = exp.clone().to_rational()?;

        let res = base.pow(&exp)?;
        if res.denominator().is_one() {
            Ok(Number::Integer(res.take_numerator()))
        } else {
            Ok(Number::Rational(res))
        }
    }
}

impl FromStr for Number {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(i) = BigInteger::from_str_radix(s, 10) {
            Ok(Number::Integer(i))
        } else if let Ok(r) = rational::BigRational::from_decimal_str(s) {
            Ok(Number::Rational(r))
        } else {
            Err(format!("Invalid real scalar: {}", s))
        }
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Self) -> Ordering {
        use Number::*;
        match (self, other) {
            (Integer(i1), Integer(i2)) => i1.cmp(i2),
            (Rational(r1), Rational(r2)) => r1.cmp(r2),
            (Integer(i1), Rational(r2)) => BigRational::from_big_integer(i1.clone()).cmp(r2),
            (Rational(r1), Integer(i2)) => r1.cmp(&BigRational::from_big_integer(i2.clone())),
        }
    }
}

impl Eq for Number {}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Integer(i) => write!(f, "{}", i),
            Number::Rational(r) => write!(f, "{}/{}", r.numerator(), r.denominator()),
        }
    }
}

impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use Number::*;

        match self {
            Integer(x) => {
                0u8.hash(state);
                x.digits().hash(state);
            }
            Rational(x) => {
                1u8.hash(state);
                x.numerator().digits().hash(state);
                x.denominator().digits().hash(state);
            }
        }
    }
}
