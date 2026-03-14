use std::rc::Rc;

use numbers::alg::binomial::BinomialGenerator;

use crate::{
    builtins::traits::{BuiltIn, PatternDoc},
    expr::NormExpr,
    norm_expr,
    pattern::environment::Environment,
    raw_expr,
    rewrite::Rewriter,
};

pub struct Expand {
    pattern_doc: Vec<PatternDoc>,
    rewriter: Rewriter,
    binomial_generator: Rc<BinomialGenerator>,
}

impl Expand {
    pub fn new(binomial_generator: Rc<BinomialGenerator>) -> Self {
        Self {
            pattern_doc: vec![PatternDoc::new("Expand[t_]", "Expands the given term $t$.")],
            rewriter: build_rewriter(),
            binomial_generator,
        }
    }
}

impl BuiltIn for Expand {
    fn category(&self) -> &'static str {
        "Simplification"
    }

    fn title(&self) -> &'static str {
        "Term expansion"
    }

    fn head_symbol(&self) -> &'static str {
        "Expand"
    }

    fn summary(&self) -> &'static str {
        "Expand factors."
    }

    fn pattern_doc(&self) -> Vec<PatternDoc> {
        self.pattern_doc.clone()
    }

    fn examples(&self) -> Vec<(&'static str, &'static str)> {
        vec![("x*(4 + x*(5 - x))", "4*x + 5*x^2 - x^3")]
    }

    fn related(&self) -> Vec<&'static str> {
        vec!["Simplify"]
    }

    fn apply_all(&self, expr: NormExpr) -> NormExpr {
        expr.rewrite_all(&self.rewriter, 100)
    }
}

pub(super) fn build_rewriter() -> Rewriter {
    let rules = vec![
        (
            norm_expr!(Expand[a_ + b__]),
            raw_expr!(Expand[a] + Expand[Add[b]]),
        ),
        (
            norm_expr!(Expand[a__ * (b_ + c__)]),
            raw_expr!(Expand[Mul[a] * b] + Expand[Mul[a] * Add[c]]),
        ),
        (
            norm_expr!(Expand[a__ * (b_ + c__) ^ PatternTest[m_, IsPositiveInteger]]),
            raw_expr!(Expand[Mul[a] * (b + c) ^ (m - 1) * b + Mul[a] * (b + c) ^ (m - 1) * c]),
        ),
        (
            norm_expr!(Expand[(b_ + c__) ^ PatternTest[m_, IsPositiveInteger]]),
            raw_expr!(Expand[(b + c) ^ (m - 1) * b + (b + c) ^ (m - 1) * c]),
        ),
        // Base case
        (norm_expr!(Expand[a_]), raw_expr!(a)),
    ];

    Rewriter::new().with_rules(rules.into_iter().map(|(pat, repl)| {
        (pat, move |ctx: &Environment<'_, '_>| {
            ctx.fill(repl.clone()).normalize()
        })
    }))
}
