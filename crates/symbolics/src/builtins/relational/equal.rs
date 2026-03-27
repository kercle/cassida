use crate::{
    builtins::{
        BuiltInCategory,
        traits::{ApplicationError, BuiltIn, BuiltInDoc},
    },
    ensure,
    expr::Expr,
};

#[derive(Default)]
pub struct Equal;

impl BuiltIn for Equal {
    #[inline(always)]
    fn head() -> &'static str {
        "Equal"
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::Relational,
            title: Self::head(),
            summary: "Internal representation of the $=$ comparison relation.",
            pattern_doc: vec![],
            examples: vec![],
            related: vec![],
        }
    }

    fn validate_application_of<S>(
        head: &Expr<S>,
        _children: &[Expr<S>],
    ) -> Result<(), ApplicationError> {
        ensure!(
            head.matches_symbol(Self::head()),
            ApplicationError::HeadMismatch
        );
        Ok(())
    }
}
