use numbers::Number;

use crate::{atom::Atom, expr::Expr};

impl<A> Expr<A> {
    pub fn new_compound_with_annotation(head: Expr<A>, args: Vec<Expr<A>>, ann: A) -> Self {
        Expr::Compound {
            head: Box::new(head),
            args,
            annotation: ann,
        }
    }
}

impl<A> Expr<A>
where
    A: Default,
{
    pub fn new_compound<T: Into<Expr<A>>>(head: T, args: Vec<Expr<A>>) -> Self {
        Expr::Compound {
            head: Box::new(head.into()),
            args,
            annotation: A::default(),
        }
    }

    pub fn new_number(value: Number) -> Self {
        Expr::Atom {
            entry: Atom::Number(value),
            annotation: A::default(),
        }
    }

    pub fn new_number_zero() -> Self {
        Self::new_number(Number::zero())
    }

    pub fn new_number_one() -> Self {
        Self::new_number(Number::one())
    }

    pub fn new_number_minus_one() -> Self {
        Self::new_number(Number::minus_one())
    }

    pub fn new_number_rational(numerator: i64, denominator: u64) -> Result<Self, String> {
        Ok(Self::new_number(Number::new_rational_from_i64(
            numerator,
            denominator,
        )?))
    }

    pub fn new_symbol<T: AsRef<str>>(symb: T) -> Self {
        Expr::Atom {
            entry: Atom::Symbol(symb.as_ref().to_string()),
            annotation: A::default(),
        }
    }
}
