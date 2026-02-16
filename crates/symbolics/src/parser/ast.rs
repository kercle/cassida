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
    FunctionCall = 8,
    Block = 9,
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

    pub fn constant_from_i64(value: i64) -> Self {
        AstNode::new_constant(RealScalar::from_i64(value))
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

    pub fn normalize(self) -> Self {
        use AstNode::*;

        match self {
            Constant { value, .. } => AstNode::new_constant(value),
            NamedValue { name, .. } => AstNode::new_named_value(name),
            Block { nodes, .. } => {
                let normalized_nodes = nodes.iter().map(|a| a.to_owned().normalize()).collect();
                AstNode::new_block(normalized_nodes)
            }
            FunctionCall { name, args, .. } => {
                let normalized_args = args.iter().map(|a| a.to_owned().normalize()).collect();
                AstNode::new_function_call(name, normalized_args)
            }
            Add { nodes, .. } => {
                let mut flattened_nodes = Vec::new();
                for n in nodes.into_iter().map(|n| n.normalize()) {
                    match n {
                        Add { nodes, .. } => flattened_nodes.extend(nodes),
                        other => flattened_nodes.push(other),
                    }
                }

                if flattened_nodes.is_empty() {
                    AstNode::new_constant(RealScalar::zero())
                } else if flattened_nodes.len() == 1 {
                    flattened_nodes.pop().unwrap()
                } else {
                    flattened_nodes.sort_by(|a, b| a.cmp_struct(b));
                    AstNode::new_add(flattened_nodes)
                }
            }
            Negation { arg, .. } => {
                AstNode::new_mul_pair(AstNode::new_constant(RealScalar::minus_one()), *arg)
                    .normalize()
            }
            Mul { nodes, .. } => {
                let mut flattened_nodes = Vec::new();
                for n in nodes.into_iter().map(|n| n.normalize()) {
                    match n {
                        Mul { nodes, .. } => flattened_nodes.extend(nodes),
                        other => flattened_nodes.push(other),
                    }
                }

                if flattened_nodes.is_empty() {
                    AstNode::new_constant(RealScalar::one())
                } else if flattened_nodes.len() == 1 {
                    flattened_nodes.pop().unwrap()
                } else {
                    flattened_nodes.sort_by(|a, b| a.cmp_struct(b));
                    AstNode::new_mul(flattened_nodes)
                }
            }
            Sub { lhs, rhs, .. } => AstNode::new_add_pair(
                *lhs,
                AstNode::new_mul_pair(AstNode::new_constant(RealScalar::minus_one()), *rhs),
            )
            .normalize(),
            Div { lhs, rhs, .. } => AstNode::new_mul_pair(
                *lhs,
                AstNode::new_pow(*rhs, AstNode::new_constant(RealScalar::minus_one())),
            )
            .normalize(),
            Pow { lhs, rhs, .. } => AstNode::new_pow(lhs.normalize(), rhs.normalize()),
        }
    }

    pub fn fold_constants(self) -> Self {
        use AstNode::*;

        match self {
            Constant { .. } => self,
            NamedValue { .. } => self,
            Negation { arg, .. } => {
                if let Constant { value, .. } = *arg {
                    AstNode::new_constant(-value)
                } else {
                    AstNode::new_negation(*arg)
                }
            }
            Sub { lhs, rhs, .. } => {
                if let (Constant { value: a, .. }, Constant { value: b, .. }) = (&*lhs, &*rhs) {
                    AstNode::new_constant(a - b)
                } else {
                    AstNode::new_sub(*lhs, *rhs)
                }
            }
            Div { lhs, rhs, .. } => {
                if let (Constant { .. }, Constant { .. }) = (&*lhs, &*rhs) {
                    todo!("Handle division of constants");
                } else {
                    AstNode::new_div(*lhs, *rhs)
                }
            }
            Pow { lhs, rhs, .. } => {
                if let (Constant { .. }, Constant { .. }) = (&*lhs, &*rhs) {
                    todo!("Handle division of constants");
                } else {
                    AstNode::new_pow(*lhs, *rhs)
                }
            }
            Block { nodes, .. } => {
                let new_nodes = nodes
                    .iter()
                    .map(|a| a.to_owned().fold_constants())
                    .collect();
                AstNode::new_block(new_nodes)
            }
            FunctionCall { name, args, .. } => {
                let new_args = args.iter().map(|a| a.to_owned().fold_constants()).collect();
                AstNode::new_function_call(name, new_args)
            }
            Add { nodes, .. } => {
                let mut new_nodes = vec![];
                let mut running_constant = RealScalar::zero();

                for n in nodes {
                    let n = n.fold_constants();
                    match n {
                        Constant { value, .. } => running_constant += value,
                        _ => new_nodes.push(n),
                    }
                }

                if !running_constant.is_zero() {
                    new_nodes.push(AstNode::new_constant(running_constant));
                }

                if new_nodes.is_empty() {
                    AstNode::constant_from_i64(0)
                } else if new_nodes.len() == 1 {
                    new_nodes.pop().unwrap()
                } else {
                    AstNode::new_add(new_nodes)
                }
            }
            Mul { nodes, .. } => {
                let mut new_nodes = vec![];
                let mut running_constant = RealScalar::one();

                for n in nodes {
                    let n = n.fold_constants();
                    match n {
                        Constant { value, .. } => running_constant *= value,
                        _ => new_nodes.push(n),
                    }
                }

                if !running_constant.is_one() {
                    new_nodes.push(AstNode::new_constant(running_constant));
                }

                if new_nodes.is_empty() {
                    AstNode::constant_from_i64(1)
                } else if new_nodes.len() == 1 {
                    new_nodes.pop().unwrap()
                } else {
                    AstNode::new_mul(new_nodes)
                }
            }
        }
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
            FunctionCall { .. } => Kind::FunctionCall,
            Block { .. } => Kind::Block,
        }
    }

    pub fn cmp_struct(&self, other: &Self) -> Ordering {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn c(n: i64) -> AstNode<()> {
        AstNode::constant_from_i64(n)
    }

    fn x() -> AstNode<()> {
        AstNode::new_named_value("x")
    }

    fn y() -> AstNode<()> {
        AstNode::new_named_value("y")
    }

    fn sin(a: AstNode<()>) -> AstNode<()> {
        AstNode::new_function_call("sin", vec![a])
    }

    fn assert_struct_eq(a: &AstNode<()>, b: &AstNode<()>) {
        assert!(
            a.cmp_struct(b).is_eq(),
            "ASTs not structurally equal.\nleft:  {:#?}\nright: {:#?}",
            a,
            b
        );
    }

    #[test]
    fn test_normalize_flattens_add_and_sorts() {
        let ast = parse("(x + 2) + (1 + 2)").unwrap();
        let expected = AstNode::new_add(vec![c(1), c(2), c(2), x()]);
        assert_struct_eq(&ast.normalize(), &expected);
    }

    #[test]
    fn test_normalize_flattens_mul_and_sorts() {
        let ast = parse("(x * 2) * (y * 3)").unwrap();
        let expected = AstNode::new_mul(vec![c(2), c(3), x(), y()]);
        assert_struct_eq(&ast.normalize(), &expected);
    }

    #[test]
    fn test_normalize_collapses_singleton_add_and_mul() {
        let add1 = AstNode::new_add(vec![x()]);
        let mul1 = AstNode::new_mul(vec![x()]);

        assert_struct_eq(&add1.normalize(), &x());
        assert_struct_eq(&mul1.normalize(), &x());
    }

    #[test]
    fn test_normalize_empty_add_is_zero_empty_mul_is_one() {
        let add0 = AstNode::new_add(vec![]);
        let mul0 = AstNode::new_mul(vec![]);

        assert_struct_eq(&add0.normalize(), &c(0));
        assert_struct_eq(&mul0.normalize(), &c(1));
    }

    #[test]
    fn test_normalize_rewrites_negation_to_mul_minus_one() {
        let ast = parse("-(x + 1)").unwrap();
        let expected = c(-1) * (c(1) + x());

        assert_struct_eq(&ast.normalize(), &expected);
    }

    #[test]
    fn test_normalize_rewrites_sub_to_add_with_negated_rhs() {
        let ast = parse("x - (y + 2)").unwrap();
        let expected = x() + c(-1) * (c(2) + y());

        assert_struct_eq(&ast.normalize(), &expected);
    }

    #[test]
    fn test_normalize_pow_normalizes_children() {
        let ast = parse("(x + 2) ^ (1 + 3)").unwrap();
        let expected = AstNode::new_pow(c(2) + x(), c(1) + c(3));

        assert_struct_eq(&ast.normalize(), &expected);
    }

    #[test]
    fn test_normalize_function_call_normalizes_args() {
        let ast = parse("sin[(x + 2) + (1 + 2)]").unwrap();
        let expected = sin(AstNode::new_add(vec![c(1), c(2), c(2), x()]));

        assert_struct_eq(&ast.normalize(), &expected);
    }

    #[test]
    fn test_normalize_block_normalizes_each_statement() {
        let ast = AstNode::new_block(vec![
            AstNode::new_add(vec![x(), c(2), c(1)]),
            AstNode::new_mul(vec![y(), c(3), c(2)]),
        ]);

        let expected = AstNode::new_block(vec![
            AstNode::new_add(vec![c(1), c(2), x()]),
            AstNode::new_mul(vec![c(2), c(3), y()]),
        ]);

        assert_struct_eq(&ast.normalize(), &expected);
    }

    #[test]
    fn test_normalize_is_idempotent() {
        let ast = parse("((x + 2) + (1 + 2)) + (y + 0)").unwrap();
        let n1 = ast.normalize();
        let n2 = n1.clone().normalize();
        assert_struct_eq(&n1, &n2);
    }

    #[test]
    fn test_normalize_nested_negations_flatten_into_mul_chain() {
        let ast = parse("-(-x)").unwrap();
        let expected = AstNode::new_mul(vec![c(-1), c(-1), x()]);
        assert_struct_eq(&ast.normalize(), &expected);
    }

    #[test]
    fn test_normalize_div_rewrites_to_mul_with_pow_minus_one() {
        let ast = parse("x / (y * 2)").unwrap();
        let expected = AstNode::new_mul(vec![x(), AstNode::new_pow(c(2) * y(), c(-1))]);

        assert_struct_eq(&ast.normalize(), &expected);
    }

    #[test]
    fn test_normalize_add_and_mul_sorting_is_deterministic() {
        let a1 = parse("y + x + 2 + 1").unwrap().normalize();
        let a2 = parse("(2 + y) + (x + 1)").unwrap().normalize();

        assert_struct_eq(&a1, &a2);

        let m1 = parse("y * x * 2 * 1").unwrap().normalize();
        let m2 = parse("(2 * y) * (x * 1)").unwrap().normalize();

        assert_struct_eq(&m1, &m2);
    }
}
