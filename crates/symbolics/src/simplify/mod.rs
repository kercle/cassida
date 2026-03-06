mod factorize;
mod functions_known_values;
mod trigonometric_functions;

use crate::{
    calculus::{derivative::derivative_rules, integrate::indefinite_integrals_rules},
    expr::{Expr, NormalizedExpr},
    pattern::environment::Environment,
    simplify::{
        factorize::factorization_rules, functions_known_values::known_function_values_rules,
        trigonometric_functions::trigonometric_rules,
    },
};

pub struct Simplifier {
    expr: NormalizedExpr,
    limit_guard: u32,
}

impl Simplifier {
    pub fn new(expr: NormalizedExpr) -> Simplifier {
        Simplifier {
            expr: expr.collect_like_terms(),
            limit_guard: 100,
        }
    }

    pub fn simple(self) -> NormalizedExpr {
        let limit_guard = self.limit_guard;
        let mut current = self;

        for _ in 0..limit_guard {
            let prev_expr = current.expr.clone();

            let next_expr = current
                .with_factorization() // TODO: There is a nasty bug in the matcher :(
                .with_known_function_values()
                .with_resolved_derivatives()
                .with_resolved_indefinite_integrals()
                .with_trigonometric_identities()
                .finish_normalized();

            if next_expr == prev_expr {
                return next_expr;
            }

            current = Simplifier::new(next_expr);
        }

        current.finish_normalized()
    }

    pub fn with_resolved_derivatives(self) -> Simplifier {
        Simplifier::new(self.simplify_with_rules_until_stable(derivative_rules()))
    }

    pub fn with_resolved_indefinite_integrals(self) -> Simplifier {
        Simplifier::new(self.simplify_with_rules_until_stable(indefinite_integrals_rules()))
    }

    pub fn with_trigonometric_identities(self) -> Simplifier {
        Simplifier::new(self.simplify_with_rules_until_stable(trigonometric_rules()))
    }

    pub fn with_known_function_values(self) -> Simplifier {
        Simplifier::new(self.simplify_with_rules_until_stable(known_function_values_rules()))
    }

    pub fn with_factorization(self) -> Simplifier {
        Simplifier::new(self.simplify_with_rules_until_stable(factorization_rules()))
    }

    pub fn finish(self) -> Expr {
        self.expr.take_expr()
    }

    pub fn finish_normalized(self) -> NormalizedExpr {
        self.expr
    }

    fn simplify_with_rules_until_stable(
        self,
        rules: Vec<(NormalizedExpr, Expr)>,
    ) -> NormalizedExpr {
        self.expr
            .take_expr()
            .drop_annotation()
            .apply_until_fixed_point(
                rules.into_iter().map(|(pat, repl)| {
                    (pat, move |ctx: &Environment<'_, '_, ()>| {
                        ctx.fill(repl.clone())
                    })
                }),
                1000,
            )
    }
}
