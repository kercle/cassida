use numbers::Number;

use crate::{expr::NormExpr};

#[derive(Clone)]
pub struct UnivariatePolynomial(Vec<Number>);

impl UnivariatePolynomial {
    pub fn from_expr(_expr: &NormExpr) -> Option<Self> {
        // needs polynom checking predicate
        // which in turn needs Optional in
        // pattern matcher.
        todo!()
    }

    pub fn zero() -> Self {
        Self(vec![Number::zero()])
    }

    pub fn degree(&self) -> usize {
        self.0.len() - 1
    }

    pub fn coeff(&self, i: usize) -> &Number {
        self.0.get(i).unwrap_or(&numbers::ZERO)
    }

    pub fn long_division(&self, other: &Self) -> (Self, Self) {
        if other.degree() > self.degree() {
            return (UnivariatePolynomial::zero(), other.clone());
        }
        todo!()
    }
}
