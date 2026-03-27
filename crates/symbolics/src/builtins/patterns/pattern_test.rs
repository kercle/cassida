use crate::{
    builtins::{
        BuiltInCategory,
        traits::{ApplicationError, BuiltIn, BuiltInDoc, PatternDoc},
    }, ensure, expr::{Expr, }, raw_expr
};

#[derive(Default)]
pub struct PatternTest;

impl PatternTest {
    pub const HEAD: &'static str = "PatternTest";
}

impl BuiltIn for PatternTest {
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
            summary: "Runs a given predicate on a matched subject.",
            pattern_doc: vec![PatternDoc::new(
                raw_expr!(PatternTest[p_, t_?IsSymbol]),
                "If the pattern $p$ matches a given subject, the test $t$ is executated. The pattern matches iff the predicte returns true.",
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
