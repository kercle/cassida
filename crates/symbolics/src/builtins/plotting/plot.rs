use crate::{
    builtins::{
        self, BuiltInCategory,
        traits::{ApplicationError, BuiltIn, BuiltInDoc, PatternDoc},
    },
    ensure,
    expr::{Expr, },
    raw_expr,
};

#[derive(Default)]
pub struct Plot;

impl Plot {
    pub const HEAD: &'static str = "Plot";
}

impl BuiltIn for Plot {
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
            summary: "Plots a function in one variable.",
            pattern_doc: vec![PatternDoc::new(
                raw_expr!(Plot[f_, (x_?IsSymbol, x0_?IsNumber, x1_?IsNumber)]),
                "Plots the function $f(x)$ on the interval $[x_0,x_1]$.",
            )],
            examples: vec![],
            related: vec![],
        }
    }

    fn validate_application<S>(expr: &Expr<S>) -> Result<(), ApplicationError> {
        ensure!(expr.args_len() == 2, ApplicationError::ArityMismatch);
        ensure!(expr.is_head(Self::head()), ApplicationError::HeadMismatch);
        ensure!(
            expr.get_arg(1)
                .is_some_and(|e| e.is_head(builtins::Tuple::head())),
            ApplicationError::ExpectedTupleAt(1)
        );
        Ok(())
    }
}
