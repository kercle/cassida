pub mod atom;
pub mod fmt;
pub mod norm;
pub mod ops;

use numbers::Number;
use atom::Atom;

use crate::parser::ast::AstNode;

#[derive(Clone, PartialEq)]
pub enum Expr<A = ()>
where
    A: Clone + PartialEq,
{
    Atom {
        entry: Atom,
        ann: A,
    },
    App {
        head: Box<Expr<A>>,
        args: Vec<Expr<A>>,
        ann: A,
    },
}

impl<A> Expr<A>
where
    A: Default + Clone + PartialEq,
{
    pub fn new_number(value: Number) -> Self {
        Expr::Atom {
            entry: Atom::Number(value),
            ann: A::default(),
        }
    }

    pub fn new_symbol<T: ToString>(symb: T) -> Self {
        Expr::Atom {
            entry: Atom::Symbol(symb.to_string()),
            ann: A::default(),
        }
    }

    pub fn from_i64(value: i64) -> Self {
        Self::new_number(Number::from_i64(value))
    }
}

impl<A: Clone + PartialEq> Expr<A> {
    pub fn from_ast(_ast: &AstNode<A>) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_ordering() {
        let x: Expr<()> = Expr::new_symbol("x");

        let expr1 = 2 + x + 3 * (Expr::from_i64(5) + 2);
        let expr2 = expr1.clone();

        assert_eq!(expr1, expr2);

        let x: Expr<()> = Expr::new_symbol("x");
        assert!(x > Expr::from_i64(2));
    }
}
