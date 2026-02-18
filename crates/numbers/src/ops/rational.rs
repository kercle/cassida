use std::ops;

use crate::rational::BigRational;

impl ops::Add for BigRational {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::add(&self, &other)
    }
}

impl ops::Add for &BigRational {
    type Output = BigRational;

    fn add(self, other: Self) -> Self::Output {
        BigRational::add(self, other)
    }
}

impl ops::Add<&BigRational> for BigRational {
    type Output = BigRational;

    fn add(self, other: &Self) -> Self::Output {
        BigRational::add(&self, other)
    }
}

impl ops::Add<BigRational> for &BigRational {
    type Output = BigRational;

    fn add(self, other: BigRational) -> Self::Output {
        BigRational::add(self, &other)
    }
}

impl ops::Sub for BigRational {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::sub(&self, &other)
    }
}

impl ops::Sub for &BigRational {
    type Output = BigRational;

    fn sub(self, other: Self) -> Self::Output {
        BigRational::sub(self, other)
    }
}

impl ops::Mul for BigRational {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        BigRational::mul(&self, &other)
    }
}

impl ops::Mul for &BigRational {
    type Output = BigRational;

    fn mul(self, other: Self) -> Self::Output {
        BigRational::mul(self, other)
    }
}

impl ops::Mul<&BigRational> for BigRational {
    type Output = BigRational;

    fn mul(self, other: &Self) -> Self::Output {
        BigRational::mul(&self, other)
    }
}

impl ops::Mul<BigRational> for &BigRational {
    type Output = BigRational;

    fn mul(self, other: BigRational) -> Self::Output {
        BigRational::mul(self, &other)
    }
}

impl ops::Neg for &BigRational {
    type Output = BigRational;

    fn neg(self) -> Self::Output {
        self.neg()
    }
}

impl ops::Neg for BigRational {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -&self
    }
}

impl ops::Div for &BigRational {
    type Output = Option<BigRational>;

    fn div(self, other: Self) -> Self::Output {
        BigRational::div(self, other)
    }
}

impl ops::Div for BigRational {
    type Output = Option<Self>;

    fn div(self, other: Self) -> Self::Output {
        &self / &other
    }
}
