use crate::builtins::{
    BuiltInCategory,
    traits::{BuiltIn, BuiltInDoc, PatternDoc},
};

pub const POW_HEAD: &str = "Pow";

#[derive(Default)]
pub struct Pow;

impl BuiltIn for Pow {
    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::ElementaryArithmetic,
            title: POW_HEAD,
            summary: "Internal representation of the power of two terms. Contrary to Mathematica, Pow is only meaningful when having arity two.",
            pattern_doc: vec![
                PatternDoc::new("Pow[x, Absent]", "Reduces to x."),
                PatternDoc::new(
                    "Pow[Absent, x]",
                    "Reduces to Pow[x], which loses its meaning as power.",
                ),
                PatternDoc::new("Power[b_, e_]", "Represents $b^e$."),
            ],
            examples: vec![("Pow[x, Absent]", "x")],
            related: vec!["Add", "Sub", "Mul", "Div"],
        }
    }

    fn head_symbol(&self) -> &'static str {
        POW_HEAD
    }
}
