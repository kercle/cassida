pub mod atom;
pub mod derivative;
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

use crate::parser::ast::{ADD_HEAD, MUL_HEAD, POW_HEAD, ParserAst};

#[derive(Clone, PartialEq)]
pub enum Expr<A = ()> {
    Atom {
        entry: Atom,
        annotation: A,
    },
    Compound {
        head: Box<Expr<A>>,
        args: Vec<Expr<A>>,
        annotation: A,
    },
}

#[repr(transparent)]
#[derive(Debug)]
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
            annotation: A::default(),
        }
    }
}

pub struct ParserAstToExprError;

impl<A: Clone + PartialEq> TryFrom<ParserAst<A>> for Expr {
    type Error = ParserAstToExprError;

    fn try_from(_: ParserAst<A>) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl<A: Clone + PartialEq + Default> NormalizedExpr<A> {
    pub fn new(expr: Expr<A>) -> Self {
        NormalizedExpr(expr.normalize())
    }
}

impl<A: Clone + PartialEq> NormalizedExpr<A> {
    pub fn take_expr(self) -> Expr<A> {
        self.0
    }
}

impl<A: Clone + PartialEq> AsRef<Expr<A>> for NormalizedExpr<A> {
    fn as_ref(&self) -> &Expr<A> {
        &self.0
    }
}

impl<A> Expr<A> {
    pub fn new_compound_with_annotation(head: Expr<A>, args: Vec<Expr<A>>, ann: A) -> Self {
        Expr::Compound {
            head: Box::new(head),
            args,
            annotation: ann,
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
    A: Default,
{
    pub fn new_compound<T: Into<Expr<A>>>(head: T, args: Vec<Expr<A>>) -> Self {
        Expr::Compound {
            head: Box::new(head.into()),
            args,
            annotation: A::default(),
        }
    }

    pub fn new_number(value: Number) -> Self {
        Expr::Atom {
            entry: Atom::Number(value),
            annotation: A::default(),
        }
    }

    pub fn new_symbol<T: AsRef<str>>(symb: T) -> Self {
        Expr::Atom {
            entry: Atom::Symbol(symb.as_ref().to_string()),
            annotation: A::default(),
        }
    }
}

impl<A> Expr<A>
where
    A: Default + Clone + PartialEq,
{
    pub fn matches_head<T: Into<Expr<A>>>(&self, test_head: T) -> bool {
        if let Some(head) = self.head() {
            let test_head = test_head.into();
            *head == test_head
        } else {
            false
        }
    }

    pub fn from_parser_ast(parser_ast: &ParserAst<A>) -> Self {
        match parser_ast {
            ParserAst::Constant { value, annotation } => {
                Self::new_number(value.clone()).with_annotation(annotation.clone())
            }
            ParserAst::Symbol { name, annotation } => {
                Self::new_symbol(name.clone()).with_annotation(annotation.clone())
            }
            ParserAst::Add { nodes, annotation } => {
                let head = Self::new_symbol(ADD_HEAD);
                let args = nodes
                    .iter()
                    .map(|node| Self::from_parser_ast(node))
                    .collect();
                Self::new_compound(head, args).with_annotation(annotation.clone())
            }
            ParserAst::Sub {
                lhs,
                rhs,
                annotation,
            } => {
                let head = Self::new_symbol(ADD_HEAD);
                let lhs = Self::from_parser_ast(lhs.as_ref());
                let rhs = Self::from_parser_ast(rhs.as_ref());

                Self::new_compound(
                    head,
                    vec![
                        lhs,
                        Self::new_compound(
                            Self::new_symbol(MUL_HEAD),
                            vec![Self::new_number(Number::from_i64(-1)), rhs],
                        ),
                    ],
                )
                .with_annotation(annotation.clone())
            }
            ParserAst::Negation { arg, annotation } => {
                let arg = Self::from_parser_ast(arg.as_ref());
                Self::new_compound(
                    Self::new_symbol(MUL_HEAD),
                    vec![Self::new_number(Number::from_i64(-1)), arg],
                )
                .with_annotation(annotation.clone())
            }
            ParserAst::Mul { nodes, annotation } => {
                let head = Self::new_symbol(MUL_HEAD);
                let args = nodes
                    .iter()
                    .map(|node| Self::from_parser_ast(node))
                    .collect();
                Self::new_compound(head, args).with_annotation(annotation.clone())
            }
            ParserAst::Div {
                lhs,
                rhs,
                annotation,
            } => {
                let head = Self::new_symbol(MUL_HEAD);
                let lhs = Self::from_parser_ast(lhs.as_ref());
                let rhs = Self::from_parser_ast(rhs.as_ref());

                Self::new_compound(
                    head,
                    vec![
                        lhs,
                        Self::new_compound(
                            Self::new_symbol(POW_HEAD),
                            vec![rhs, Self::new_number(Number::from_i64(-1))],
                        ),
                    ],
                )
                .with_annotation(annotation.clone())
            }
            ParserAst::Pow {
                lhs,
                rhs,
                annotation,
            } => {
                let head = Self::new_symbol(POW_HEAD);
                let lhs = Self::from_parser_ast(lhs.as_ref());
                let rhs = Self::from_parser_ast(rhs.as_ref());

                Self::new_compound(head, vec![lhs, rhs]).with_annotation(annotation.clone())
            }
            ParserAst::FunctionCall {
                name,
                args,
                annotation,
            } => {
                let head = Self::new_symbol(name);
                let args = args
                    .iter()
                    .map(|node| Self::from_parser_ast(node))
                    .collect();

                Self::new_compound(head, args).with_annotation(annotation.clone())
            }
            ParserAst::Block { .. } => todo!(),
        }
    }

    pub fn from_i64(value: i64) -> Self {
        Self::new_number(Number::from_i64(value))
    }

    pub fn drop_annotation(self) -> Self {
        match self {
            Expr::Atom { entry, .. } => Expr::Atom {
                entry,
                annotation: A::default(),
            },
            Expr::Compound { head, args, .. } => Expr::Compound {
                head,
                args,
                annotation: A::default(),
            },
        }
    }

    pub fn with_annotation(self, annotation: A) -> Self {
        use Expr::*;
        match self {
            Atom { entry, .. } => Atom { entry, annotation },
            Compound { head, args, .. } => Compound {
                head,
                args,
                annotation,
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
