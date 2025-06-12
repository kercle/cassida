pub mod ast;
pub mod error;
pub mod lex;
pub mod parser;

pub trait LatexDisplay {
    fn to_latex(&self) -> String;
}
