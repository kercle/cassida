use crate::builtins::{
    BuiltInCategory,
    traits::{BuiltIn, BuiltInDoc},
};

#[derive(Default)]
pub struct GreaterEqual;

impl BuiltIn for GreaterEqual {
    #[inline(always)]
    fn head() -> &'static str {
        "GreaterEqual"
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::Relational,
            title: Self::head(),
            summary: "Internal representation of the $\\geq$ comparison relation.",
            pattern_doc: vec![],
            examples: vec![],
            related: vec![],
        }
    }
}
