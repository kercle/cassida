use crate::builtins::traits::{BuiltIn, BuiltInDoc, PatternDoc};

#[derive(Default)]
pub struct Help;

impl BuiltIn for Help {
    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: "System",
            title: "Help",
            summary: "Documentation for builtin functionality.",
            pattern_doc: vec![
                PatternDoc::new(
                    "Help[]",
                    "Print table of contents with all built-in symbols.",
                ),
                PatternDoc::new(
                    "Help[s_?IsSymbol]",
                    "Specific documentation of the given symbol.",
                ),
            ],
            examples: vec![],
            related: vec![],
        }
    }

    fn head_symbol(&self) -> &'static str {
        "Help"
    }
}
