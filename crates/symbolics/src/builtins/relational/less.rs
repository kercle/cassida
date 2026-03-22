use crate::builtins::{
    BuiltInCategory,
    traits::{BuiltIn, BuiltInDoc},
};

#[derive(Default)]
pub struct Less;

impl BuiltIn for Less {
    #[inline(always)]
    fn head() -> &'static str {
        "Less"
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::ElementaryArithmetic,
            title: Self::head(),
            summary: "Internal representation of the $<$ relation.",
            pattern_doc: vec![],
            examples: vec![],
            related: vec![],
        }
    }
}
