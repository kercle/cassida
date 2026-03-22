use crate::builtins::{
    BuiltInCategory,
    traits::{BuiltIn, BuiltInDoc},
};

#[derive(Default)]
pub struct Greater;

impl BuiltIn for Greater {
    #[inline(always)]
    fn head() -> &'static str {
        "Greater"
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::ElementaryArithmetic,
            title: Self::head(),
            summary: "Internal representation of the $>$ comparison relation.",
            pattern_doc: vec![],
            examples: vec![],
            related: vec![],
        }
    }
}
