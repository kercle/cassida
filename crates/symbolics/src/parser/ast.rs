use std::cmp::Ordering;

use numbers::RealScalar;

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode<Annotation = ()>
where
    Annotation: Clone + PartialEq,
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
        nodes: Vec<AstNode<Annotation>>,
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
    Block {
        nodes: Vec<AstNode<Annotation>>,
        annotation: Annotation,
    },
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Kind {
    Constant = 0,
    NamedValue = 1,
    Negation = 2,
    Pow = 3,
    Mul = 4,
    Div = 5,
    Add = 6,
    Sub = 7,
    Sin = 8,
    Cos = 9,
    Tan = 10,
    Sqrt = 11,
    FunctionCall = 12,
    Block = 13,
}

impl<A> AstNode<A>
where
    A: Default + Clone + PartialEq,
{
    pub fn new_constant(value: RealScalar) -> Self {
        AstNode::Constant {
            annotation: A::default(),
            value,
        }
    }

    pub fn new_named_value(name: String) -> Self {
        AstNode::NamedValue {
            annotation: A::default(),
            name,
        }
    }

    pub fn new_add(nodes: Vec<AstNode<A>>) -> Self {
        AstNode::Add {
            nodes,
            annotation: A::default(),
        }
    }

    pub fn new_add_pair(lhs: AstNode<A>, rhs: AstNode<A>) -> Self {
        Self::new_add(vec![lhs, rhs])
    }

    pub fn new_negation(arg: AstNode<A>) -> Self {
        AstNode::Negation {
            arg: Box::new(arg),
            annotation: A::default(),
        }
    }

    pub fn new_sub(lhs: AstNode<A>, rhs: AstNode<A>) -> Self {
        AstNode::Sub {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            annotation: A::default(),
        }
    }

    pub fn new_mul_pair(lhs: AstNode<A>, rhs: AstNode<A>) -> Self {
        Self::new_mul(vec![lhs, rhs])
    }

    pub fn new_mul(nodes: Vec<AstNode<A>>) -> Self {
        AstNode::Mul {
            nodes,
            annotation: A::default(),
        }
    }

    pub fn new_div(lhs: AstNode<A>, rhs: AstNode<A>) -> Self {
        AstNode::Div {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            annotation: A::default(),
        }
    }

    pub fn new_pow(lhs: AstNode<A>, rhs: AstNode<A>) -> Self {
        AstNode::Pow {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
            annotation: A::default(),
        }
    }

    pub fn new_sin(arg: AstNode<A>) -> Self {
        AstNode::Sin {
            arg: Box::new(arg),
            annotation: A::default(),
        }
    }

    pub fn new_cos(arg: AstNode<A>) -> Self {
        AstNode::Cos {
            arg: Box::new(arg),
            annotation: A::default(),
        }
    }

    pub fn new_tan(arg: AstNode<A>) -> Self {
        AstNode::Tan {
            arg: Box::new(arg),
            annotation: A::default(),
        }
    }

    pub fn new_sqrt(arg: AstNode<A>) -> Self {
        AstNode::Sqrt {
            arg: Box::new(arg),
            annotation: A::default(),
        }
    }

    pub fn new_function_call(name: String, args: Vec<AstNode<A>>) -> Self {
        AstNode::FunctionCall {
            name,
            args,
            annotation: A::default(),
        }
    }

    pub fn new_block(nodes: Vec<AstNode<A>>) -> Self {
        AstNode::Block {
            nodes,
            annotation: A::default(),
        }
    }

    pub fn drop_annotation(self) -> AstNode {
        self.map_annotation(&mut |_| ())
    }

    pub fn with_annotation(self, annotation: A) -> Self {
        use AstNode::*;
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
            Sin { arg, .. } => Sin { arg, annotation },
            Cos { arg, .. } => Cos { arg, annotation },
            Tan { arg, .. } => Tan { arg, annotation },
            Sqrt { arg, .. } => Sqrt { arg, annotation },
            FunctionCall { name, args, .. } => FunctionCall {
                name,
                args,
                annotation,
            },
            Block { nodes, annotation } => Block { nodes, annotation },
        }
    }

    pub fn annotation(&self) -> &A {
        use AstNode::*;
        match self {
            Constant { annotation, .. } => annotation,
            NamedValue { annotation, .. } => annotation,
            Add { annotation, .. } => annotation,
            Negation { annotation, .. } => annotation,
            Sub { annotation, .. } => annotation,
            Mul { annotation, .. } => annotation,
            Div { annotation, .. } => annotation,
            Pow { annotation, .. } => annotation,
            Sin { annotation, .. } => annotation,
            Cos { annotation, .. } => annotation,
            Tan { annotation, .. } => annotation,
            Sqrt { annotation, .. } => annotation,
            FunctionCall { annotation, .. } => annotation,
            Block { annotation, .. } => annotation,
        }
    }

    pub fn from_function_call(name: String, mut args: Vec<AstNode<A>>) -> Result<Self, String> {
        let initial_args_len = args.len();

        let result = match name.as_str() {
            "sin" => Ok(AstNode::new_sin(
                args.pop().ok_or("sin requires one argument")?,
            )),
            "cos" => Ok(AstNode::new_cos(
                args.pop().ok_or("cos requires one argument")?,
            )),
            "tan" => Ok(AstNode::new_tan(
                args.pop().ok_or("tan requires one argument")?,
            )),
            "sqrt" => Ok(AstNode::new_sqrt(
                args.pop().ok_or("sqrt requires one argument")?,
            )),
            _ => {
                return Ok(AstNode::new_function_call(name.clone(), args));
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

    pub fn value_from_constant(&self) -> Option<RealScalar> {
        if let AstNode::Constant { value, .. } = self {
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
        use AstNode::*;
        let mapped = match self {
            Add { nodes, annotation } => AstNode::Add {
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
            Block { nodes, annotation } => Block {
                nodes: nodes.into_iter().map(|n| n.map_inner(f)).collect(),
                annotation,
            },
            node @ Constant { .. } | node @ NamedValue { .. } => node,
        };

        f(mapped)
    }
}

impl<A> AstNode<A>
where
    A: Clone + PartialEq,
{
    pub fn is_constant(&self) -> bool {
        matches!(self, AstNode::Constant { .. })
    }

    pub fn map_annotation<B, F>(self, f: &mut F) -> AstNode<B>
    where
        B: Clone + PartialEq,
        F: FnMut(A) -> B,
    {
        use AstNode::*;
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
            Sin { arg, annotation } => Sin {
                arg: Box::new(arg.map_annotation(f)),
                annotation: f(annotation),
            },
            Cos { arg, annotation } => Cos {
                arg: Box::new(arg.map_annotation(f)),
                annotation: f(annotation),
            },
            Tan { arg, annotation } => Tan {
                arg: Box::new(arg.map_annotation(f)),
                annotation: f(annotation),
            },
            Sqrt { arg, annotation } => Sqrt {
                arg: Box::new(arg.map_annotation(f)),
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

    fn kind(&self) -> Kind {
        use AstNode::*;
        match self {
            Constant { .. } => Kind::Constant,
            NamedValue { .. } => Kind::NamedValue,
            Negation { .. } => Kind::Negation,
            Pow { .. } => Kind::Pow,
            Mul { .. } => Kind::Mul,
            Div { .. } => Kind::Div,
            Add { .. } => Kind::Add,
            Sub { .. } => Kind::Sub,
            Sin { .. } => Kind::Sin,
            Cos { .. } => Kind::Cos,
            Tan { .. } => Kind::Tan,
            Sqrt { .. } => Kind::Sqrt,
            FunctionCall { .. } => Kind::FunctionCall,
            Block { .. } => Kind::Block,
        }
    }

    pub fn cmp_struct(&self, other: &Self) -> Ordering {
        let k = self.kind().cmp(&other.kind());
        if k != Ordering::Equal {
            return k;
        }

        use AstNode::*;
        match (self, other) {
            (Constant { value: a, .. }, Constant { value: b, .. }) => a.cmp(b),
            (NamedValue { name: a, .. }, NamedValue { name: b, .. }) => a.cmp(b),
            (Negation { arg: a, .. }, Negation { arg: b, .. }) => a.cmp_struct(b),
            (
                Pow {
                    lhs: a1, rhs: a2, ..
                },
                Pow {
                    lhs: b1, rhs: b2, ..
                },
            ) => a1.cmp_struct(b1).then_with(|| a2.cmp_struct(b2)),
            (
                Div {
                    lhs: a1, rhs: a2, ..
                },
                Div {
                    lhs: b1, rhs: b2, ..
                },
            ) => a1.cmp_struct(b1).then_with(|| a2.cmp_struct(b2)),
            (
                Sub {
                    lhs: a1, rhs: a2, ..
                },
                Sub {
                    lhs: b1, rhs: b2, ..
                },
            ) => a1.cmp_struct(b1).then_with(|| a2.cmp_struct(b2)),
            (Add { nodes: a, .. }, Add { nodes: b, .. })
            | (Mul { nodes: a, .. }, Mul { nodes: b, .. })
            | (Block { nodes: a, .. }, Block { nodes: b, .. }) => cmp_vec(a, b),
            (Sin { arg: a, .. }, Sin { arg: b, .. })
            | (Cos { arg: a, .. }, Cos { arg: b, .. })
            | (Tan { arg: a, .. }, Tan { arg: b, .. })
            | (Sqrt { arg: a, .. }, Sqrt { arg: b, .. }) => a.cmp_struct(b),
            (
                FunctionCall {
                    name: an, args: aa, ..
                },
                FunctionCall {
                    name: bn, args: ba, ..
                },
            ) => an.cmp(bn).then_with(|| cmp_vec(aa, ba)),
            _ => Ordering::Equal,
        }
    }
}

fn cmp_vec<A: Clone + PartialEq>(a: &[AstNode<A>], b: &[AstNode<A>]) -> Ordering {
    let len_cmp = a.len().cmp(&b.len());
    if len_cmp != Ordering::Equal {
        return len_cmp;
    }
    for (x, y) in a.iter().zip(b.iter()) {
        let c = x.cmp_struct(y);
        if c != Ordering::Equal {
            return c;
        }
    }
    Ordering::Equal
}
