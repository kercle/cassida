use std::fmt::Debug;

use numbers::Number;

use crate::expr::{Expr, NormalizedExpr, atom::Atom};

pub const ADD_HEAD: &str = "Add";
pub const MUL_HEAD: &str = "Mul";
pub const POW_HEAD: &str = "Pow";

#[derive(Debug, Clone, PartialEq)]
pub enum ParserAst<Annotation = ()>
where
    Annotation: Clone + PartialEq,
{
    Constant {
        value: Number,
        annotation: Annotation,
    },
    NamedValue {
        name: String,
        annotation: Annotation,
    },
    Add {
        nodes: Vec<ParserAst<Annotation>>,
        annotation: Annotation,
    },
    Negation {
        arg: Box<ParserAst<Annotation>>,
        annotation: Annotation,
    },
    Sub {
        lhs: Box<ParserAst<Annotation>>,
        rhs: Box<ParserAst<Annotation>>,
        annotation: Annotation,
    },
    Mul {
        nodes: Vec<ParserAst<Annotation>>,
        annotation: Annotation,
    },
    Div {
        lhs: Box<ParserAst<Annotation>>,
        rhs: Box<ParserAst<Annotation>>,
        annotation: Annotation,
    },
    Pow {
        lhs: Box<ParserAst<Annotation>>,
        rhs: Box<ParserAst<Annotation>>,
        annotation: Annotation,
    },
    FunctionCall {
        name: String,
        args: Vec<ParserAst<Annotation>>,
        annotation: Annotation,
    },
    Block {
        nodes: Vec<ParserAst<Annotation>>,
        annotation: Annotation,
    },
}

impl<A> ParserAst<A>
where
    A: Default + Clone + PartialEq,
{
    pub fn new_constant(value: Number) -> Self {
        ParserAst::Constant {
            annotation: A::default(),
            value,
        }
    }

    pub fn new_constant_from_i64(value: i64) -> Self {
        ParserAst::new_constant(Number::from_i64(value))
    }

    pub fn new_constant_one() -> Self {
        Self::new_constant_from_i64(1)
    }

    pub fn new_constant_zero() -> Self {
        Self::new_constant_from_i64(0)
    }

    pub fn new_named_value<T: ToString>(name: T) -> Self {
        ParserAst::NamedValue {
            name: name.to_string(),
            annotation: A::default(),
        }
    }

    pub fn new_add(nodes: Vec<ParserAst<A>>) -> Self {
        ParserAst::Add {
            nodes,
            annotation: A::default(),
        }
    }

    pub fn new_add_pair(lhs: ParserAst<A>, rhs: ParserAst<A>) -> Self {
        Self::new_add(vec![lhs, rhs])
    }

    pub fn new_negation(arg: ParserAst<A>) -> Self {
        ParserAst::Negation {
            arg: Box::new(arg),
            annotation: A::default(),
        }
    }

    pub fn new_sub(lhs: ParserAst<A>, rhs: ParserAst<A>) -> Self {
        ParserAst::Sub {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            annotation: A::default(),
        }
    }

    pub fn new_mul_pair(lhs: ParserAst<A>, rhs: ParserAst<A>) -> Self {
        Self::new_mul(vec![lhs, rhs])
    }

    pub fn new_mul(nodes: Vec<ParserAst<A>>) -> Self {
        ParserAst::Mul {
            nodes,
            annotation: A::default(),
        }
    }

    pub fn new_div(lhs: ParserAst<A>, rhs: ParserAst<A>) -> Self {
        ParserAst::Div {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            annotation: A::default(),
        }
    }

    pub fn new_pow(lhs: ParserAst<A>, rhs: ParserAst<A>) -> Self {
        ParserAst::Pow {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            annotation: A::default(),
        }
    }

    pub fn new_cos(arg: ParserAst<A>) -> Self {
        Self::new_function_call("cos".to_string(), vec![arg])
    }

    pub fn new_sin(arg: ParserAst<A>) -> Self {
        Self::new_function_call("sin".to_string(), vec![arg])
    }

    pub fn new_tan(arg: ParserAst<A>) -> Self {
        Self::new_function_call("tan".to_string(), vec![arg])
    }

    pub fn new_sqrt(arg: ParserAst<A>) -> Self {
        Self::new_function_call("sqrt".to_string(), vec![arg])
    }

    pub fn new_function_call<T: ToString>(name: T, args: Vec<ParserAst<A>>) -> Self {
        ParserAst::FunctionCall {
            name: name.to_string(),
            args,
            annotation: A::default(),
        }
    }

    pub fn new_block(nodes: Vec<ParserAst<A>>) -> Self {
        ParserAst::Block {
            nodes,
            annotation: A::default(),
        }
    }

    pub fn drop_annotation(self) -> ParserAst {
        self.map_annotation(&mut |_| ())
    }

    pub fn with_annotation(self, annotation: A) -> Self {
        use ParserAst::*;
        match self {
            Constant { value, .. } => Constant { value, annotation },
            NamedValue { name, .. } => NamedValue { name, annotation },
            Add { nodes, .. } => Add { nodes, annotation },
            Negation { arg, .. } => Negation { arg, annotation },
            Sub { lhs, rhs, .. } => Sub {
                lhs,
                rhs,
                annotation,
            },
            Mul { nodes, .. } => Mul { nodes, annotation },
            Div { lhs, rhs, .. } => Div {
                lhs,
                rhs,
                annotation,
            },
            Pow { lhs, rhs, .. } => Pow {
                lhs,
                rhs,
                annotation,
            },
            FunctionCall { name, args, .. } => FunctionCall {
                name,
                args,
                annotation,
            },
            Block { nodes, annotation } => Block { nodes, annotation },
        }
    }

    pub fn annotation(&self) -> &A {
        use ParserAst::*;
        match self {
            Constant { annotation, .. }
            | NamedValue { annotation, .. }
            | Add { annotation, .. }
            | Negation { annotation, .. }
            | Sub { annotation, .. }
            | Mul { annotation, .. }
            | Div { annotation, .. }
            | Pow { annotation, .. }
            | FunctionCall { annotation, .. }
            | Block { annotation, .. } => annotation,
        }
    }

    pub fn value_from_constant(&self) -> Option<Number> {
        if let ParserAst::Constant { value, .. } = self {
            Some(value.clone())
        } else {
            None
        }
    }

    pub fn map<F>(self, mut f: F) -> Self
    where
        F: FnMut(Self) -> Self,
    {
        self.map_inner(&mut f)
    }

    fn map_inner<F>(self, f: &mut F) -> Self
    where
        F: FnMut(Self) -> Self,
    {
        use ParserAst::*;
        let mapped = match self {
            Add { nodes, annotation } => ParserAst::Add {
                nodes: nodes.into_iter().map(|n| n.map_inner(f)).collect(),
                annotation,
            },
            Negation { arg, annotation } => Negation {
                arg: Box::new(arg.map_inner(f)),
                annotation,
            },
            Sub {
                lhs,
                rhs,
                annotation,
            } => Sub {
                lhs: Box::new(lhs.map_inner(f)),
                rhs: Box::new(rhs.map_inner(f)),
                annotation,
            },
            Mul { nodes, annotation } => Mul {
                nodes: nodes.into_iter().map(|n| n.map_inner(f)).collect(),
                annotation,
            },
            Div {
                lhs,
                rhs,
                annotation,
            } => Div {
                lhs: Box::new(lhs.map_inner(f)),
                rhs: Box::new(rhs.map_inner(f)),
                annotation,
            },
            Pow {
                lhs,
                rhs,
                annotation,
            } => Pow {
                lhs: Box::new(lhs.map_inner(f)),
                rhs: Box::new(rhs.map_inner(f)),
                annotation,
            },
            FunctionCall {
                name,
                args,
                annotation,
            } => FunctionCall {
                name,
                args: args.into_iter().map(|a| a.map_inner(f)).collect(),
                annotation,
            },
            Block { nodes, annotation } => Block {
                nodes: nodes.into_iter().map(|n| n.map_inner(f)).collect(),
                annotation,
            },
            node @ Constant { .. } | node @ NamedValue { .. } => node,
        };

        f(mapped)
    }

    fn try_from_inner(expr: Expr<A>) -> Result<Self, ExprToParserAstError> {
        match expr {
            Expr::Atom {
                entry: Atom::Number(x),
                ann,
            } => Ok(ParserAst::new_constant(x).with_annotation(ann)),
            Expr::Atom {
                entry: Atom::Symbol(x),
                ann,
            } => Ok(ParserAst::new_named_value(x).with_annotation(ann)),
            Expr::Atom {
                entry: Atom::StringLiteral(_),
                ..
            } => todo!(),
            Expr::Compound { head, args, ann } if head.matches_symbol(ADD_HEAD) => {
                let args = args
                    .into_iter()
                    .map(|e| ParserAst::try_from_inner(e))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(ParserAst::new_add(args).with_annotation(ann))
            }
            Expr::Compound { head, args, ann } if head.matches_symbol(MUL_HEAD) => {
                let args = args
                    .into_iter()
                    .map(|e| ParserAst::try_from_inner(e))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(ParserAst::new_mul(args).with_annotation(ann))
            }
            Expr::Compound {
                head,
                mut args,
                ann,
            } if head.matches_symbol(POW_HEAD) && args.len() == 2 => {
                let rhs = ParserAst::<A>::try_from_inner(args.pop().unwrap())?;
                let lhs = ParserAst::try_from_inner(args.pop().unwrap())?;
                Ok(ParserAst::new_pow(lhs, rhs).with_annotation(ann))
            }
            Expr::Compound { head, args, ann } => {
                let name = head.get_symbol().ok_or(ExprToParserAstError)?;
                let args = args
                    .into_iter()
                    .map(|e| ParserAst::try_from_inner(e))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(ParserAst::new_function_call(name, args).with_annotation(ann))
            }
        }
    }
}

impl<A> ParserAst<A>
where
    A: Clone + PartialEq,
{
    pub fn is_constant(&self) -> bool {
        matches!(self, ParserAst::Constant { .. })
    }

    pub fn is_one(&self) -> bool {
        if let ParserAst::Constant { value, .. } = self {
            value.is_one()
        } else {
            false
        }
    }

    pub fn is_zero(&self) -> bool {
        if let ParserAst::Constant { value, .. } = self {
            value.is_zero()
        } else {
            false
        }
    }

    pub fn map_annotation<B, F>(self, f: &mut F) -> ParserAst<B>
    where
        B: Clone + PartialEq,
        F: FnMut(A) -> B,
    {
        use ParserAst::*;
        match self {
            Constant { value, annotation } => Constant {
                value,
                annotation: f(annotation),
            },
            NamedValue { name, annotation } => NamedValue {
                name,
                annotation: f(annotation),
            },
            Add { nodes, annotation } => Add {
                nodes: nodes.into_iter().map(|n| n.map_annotation(f)).collect(),
                annotation: f(annotation),
            },
            Negation { arg, annotation } => Negation {
                arg: Box::new(arg.map_annotation(f)),
                annotation: f(annotation),
            },
            Sub {
                lhs,
                rhs,
                annotation,
            } => Sub {
                lhs: Box::new(lhs.map_annotation(f)),
                rhs: Box::new(rhs.map_annotation(f)),
                annotation: f(annotation),
            },
            Mul { nodes, annotation } => Mul {
                nodes: nodes.into_iter().map(|n| n.map_annotation(f)).collect(),
                annotation: f(annotation),
            },
            Div {
                lhs,
                rhs,
                annotation,
            } => Div {
                lhs: Box::new(lhs.map_annotation(f)),
                rhs: Box::new(rhs.map_annotation(f)),
                annotation: f(annotation),
            },
            Pow {
                lhs,
                rhs,
                annotation,
            } => Pow {
                lhs: Box::new(lhs.map_annotation(f)),
                rhs: Box::new(rhs.map_annotation(f)),
                annotation: f(annotation),
            },
            FunctionCall {
                name,
                args,
                annotation,
            } => FunctionCall {
                name,
                args: args.into_iter().map(|a| a.map_annotation(f)).collect(),
                annotation: f(annotation),
            },
            Block { nodes, annotation } => Block {
                nodes: nodes.into_iter().map(|n| n.map_annotation(f)).collect(),
                annotation: f(annotation),
            },
        }
    }
}

pub struct ExprToParserAstError;

impl<A: Default + Clone + PartialEq> TryFrom<NormalizedExpr<A>> for ParserAst<A> {
    type Error = ExprToParserAstError;

    fn try_from(expr: NormalizedExpr<A>) -> Result<Self, ExprToParserAstError> {
        Self::try_from_inner(expr.take_expr())
    }
}
