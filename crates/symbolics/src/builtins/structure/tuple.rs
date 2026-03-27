use crate::{
    builtins::{
        BuiltInCategory,
        traits::{ApplicationError, BuiltIn, BuiltInDoc, PatternDoc},
    }, ensure, expr::{Expr, }, raw_expr
};

#[derive(Default)]
pub struct Tuple;

impl BuiltIn for Tuple {
    #[inline(always)]
    fn head() -> &'static str {
        "Tuple"
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::Structure,
            title: Self::head(),
            summary: "Represents a finite ordered list of expressions. It can also be considered a vector.",
            pattern_doc: vec![PatternDoc::new(
                raw_expr!(Tuple[t___]),
                "The vector $(t_1,\\dots,t_n)$.",
            )],
            examples: vec![],
            related: vec![],
        }
    }

    fn validate_application<S>(expr: &Expr<S>) -> Result<(), ApplicationError> {
        ensure!(expr.is_head(Self::head()), ApplicationError::HeadMismatch);
        Ok(())
    }
}
