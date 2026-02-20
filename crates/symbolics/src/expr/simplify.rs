use crate::expr::{Expr, NormalizedExpr, derivative::resolve_derivatives};

pub struct Simplifier {
    expr: NormalizedExpr,
}

impl Simplifier {
    pub fn new(expr: Expr) -> Simplifier {
        Simplifier {
            expr: NormalizedExpr::new(expr).collect_like_terms(),
        }
    }

    pub fn with_resolved_derivatives(self) -> Simplifier {
        Simplifier::new(resolve_derivatives(self.expr.take_expr()))
    }

    pub fn finish(self) -> Expr {
        self.expr.take_expr()
    }

    pub fn finish_normalized(self) -> NormalizedExpr {
        self.expr
    }
}
