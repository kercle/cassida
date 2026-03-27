use crate::{
    builtins::{
        BuiltInCategory,
        traits::{ApplicationError, BuiltIn, BuiltInDoc, PatternDoc},
    }, ensure, expr::{Expr, }, raw_expr
};

#[derive(Default)]
pub struct BlankSeq;

impl BlankSeq {
    pub const HEAD: &'static str = "BlankSeq";
}

impl BuiltIn for BlankSeq {
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
            summary: "Matches any non-empty sub-sequence of subset in node arguments.",
            pattern_doc: vec![
                PatternDoc::new(
                    raw_expr!(BlankSeq[]),
                    "Matches any non-empty sub-sequence of subset in node arguments.",
                ),
                PatternDoc::new(
                    raw_expr!(BlankSeq[h_?IsSymbol]),
                    "Matches any non-empty sub-sequence of subset in node arguments with head $h$.",
                ),
            ],
            examples: vec![],
            related: vec![],
        }
    }

    fn validate_application<S>(expr: &Expr<S>) -> Result<(), ApplicationError> {
        ensure!(expr.args_len() <= 1, ApplicationError::ArityMismatch);
        ensure!(expr.is_head(Self::head()), ApplicationError::HeadMismatch);
        Ok(())
    }
}
