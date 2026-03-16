use crate::builtins::traits::{BuiltIn, BuiltInDoc, PatternDoc};

#[derive(Default)]
pub struct Compound;

pub const COMPOUND_HEAD: &'static str = "Compound";

impl BuiltIn for Compound {
    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: "Scoping",
            title: "Compound",
            summary: "Groups multiple expressions and evaluates to the last expression in the compound.",
            pattern_doc: vec![
                PatternDoc::new(
                    "Compound[t___]",
                    "If $t=[t_1,\\dots,t_n]$, arguments $t_1,\\dots,t_n$ are evaluated in succession. The resulting expression is given by $t_n$.",
                ),
            ],
            examples: vec![],
            related: vec![],
        }
    }

    fn head_symbol(&self) -> &'static str {
        COMPOUND_HEAD
    }
}
