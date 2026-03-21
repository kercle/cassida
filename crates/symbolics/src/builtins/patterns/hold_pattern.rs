use crate::builtins::{
    BuiltInCategory,
    traits::{BuiltIn, BuiltInDoc, PatternDoc},
};

pub const HOLD_PATTERN_HEAD: &str = "HoldPattern";

#[derive(Default)]
pub struct HoldPattern;

impl BuiltIn for HoldPattern {
    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::Patterns,
            title: "HoldPattern",
            summary: "Holding patterns without normalizing them.",
            pattern_doc: vec![PatternDoc::new(
                "HoldPattern[p_]",
                "The pattern $p$ is not evaluated until the pattern is being used.",
            )],
            examples: vec![],
            related: vec![],
        }
    }

    fn head_symbol(&self) -> &'static str {
        HOLD_PATTERN_HEAD
    }
}
