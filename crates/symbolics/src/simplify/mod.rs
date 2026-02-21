mod functions_known_values;
mod trigonometric_functions;

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

    pub fn with_trigonometric_identities(self) -> Simplifier {
        Simplifier::new(trigonometric_functions::simplify_trigon(
            self.expr.take_expr(),
        ))
    }

    pub fn with_known_function_values(self) -> Simplifier {
        Simplifier::new(functions_known_values::simplify_evaluation_at_zero(
            self.expr.take_expr(),
        ))
    }

    pub fn finish(self) -> Expr {
        self.expr.take_expr()
    }

    pub fn finish_normalized(self) -> NormalizedExpr {
        self.expr
    }
}
