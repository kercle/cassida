use crate::{
    builtins::{
        BuiltInCategory,
        traits::{BuiltIn, BuiltInDoc, PatternDoc},
    },
    raw_expr,
};

#[derive(Default)]
pub struct Exp;

impl Exp {
    pub const HEAD: &'static str = "Exp";
}

impl BuiltIn for Exp {
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
            summary: "Exponential function $\\exp(x)$.",
            pattern_doc: vec![PatternDoc::new(raw_expr!(Exp[x_]), "Exponential of $x$")],
            examples: vec![],
            related: vec![],
        }
    }
}
