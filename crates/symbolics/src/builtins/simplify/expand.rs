use crate::{hold_expr, norm_expr, pattern::environment::Environment, rewrite::Rewriter};

pub(super) fn _build_rewriter() -> Rewriter {
    let rules = vec![
        // (a + b]² → a² + 2ab + b²
        (
            norm_expr!((a_ + b_) ^ 2 + r___),
            hold_expr!(a ^ 2 + 2 * a * b + b ^ 2 + Add[r]),
        ),
        // (a - b]² → a² - 2ab + b²
        (
            norm_expr!((a_ - b_) ^ 2 + r___),
            hold_expr!(a ^ 2 - 2 * a * b + b ^ 2 + Add[r]),
        ),
        // (a + b]³ → a³ + 3a²b + 3ab² + b³
        (
            norm_expr!((a_ + b_) ^ 3 + r___),
            hold_expr!(a ^ 3 + 3 * a ^ 2 * b + 3 * a * b ^ 2 + b ^ 3 + Add[r]),
        ),
        // (a - b]³ → a³ - 3a²b + 3ab² - b³
        (
            norm_expr!((a_ - b_) ^ 3 + r___),
            hold_expr!(a ^ 3 - 3 * a ^ 2 * b + 3 * a * b ^ 2 - b ^ 3 + Add[r]),
        ),
        // (a + b](a - b] → a² - b²
        (
            norm_expr!((a_ + b_) * (a_ - b_) + r___),
            hold_expr!(a ^ 2 - b ^ 2 + Add[r]),
        ),
        // (a + b](c + d) → ac + ad + bc + bd
        (
            norm_expr!((a_ + b_) * (c_ + d_) + r___),
            hold_expr!(a * c + a * d + b * c + b * d + Add[r]),
        ),
        // a(b + c) → ab + ac
        (
            norm_expr!(a_ * (b_ + c__) + r___),
            hold_expr!(a * b + a * Mul[c] + Add[r]),
        ),
        // a(b - c) → ab - ac
        (
            norm_expr!(a_ * (b_ - c__) + r___),
            hold_expr!(a * b - a * Mul[c] + Add[r]),
        ),
        // Sin[a + b] → Sin[a]Cos[b] + Cos[a]Sin[b]
        (
            norm_expr!(Sin[a_ + b_] + r___),
            hold_expr!(Sin[a] * Cos[b] + Cos[a] * Sin[b] + Add[r]),
        ),
        // Sin[a - b] → Sin[a]Cos[b] - Cos[a]Sin[b]
        (
            norm_expr!(Sin[a_ - b_] + r___),
            hold_expr!(Sin[a] * Cos[b] - Cos[a] * Sin[b] + Add[r]),
        ),
        // Cos[a + b] → Cos[a]Cos[b] - Sin[a]Sin[b]
        (
            norm_expr!(Cos[a_ + b_] + r___),
            hold_expr!(Cos[a] * Cos[b] - Sin[a] * Sin[b] + Add[r]),
        ),
        // Cos[a - b] → Cos[a]Cos[b] + Sin[a]Sin[b]
        (
            norm_expr!(Cos[a_ - b_] + r___),
            hold_expr!(Cos[a] * Cos[b] + Sin[a] * Sin[b] + Add[r]),
        ),
        // Exp[a + b] → Exp[a] * Exp[b]
        (
            norm_expr!(Exp[a_ + b__] + r___),
            hold_expr!(Exp[a] * Exp[Add[b]] + Add[r]),
        ),
        // Log[a * b] → Log[a] + Log[b]
        (
            norm_expr!(Log[a_ * b__] + r___),
            hold_expr!(Log[a] + Log[Mul[b]] + Add[r]),
        ),
        // Log[a ^ n] → n * Log[a]
        (
            norm_expr!(Log[a_ ^ n_] + r___),
            hold_expr!(n * Log[a] + Add[r]),
        ),
    ];

    Rewriter::new().with_rules(rules.into_iter().map(|(pat, repl)| {
        (pat, move |ctx: &Environment<'_, '_>| {
            ctx.fill(repl.clone()).normalize()
        })
    }))
}
