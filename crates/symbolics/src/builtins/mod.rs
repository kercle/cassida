use std::fmt::{Debug, Display, Formatter};

pub mod calculus;
pub mod elementary;
pub mod evaluation_control;
pub mod logic;
pub mod patterns;
pub mod relational;
pub mod scoping;
pub mod simplification;
pub mod system;
pub mod traits;

pub mod symbols;

mod prelude;
pub use prelude::*;

#[derive(Clone, Copy)]
pub enum BuiltInCategory {
    Calculus,
    ElementaryArithmetic,
    ElementaryFunctions,
    EvaluationControl,
    Patterns,
    Relational,
    Scoping,
    Simplification,
    System,
}

impl Display for BuiltInCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        use BuiltInCategory::*;

        match self {
            Calculus => write!(f, "Calculus"),
            ElementaryArithmetic => write!(f, "Elementary arithmetic"),
            ElementaryFunctions => write!(f, "Elementary functions"),
            EvaluationControl => write!(f, "Evaluation control"),
            Patterns => write!(f, "Patterns"),
            Relational => write!(f, "Relational"),
            Scoping => write!(f, "Scoping"),
            Simplification => write!(f, "Simplification"),
            System => write!(f, "System"),
        }
    }
}

impl Debug for BuiltInCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{self}")
    }
}
