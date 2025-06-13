use std::{cmp, ops};

use crate::integer::BigInteger;

impl ops::Add for BigInteger {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        BigInteger::add(&self, &other)
    }
}

impl ops::Add for &BigInteger {
    type Output = BigInteger;

    fn add(self, other: &BigInteger) -> Self::Output {
        BigInteger::add(self, other)
    }
}

impl ops::Sub for BigInteger {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        BigInteger::sub(&self, &other)
    }
}

impl ops::Sub for &BigInteger {
    type Output = BigInteger;

    fn sub(self, other: &BigInteger) -> Self::Output {
        BigInteger::sub(self, other)
    }
}

impl ops::Mul for BigInteger {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        BigInteger::mul(&self, &other)
    }
}

impl ops::Mul for &BigInteger {
    type Output = BigInteger;

    fn mul(self, other: &BigInteger) -> Self::Output {
        BigInteger::mul(self, other)
    }
}

impl ops::Neg for &BigInteger {
    type Output = BigInteger;

    fn neg(self) -> Self::Output {
        let mut ret = self.clone();
        ret.flip_sign();
        ret
    }
}

impl ops::Neg for BigInteger {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -&self
    }
}

impl ops::Div for &BigInteger {
    type Output = Option<BigInteger>;

    fn div(self, other: &BigInteger) -> Self::Output {
        if let Some((quotient, _)) = BigInteger::div(&self, &other) {
            Some(quotient)
        } else {
            None
        }
    }
}

impl ops::Div for BigInteger {
    type Output = Option<Self>;

    fn div(self, other: Self) -> Self::Output {
        &self / &other
    }
}

impl ops::Rem for &BigInteger {
    type Output = Option<BigInteger>;

    fn rem(self, other: &BigInteger) -> Self::Output {
        if let Some((_, remainder)) = BigInteger::div(&self, &other) {
            Some(remainder)
        } else {
            None
        }
    }
}

impl ops::Rem for BigInteger {
    type Output = Option<Self>;

    fn rem(self, other: Self) -> Self::Output {
        &self % &other
    }
}

impl cmp::PartialEq for BigInteger {
    fn eq(&self, other: &Self) -> bool {
        self.eq(other)
    }
}
