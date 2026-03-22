use crate::{
    builtins::{
        BuiltInCategory,
        traits::{BuiltIn, BuiltInDoc, PatternDoc},
    },
    raw_expr,
};

#[derive(Default)]
pub struct Log;

impl Log {
    pub const HEAD: &'static str = "Log";
}

impl BuiltIn for Log {
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
            summary: "Logarithm function $\\log(x)$.",
            pattern_doc: vec![PatternDoc::new(raw_expr!(Log), "Logarithm of $x$")],
            examples: vec![],
            related: vec![],
        }
    }
}
