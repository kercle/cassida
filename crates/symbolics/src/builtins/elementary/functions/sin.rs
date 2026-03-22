use crate::{
    builtins::{
        BuiltInCategory,
        traits::{BuiltIn, BuiltInDoc, PatternDoc},
    },
    raw_expr,
};

#[derive(Default)]
pub struct Sin;

impl Sin {
    pub const HEAD: &'static str = "Sin";
}

impl BuiltIn for Sin {
    #[inline(always)]
    fn head() -> &'static str {
        Self::HEAD
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::ElementaryFunctions,
            title: Self::head(),
            summary: "Sine function $\\sin(x)$.",
            pattern_doc: vec![PatternDoc::new(raw_expr!(Sin[x_]), "Sine of $x$")],
            examples: vec![],
            related: vec![],
        }
    }
}
