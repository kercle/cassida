use expr_macro::{expr, norm_expr};

use crate::{
    atom::Atom,
    expr::{Expr, NormalizedExpr},
};

pub(crate) fn derivative_rules() -> Vec<(NormalizedExpr, Expr)> {
    vec![
        // =============== Linearity ===============
        (
            norm_expr!(
            D[
                Pattern[f, Blank[]] + Pattern[r, BlankSeq[]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(
            D[f, x] + D[Add[r],x]
            ),
        ),
        (
            norm_expr!(
            D[
                PatternTest[Pattern[c, Blank[]], IsNumberQ] * Pattern[r, BlankSeq[]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(
            c * D[Mul[r],x]
            ),
        ),
        // =============== Basic ===============
        (
            norm_expr!(
            D[
                PatternTest[Pattern[c, Blank[]], IsNumberQ],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(0),
        ),
        (
            norm_expr!(
            D[
                PatternTest[Pattern[a, Blank[]], IsSymbolQ],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(a),
        ),
        (
            norm_expr!(
            D[
                Pattern[x, Blank[]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(1),
        ),
        (
            norm_expr!(
            D[
                Pattern[f, Blank[]] * Pattern[g, Blank[]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(D[f, x] * g + f * D[g, x]),
        ),
        // =============== Powers ===============
        (
            norm_expr!(
            D[
                Pattern[f, Blank[]] ^ Pattern[g, Blank[]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!((f ^ g) *(g / f + D[g, x] * Log[f])),
        ),
        // =============== Exponential ===============
        (
            norm_expr!(
            D[
                Exp[Pattern[f, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(Exp[f] * D[f, x]),
        ),
        // =============== Logarithms ===============
        (
            norm_expr!(
            D[
                Log[Pattern[f, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!((1 / f) * D[f, x]),
        ),
        // =============== Trigonometric functions ===============
        (
            norm_expr!(
            D[
                Sin[Pattern[f, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(Cos[f] * D[f, x]),
        ),
        (
            norm_expr!(
            D[
                Cos[Pattern[f, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(-Sin[f] * D[f, x]),
        ),
        (
            norm_expr!(
            D[
                Tan[Pattern[f, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!((1 / (Cos[f] ^ 2)) * D[f, x]),
        ),
        (
            norm_expr!(
            D[
                Cot[Pattern[f, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(-(1 / (Sin[f] ^ 2)) * D[f, x]),
        ),
        (
            norm_expr!(
            D[
                Sec[Pattern[f, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(Sec[f] * Tan[f] * D[f, x]),
        ),
        (
            norm_expr!(
            D[
                Csc[Pattern[f, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(-Csc[f] * Cot[f] * D[f, x]),
        ),
        // =============== Inverse Trigonometric functions ===============
        (
            norm_expr!(
            D[
                ArcSin[Pattern[f, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!((1 / (1 - f ^ 2) ^ (1 / 2)) * D[f, x]),
        ),
        (
            norm_expr!(
            D[
                ArcCos[Pattern[f, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(-(1 / (1 - f ^ 2) ^ (1 / 2)) * D[f, x]),
        ),
        (
            norm_expr!(
            D[
                ArcTan[Pattern[f, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!((1 / (1 + f ^ 2)) * D[f, x]),
        ),
        // =============== Inverse Trigonometric functions ===============
        (
            norm_expr!(
            D[
                ArcSin[Pattern[f, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!((1 / (1 - f ^ 2) ^ (1 / 2)) * D[f, x]),
        ),
        (
            norm_expr!(
            D[
                ArcCos[Pattern[f, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(-(1 / (1 - f ^ 2) ^ (1 / 2)) * D[f, x]),
        ),
        (
            norm_expr!(
            D[
                ArcTan[Pattern[f, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!((1 / (1 + f ^ 2)) * D[f, x]),
        ),
        // =============== Hyperbolic functions ===============
        (
            norm_expr!(
            D[
                Sinh[Pattern[f, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(Cosh[f] * D[f, x]),
        ),
        (
            norm_expr!(
            D[
                Cosh[Pattern[f, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!(Sinh[f] * D[f, x]),
        ),
        (
            norm_expr!(
            D[
                Tanh[Pattern[f, Blank[]]],
                PatternTest[Pattern[x, Blank[]], IsSymbolQ]
            ]),
            expr!((1 / (Cosh[f] ^ 2)) * D[f, x]),
        ),
    ]
}
