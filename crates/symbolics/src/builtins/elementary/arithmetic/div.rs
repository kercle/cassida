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
pub struct Div;

impl Div {
    pub const HEAD: &'static str = "Div";
}

impl BuiltIn for Div {
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
            title: "Div",
            summary: "Internal representation of the quotient of two terms.",
            pattern_doc: vec![PatternDoc::new(
                raw_expr!(Div[a_, b_]),
                "Syntactic sugar for Mul[a, Pow[b, -1]].",
            )],
            examples: vec![],
            related: vec!["Div", "Mul", "Div"],
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
