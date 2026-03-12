use crate::{
    builtins::traits::{BuiltIn, PatternDoc},
    expr::NormExpr,
    format::MathDisplay,
    hold_expr, norm_expr,
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
            pattern_doc: vec![PatternDoc {
                pattern: raw_expr!( Integrate[f_, PatternTest[x_, IsSymbol]] ).to_input_form(),
                summary: "gives the indefinite integral (anti-derivative) if $f(x)$: $\\int f\\,{\rm d}x$"
                    .to_string(),
            }],
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
    fn title(&self) -> String {
        "Integration".to_string()
    }

    fn head_symbol(&self) -> &'static str {
        "Integrate"
    }

    fn summary(&self) -> &'static str {
        "Determine integrals."
    }

    fn pattern_doc(&self) -> Vec<PatternDoc> {
        self.pattern_doc.clone()
    }

    fn apply_all(&self, expr: NormExpr) -> NormExpr {
        expr.rewrite_all(&self.rewriter, 1000)
    }
}

fn build_rewriter() -> Rewriter {
    let rules = vec![
        // =============== Linearity ===============
        (
            norm_expr!( Integrate[f_ + r__, PatternTest[x_, IsSymbol]] ),
            hold_expr!( Integrate[f, x] + Integrate[Add[r],x] ),
        ),
        (
            norm_expr!( Integrate[PatternTest[c_, IsNumber] * r__, PatternTest[x_, IsSymbol]] ),
            hold_expr!( c * Integrate[Mul[r],x] ),
        ),
        // =============== Basic ===============
        (
            norm_expr!( Integrate[PatternTest[c_, IsNumber], PatternTest[x_, IsSymbol]] ),
            hold_expr!(c * x),
        ),
        (
            norm_expr!(
            Integrate[
                x_,
                PatternTest[x_, IsSymbol]
            ]),
            hold_expr!(x ^ 2 / 2),
        ),
        (
            norm_expr!(
            Integrate[
                PatternTest[c_, IsSymbol],
                PatternTest[x_, IsSymbol]
            ]),
            hold_expr!(c * x),
        ),
        (
            norm_expr!(
            Integrate[
                PatternTest[a_, IsSymbol],
                PatternTest[x_, IsSymbol]
            ]),
            hold_expr!(a * x),
        ),
        // =============== Powers ===============
        (
            norm_expr!(
            Integrate[
                1 / x_,
                PatternTest[x_, IsSymbol]
            ]),
            hold_expr!(Log[Abs[x]]),
        ),
        (
            norm_expr!(
            Integrate[
                x_ ^ PatternTest[k_, IsNumber],
                PatternTest[x_, IsSymbol]
            ]),
            hold_expr!(x ^ (k + 1) / (k + 1)),
        ),
        // =============== Exponentials ===============
        (
            norm_expr!(
            Integrate[
                Exp[x_],
                PatternTest[x_, IsSymbol]
            ]),
            hold_expr!(Exp[x]),
        ),
        // =============== Logarithms ===============
        (
            norm_expr!(
            Integrate[
                Log[x_],
                PatternTest[x_, IsSymbol]
            ]),
            hold_expr!(x * Log[x] - x),
        ),
        // =============== Trigonometric functions ===============
        (
            norm_expr!(
            Integrate[
                Sin[x_],
                PatternTest[x_, IsSymbol]
            ]),
            hold_expr!(-Cos[x]),
        ),
        (
            norm_expr!(
            Integrate[
                Cos[x_],
                PatternTest[x_, IsSymbol]
            ]),
            hold_expr!(Sin[x]),
        ),
    ];

    Rewriter::new().with_rules(rules.into_iter().map(|(pat, repl)| {
        (pat, move |ctx: &Environment<'_, '_>| {
            ctx.fill(repl.clone()).normalize().release_all_holds()
        })
    }))
}
