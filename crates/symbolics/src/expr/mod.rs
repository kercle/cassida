pub mod constructors;
pub mod convert;
pub mod fmt;
pub mod generator;
pub mod hash;
pub mod norm;
pub mod ops;
pub mod types;
pub mod walk;

use numbers::Number;

pub use types::*;

use crate::{
    atom::Atom,
    parser::ast::{ADD_HEAD, MUL_HEAD, POW_HEAD, ParserAst},
};

impl<A> Expr<A> {
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

    pub fn args_len(&self) -> usize {
        match self {
            Expr::Atom { .. } => 0,
            Expr::Compound { args, .. } => args.len(),
        }
    }

    pub fn get_arg(&self, index: usize) -> Option<&Self> {
        match self {
            Expr::Atom { .. } => None,
            Expr::Compound { args, .. } => args.get(index),
        }
    }

    pub fn matches_symbol<T: AsRef<str>>(&self, s: T) -> bool {
        matches!(self, Expr::Atom { entry: Atom::Symbol(t), .. } if t == s.as_ref())
    }

    pub fn unpack_binary_node<T: AsRef<str>>(&self, s: T) -> Option<(&Self, &Self)> {
        if self.head().map(|e| e.matches_symbol(s)).unwrap_or(false) && self.args_len() == 2 {
            Some((self.get_arg(0).unwrap(), self.get_arg(1).unwrap()))
        } else {
            None
        }
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

    pub fn is_number_zero(&self) -> bool {
        self.get_number().map(|n| n.is_zero()).unwrap_or(false)
    }

    pub fn is_number_one(&self) -> bool {
        self.get_number().map(|n| n.is_one()).unwrap_or(false)
    }

    pub fn is_number_negative(&self) -> bool {
        self.get_number().map(|n| n.is_negative()).unwrap_or(false)
    }

    pub fn is_number_positive(&self) -> bool {
        self.get_number().map(|n| n.is_positive()).unwrap_or(false)
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

    pub fn is_application_of<T: AsRef<str>>(&self, head_sym: T, arity: usize) -> bool {
        match self {
            Expr::Atom { .. } => false,
            Expr::Compound { head, args, .. } => {
                head.matches_symbol(head_sym) && args.len() == arity
            }
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

    pub fn annotation_to_default(self) -> Self {
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

    pub fn drop_annotation(self) -> Expr {
        self.map_annotations(&|_| ())
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

    pub fn map_annotations<B, F>(self, f: &F) -> Expr<B>
    where
        F: Fn(A) -> B + Copy,
    {
        match self {
            Expr::Atom { entry, annotation } => Expr::Atom {
                entry,
                annotation: f(annotation),
            },
            Expr::Compound {
                head,
                args,
                annotation,
            } => {
                let head = head.map_annotations(f);
                let args = args.into_iter().map(|a| a.map_annotations(f)).collect();
                let annotation = f(annotation);

                Expr::Compound {
                    head: Box::new(head),
                    args,
                    annotation,
                }
            }
        }
    }

    pub fn map_bottom_up<F>(self, f: &F) -> Expr<A>
    where
        F: Fn(Expr<A>) -> Expr<A> + Copy,
    {
        match self {
            Expr::Atom { .. } => f(self),
            Expr::Compound { head, args, .. } => {
                let head = f(head.map_bottom_up(f));
                let args = args.into_iter().map(|a| f(a.map_bottom_up(f))).collect();
                f(Expr::new_compound(head, args))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        expr::{
            generator::{ExprBuilder, SymbolGenerator, cos, exp, pow},
            walk::ExprBottomUpWalker,
        },
        symbol,
    };

    use super::*;

    fn dd(f: Expr, x: SymbolGenerator) -> Expr {
        Expr::new_compound("D", vec![f, x.build()])
    }

    #[test]
    fn test_expr_ordering() {
        let x: Expr<()> = Expr::new_symbol("x");

        let expr1: Expr<()> = 2 + x + 3 * (Expr::from_i64(5) + 2);
        let expr2 = expr1.clone();

        assert_eq!(expr1, expr2);

        let x: Expr<()> = Expr::new_symbol("x");
        assert!(x > Expr::from_i64(2));
    }

    #[test]
    fn test_walker() {
        let (x, y, z) = symbol!("x", "y", "z");

        let expr = 2 + x * cos(x + dd(exp(pow(y, 2) + 7 * z), x));
        dbg!(&expr);
        for e in ExprBottomUpWalker::new(&expr) {
            dbg!(e);
        }
    }
}
