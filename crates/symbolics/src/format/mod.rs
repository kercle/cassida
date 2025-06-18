use crate::parser::ast::AstNode;

mod key;
mod latex;
mod yasc;

pub trait MathDisplay {
    fn to_latex(&self) -> String;
    fn to_yasc(&self) -> String;
    fn to_key_string(&self) -> String;
}

impl<A: Clone + PartialEq> MathDisplay for AstNode<A> {
    fn to_latex(&self) -> String {
        latex::ast_to_latex(self, None)
    }

    fn to_yasc(&self) -> String {
        yasc::ast_to_yasc(self, None)
    }

    fn to_key_string(&self) -> String {
        key::ast_to_key_string(self)
    }
}
