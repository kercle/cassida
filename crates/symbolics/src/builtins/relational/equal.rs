use crate::builtins::{
    BuiltInCategory,
    traits::{BuiltIn, BuiltInDoc},
};

#[derive(Default)]
pub struct Equal;

impl BuiltIn for Equal {
    #[inline(always)]
    fn head() -> &'static str {
        "Equal"
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::ElementaryArithmetic,
            title: Self::head(),
            summary: "Internal representation of the $=$ comparison relation.",
            pattern_doc: vec![],
            examples: vec![],
            related: vec![],
        }
    }
}
