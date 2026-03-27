use crate::{
    builtins::{
        BuiltInCategory,
        traits::{ApplicationError, BuiltIn, BuiltInDoc},
    },
    ensure,
    expr::Expr,
};

#[derive(Default)]
pub struct Greater;

impl BuiltIn for Greater {
    #[inline(always)]
    fn head() -> &'static str {
        "Greater"
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::Relational,
            title: Self::head(),
            summary: "Internal representation of the $>$ comparison relation.",
            pattern_doc: vec![],
            examples: vec![],
            related: vec![],
        }
    }

    fn validate_application_of<S>(
        head: &Expr<S>,
        children: &[Expr<S>],
    ) -> Result<(), ApplicationError> {
        ensure!(children.len() == 2, ApplicationError::ArityMismatch);
        ensure!(
            head.matches_symbol(Self::head()),
            ApplicationError::HeadMismatch
        );
        Ok(())
    }
}
