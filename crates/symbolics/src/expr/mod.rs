pub mod atom;
pub mod fmt;
pub mod generator;
pub mod hash;
pub mod macros;
pub mod matcher;
pub mod norm;
pub mod ops;
pub mod pattern;

use atom::Atom;
use numbers::Number;

#[derive(Clone, PartialEq)]
pub enum Expr<A = ()> {
    Atom {
        entry: Atom,
        ann: A,
    },
    Compound {
        head: Box<Expr<A>>,
        args: Vec<Expr<A>>,
        ann: A,
    },
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq)]
pub struct NormalizedExpr<A = ()>(Expr<A>)
where
    A: Clone + PartialEq;

impl<A, T: Into<Atom>> From<T> for Expr<A>
where
    A: Default,
{
    fn from(x: T) -> Self {
        Expr::Atom {
            entry: x.into(),
            ann: A::default(),
        }
    }
}

impl<A: Clone + PartialEq + Default> NormalizedExpr<A> {
    pub fn new(expr: Expr<A>) -> Self {
        NormalizedExpr(expr.normalize())
    }
}

impl<A> Expr<A> {
    pub fn new_compound_with_annotation(head: Expr<A>, args: Vec<Expr<A>>, ann: A) -> Self {
        Expr::Compound {
            head: Box::new(head),
            args,
            ann,
        }
    }

    pub fn as_atom(&self) -> Option<&Atom> {
        match self {
            Expr::Atom { entry, .. } => Some(entry),
            Expr::Compound { .. } => None,
        }
    }

    pub fn head(&self) -> Option<&Expr<A>> {
        match self {
            Expr::Atom { .. } => None,
            Expr::Compound { head, .. } => Some(head),
        }
    }

    pub fn matches_symbol<T: AsRef<str>>(&self, s: T) -> bool {
        matches!(self, Expr::Atom { entry: Atom::Symbol(t), .. } if t == s.as_ref())
    }

    pub fn is_symbol(&self) -> bool {
        matches!(
            self,
            Expr::Atom {
                entry: Atom::Symbol(_),
                ..
            }
        )
    }

    pub fn get_symbol(&self) -> Option<&str> {
        match self {
            Expr::Atom {
                entry: Atom::Symbol(s),
                ..
            } => Some(s),
            _ => None,
        }
    }

    pub fn is_number(&self) -> bool {
        matches!(
            self,
            Expr::Atom {
                entry: Atom::Number(_),
                ..
            }
        )
    }

    pub fn get_number(&self) -> Option<&Number> {
        match self {
            Expr::Atom {
                entry: Atom::Number(n),
                ..
            } => Some(n),
            _ => None,
        }
    }
}

impl<A> Expr<A>
where
    A: Default + Clone + PartialEq,
{
    pub fn new_compound(head: Expr<A>, args: Vec<Expr<A>>) -> Self {
        Expr::Compound {
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
            Expr::Compound { head, args, .. } => Expr::Compound {
                head,
                args,
                ann: A::default(),
            },
        }
    }
}

pub struct ExprWalker<'a, A> {
    stack: Vec<&'a Expr<A>>,
}

impl<'a, A> ExprWalker<'a, A>
where
    A: PartialEq,
{
    pub fn new(root: &'a Expr<A>) -> Self {
        Self { stack: vec![root] }
    }
}

impl<'a, A> Iterator for ExprWalker<'a, A> {
    type Item = &'a Expr<A>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        if let Expr::Compound { head, args, .. } = node {
            for a in args.iter().rev() {
                self.stack.push(a);
            }
            self.stack.push(head);
        }

        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expr_ordering() {
        let x: Expr<()> = Expr::new_symbol("x");

        let expr1: Expr<()> = 2 + x + 3 * (Expr::from_i64(5) + 2);
        let expr2 = expr1.clone();

        assert_eq!(expr1, expr2);

        let x: Expr<()> = Expr::new_symbol("x");
        assert!(x > Expr::from_i64(2));
    }
}
