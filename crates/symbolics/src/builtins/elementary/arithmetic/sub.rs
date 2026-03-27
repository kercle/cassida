use crate::{
    builtins::{
        BuiltInCategory,
        traits::{ApplicationError, BuiltIn, BuiltInDoc, PatternDoc},
    },
    ensure,
    expr::Expr,
    raw_expr,
};

#[derive(Default)]
pub struct Sub;

impl Sub {
    pub const HEAD: &'static str = "Sub";
}

impl BuiltIn for Sub {
    #[inline(always)]
    fn head() -> &'static str {
        Self::HEAD
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::ElementaryArithmetic,
            title: Self::head(),
            summary: "Internal representation of the difference of two terms.",
            pattern_doc: vec![PatternDoc::new(
                raw_expr!(Sub[a_, b_]),
                "Syntactic sugar for Add[a, Mul[-1, b]].",
            )],
            examples: vec![],
            related: vec!["Add", "Mul", "Div"],
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
