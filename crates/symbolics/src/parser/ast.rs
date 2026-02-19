use std::fmt::Debug;

use numbers::Number;

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode<Annotation = ()>
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

impl<A> AstNode<A>
where
    A: Default + Clone + PartialEq,
{
    pub fn new_constant(value: Number) -> Self {
        AstNode::Constant {
            annotation: A::default(),
            value,
        }
    }

    pub fn new_constant_from_i64(value: i64) -> Self {
        AstNode::new_constant(Number::from_i64(value))
    }

    pub fn new_constant_one() -> Self {
        Self::new_constant_from_i64(1)
    }

    pub fn new_constant_zero() -> Self {
        Self::new_constant_from_i64(0)
    }

    pub fn new_named_value<T: ToString>(name: T) -> Self {
        AstNode::NamedValue {
            name: name.to_string(),
            annotation: A::default(),
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

    pub fn new_cos(arg: AstNode<A>) -> Self {
        Self::new_function_call("cos".to_string(), vec![arg])
    }

    pub fn new_sin(arg: AstNode<A>) -> Self {
        Self::new_function_call("sin".to_string(), vec![arg])
    }

    pub fn new_tan(arg: AstNode<A>) -> Self {
        Self::new_function_call("tan".to_string(), vec![arg])
    }

    pub fn new_sqrt(arg: AstNode<A>) -> Self {
        Self::new_function_call("sqrt".to_string(), vec![arg])
    }

    pub fn new_function_call<T: ToString>(name: T, args: Vec<AstNode<A>>) -> Self {
        AstNode::FunctionCall {
            name: name.to_string(),
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

    pub fn is_one(&self) -> bool {
        if let AstNode::Constant { value, .. } = self {
            value.is_one()
        } else {
            false
        }
    }

    pub fn is_zero(&self) -> bool {
        if let AstNode::Constant { value, .. } = self {
            value.is_zero()
        } else {
            false
        }
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
