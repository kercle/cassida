pub mod parser;

pub trait LatexDisplay {
    fn to_latex(&self) -> String;
}
