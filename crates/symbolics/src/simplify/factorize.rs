use expr_macro::{expr, norm_expr};

use crate::{
    atom::Atom,
    expr::{Expr, NormalizedExpr},
    matcher::context::MatchContext,
};

pub fn factorize<A>(expr: Expr<A>) -> NormalizedExpr
where
    A: Default + Clone + PartialEq,
{
    expr.drop_annotation().apply_until_fixed_point(
        factorization_rules().into_iter().map(|(pat, repl)| {
            (pat, move |ctx: &mut MatchContext<'_>| {
                ctx.fill(repl.clone())
            })
        }),
        1000,
    )
}

fn factorization_rules() -> Vec<(NormalizedExpr, Expr)> {
    vec![
        (
            norm_expr!(
                Pattern[r, BlankNullSeq[]]
                    + Pattern[a, Blank[]] * Pattern[b, BlankSeq[]]
                    + Pattern[a, Blank[]] * Pattern[c, BlankSeq[]]
            ),
            expr!(
                a*(Apply[Mul,b] + Apply[Mul,c]) + Apply[Add, r]
            ),
        ),
        (
            norm_expr!(
                Pattern[r, BlankNullSeq[]]
                    + Pattern[a, Blank[]]
                    + Pattern[a, Blank[]] * Pattern[b, BlankSeq[]]
            ),
            expr!(
                a*(1 + Apply[Mul,b]) + Apply[Add, r]
            ),
        ),
    ]
}
