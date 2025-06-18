use numbers::{RealScalar, integer::BigInteger, rational::Rational};

use crate::parser::ast::AstNode;

impl<A> AstNode<A>
where
    A: Default + Clone + PartialEq + PartialOrd,
{
    pub fn flatten_commutative(self) -> Self {
        flatten_commutative(self)
    }

    pub fn unflatten_commutative(self) -> Self {
        unflatten_commutative(self)
    }

    pub fn transform_inverses(self) -> Self {
        transform_inverses(self)
    }

    pub fn cannonical_order(self) -> Self {
        cannonical_order(self)
    }

    pub fn fold_constants(self) -> Self {
        fold_constants(self)
    }
}

fn flatten_commutative<A>(node: AstNode<A>) -> AstNode<A>
where
    A: Default + Clone + PartialEq,
{
    use AstNode::*;

    fn flatten_commutative_inner<A, F>(nodes: &[AstNode<A>], extract_func: F) -> Vec<AstNode<A>>
    where
        A: Default + Clone + PartialEq,
        F: Fn(&AstNode<A>) -> Option<Vec<AstNode<A>>>,
    {
        let mut flattened_nodes = vec![];
        for node in nodes.iter() {
            let node = flatten_commutative(node.clone());

            if let Some(mut inner_nodes) = extract_func(&node) {
                flattened_nodes.append(&mut inner_nodes);
            } else {
                flattened_nodes.push(node);
            }
        }

        flattened_nodes
    }

    match &node {
        Add { lhs, rhs, .. } => {
            return flatten_commutative(AstNode::new_add_seq(vec![
                *lhs.to_owned(),
                *rhs.to_owned(),
            ]));
        }
        AddSeq { nodes, .. } => {
            let mut flattened_nodes = flatten_commutative_inner(nodes, |node| {
                if let AddSeq {
                    nodes: inner_nodes, ..
                } = node
                {
                    Some(inner_nodes.clone())
                } else {
                    None
                }
            });

            if flattened_nodes.is_empty() {
                return AstNode::new_constant(RealScalar::zero());
            } else if flattened_nodes.len() == 1 {
                return flattened_nodes.pop().unwrap();
            } else {
                return AstNode::new_add_seq(flattened_nodes);
            }
        }
        Negation { arg, .. } => {
            return flatten_commutative(AstNode::new_mul_seq(vec![
                AstNode::new_constant(RealScalar::minus_one()),
                *arg.to_owned(),
            ]));
        }
        Mul { lhs, rhs, .. } => {
            return flatten_commutative(AstNode::new_mul_seq(vec![
                *lhs.to_owned(),
                *rhs.to_owned(),
            ]));
        }
        MulSeq { nodes, .. } => {
            let mut flattened_nodes = flatten_commutative_inner(nodes, |node| {
                if let MulSeq {
                    nodes: inner_nodes, ..
                } = node
                {
                    Some(inner_nodes.clone())
                } else {
                    None
                }
            });

            if flattened_nodes.is_empty() {
                return AstNode::new_constant(RealScalar::one());
            } else if flattened_nodes.len() == 1 {
                return flattened_nodes.pop().unwrap();
            } else {
                return AstNode::new_mul_seq(flattened_nodes);
            }
        }
        _ => {}
    }

    node
}

fn unflatten_commutative<A>(node: AstNode<A>) -> AstNode<A>
where
    A: Default + Clone + PartialEq,
{
    use AstNode::*;

    let altered_node = match &node {
        AddSeq { nodes, .. } => Some(if nodes.len() == 1 {
            nodes[0].clone()
        } else if nodes.len() == 2 {
            AstNode::new_add(nodes[0].clone(), nodes[1].clone())
        } else {
            AstNode::new_add_seq(vec![
                AstNode::new_add(nodes[0].clone(), nodes[1].clone()),
                AstNode::new_add_seq(nodes[2..].to_vec()),
            ])
        }),
        MulSeq { nodes, .. } => Some(if nodes.len() == 1 {
            nodes[0].clone()
        } else if nodes.len() == 2 {
            AstNode::new_mul(nodes[0].clone(), nodes[1].clone())
        } else {
            AstNode::new_mul_seq(vec![
                AstNode::new_mul(nodes[0].clone(), nodes[1].clone()),
                AstNode::new_mul_seq(nodes[2..].to_vec()),
            ])
        }),
        _ => None,
    };

    altered_node.map_or_else(|| node, |n| n.map(unflatten_commutative))
}

fn transform_inverses<A>(node: AstNode<A>) -> AstNode<A>
where
    A: Default + Clone + PartialEq,
{
    use AstNode::*;
    match &node {
        Sub { lhs, rhs, .. } => {
            let lhs = transform_inverses(*lhs.to_owned());
            let rhs = transform_inverses(*rhs.to_owned());

            return AstNode::new_add(lhs, AstNode::new_negation(rhs));
        }
        Div { lhs, rhs, .. } => {
            let lhs = transform_inverses(*lhs.to_owned());
            let rhs = transform_inverses(*rhs.to_owned());

            return AstNode::new_mul(lhs, AstNode::new_reciprocal(rhs));
        }
        _ => {}
    }

    node
}

fn cannonical_order<A>(node: AstNode<A>) -> AstNode<A>
where
    A: Default + Clone + PartialOrd,
{
    use AstNode::*;

    match &node {
        AddSeq { nodes, .. } => {
            let mut sorted_nodes = nodes.clone();
            sorted_nodes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            return AstNode::new_add_seq(sorted_nodes);
        }
        MulSeq { nodes, .. } => {
            let mut sorted_nodes = nodes.clone();
            sorted_nodes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            return AstNode::new_mul_seq(sorted_nodes);
        }
        _ => {}
    }

    node
}

fn fold_constants<A>(node: AstNode<A>) -> AstNode<A>
where
    A: Default + Clone + PartialEq,
{
    use AstNode::*;
    match &node {
        Add { lhs, rhs, .. } => {
            return fold_constants(AstNode::new_add_seq(vec![*lhs.to_owned(), *rhs.to_owned()]));
        }
        Sub { lhs, rhs, .. } => {
            if let (Constant { value: l, .. }, Constant { value: r, .. }) =
                (lhs.as_ref(), rhs.as_ref())
            {
                return (l - r).map_or_else(|| node, |value| AstNode::new_constant(value));
            }
        }
        Mul { lhs, rhs, .. } => {
            return fold_constants(AstNode::new_mul_seq(vec![*lhs.to_owned(), *rhs.to_owned()]));
        }
        Div { lhs, rhs, .. } => {
            if let (
                Constant {
                    value: RealScalar::Integer(l),
                    ..
                },
                Constant {
                    value: RealScalar::Integer(r),
                    ..
                },
            ) = (lhs.as_ref(), rhs.as_ref())
            {
                if r.is_zero() {
                    todo!("Handle division by zero");
                }
                let rational = Rational::new(l.clone().into(), r.clone().into())
                    .expect("todo: handle invalid rational");

                return AstNode::new_constant(RealScalar::Rational(rational));
            }
        }
        AddSeq { nodes, .. } => {
            let mut sum = RealScalar::zero();
            let mut new_nodes = vec![];

            for node in nodes.iter() {
                if let Constant { value, .. } = node {
                    sum = value + &sum;
                } else {
                    new_nodes.push(node.clone());
                }
            }

            if !sum.is_zero() || new_nodes.is_empty() {
                new_nodes.insert(0, AstNode::new_constant(sum.clone()));
            }

            if new_nodes.len() == 1 {
                return new_nodes.pop().unwrap();
            } else {
                return AstNode::new_add_seq(new_nodes);
            }
        }
        MulSeq { nodes, .. } => {
            let mut product = RealScalar::one();
            let mut new_nodes = vec![];

            for node in nodes.iter() {
                if let Constant { value, .. } = node {
                    if let Some(value) = value * &product {
                        product = value;
                    } else {
                        new_nodes.push(node.clone());
                    }
                } else {
                    new_nodes.push(node.clone());
                }

                if product.is_zero() {
                    return AstNode::new_constant(RealScalar::zero());
                }
            }

            if !product.is_one() || new_nodes.is_empty() {
                new_nodes.insert(0, AstNode::new_constant(product.clone()));
            }

            if new_nodes.len() == 1 {
                return new_nodes.pop().unwrap();
            } else {
                return AstNode::new_mul_seq(new_nodes);
            }
        }
        Pow { lhs, rhs, .. } => {
            if let (
                Constant {
                    value: RealScalar::Integer(base),
                    ..
                },
                Constant {
                    value: RealScalar::Integer(exp),
                    ..
                },
            ) = (lhs.as_ref(), rhs.as_ref())
            {
                if exp.is_zero() {
                    return AstNode::new_constant(RealScalar::one());
                } else if exp.is_one() {
                    return AstNode::new_constant(RealScalar::Integer(base.clone()));
                }

                let abs_exp = exp.abs();
                let result = base.pow(abs_exp.abs());

                if let Ok(result) = result {
                    if exp.is_positive() {
                        return AstNode::new_constant(RealScalar::Integer(result));
                    }

                    return AstNode::new_constant(RealScalar::Rational(
                        Rational::new(BigInteger::one(), result)
                            .expect("todo: handle invalid rational"),
                    ));
                }
            }
        }
        Negation { arg, .. } => {
            if let Constant { value, .. } = arg.as_ref() {
                return AstNode::new_constant(-value.clone());
            }
        }
        _ => {}
    };

    node
}

pub fn normalize_tree<A>(tree: AstNode<A>) -> AstNode<A>
where
    A: Default + Clone + PartialOrd,
{
    tree.map(transform_inverses)
        .map(flatten_commutative)
        .map(cannonical_order)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{format::MathDisplay, parser::parse};

    #[test]
    fn test_normalize_tree_simple() {
        let equivalent_trees = vec![
            parse("(a+1)*4+19").map(normalize_tree).unwrap(),
            parse("4*(a+1)+19").map(normalize_tree).unwrap(),
            parse("19+4*(a+1)").map(normalize_tree).unwrap(),
        ];

        for wnd in equivalent_trees.windows(2) {
            let tree_a = &wnd[0];
            let tree_b = &wnd[1];

            assert_eq!(
                tree_a, tree_b,
                "Trees should be equivalent after normalization"
            );
        }
    }

    #[test]
    fn test_normalize_tree_simple_second() {
        let equivalent_trees = vec![
            parse("(y+x+8)*(a+b*(1+x))").map(normalize_tree).unwrap(),
            parse("(a+b*(1+x))*(y+x+8)").map(normalize_tree).unwrap(),
        ];

        for wnd in equivalent_trees.windows(2) {
            let tree_a = &wnd[0];
            let tree_b = &wnd[1];

            assert_eq!(
                tree_a, tree_b,
                "Trees should be equivalent after normalization"
            );
        }
    }

    #[test]
    fn test_normalize_tree_complex() {
        let equivalent_trees = vec![
            parse("sin[2*x+3]+4*(y-5)-cos[z^2+1]+(a+b)*(c-d/2)+7")
                .map(normalize_tree)
                .unwrap(),
            parse("sin[2*x+3]+4*(y-5)-cos[z^2+1]+(a+b)*(c-d/2)+7")
                .map(normalize_tree)
                .unwrap(),
            parse("4*(y-5)+sin[2*x+3]-cos[1+z^2]+(a+b)*(c-d/2)+7")
                .map(normalize_tree)
                .unwrap(),
            parse("7+sin[3+2*x]+4*(y-5)-cos[z^2+1]+(a+b)*(c-d/2)")
                .map(normalize_tree)
                .unwrap(),
        ];

        for wnd in equivalent_trees.windows(2) {
            let tree_a = &wnd[0];
            let tree_b = &wnd[1];

            assert_eq!(
                tree_a, tree_b,
                "Trees should be equivalent after normalization"
            );
        }

        dbg!(equivalent_trees[0].to_yasc());
    }
}
