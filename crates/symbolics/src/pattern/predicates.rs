use numbers::Number;

use crate::{
    atom::Atom,
    expr::{Expr, ExprKind, NormExpr},
};

pub fn is_number<S>(expr: &Expr<S>) -> bool {
    expr.is_number()
}

pub fn is_symbol<S>(expr: &Expr<S>) -> bool {
    expr.is_symbol()
}

pub fn is_integer<S>(expr: &Expr<S>) -> bool {
    matches!(
        expr.kind(),
        ExprKind::Atom {
            entry: Atom::Number(Number::Integer(_))
        }
    )
}

pub fn is_rational<S>(expr: &Expr<S>) -> bool {
    matches!(
        expr.kind(),
        ExprKind::Atom {
            entry: Atom::Number(Number::Rational(_))
        }
    )
}

pub fn is_positive<S>(expr: &Expr<S>) -> bool {
    expr.is_number_positive()
}

pub fn is_negative<S>(expr: &Expr<S>) -> bool {
    expr.is_number_negative()
}

pub fn is_univ_poly_over_q(_expr: NormExpr) -> bool {
    todo!()
}
