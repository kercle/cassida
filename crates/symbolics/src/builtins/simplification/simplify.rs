use crate::{
    builtins::{
        simplification::{factor, known_values, trigonometric},
        traits::{BuiltIn, BuiltInDoc, PatternDoc},
    },
    expr::NormExpr,
    rewrite::Rewriter,
};

pub struct Simplify {
    pattern_doc: Vec<PatternDoc>,
    factorization_rewriter: Rewriter,
    trigonometric_rewriter: Rewriter,

    // this should be handled by the builtin functions individually
    known_values_rewriter: Rewriter,
}

impl Simplify {
    pub fn new() -> Self {
        let factorization_rewriter = factor::build_rewriter();
        let trigonometric_rewriter = trigonometric::build_rewriter();
        let known_values_rewriter = known_values::build_rewriter();

        Self {
            pattern_doc: vec![PatternDoc::new("Simplify[t_]", "simplifies the term $t$")],
            factorization_rewriter,
            trigonometric_rewriter,
            known_values_rewriter,
        }
    }
}

impl Default for Simplify {
    fn default() -> Self {
        Self::new()
    }
}

impl BuiltIn for Simplify {
    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: "Simplification",
            title: "Basic simplification",
            summary: "Simplify a given expression.",
            pattern_doc: self.pattern_doc.clone(),
            examples: vec![],
            related: vec!["Expand"],
        }
    }

    fn head_symbol(&self) -> &'static str {
        "Simplify"
    }

    fn apply_all(&self, expr: NormExpr) -> NormExpr {
        expr.rewrite_all(&self.known_values_rewriter, 10)
            .rewrite_all(&self.factorization_rewriter, 10)
            .rewrite_all(&self.trigonometric_rewriter, 10)
    }
}
