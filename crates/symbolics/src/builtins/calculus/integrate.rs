use crate::{
    builtins::{
        BuiltInCategory,
        traits::{BuiltIn, BuiltInDoc, PatternDoc},
    },
    expr::NormExpr,
    norm_expr, raw_expr,
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
                raw_expr!( Integrate[f_, x_?IsSymbol] ),
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
    #[inline(always)]
    fn head() -> &'static str {
        "Integrate"
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::Calculus,
            title: Self::head(),
            summary: "Symbolically determine integrals of expressions.",
            pattern_doc: self.pattern_doc.clone(),
            examples: vec![
                ("Integrate[x,x]", "x^2/2"),
                ("Integrate[2 * Sqrt[x] + x^6, x]", "(4/3)*x^(3/2) + x^7/7"),
                ("Integrate[Cos[x+f[y]],x]", "Sin[x+f[y]]"),
            ],
            related: vec!["Diff"],
        }
    }

    fn apply_all(&self, expr: NormExpr) -> NormExpr {
        expr.rewrite_all(&self.rewriter, 1000)
    }
}

fn build_rewriter() -> Rewriter {
    Rewriter::new().with_rules(&norm_expr!({
        // =============== Linearity ===============

        Integrate[f_ + r__, x_?IsSymbol] :> Integrate[f, x] + Integrate[Add[r],x];
        Integrate[c_ * r__, x_?IsSymbol] /; FreeOf[c, x] :> c * Integrate[Mul[r],x];

        // =============== Basic ===============

         Integrate[c_, x_?IsSymbol] /; FreeOf[c, x] :> c * x;
         Integrate[x_, x_?IsSymbol] :>  x ^ 2 / 2;
         Integrate[x_ * f__, x_?IsSymbol] :>  x * Integrate[Mul[f], x] - Integrate[Integrate[Mul[f], x], x];
         Integrate[x_^n_?IsPositiveInteger * f__, x_?IsSymbol] :>  x^n * Integrate[Mul[f], x] - n * Integrate[x^(n-1) * Integrate[Mul[f], x], x];

        // =============== Powers ===============

        Integrate[1 / (a_. + b_. * x_), x_?IsSymbol] /; FreeOf[(a, b), x] :> Log[Abs[a + b * x]] / b;
        Integrate[x_ ^ k_?IsNumber, x_?IsSymbol] :> x ^ (k + 1) / (k + 1);

        // =============== Exponentials ===============

        Integrate[Exp[a_. + b_. * x_], x_?IsSymbol] /; FreeOf[(a, b), x] :> Exp[a + b * x] / b;

        // =============== Logarithms ===============

        Integrate[Log[a_. + b_. * x_], x_?IsSymbol] /; FreeOf[(a, b), x] :> (a / b + x) * Log[a + b * x] - x;

        // =============== Trigonometric functions ===============

        Integrate[Sin[a_. + b_. * x_], x_?IsSymbol] /; FreeOf[(a, b), x] :> -Cos[a + b * x] / b;
        Integrate[Cos[a_. + b_. * x_], x_?IsSymbol] /; FreeOf[(a, b), x] :> Sin[a + b * x] / b;
    }))
}
