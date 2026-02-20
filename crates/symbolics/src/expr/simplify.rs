use crate::expr::{Expr, NormalizedExpr};

pub fn simplify(expr: Expr) -> NormalizedExpr {
    NormalizedExpr::new(expr).collect_like_terms()
}
