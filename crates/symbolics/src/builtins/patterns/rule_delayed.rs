use crate::{
    builtins::{
        BuiltInCategory,
        traits::{ApplicationError, BuiltIn, BuiltInDoc, PatternDoc},
    }, ensure, expr::{Expr, }, raw_expr
};

#[derive(Default)]
pub struct RuleDelayed;

impl RuleDelayed {
    pub const HEAD: &'static str = "RuleDelayed";
}

impl BuiltIn for RuleDelayed {
    #[inline(always)]
    fn head() -> &'static str {
        Self::HEAD
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::Patterns,
            title: Self::head(),
            summary: "Creates a rewrite rule which does not evaluate (normalize) the replacement.",
            pattern_doc: vec![PatternDoc::new(
                raw_expr!(RuleDelayed[p_, r_]),
                "If the rule is applied to an expression and the pattern $p$ matches, it is transformed into the replacement.",
            )],
            examples: vec![],
            related: vec![],
        }
    }

    fn validate_application<S>(expr: &Expr<S>) -> Result<(), ApplicationError> {
        ensure!(expr.args_len() == 2, ApplicationError::ArityMismatch);
        ensure!(expr.is_head(Self::head()), ApplicationError::HeadMismatch);
        Ok(())
    }
}
