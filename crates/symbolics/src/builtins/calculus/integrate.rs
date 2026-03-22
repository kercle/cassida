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

pub struct Integrate {
    pattern_doc: Vec<PatternDoc>,
    rewriter: Rewriter,
}

impl Integrate {
    pub fn new() -> Self {
        Self {
            pattern_doc: vec![PatternDoc::new(
                "Integrate[f_, x_?IsSymbol]",
                "gives the indefinite integral (anti-derivative) $\\int f(x)\\!{\\rm d}x$",
            )],
            rewriter: build_rewriter(),
        }
    }
}

impl Default for Integrate {
    fn default() -> Self {
        Self::new()
    }
}

impl BuiltIn for Integrate {
    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::Calculus,
            title: "Integration",
            summary: "Symbolically determine integrals of expressions.",
            pattern_doc: self.pattern_doc.clone(),
            examples: vec![
                ("Integrate[x,x]", "x^2/2"),
                ("Integrate[2 * Sqrt[x] + x^6, x]", "(4/3)*x^(3/2) + x^7/7"),
            ],
            related: vec!["Diff"],
        }
    }

    fn head_symbol(&self) -> &'static str {
        "Integrate"
    }

    fn apply_all(&self, expr: NormExpr) -> NormExpr {
        expr.rewrite_all(&self.rewriter, 1000)
    }
}

fn build_rewriter() -> Rewriter {
    let rules = vec![
        // =============== Linearity ===============
        (
            norm_expr!( Integrate[f_ + r__, x_?IsSymbol] ),
            raw_expr!( Integrate[f, x] + Integrate[Add[r],x] ),
        ),
        (
            norm_expr!( Integrate[c_?IsNumber * r__, x_?IsSymbol] ),
            raw_expr!( c * Integrate[Mul[r],x] ),
        ),
        // =============== Basic ===============
        (
            norm_expr!( Integrate[c_?IsNumber, x_?IsSymbol] ),
            raw_expr!(c * x),
        ),
        (
            norm_expr!( Integrate[x_, x_?IsSymbol] ),
            raw_expr!(x ^ 2 / 2),
        ),
        (
            norm_expr!( Integrate[c_?IsSymbol, x_?IsSymbol] ),
            raw_expr!(c * x),
        ),
        // (
        //     norm_expr!( Integrate[x_ * f__, x_?IsSymbol] ),
        //     raw_expr!(x * Integrate[Mul[f], x] - Integrate[Integrate[Mul[f], x], x]),
        // ),
        // (
        //     norm_expr!( Integrate[x_^n_?IsPositiveInteger * f__, x_?IsSymbol] ),
        //     raw_expr!(x^n * Integrate[Mul[f], x] - n * Integrate[x^(n-1) * Integrate[Mul[f], x], x]),
        // ),
        // =============== Powers ===============
        (
            norm_expr!( Integrate[1 / x_, x_?IsSymbol] ),
            raw_expr!(Log[Abs[x]]),
        ),
        (
            norm_expr!( Integrate[x_ ^ k_?IsNumber, x_?IsSymbol] ),
            raw_expr!(x ^ (k + 1) / (k + 1)),
        ),
        // =============== Exponentials ===============
        (
            norm_expr!( Integrate[Exp[x_], x_?IsSymbol] ),
            raw_expr!(Exp[x]),
        ),
        // =============== Logarithms ===============
        (
            norm_expr!( Integrate[Log[x_], x_?IsSymbol] ),
            raw_expr!(x * Log[x] - x),
        ),
        // =============== Trigonometric functions ===============
        (
            norm_expr!( Integrate[Sin[x_], x_?IsSymbol] ),
            raw_expr!(-Cos[x]),
        ),
        (
            norm_expr!( Integrate[Cos[x_], x_?IsSymbol] ),
            raw_expr!(Sin[x]),
        ),
    ];

    Rewriter::new().with_rules(
        rules
            .into_iter()
            .map(|(pat, repl)| (pat, move |ctx: &Environment<'_, '_>| ctx.fill(repl.clone()))),
    )
}
