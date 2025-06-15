use std::cmp::{self, Ordering};

use crate::format::MathDisplay;
use numbers::RealScalar;

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode<Annotation = ()>
where
    Annotation: Clone,
{
    Constant {
        value: RealScalar,
        annotation: Annotation,
    },
    NamedValue {
        name: String,
        annotation: Annotation,
    },
    Add {
        lhs: Box<AstNode<Annotation>>,
        rhs: Box<AstNode<Annotation>>,
        annotation: Annotation,
    },
    AddSeq {
        nodes: Vec<AstNode<Annotation>>,
        annotation: Annotation,
    },
    Negation {
        arg: Box<AstNode<Annotation>>,
        annotation: Annotation,
    },
    Sub {
        lhs: Box<AstNode<Annotation>>,
        rhs: Box<AstNode<Annotation>>,
        annotation: Annotation,
    },
    Mul {
        lhs: Box<AstNode<Annotation>>,
        rhs: Box<AstNode<Annotation>>,
        annotation: Annotation,
    },
    MulSeq {
        nodes: Vec<AstNode<Annotation>>,
        annotation: Annotation,
    },
    Reciprocal {
        arg: Box<AstNode<Annotation>>,
        annotation: Annotation,
    },
    Div {
        lhs: Box<AstNode<Annotation>>,
        rhs: Box<AstNode<Annotation>>,
        annotation: Annotation,
    },
    Pow {
        lhs: Box<AstNode<Annotation>>,
        rhs: Box<AstNode<Annotation>>,
        annotation: Annotation,
    },
    Sin {
        arg: Box<AstNode<Annotation>>,
        annotation: Annotation,
    },
    Cos {
        arg: Box<AstNode<Annotation>>,
        annotation: Annotation,
    },
    Tan {
        arg: Box<AstNode<Annotation>>,
        annotation: Annotation,
    },
    Sqrt {
        arg: Box<AstNode<Annotation>>,
        annotation: Annotation,
    },
    FunctionCall {
        name: String,
        args: Vec<AstNode<Annotation>>,
        annotation: Annotation,
    },
    Block(Vec<AstNode<Annotation>>),
}

impl<A> AstNode<A>
where
    A: Default + Clone,
{
    pub fn constant(value: RealScalar) -> Self {
        AstNode::Constant {
            annotation: A::default(),
            value,
        }
    }

    pub fn named_value(name: String) -> Self {
        AstNode::NamedValue {
            annotation: A::default(),
            name,
        }
    }

    pub fn add(lhs: AstNode<A>, rhs: AstNode<A>) -> Self {
        AstNode::Add {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            annotation: A::default(),
        }
    }

    pub fn add_seq(nodes: Vec<AstNode<A>>) -> Self {
        AstNode::AddSeq {
            nodes,
            annotation: A::default(),
        }
    }

    pub fn negation(arg: AstNode<A>) -> Self {
        AstNode::Negation {
            arg: Box::new(arg),
            annotation: A::default(),
        }
    }

    pub fn sub(lhs: AstNode<A>, rhs: AstNode<A>) -> Self {
        AstNode::Sub {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            annotation: A::default(),
        }
    }

    pub fn mul(lhs: AstNode<A>, rhs: AstNode<A>) -> Self {
        AstNode::Mul {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            annotation: A::default(),
        }
    }

    pub fn mul_seq(nodes: Vec<AstNode<A>>) -> Self {
        AstNode::MulSeq {
            nodes,
            annotation: A::default(),
        }
    }

    pub fn reciprocal(arg: AstNode<A>) -> Self {
        AstNode::Reciprocal {
            arg: Box::new(arg),
            annotation: A::default(),
        }
    }

    pub fn div(lhs: AstNode<A>, rhs: AstNode<A>) -> Self {
        AstNode::Div {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            annotation: A::default(),
        }
    }

    pub fn pow(lhs: AstNode<A>, rhs: AstNode<A>) -> Self {
        AstNode::Pow {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            annotation: A::default(),
        }
    }

    pub fn sin(arg: AstNode<A>) -> Self {
        AstNode::Sin {
            arg: Box::new(arg),
            annotation: A::default(),
        }
    }

    pub fn cos(arg: AstNode<A>) -> Self {
        AstNode::Cos {
            arg: Box::new(arg),
            annotation: A::default(),
        }
    }

    pub fn tan(arg: AstNode<A>) -> Self {
        AstNode::Tan {
            arg: Box::new(arg),
            annotation: A::default(),
        }
    }

    pub fn sqrt(arg: AstNode<A>) -> Self {
        AstNode::Sqrt {
            arg: Box::new(arg),
            annotation: A::default(),
        }
    }

    pub fn function_call(name: String, args: Vec<AstNode<A>>) -> Self {
        AstNode::FunctionCall {
            name,
            args,
            annotation: A::default(),
        }
    }

    pub fn block(nodes: Vec<AstNode<A>>) -> Self {
        AstNode::Block(nodes)
    }

    pub fn from_function_call(name: String, mut args: Vec<AstNode<A>>) -> Result<Self, String> {
        let initial_args_len = args.len();

        let result = match name.as_str() {
            "sin" => Ok(AstNode::sin(args.pop().ok_or("sin requires one argument")?)),
            "cos" => Ok(AstNode::cos(args.pop().ok_or("cos requires one argument")?)),
            "tan" => Ok(AstNode::tan(args.pop().ok_or("tan requires one argument")?)),
            "sqrt" => Ok(AstNode::sqrt(
                args.pop().ok_or("sqrt requires one argument")?,
            )),
            _ => {
                return Ok(AstNode::function_call(name.clone(), args));
            }
        };

        if !args.is_empty() {
            let expected_arg_count = initial_args_len - args.len();

            let arguments = if expected_arg_count == 1 {
                "argument"
            } else {
                "arguments"
            };

            return Err(format!(
                "Function {} takes {} {arguments} but {} given.",
                name,
                initial_args_len - args.len(),
                initial_args_len
            ));
        }

        result
    }

    pub fn map<F>(self, mut f: F) -> Self
    where
        F: FnMut(Self) -> Self,
    {
        self.map_inner(&mut f)
    }

    fn map_inner<F>(self, f: &mut F) -> AstNode<A>
    where
        F: FnMut(AstNode<A>) -> AstNode<A>,
    {
        use AstNode::*;
        let mapped = match self {
            Add {
                lhs,
                rhs,
                annotation,
            } => Add {
                lhs: Box::new(lhs.map_inner(f)),
                rhs: Box::new(rhs.map_inner(f)),
                annotation,
            },
            AddSeq { nodes, annotation } => AstNode::AddSeq {
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
            Mul {
                lhs,
                rhs,
                annotation,
            } => Mul {
                lhs: Box::new(lhs.map_inner(f)),
                rhs: Box::new(rhs.map_inner(f)),
                annotation,
            },
            MulSeq { nodes, annotation } => MulSeq {
                nodes: nodes.into_iter().map(|n| n.map_inner(f)).collect(),
                annotation,
            },
            Reciprocal { arg, annotation } => Reciprocal {
                arg: Box::new(arg.map_inner(f)),
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
            Sin { arg, annotation } => Sin {
                arg: Box::new(arg.map_inner(f)),
                annotation,
            },
            Cos { arg, annotation } => Cos {
                arg: Box::new(arg.map_inner(f)),
                annotation,
            },
            Tan { arg, annotation } => Tan {
                arg: Box::new(arg.map_inner(f)),
                annotation,
            },
            Sqrt { arg, annotation } => Sqrt {
                arg: Box::new(arg.map_inner(f)),
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
            Block(nodes) => Block(nodes.into_iter().map(|n| n.map_inner(f)).collect()),
            node @ Constant { .. } | node @ NamedValue { .. } => node.clone(),
        };

        f(mapped)
    }
}

impl<A> AstNode<A>
where
    A: Clone,
{
    pub fn is_constant(&self) -> bool {
        matches!(self, AstNode::Constant { .. })
    }

    pub fn map_annotation<B, F>(self, f: &mut F) -> AstNode<B>
    where
        B: Clone,
        F: FnMut(A) -> B,
    {
        match self {
            AstNode::Constant { value, annotation } => AstNode::Constant {
                value,
                annotation: f(annotation),
            },
            AstNode::NamedValue { name, annotation } => AstNode::NamedValue {
                name,
                annotation: f(annotation),
            },
            AstNode::Add {
                lhs,
                rhs,
                annotation,
            } => AstNode::Add {
                lhs: Box::new(lhs.map_annotation(f)),
                rhs: Box::new(rhs.map_annotation(f)),
                annotation: f(annotation),
            },
            AstNode::AddSeq { nodes, annotation } => AstNode::AddSeq {
                nodes: nodes.into_iter().map(|n| n.map_annotation(f)).collect(),
                annotation: f(annotation),
            },
            AstNode::Negation { arg, annotation } => AstNode::Negation {
                arg: Box::new(arg.map_annotation(f)),
                annotation: f(annotation),
            },
            AstNode::Sub {
                lhs,
                rhs,
                annotation,
            } => AstNode::Sub {
                lhs: Box::new(lhs.map_annotation(f)),
                rhs: Box::new(rhs.map_annotation(f)),
                annotation: f(annotation),
            },
            AstNode::Mul {
                lhs,
                rhs,
                annotation,
            } => AstNode::Mul {
                lhs: Box::new(lhs.map_annotation(f)),
                rhs: Box::new(rhs.map_annotation(f)),
                annotation: f(annotation),
            },
            AstNode::MulSeq { nodes, annotation } => AstNode::MulSeq {
                nodes: nodes.into_iter().map(|n| n.map_annotation(f)).collect(),
                annotation: f(annotation),
            },
            AstNode::Reciprocal { arg, annotation } => AstNode::Reciprocal {
                arg: Box::new(arg.map_annotation(f)),
                annotation: f(annotation),
            },
            AstNode::Div {
                lhs,
                rhs,
                annotation,
            } => AstNode::Div {
                lhs: Box::new(lhs.map_annotation(f)),
                rhs: Box::new(rhs.map_annotation(f)),
                annotation: f(annotation),
            },
            AstNode::Pow {
                lhs,
                rhs,
                annotation,
            } => AstNode::Pow {
                lhs: Box::new(lhs.map_annotation(f)),
                rhs: Box::new(rhs.map_annotation(f)),
                annotation: f(annotation),
            },
            AstNode::Sin { arg, annotation } => AstNode::Sin {
                arg: Box::new(arg.map_annotation(f)),
                annotation: f(annotation),
            },
            AstNode::Cos { arg, annotation } => AstNode::Cos {
                arg: Box::new(arg.map_annotation(f)),
                annotation: f(annotation),
            },
            AstNode::Tan { arg, annotation } => AstNode::Tan {
                arg: Box::new(arg.map_annotation(f)),
                annotation: f(annotation),
            },
            AstNode::Sqrt { arg, annotation } => AstNode::Sqrt {
                arg: Box::new(arg.map_annotation(f)),
                annotation: f(annotation),
            },
            AstNode::FunctionCall {
                name,
                args,
                annotation,
            } => AstNode::FunctionCall {
                name,
                args: args.into_iter().map(|a| a.map_annotation(f)).collect(),
                annotation: f(annotation),
            },
            AstNode::Block(nodes) => {
                AstNode::Block(nodes.into_iter().map(|n| n.map_annotation(f)).collect())
            }
        }
    }
}

impl cmp::PartialOrd for AstNode {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        use AstNode::*;
        match (self, other) {
            (Constant { value: a, .. }, Constant { value: b, .. }) => a.partial_cmp(b),
            (Constant { .. }, _) => Some(Ordering::Less),
            (_, Constant { .. }) => Some(Ordering::Greater),
            (a, b) => a.to_yasc().partial_cmp(&b.to_yasc()),
        }
    }
}
