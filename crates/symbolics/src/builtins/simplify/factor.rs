use crate::{hold_expr, norm_expr, pattern::environment::Environment, rewrite::Rewriter};

pub(super) fn build_rewriter() -> Rewriter {
    let rules = vec![
        (
            // a² + 2ab + b² → (a + b)²
            norm_expr!(a_ ^ 2 + 2 * a_ * b_ + b_ ^ 2 + r___),
            hold_expr!((a + b) ^ 2 + Add[r]),
        ),
        (
            // a² - 2ab + b² → (a - b)²
            norm_expr!(a_ ^ 2 - 2 * a_ * b_ + b_ ^ 2 + r___),
            hold_expr!((a - b) ^ 2 + Add[r]),
        ),
        (
            // ab + ac → a(b + c)
            norm_expr!(a_ * b__ + a_ * c__ + r___),
            hold_expr!(a * (Mul[b] + Mul[c]) + Add[r]),
        ),
        (
            // a + ab → a(1 + b)
            norm_expr!(a_ + a_ * b__ + r___),
            hold_expr!(a * (1 + Mul[b]) + Add[r]),
        ),
        (
            // a² - b² → (a + b)*(a - b)
            norm_expr!(a_ ^ 2 - b_ ^ 2 + r___),
            hold_expr!((a + b) * (a - b) + Add[r]),
        ),
        (
            // a² - 1  →  (a+1)(a-1)
            norm_expr!(a_ ^ 2 - 1 + r___),
            hold_expr!((a + 1) * (a - 1) + Add[r]),
        ),
        (
            // a² + 2a + 1 → (a + 1)²
            norm_expr!(a_ ^ 2 + 2 * a_ + 1 + r___),
            hold_expr!((a + 1) ^ 2 + Add[r]),
        ),
        (
            // a² - 2a + 1 → (a - 1)²
            norm_expr!(a_ ^ 2 - 2 * a_ + 1 + r___),
            hold_expr!((a - 1) ^ 2 + Add[r]),
        ),
        (
            // a² + ab → a(a + b)
            // TODO: generalize to arbitrary powers using
            // a suitable predicate e.g. IsPositiveQ
            norm_expr!(a_ ^ 2 + a_ * b___ + r___),
            hold_expr!(a * (a + Mul[b]) + Add[r]),
        ),
    ];

    Rewriter::new().with_rules(rules.into_iter().map(|(pat, repl)| {
        (pat, move |ctx: &Environment<'_, '_>| {
            ctx.fill(repl.clone()).normalize()
        })
    }))
}
