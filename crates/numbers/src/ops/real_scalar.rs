use crate::RealScalar;
use std::ops;

impl ops::Add for &RealScalar {
    type Output = Option<RealScalar>;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (RealScalar::Integer(a), RealScalar::Integer(b)) => Some(RealScalar::Integer(a + b)),
            (RealScalar::Rational(_a), RealScalar::Rational(_b)) => {
                todo!("Implement addition for Rational")
            }
            _ => None, // Handle mixed types or unsupported operations
        }
    }
}

impl ops::Add for RealScalar {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        &self + &other
    }
}

impl ops::Sub for &RealScalar {
    type Output = Option<RealScalar>;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (RealScalar::Integer(a), RealScalar::Integer(b)) => Some(RealScalar::Integer(a - b)),
            (RealScalar::Rational(_a), RealScalar::Rational(_b)) => {
                todo!("Implement subtraction for Rational")
            }
            _ => None, // Handle mixed types or unsupported operations
        }
    }
}

impl ops::Sub for RealScalar {
    type Output = Option<Self>;

    fn sub(self, other: Self) -> Self::Output {
        &self - &other
    }
}

impl ops::Mul for &RealScalar {
    type Output = Option<RealScalar>;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (RealScalar::Integer(a), RealScalar::Integer(b)) => Some(RealScalar::Integer(a * b)),
            (RealScalar::Rational(_a), RealScalar::Rational(_b)) => {
                todo!("Implement multiplication for Rational")
            }
            _ => None, // Handle mixed types or unsupported operations
        }
    }
}

impl ops::Mul for RealScalar {
    type Output = Option<Self>;

    fn mul(self, other: Self) -> Self::Output {
        &self * &other
    }
}

impl ops::Neg for &RealScalar {
    type Output = RealScalar;

    fn neg(self) -> Self::Output {
        match self {
            RealScalar::Integer(a) => RealScalar::Integer(-a),
            RealScalar::Rational(_a) => todo!("Implement negation for Rational"),
            RealScalar::PiMultiple(_a) => todo!("Implement negation for Rational"),
        }
    }
}

impl ops::Neg for RealScalar {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -&self
    }
}
