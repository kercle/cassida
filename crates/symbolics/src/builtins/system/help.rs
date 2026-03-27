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
pub struct Help;

impl BuiltIn for Help {
    #[inline(always)]
    fn head() -> &'static str {
        "Help"
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::System,
            title: Self::head(),
            summary: "Documentation for builtin functionality.",
            pattern_doc: vec![
                PatternDoc::new(
                    raw_expr!( Help[] ),
                    "Print table of contents with all built-in symbols.",
                ),
                PatternDoc::new(
                    raw_expr!( Help[s_?IsSymbol] ),
                    "Specific documentation of the given symbol.",
                ),
            ],
            examples: vec![],
            related: vec![],
        }
    }

    fn validate_application_of<S>(
        head: &Expr<S>,
        children: &[Expr<S>],
    ) -> Result<(), ApplicationError> {
        ensure!(children.len() <= 1, ApplicationError::ArityMismatch);
        ensure!(
            head.matches_symbol(Self::head()),
            ApplicationError::HeadMismatch
        );
        Ok(())
    }
}
