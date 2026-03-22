use crate::{
    builtins::{
        BuiltInCategory,
        traits::{BuiltIn, BuiltInDoc, PatternDoc},
    },
    raw_expr,
};

#[derive(Default)]
pub struct Sqrt;

impl Sqrt {
    pub const HEAD: &'static str = "Sqrt";
}

impl BuiltIn for Sqrt {
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
            summary: "Square root $\\sqrt(x)$.",
            pattern_doc: vec![PatternDoc::new(raw_expr!(Sqrt[x_]), "Square root of $x$")],
            examples: vec![],
            related: vec![],
        }
    }
}
