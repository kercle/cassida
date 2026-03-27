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
pub struct Add;

impl Add {
    pub const HEAD: &'static str = "Add";
}

impl BuiltIn for Add {
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
            summary: "Internal representation of the sum of terms.",
            pattern_doc: vec![
                PatternDoc::new(raw_expr!(Add[]), "Reduces to zero."),
                PatternDoc::new(
                    raw_expr!(Add[t__]),
                    "If $t=[t_1,\\dots,t_n]$, this expression represents $t_1+\\cdots+t_n$.",
                ),
            ],
            examples: vec![],
            related: vec!["Sub", "Mul", "Div"],
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
