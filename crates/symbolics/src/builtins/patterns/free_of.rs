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
pub struct FreeOf;

impl FreeOf {
    pub const HEAD: &'static str = "FreeOf";
}

impl BuiltIn for FreeOf {
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
            summary: "Tests if no sub-expressions of a given subject matches the provided pattern. This is Cassidas analogue to Mathematicas `FreeQ`.",
            pattern_doc: vec![PatternDoc::new(
                raw_expr!(FreeOf[t_, p_]),
                "Walks over all subexpressions of $t$ and returns `False` if any match if found. Otherwise it returns `True`.",
            )],
            examples: vec![
                ("FreeOf[u^2 (1 - 1 / Exp[v]), x]", "True"),
                ("FreeOf[x^2+1,x]", "False"),
            ],
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
