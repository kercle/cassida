use crate::norm_expr;
use crate::pattern::environment::Environment;
use crate::rewrite::Rewriter;

pub(super) fn build_rewriter() -> Rewriter {
    let rules = vec![
        // =============== Sin exact values ===============
        (norm_expr!(Sin[0]), norm_expr!(0)),
        (
            norm_expr!(Sin[pi / 12]),
            norm_expr!(Sqrt[2] * (Sqrt[3] - 1) / 4),
        ),
        (norm_expr!(Sin[pi / 10]), norm_expr!((Sqrt[5] - 1) / 4)),
        (norm_expr!(Sin[pi / 8]), norm_expr!(Sqrt[2 - Sqrt[2]] / 2)),
        (norm_expr!(Sin[pi / 6]), norm_expr!(1 / 2)),
        (
            norm_expr!(Sin[pi / 5]),
            norm_expr!(Sqrt[2] * Sqrt[5 - Sqrt[5]] / 4),
        ),
        (norm_expr!(Sin[pi / 4]), norm_expr!(1 / Sqrt[2])),
        // =============== Cos exact values ===============
        (norm_expr!(Cos[0]), norm_expr!(1)),
        (
            norm_expr!(Cos[pi / 12]),
            norm_expr!(Sqrt[2] * (Sqrt[3] + 1) / 4),
        ),
        (
            norm_expr!(Cos[pi / 10]),
            norm_expr!(Sqrt[2] * Sqrt[5 + Sqrt[5]] / 4),
        ),
        (norm_expr!(Cos[pi / 8]), norm_expr!(Sqrt[2 + Sqrt[2]] / 2)),
        (norm_expr!(Cos[pi / 6]), norm_expr!(Sqrt[3] / 2)),
        (norm_expr!(Cos[pi / 5]), norm_expr!((Sqrt[5] + 1) / 4)),
        (norm_expr!(Cos[pi / 4]), norm_expr!(1 / Sqrt[2])),
        // =============== Tan exact values ===============
        (norm_expr!(Tan[0]), norm_expr!(0)),
        // =============== Exp exact values ===============
        (norm_expr!(Exp[0]), norm_expr!(1)),
        // =============== Log exact values ===============
        (norm_expr!(Log[0]), norm_expr! { -Infinity }),
        (norm_expr!(Log[1]), norm_expr!(0)),
    ];

    Rewriter::new().with_rules(rules.into_iter().map(|(pat, repl)| {
        (pat, move |ctx: &Environment<'_, '_>| {
            ctx.fill(repl.clone()).normalize()
        })
    }))
}
