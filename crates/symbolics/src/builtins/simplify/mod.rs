pub mod expand;
pub mod factor;
pub mod known_values;
pub mod trigonometric;

use crate::{
    builtins::traits::{BuiltIn, PatternDoc},
    expr::NormExpr,
    format::MathDisplay,
    raw_expr,
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
            pattern_doc: vec![PatternDoc {
                pattern: raw_expr!(Simplify[t_]).to_input_form(),
                summary: "simplifies the term $t$".to_string(),
            }],
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
    fn title(&self) -> String {
        "Simplify".to_string()
    }

    fn head_symbol(&self) -> &'static str {
        "Simplify"
    }

    fn summary(&self) -> &'static str {
        "Simplify a given expression."
    }

    fn pattern_doc(&self) -> Vec<PatternDoc> {
        self.pattern_doc.clone()
    }

    fn apply_all(&self, expr: NormExpr) -> NormExpr {
        expr.rewrite_all(&self.known_values_rewriter, 10)
            .rewrite_all(&self.factorization_rewriter, 10)
            .rewrite_all(&self.trigonometric_rewriter, 10)
    }
}
