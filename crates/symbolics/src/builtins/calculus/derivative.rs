use crate::{
    builtins::{
        BuiltInCategory,
        traits::{BuiltIn, BuiltInDoc, PatternDoc},
    },
    expr::NormExpr,
    norm_expr,
    pattern::environment::Environment,
    raw_expr,
    rewrite::Rewriter,
};

pub struct Derivative {
    pattern_doc: Vec<PatternDoc>,
    rewriter: Rewriter,
}

impl Derivative {
    pub fn new() -> Self {
        Self {
            pattern_doc: vec![PatternDoc::new(
                "Diff[f_,x_?IsSymbol]",
                "gives the partial derivative $\\frac{\\partial f}{\\partial x}$",
            )],
            rewriter: build_rewriter(),
        }
    }
}

impl Default for Derivative {
    fn default() -> Self {
        Self::new()
    }
}

impl BuiltIn for Derivative {
    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::Calculus,
            title: "Derivative",
            summary: "Symbolically determine derivatives of expressions.",
            pattern_doc: self.pattern_doc.clone(),
            examples: vec![
                ("Diff[Cos[Exp[x]], x]", "-Exp[x]*Sin[Exp[x]]"),
                ("Diff[y,x]", "0"),
            ],
            related: vec!["Integrate"],
        }
    }

    fn head_symbol(&self) -> &'static str {
        "Diff"
    }

    fn apply_all(&self, expr: NormExpr) -> NormExpr {
        expr.rewrite_all(&self.rewriter, 1000)
    }
}

fn build_rewriter() -> Rewriter {
    let rules = vec![
        // =============== Linearity ===============
        (
            norm_expr!( Diff[f_ + r__, PatternTest[x_, IsSymbol]] ),
            raw_expr!( Diff[f, x] + Diff[Add[r],x] ),
        ),
        (
            norm_expr!(
            Diff[
                PatternTest[c_, IsNumber] * r__,
                PatternTest[x_, IsSymbol]
            ]),
            raw_expr!(
            c * Diff[Mul[r],x]
            ),
        ),
        // =============== Basic ===============
        (
            norm_expr!( Diff[x_, PatternTest[x_, IsSymbol]] ),
            raw_expr!(1),
        ),
        (
            norm_expr!(
            Diff[
                PatternTest[c_, IsNumber],
                PatternTest[x_, IsSymbol]
            ]),
            raw_expr!(0),
        ),
        (
            norm_expr!(
            Diff[
                PatternTest[a_, IsSymbol],
                PatternTest[x_, IsSymbol]
            ]),
            raw_expr!(0),
        ),
        (
            norm_expr!( Diff[f_ * g__, PatternTest[x_, IsSymbol]] ),
            raw_expr!(Diff[f, x] * g + f * Diff[Mul[g], x]),
        ),
        // =============== Powers ===============
        (
            norm_expr!(
            Diff[
                f_ ^ g_,
                PatternTest[x_, IsSymbol]
            ]),
            raw_expr!((f ^ g) *((g / f) * Diff[f, x] + Log[f] * Diff[g, x])),
        ),
        // =============== Exponential ===============
        (
            norm_expr!(
            Diff[
                Exp[f_],
                PatternTest[x_, IsSymbol]
            ]),
            raw_expr!(Exp[f] * Diff[f, x]),
        ),
        // =============== Logarithms ===============
        (
            norm_expr!(
            Diff[
                Log[f_],
                PatternTest[x_, IsSymbol]
            ]),
            raw_expr!((1 / f) * Diff[f, x]),
        ),
        // =============== Trigonometric functions ===============
        (
            norm_expr!(
            Diff[
                Sin[f_],
                PatternTest[x_, IsSymbol]
            ]),
            raw_expr!(Cos[f] * Diff[f, x]),
        ),
        (
            norm_expr!(
            Diff[
                Cos[f_],
                PatternTest[x_, IsSymbol]
            ]),
            raw_expr!(-Sin[f] * Diff[f, x]),
        ),
        (
            norm_expr!(
            Diff[
                Tan[f_],
                PatternTest[x_, IsSymbol]
            ]),
            raw_expr!((1 / (Cos[f] ^ 2)) * Diff[f, x]),
        ),
        (
            norm_expr!(
            Diff[
                Cot[f_],
                PatternTest[x_, IsSymbol]
            ]),
            raw_expr!(-(1 / (Sin[f] ^ 2)) * Diff[f, x]),
        ),
        (
            norm_expr!(
            Diff[
                Sec[f_],
                PatternTest[x_, IsSymbol]
            ]),
            raw_expr!(Sec[f] * Tan[f] * Diff[f, x]),
        ),
        (
            norm_expr!(
            Diff[
                Csc[f_],
                PatternTest[x_, IsSymbol]
            ]),
            raw_expr!(-Csc[f] * Cot[f] * Diff[f, x]),
        ),
        // =============== Inverse Trigonometric functions ===============
        (
            norm_expr!(
            Diff[
                ArcSin[f_],
                PatternTest[x_, IsSymbol]
            ]),
            raw_expr!((1 / (1 - f ^ 2) ^ (1 / 2)) * Diff[f, x]),
        ),
        (
            norm_expr!(
            Diff[
                ArcCos[f_],
                PatternTest[x_, IsSymbol]
            ]),
            raw_expr!(-(1 / (1 - f ^ 2) ^ (1 / 2)) * Diff[f, x]),
        ),
        (
            norm_expr!(
            Diff[
                ArcTan[f_],
                PatternTest[x_, IsSymbol]
            ]),
            raw_expr!((1 / (1 + f ^ 2)) * Diff[f, x]),
        ),
        // =============== Inverse Trigonometric functions ===============
        (
            norm_expr!(
            Diff[
                ArcSin[f_],
                PatternTest[x_, IsSymbol]
            ]),
            raw_expr!((1 / (1 - f ^ 2) ^ (1 / 2)) * Diff[f, x]),
        ),
        (
            norm_expr!(
            Diff[
                ArcCos[f_],
                PatternTest[x_, IsSymbol]
            ]),
            raw_expr!(-(1 / (1 - f ^ 2) ^ (1 / 2)) * Diff[f, x]),
        ),
        (
            norm_expr!(
            Diff[
                ArcTan[f_],
                PatternTest[x_, IsSymbol]
            ]),
            raw_expr!((1 / (1 + f ^ 2)) * Diff[f, x]),
        ),
        // =============== Hyperbolic functions ===============
        (
            norm_expr!(
            Diff[
                Sinh[f_],
                PatternTest[x_, IsSymbol]
            ]),
            raw_expr!(Cosh[f] * Diff[f, x]),
        ),
        (
            norm_expr!(
            Diff[
                Cosh[f_],
                PatternTest[x_, IsSymbol]
            ]),
            raw_expr!(Sinh[f] * Diff[f, x]),
        ),
        (
            norm_expr!(
            Diff[
                Tanh[f_],
                PatternTest[x_, IsSymbol]
            ]),
            raw_expr!((1 / (Cosh[f] ^ 2)) * Diff[f, x]),
        ),
    ];

    Rewriter::new().with_rules(
        rules
            .into_iter()
            .map(|(pat, repl)| (pat, move |ctx: &Environment<'_, '_>| ctx.fill(repl.clone()))),
    )
}
