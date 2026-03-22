use crate::{
    builtins::{
        BuiltInCategory,
        traits::{BuiltIn, BuiltInDoc, PatternDoc},
    },
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
}
