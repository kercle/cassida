use expr_macro::{expr, norm_expr};

use crate::{
    atom::Atom,
    expr::{Expr, NormalizedExpr},
    matcher::context::MatchContext,
    parser::ast::{ADD_HEAD, MUL_HEAD},
    rewrite::Rewriter,
};

pub fn resolve_indefinite_integrals<A>(expr: Expr<A>) -> Expr
where
    A: Default + Clone + PartialEq,
{
    let rules = indefinite_integrals_rules();
    let rw: Rewriter<()> = Rewriter::new()
        .commutative_if(|head| head.matches_symbol(ADD_HEAD) || head.matches_symbol(MUL_HEAD))
        .with_rules(rules.into_iter().map(|(pat, repl)| {
            (pat, move |ctx: &mut MatchContext<'_, ()>| {
                ctx.fill(repl.clone())
            })
        }));

    let mut expr = expr.drop_annotation();

    loop {
        let expr_next_iter = rw.apply_first_match(expr.clone());

        if expr != expr_next_iter {
            expr = expr_next_iter;
        } else {
            return expr;
        }
    }
}

fn indefinite_integrals_rules() -> Vec<(NormalizedExpr, Expr)> {
    vec![
        // =============== Basic ===============
        (
            norm_expr!(
            Integrate[
                PatternTest[Pattern[c, Blank[]], IsNumberQ],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(c * x),
        ),
        (
            norm_expr!(
            Integrate[
                Pattern[x, Blank[]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(x ^ 2 / 2),
        ),
        // =============== Linearity ===============
        (
            norm_expr!(
            Integrate[
                Pattern[f, Blank[]] + Pattern[r, BlankSeq[]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(
            Integrate[f,x] + Integrate[Add[r],x]
            ),
        ),
        (
            norm_expr!(
            Integrate[
                PatternTest[Pattern[c, Blank[]], IsNumberQ] * Pattern[r, BlankSeq[]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(
            c * Integrate[Mul[r],x]
            ),
        ),
        // =============== Powers ===============
        (
            norm_expr!(
            Integrate[
                1 / Pattern[x, Blank[]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(Log[Abs[x]]),
        ),
        (
            norm_expr!(
            Integrate[
                Pattern[x, Blank[]] ^ PatternTest[Pattern[k, Blank[]], IsNumberQ],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(x ^ (k + 1) / (k + 1)),
        ),
        // =============== Exponentials ===============
        (
            norm_expr!(
            Integrate[
                Exp[Pattern[x, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(Exp[x]),
        ),
        // =============== Logarithms ===============
        (
            norm_expr!(
            Integrate[
                Log[Pattern[x, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(x * Log[x] - x),
        ),
        // =============== Trigonometric functions ===============
        (
            norm_expr!(
            Integrate[
                Sin[Pattern[x, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(-Cos[x]),
        ),
        (
            norm_expr!(
            Integrate[
                Cos[Pattern[x, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(Sin[x]),
        ),
    ]
}
