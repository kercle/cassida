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
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
        annotation: Annotation,
    },
    AddSeq {
        nodes: Vec<AstNode>,
        annotation: Annotation,
    },
    Negation {
        arg: Box<AstNode>,
        annotation: Annotation,
    },
    Sub {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
        annotation: Annotation,
    },
    Mul {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
        annotation: Annotation,
    },
    MulSeq {
        nodes: Vec<AstNode>,
        annotation: Annotation,
    },
    Reciprocal {
        arg: Box<AstNode>,
        annotation: Annotation,
    },
    Div {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
        annotation: Annotation,
    },
    Pow {
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
        annotation: Annotation,
    },
    Sin {
        arg: Box<AstNode>,
        annotation: Annotation,
    },
    Cos {
        arg: Box<AstNode>,
        annotation: Annotation,
    },
    Tan {
        arg: Box<AstNode>,
        annotation: Annotation,
    },
    Sqrt {
        arg: Box<AstNode>,
        annotation: Annotation,
    },
    FunctionCall {
        name: String,
        args: Vec<AstNode>,
        annotation: Annotation,
    },
    Block(Vec<AstNode>),
}

impl<Annotation> AstNode<Annotation>
where
    Annotation: Default + Clone,
{
    pub fn constant(value: RealScalar) -> Self {
        AstNode::Constant {
            annotation: Annotation::default(),
            value,
        }
    }

    pub fn named_value(name: String) -> Self {
        AstNode::NamedValue {
            annotation: Annotation::default(),
            name,
        }
    }

    pub fn add(lhs: AstNode, rhs: AstNode) -> Self {
        AstNode::Add {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            annotation: Annotation::default(),
        }
    }

    pub fn add_seq(nodes: Vec<AstNode>) -> Self {
        AstNode::AddSeq {
            nodes,
            annotation: Annotation::default(),
        }
    }

    pub fn negation(arg: AstNode) -> Self {
        AstNode::Negation {
            arg: Box::new(arg),
            annotation: Annotation::default(),
        }
    }

    pub fn sub(lhs: AstNode, rhs: AstNode) -> Self {
        AstNode::Sub {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            annotation: Annotation::default(),
        }
    }

    pub fn mul(lhs: AstNode, rhs: AstNode) -> Self {
        AstNode::Mul {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            annotation: Annotation::default(),
        }
    }

    pub fn mul_seq(nodes: Vec<AstNode>) -> Self {
        AstNode::MulSeq {
            nodes,
            annotation: Annotation::default(),
        }
    }

    pub fn reciprocal(arg: AstNode) -> Self {
        AstNode::Reciprocal {
            arg: Box::new(arg),
            annotation: Annotation::default(),
        }
    }

    pub fn div(lhs: AstNode, rhs: AstNode) -> Self {
        AstNode::Div {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            annotation: Annotation::default(),
        }
    }

    pub fn pow(lhs: AstNode, rhs: AstNode) -> Self {
        AstNode::Pow {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            annotation: Annotation::default(),
        }
    }

    pub fn sin(arg: AstNode) -> Self {
        AstNode::Sin {
            arg: Box::new(arg),
            annotation: Annotation::default(),
        }
    }

    pub fn cos(arg: AstNode) -> Self {
        AstNode::Cos {
            arg: Box::new(arg),
            annotation: Annotation::default(),
        }
    }

    pub fn tan(arg: AstNode) -> Self {
        AstNode::Tan {
            arg: Box::new(arg),
            annotation: Annotation::default(),
        }
    }

    pub fn sqrt(arg: AstNode) -> Self {
        AstNode::Sqrt {
            arg: Box::new(arg),
            annotation: Annotation::default(),
        }
    }

    pub fn function_call(name: String, args: Vec<AstNode>) -> Self {
        AstNode::FunctionCall {
            name,
            args,
            annotation: Annotation::default(),
        }
    }

    pub fn block(nodes: Vec<AstNode>) -> Self {
        AstNode::Block(nodes)
    }
}

impl AstNode {
    pub fn from_function_call(name: String, mut args: Vec<AstNode>) -> Result<Self, String> {
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

    pub fn map<F>(self, mut f: F) -> AstNode
    where
        F: FnMut(AstNode) -> AstNode,
    {
        self.map_inner(&mut f)
    }

    fn map_inner<F>(self, f: &mut F) -> AstNode
    where
        F: FnMut(AstNode) -> AstNode,
    {
        use AstNode::*;
        let mapped = match self {
            Add { lhs, rhs, .. } => AstNode::add(lhs.map_inner(f), rhs.map_inner(f)),
            AddSeq { nodes, .. } => {
                AstNode::add_seq(nodes.into_iter().map(|n| n.map_inner(f)).collect())
            }
            Negation { arg, .. } => AstNode::negation(arg.map_inner(f)),
            Sub { lhs, rhs, .. } => AstNode::sub(lhs.map_inner(f), rhs.map_inner(f)),
            Mul { lhs, rhs, .. } => AstNode::mul(lhs.map_inner(f), rhs.map_inner(f)),
            MulSeq { nodes, .. } => {
                AstNode::mul_seq(nodes.into_iter().map(|n| n.map_inner(f)).collect())
            }
            Reciprocal { arg, .. } => AstNode::reciprocal(arg.map_inner(f)),
            Div { lhs, rhs, .. } => AstNode::div(lhs.map_inner(f), rhs.map_inner(f)),
            Pow { lhs, rhs, .. } => AstNode::pow(lhs.map_inner(f), rhs.map_inner(f)),
            Sin { arg, .. } => AstNode::sin(arg.map_inner(f)),
            Cos { arg, .. } => AstNode::cos(arg.map_inner(f)),
            Tan { arg, .. } => AstNode::tan(arg.map_inner(f)),
            Sqrt { arg, .. } => AstNode::sqrt(arg.map_inner(f)),
            FunctionCall { name, args, .. } => {
                AstNode::function_call(name, args.into_iter().map(|a| a.map_inner(f)).collect())
            }
            Block(nodes) => Block(nodes.into_iter().map(|n| n.map_inner(f)).collect()),
            Constant { .. } | NamedValue { .. } => self,
        };
        f(mapped)
    }

    pub fn iter(&self) -> AstNodeIter {
        AstNodeIter::new(self)
    }

    pub fn is_constant(&self) -> bool {
        matches!(self, AstNode::Constant { .. })
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

pub struct AstNodeIter<'a> {
    stack: Vec<&'a AstNode>,
}

impl<'a> AstNodeIter<'a> {
    pub fn new(root: &'a AstNode) -> Self {
        AstNodeIter { stack: vec![root] }
    }
}

impl<'a> Iterator for AstNodeIter<'a> {
    type Item = &'a AstNode;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;

        use AstNode::*;
        match node {
            Negation { arg, .. }
            | Reciprocal { arg, .. }
            | Sin { arg, .. }
            | Cos { arg, .. }
            | Tan { arg, .. }
            | Sqrt { arg, .. } => {
                self.stack.push(arg);
            }
            Add { lhs, rhs, .. }
            | Sub { lhs, rhs, .. }
            | Mul { lhs, rhs, .. }
            | Div { lhs, rhs, .. }
            | Pow { lhs, rhs, .. } => {
                self.stack.push(rhs);
                self.stack.push(lhs);
            }
            AddSeq { nodes, .. } | MulSeq { nodes, .. } => {
                for arg in nodes.iter().rev() {
                    self.stack.push(arg);
                }
            }
            FunctionCall { args, .. } | Block(args) => {
                for arg in args.iter().rev() {
                    self.stack.push(arg);
                }
            }
            Constant { .. } | NamedValue { .. } => {}
        }
        Some(node)
    }
}
