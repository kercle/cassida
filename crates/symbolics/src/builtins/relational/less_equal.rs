use crate::builtins::{
    BuiltInCategory,
    traits::{BuiltIn, BuiltInDoc},
};

#[derive(Default)]
pub struct LessEqual;

impl BuiltIn for LessEqual {
    #[inline(always)]
    fn head() -> &'static str {
        "LessEqual"
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::ElementaryArithmetic,
            title: Self::head(),
            summary: "Internal representation of the $\\leq$ relation.",
            pattern_doc: vec![],
            examples: vec![],
            related: vec![],
        }
    }
}
