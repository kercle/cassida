use crate::{norm_expr, pattern::environment::Environment, raw_expr, rewrite::Rewriter};

pub(super) fn build_rewriter() -> Rewriter {
    let rules = vec![
        // =============== Pythagorean identity ===============
        (
            norm_expr!(Cos[a_] ^ 2 + Sin[a_] ^ 2 + r___),
            raw_expr!(1 + r),
        ),
        (norm_expr!(Sqrt[1 - Cos[x_] ^ 2]), raw_expr!(Sin[x])),
        (norm_expr!(Sqrt[1 - Sin[x_] ^ 2]), raw_expr!(Cos[x])),
    ];

    Rewriter::new().with_rules(rules.into_iter().map(|(pat, repl)| {
        (pat, move |ctx: &Environment<'_, '_>| {
            ctx.fill(repl.clone())
        })
    }))
}
