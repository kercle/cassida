use crate::{
    builtins::{
        BuiltInCategory,
        traits::{BuiltIn, BuiltInDoc, PatternDoc},
    },
    raw_expr,
};

#[derive(Default)]
pub struct Cos;

impl Cos {
    pub const HEAD: &'static str = "Cos";
}

impl BuiltIn for Cos {
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
            summary: "Cosine function $\\cos(x)$.",
            pattern_doc: vec![PatternDoc::new(raw_expr!(Cos[x_]), "Cosine of $x$")],
            examples: vec![],
            related: vec![],
        }
    }
}
