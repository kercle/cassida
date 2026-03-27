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
pub struct Compound;

impl BuiltIn for Compound {
    #[inline(always)]
    fn head() -> &'static str {
        "Compound"
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::Scoping,
            title: Self::head(),
            summary: "Groups multiple expressions and evaluates to the last expression in the compound.",
            pattern_doc: vec![PatternDoc::new(
                raw_expr!(Compound[t___]),
                "If $t=[t_1,\\dots,t_n]$, arguments $t_1,\\dots,t_n$ are evaluated in succession. The resulting expression is given by $t_n$.",
            )],
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
