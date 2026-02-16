use crate::{RealScalar, integer::BigInteger};
use std::ops;

impl ops::Add for &RealScalar {
    type Output = RealScalar;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (RealScalar::Integer(a), RealScalar::Integer(b)) => RealScalar::Integer(a + b),
            (RealScalar::Rational(_a), RealScalar::Rational(_b)) => {
                todo!("Implement addition for Rational")
            }
            _ => {
                todo!("Handle mixed types or unsupported operations")
            }
        }
    }
}

impl ops::Add for RealScalar {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        &self + &other
    }
}

impl ops::AddAssign for RealScalar {
    fn add_assign(&mut self, other: RealScalar) {
        let new_value = self.clone() + other;
        *self = new_value;
    }
}

impl ops::Add<u64> for RealScalar {
    type Output = RealScalar;

    fn add(self, other: u64) -> Self::Output {
        match self {
            RealScalar::Integer(a) => RealScalar::Integer(a + BigInteger::from_u64(other)),
            RealScalar::Rational(_a) => {
                todo!("Implement addition of u64 to Rational")
            }
        }
    }
}

impl ops::Sub for &RealScalar {
    type Output = RealScalar;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (RealScalar::Integer(a), RealScalar::Integer(b)) => RealScalar::Integer(a - b),
            (RealScalar::Rational(_a), RealScalar::Rational(_b)) => {
                todo!("Implement subtraction for Rational")
            }
            _ => todo!("Implement subtraction for mixed"),
        }
    }
}

impl ops::Sub for RealScalar {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        &self - &other
    }
}

impl ops::Mul for &RealScalar {
    type Output = RealScalar;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (RealScalar::Integer(a), RealScalar::Integer(b)) => RealScalar::Integer(a * b),
            (RealScalar::Rational(_a), RealScalar::Rational(_b)) => {
                todo!("Implement multiplication for Rational")
            }
            _ => todo!("Implement multiplication for mixed types")
        }
    }
}

impl ops::Mul for RealScalar {
    type Output = RealScalar;

    fn mul(self, other: Self) -> Self::Output {
        &self * &other
    }
}

impl ops::MulAssign for RealScalar {
    fn mul_assign(&mut self, other: RealScalar) {
        let new_value = self.clone() * other;
        *self = new_value;
    }
}

impl ops::Neg for &RealScalar {
    type Output = RealScalar;

    fn neg(self) -> Self::Output {
        match self {
            RealScalar::Integer(a) => RealScalar::Integer(-a),
            RealScalar::Rational(_a) => todo!("Implement negation for Rational"),
        }
    }
}

impl ops::Neg for RealScalar {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -&self
    }
}
