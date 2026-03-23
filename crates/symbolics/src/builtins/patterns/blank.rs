use crate::{
    builtins::{
        BuiltInCategory,
        traits::{BuiltIn, BuiltInDoc, PatternDoc},
    },
    raw_expr,
};

#[derive(Default)]
pub struct Blank;

impl Blank {
    pub const HEAD: &'static str = "Blank";
}

impl BuiltIn for Blank {
    #[inline(always)]
    fn head() -> &'static str {
        Self::HEAD
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::Patterns,
            title: Self::head(),
            summary: "Wildcard in pattern matching.",
            pattern_doc: vec![
                PatternDoc::new(raw_expr!(Blank[]), "Matches any given subject."),
                PatternDoc::new(
                    raw_expr!(Blank[h_?IsSymbol]),
                    "Matches any given subject with head $h$.",
                ),
            ],
            examples: vec![],
            related: vec![],
        }
    }
}
