use std::fmt::{Debug, Display, Formatter};

pub mod calculus;
pub mod elementary;
pub mod patterns;
pub mod scoping;
pub mod simplification;
pub mod system;
pub mod traits;

#[derive(Clone, Copy)]
pub enum BuiltInCategory {
    Calculus,
    ElementaryArithmetic,
    ElementaryFunctions,
    Patterns,
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
            Patterns => write!(f, "Patterns"),
            Scoping => write!(f, "Scoping"),
            Simplification => write!(f, "Simplification"),
            System => write!(f, "System"),
        }
    }
}

impl Debug for BuiltInCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.to_string())
    }
}
