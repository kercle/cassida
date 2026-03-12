use crate::{
    builtins::traits::{BuiltIn, PatternDoc},
    expr::NormExpr,
    hold_expr, norm_expr,
    pattern::environment::Environment,
    rewrite::Rewriter,
};

pub struct Expand {
    pattern_doc: Vec<PatternDoc>,
    rewriter: Rewriter,
}

impl Expand {
    pub fn new() -> Self {
        Self {
            pattern_doc: vec![PatternDoc::new("Expand[t_]", "Expands the given term $t$.")],
            rewriter: build_rewriter(),
        }
    }
}

impl Default for Expand {
    fn default() -> Self {
        Self::new()
    }
}

impl BuiltIn for Expand {
    fn title(&self) -> String {
        "Term expansion".to_string()
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

    fn apply_all(&self, expr: NormExpr) -> NormExpr {
        expr.rewrite_all(&self.rewriter, 100)
    }
}

pub(super) fn build_rewriter() -> Rewriter {
    let rules = vec![
        (
            norm_expr!(Expand[a_ + b__]),
            hold_expr!(Expand[a] + Expand[Add[b]]),
        ),
        (
            norm_expr!(Expand[a__ * (b_ + c__)]),
            hold_expr!(Expand[Mul[a] * b] + Expand[Mul[a] * Add[c]]),
        ),
        (
            norm_expr!(Expand[a__ * (b_ + c__) ^ PatternTest[m_, IsPositiveInteger]]),
            hold_expr!(Expand[Mul[a] * (b + c) ^ (m - 1) * b + Mul[a] * (b + c) ^ (m - 1) * c]),
        ),
        (
            norm_expr!(Expand[(b_ + c__) ^ PatternTest[m_, IsPositiveInteger]]),
            hold_expr!(Expand[(b + c) ^ (m - 1) * b + (b + c) ^ (m - 1) * c]),
        ),
        // Base case
        (norm_expr!(Expand[a_]), hold_expr!(a)),
    ];

    Rewriter::new().with_rules(rules.into_iter().map(|(pat, repl)| {
        (pat, move |ctx: &Environment<'_, '_>| {
            ctx.fill(repl.clone()).normalize()
        })
    }))
}
