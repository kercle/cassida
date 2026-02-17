pub mod atom;
pub mod fmt;
pub mod norm;
pub mod ops;

use atom::Atom;
use numbers::Number;

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

impl<A, T: Into<Atom>> From<T> for Expr<A>
where
    A: Default + Clone + PartialEq,
{
    fn from(x: T) -> Self {
        Expr::Atom {
            entry: x.into(),
            ann: A::default(),
        }
    }
}

impl<A: Clone + PartialEq> Expr<A> {
    pub fn new_app_with_annotation(head: Expr<A>, args: Vec<Expr<A>>, ann: A) -> Self {
        Expr::App {
            head: Box::new(head),
            args,
            ann,
        }
    }

    pub fn from_ast(_ast: &AstNode<A>) -> Self {
        todo!()
    }

    pub fn is_symbol<T: AsRef<str>>(&self, s: T) -> bool {
        matches!(self, Expr::Atom { entry: Atom::Symbol(t), .. } if t == s.as_ref())
    }
}

impl<A> Expr<A>
where
    A: Default + Clone + PartialEq,
{
    pub fn new_app(head: Expr<A>, args: Vec<Expr<A>>) -> Self {
        Expr::App {
            head: Box::new(head),
            args,
            ann: A::default(),
        }
    }

    pub fn new_number(value: Number) -> Self {
        Expr::Atom {
            entry: Atom::Number(value),
            ann: A::default(),
        }
    }

    pub fn new_symbol<T: AsRef<str>>(symb: T) -> Self {
        Expr::Atom {
            entry: Atom::Symbol(symb.as_ref().to_string()),
            ann: A::default(),
        }
    }

    pub fn from_i64(value: i64) -> Self {
        Self::new_number(Number::from_i64(value))
    }

    pub fn drop_annotation(self) -> Self {
        match self {
            Expr::Atom { entry, .. } => Expr::Atom {
                entry,
                ann: A::default(),
            },
            Expr::App { head, args, .. } => Expr::App {
                head,
                args,
                ann: A::default(),
            },
        }
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
