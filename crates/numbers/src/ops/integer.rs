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

impl ops::SubAssign<&BigInteger> for BigInteger {
    fn sub_assign(&mut self, other: &BigInteger) {
        *self = &*self - other;
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
        if let Some((quotient, _)) = BigInteger::div(self, other) {
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
        if let Some((_, remainder)) = BigInteger::div(self, other) {
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

impl ops::ShlAssign<usize> for BigInteger {
    fn shl_assign(&mut self, n: usize) {
        *self = self.shift_left(n)
    }
}

impl ops::ShrAssign<usize> for BigInteger {
    fn shr_assign(&mut self, n: usize) {
        *self = self.shift_right(n)
    }
}

impl cmp::PartialEq for BigInteger {
    fn eq(&self, other: &Self) -> bool {
        self.eq_inner(other)
    }
}

impl cmp::PartialOrd for BigInteger {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigInteger {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.eq_inner(other) {
            cmp::Ordering::Equal
        } else if self.lt_inner(other) {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Greater
        }
    }
}

impl Eq for BigInteger {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let a = BigInteger::from_i64(100);
        let b = BigInteger::from_i64(200);

        assert_eq!(&a + &b, BigInteger::from_i64(300));
        assert_eq!(a + b, BigInteger::from_i64(300));
    }

    #[test]
    fn test_subtraction() {
        let a = BigInteger::from_i64(500);
        let b = BigInteger::from_i64(100);
        assert_eq!(&a - &b, BigInteger::from_i64(400));
        assert_eq!(a - b, BigInteger::from_i64(400));
    }

    #[test]
    fn test_negation() {
        let a = BigInteger::from_i64(100);
        assert_eq!(-&a, BigInteger::from_i64(-100));
        assert_eq!(-a, BigInteger::from_i64(-100));
    }

    #[test]
    fn test_division_and_remainder() {
        let a = BigInteger::from_i64(10);
        let b = BigInteger::from_i64(3);

        assert_eq!((&a / &b).unwrap(), BigInteger::from_i64(3));
        assert_eq!((&a % &b).unwrap(), BigInteger::from_i64(1));

        let zero = BigInteger::from_i64(0);
        assert!((&a / &zero).is_none());
    }
}
