use crate::{
    builtins::{
        BuiltInCategory,
        traits::{BuiltIn, BuiltInDoc, PatternDoc},
    },
    raw_expr,
};

#[derive(Default)]
pub struct Tan;

impl Tan {
    pub const HEAD: &'static str = "Tan";
}

impl BuiltIn for Tan {
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
            summary: "Tangens function $\\tan(x)$.",
            pattern_doc: vec![PatternDoc::new(raw_expr!(Tan[x_]), "Tangens of $x$")],
            examples: vec![],
            related: vec![],
        }
    }
}
