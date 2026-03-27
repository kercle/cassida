use crate::{
    builtins::{
        BuiltInCategory,
        traits::{ApplicationError, BuiltIn, BuiltInDoc, PatternDoc},
    },
    ensure,
    expr::{Expr, NormExpr},
    norm_expr,
    pattern::environment::Environment,
    raw_expr,
    rewrite::Rewriter,
};

pub struct Derivative {
    rewriter: Rewriter,
}

impl Derivative {
    pub fn new() -> Self {
        Self {
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
    #[inline(always)]
    fn head() -> &'static str {
        "Diff"
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::Calculus,
            title: "Derivative",
            summary: "Symbolically determine derivatives of expressions.",
            pattern_doc: vec![PatternDoc::new(
                raw_expr!( Diff[f_,x_?IsSymbol] ),
                "gives the partial derivative $\\frac{\\partial f}{\\partial x}$",
            )],
            examples: vec![
                ("Diff[Cos[Exp[x]], x]", "-Exp[x]*Sin[Exp[x]]"),
                ("Diff[y,x]", "0"),
            ],
            related: vec!["Integrate"],
        }
    }

    fn apply_all(&self, expr: NormExpr) -> NormExpr {
        expr.rewrite_all(&self.rewriter, 1000)
    }

    fn validate_application<S>(expr: &Expr<S>) -> Result<(), ApplicationError> {
        ensure!(expr.args_len() == 2, ApplicationError::ArityMismatch);
        ensure!(
            expr.get_arg(1).is_some_and(|a| a.is_symbol()),
            ApplicationError::ExpectedSymbolAt(1)
        );
        ensure!(expr.is_head(Self::head()), ApplicationError::HeadMismatch);
        Ok(())
    }
}

fn build_rewriter() -> Rewriter {
    let rules = vec![
        // =============== Linearity ===============
        (
            norm_expr!( Diff[f_ + r__, x_?IsSymbol] ),
            raw_expr!( Diff[f, x] + Diff[Add[r],x] ),
        ),
        (
            norm_expr!( Diff[c_?IsNumber * r__, x_?IsSymbol] ),
            raw_expr!( c * Diff[Mul[r],x] ),
        ),
        // =============== Basic ===============
        (norm_expr!( Diff[x_, x_?IsSymbol] ), raw_expr!(1)),
        (norm_expr!( Diff[c_?IsNumber, x_?IsSymbol] ), raw_expr!(0)),
        (norm_expr!( Diff[a_?IsSymbol,x_?IsSymbol] ), raw_expr!(0)),
        (
            norm_expr!( Diff[f_ * g__, x_?IsSymbol] ),
            raw_expr!(Diff[f, x] * g + f * Diff[Mul[g], x]),
        ),
        // =============== Powers ===============
        (
            norm_expr!( Diff[f_ ^ g_, x_?IsSymbol] ),
            raw_expr!((f ^ g) *((g / f) * Diff[f, x] + Log[f] * Diff[g, x])),
        ),
        // =============== Exponential ===============
        (
            norm_expr!( Diff[Exp[f_], x_?IsSymbol] ),
            raw_expr!(Exp[f] * Diff[f, x]),
        ),
        // =============== Logarithms ===============
        (
            norm_expr!( Diff[Log[f_], x_?IsSymbol] ),
            raw_expr!((1 / f) * Diff[f, x]),
        ),
        // =============== Trigonometric functions ===============
        (
            norm_expr!( Diff[Sin[f_], x_?IsSymbol] ),
            raw_expr!(Cos[f] * Diff[f, x]),
        ),
        (
            norm_expr!( Diff[Cos[f_], x_?IsSymbol] ),
            raw_expr!(-Sin[f] * Diff[f, x]),
        ),
        (
            norm_expr!( Diff[Tan[f_], x_?IsSymbol] ),
            raw_expr!((1 / (Cos[f] ^ 2)) * Diff[f, x]),
        ),
        (
            norm_expr!( Diff[Cot[f_], x_?IsSymbol] ),
            raw_expr!(-(1 / (Sin[f] ^ 2)) * Diff[f, x]),
        ),
        (
            norm_expr!( Diff[Sec[f_], x_?IsSymbol] ),
            raw_expr!(Sec[f] * Tan[f] * Diff[f, x]),
        ),
        (
            norm_expr!( Diff[Csc[f_], x_?IsSymbol] ),
            raw_expr!(-Csc[f] * Cot[f] * Diff[f, x]),
        ),
        // =============== Inverse Trigonometric functions ===============
        (
            norm_expr!( Diff[ArcSin[f_], x_?IsSymbol] ),
            raw_expr!((1 / (1 - f ^ 2) ^ (1 / 2)) * Diff[f, x]),
        ),
        (
            norm_expr!( Diff[ArcCos[f_], x_?IsSymbol] ),
            raw_expr!(-(1 / (1 - f ^ 2) ^ (1 / 2)) * Diff[f, x]),
        ),
        (
            norm_expr!( Diff[ArcTan[f_], x_?IsSymbol] ),
            raw_expr!((1 / (1 + f ^ 2)) * Diff[f, x]),
        ),
        // =============== Inverse Trigonometric functions ===============
        (
            norm_expr!( Diff[ArcSin[f_], x_?IsSymbol] ),
            raw_expr!((1 / (1 - f ^ 2) ^ (1 / 2)) * Diff[f, x]),
        ),
        (
            norm_expr!( Diff[ArcCos[f_], x_?IsSymbol] ),
            raw_expr!(-(1 / (1 - f ^ 2) ^ (1 / 2)) * Diff[f, x]),
        ),
        (
            norm_expr!(
            Diff[ArcTan[f_], x_?IsSymbol] ),
            raw_expr!((1 / (1 + f ^ 2)) * Diff[f, x]),
        ),
        // =============== Hyperbolic functions ===============
        (
            norm_expr!( Diff[Sinh[f_], x_?IsSymbol] ),
            raw_expr!(Cosh[f] * Diff[f, x]),
        ),
        (
            norm_expr!( Diff[Cosh[f_], x_?IsSymbol] ),
            raw_expr!(Sinh[f] * Diff[f, x]),
        ),
        (
            norm_expr!( Diff[Tanh[f_], x_?IsSymbol] ),
            raw_expr!((1 / (Cosh[f] ^ 2)) * Diff[f, x]),
        ),
    ];

    Rewriter::new().with_rules_from_tuples(
        rules
            .into_iter()
            .map(|(pat, repl)| (pat, move |ctx: &Environment<'_, '_>| ctx.fill(repl.clone()))),
    )
}
