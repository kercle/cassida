use numbers::{RealScalar, rational::Rational};

pub trait Operator {
    fn precedence(&self) -> u8;
    fn is_left_associative(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
}

impl Operator for UnaryOp {
    fn precedence(&self) -> u8 {
        3
    }

    fn is_left_associative(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl Operator for BinaryOp {
    fn precedence(&self) -> u8 {
        match self {
            BinaryOp::Add | BinaryOp::Sub => 1,
            BinaryOp::Mul | BinaryOp::Div => 2,
            BinaryOp::Pow => 4,
        }
    }

    fn is_left_associative(&self) -> bool {
        match self {
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => true,
            BinaryOp::Pow => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Constant(RealScalar),
    NamedValue(String),
    Negate(Box<AstNode>),
    Add(Box<AstNode>, Box<AstNode>),
    AddSeq(Vec<AstNode>),
    Sub(Box<AstNode>, Box<AstNode>),
    Mul(Box<AstNode>, Box<AstNode>),
    MulSeq(Vec<AstNode>),
    Div(Box<AstNode>, Box<AstNode>),
    Pow(Box<AstNode>, Box<AstNode>),
    Sin(Box<AstNode>),
    Cos(Box<AstNode>),
    Tan(Box<AstNode>),
    Sqrt(Box<AstNode>),
    FunctionCall { name: String, args: Vec<AstNode> },
    Block(Vec<AstNode>),
}

impl AstNode {
    pub fn from_named_value(name: String) -> Self {
        match name.as_str() {
            "pi" | "π" => AstNode::Constant(RealScalar::PiMultiple(Rational::one())),
            _ => AstNode::NamedValue(name),
        }
    }

    pub fn from_function_call(name: String, mut args: Vec<AstNode>) -> Result<Self, String> {
        let initial_args_len = args.len();

        let result = match name.as_str() {
            "sin" => Ok(AstNode::Sin(Box::new(
                args.pop().ok_or("sin requires one argument")?,
            ))),
            "cos" => Ok(AstNode::Cos(Box::new(
                args.pop().ok_or("cos requires one argument")?,
            ))),
            "tan" => Ok(AstNode::Tan(Box::new(
                args.pop().ok_or("tan requires one argument")?,
            ))),
            "sqrt" => Ok(AstNode::Sqrt(Box::new(
                args.pop().ok_or("sqrt requires one argument")?,
            ))),
            _ => {
                return Ok(AstNode::FunctionCall {
                    name: name.clone(),
                    args: args,
                });
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
            Negate(x) => Negate(Box::new(x.map_inner(f))),
            Add(l, r) => Add(Box::new(l.map_inner(f)), Box::new(r.map_inner(f))),
            AddSeq(nodes) => AddSeq(nodes.into_iter().map(|n| n.map_inner(f)).collect()),
            Sub(l, r) => Sub(Box::new(l.map_inner(f)), Box::new(r.map_inner(f))),
            Mul(l, r) => Mul(Box::new(l.map_inner(f)), Box::new(r.map_inner(f))),
            MulSeq(nodes) => MulSeq(nodes.into_iter().map(|n| n.map_inner(f)).collect()),
            Div(l, r) => Div(Box::new(l.map_inner(f)), Box::new(r.map_inner(f))),
            Pow(l, r) => Pow(Box::new(l.map_inner(f)), Box::new(r.map_inner(f))),
            Sin(x) => Sin(Box::new(x.map_inner(f))),
            Cos(x) => Cos(Box::new(x.map_inner(f))),
            Tan(x) => Tan(Box::new(x.map_inner(f))),
            Sqrt(x) => Sqrt(Box::new(x.map_inner(f))),
            FunctionCall { name, args } => FunctionCall {
                name,
                args: args.into_iter().map(|a| a.map_inner(f)).collect(),
            },
            Block(nodes) => Block(nodes.into_iter().map(|n| n.map_inner(f)).collect()),
            Constant(_) | NamedValue(_) => self,
        };
        f(mapped)
    }

    pub fn iter(&self) -> AstNodeIter {
        AstNodeIter::new(self)
    }

    pub fn is_constant(&self) -> bool {
        matches!(self, AstNode::Constant(_))
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
            Negate(x) | Sin(x) | Cos(x) | Tan(x) | Sqrt(x) => {
                self.stack.push(x);
            }
            Add(lhs, rhs) | Sub(lhs, rhs) | Mul(lhs, rhs) | Div(lhs, rhs) | Pow(lhs, rhs) => {
                self.stack.push(rhs);
                self.stack.push(lhs);
            }
            AddSeq(args) | MulSeq(args) => {
                for arg in args.iter().rev() {
                    self.stack.push(arg);
                }
            }
            FunctionCall { args, .. } | Block(args) => {
                for arg in args.iter().rev() {
                    self.stack.push(arg);
                }
            }
            Constant(_) | NamedValue(_) => {}
        }
        Some(node)
    }
}
