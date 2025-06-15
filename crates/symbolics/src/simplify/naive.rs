use numbers::{RealScalar, integer::BigInteger, rational::Rational};

use crate::parser::ast::AstNode;

fn fold_constants(node: AstNode) -> AstNode {
    use AstNode::*;
    match &node {
        Add { lhs, rhs, .. } => {
            return fold_constants(AstNode::add_seq(vec![*lhs.to_owned(), *rhs.to_owned()]));
        }
        Sub { lhs, rhs, .. } => {
            if let (Constant { value: l, .. }, Constant { value: r, .. }) =
                (lhs.as_ref(), rhs.as_ref())
            {
                return (l - r).map_or_else(|| node, |value| AstNode::constant(value));
            }
        }
        Mul { lhs, rhs, .. } => {
            return fold_constants(AstNode::mul_seq(vec![*lhs.to_owned(), *rhs.to_owned()]));
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

                return AstNode::constant(RealScalar::Rational(rational));
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
                new_nodes.insert(0, AstNode::constant(sum.clone()));
            }

            if new_nodes.len() == 1 {
                return new_nodes.pop().unwrap();
            } else {
                return AstNode::add_seq(new_nodes);
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
                    return AstNode::constant(RealScalar::zero());
                }
            }

            if !product.is_one() || new_nodes.is_empty() {
                new_nodes.insert(0, AstNode::constant(product.clone()));
            }

            if new_nodes.len() == 1 {
                return new_nodes.pop().unwrap();
            } else {
                return AstNode::mul_seq(new_nodes);
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
                    return AstNode::constant(RealScalar::one());
                } else if exp.is_one() {
                    return AstNode::constant(RealScalar::Integer(base.clone()));
                }

                let abs_exp = exp.abs();
                let result = base.pow(abs_exp.abs());

                if let Ok(result) = result {
                    if exp.is_positive() {
                        return AstNode::constant(RealScalar::Integer(result));
                    }

                    return AstNode::constant(RealScalar::Rational(
                        Rational::new(BigInteger::one(), result)
                            .expect("todo: handle invalid rational"),
                    ));
                }
            }
        }
        Negation { arg, .. } => {
            if let Constant { value, .. } = arg.as_ref() {
                return AstNode::constant(-value.clone());
            }
        }
        _ => {}
    };

    node
}

fn gather_common_terms(node: AstNode) -> AstNode {
    use AstNode::*;

    fn split_multiple_of_constant(node: AstNode) -> (RealScalar, AstNode) {
        if let MulSeq { nodes, .. } = &node {
            if let Some((constant, rest)) = nodes.split_first() {
                if let Constant { value, .. } = constant {
                    return (
                        value.clone(),
                        flatten_commutative(AstNode::mul_seq(rest.to_vec())),
                    );
                }
            }
        } else if let Mul { lhs, rhs, .. } = &node {
            return split_multiple_of_constant(AstNode::mul_seq(vec![
                *lhs.to_owned(),
                *rhs.to_owned(),
            ]));
        }
        (RealScalar::one(), node)
    }

    match &node {
        AddSeq { nodes, .. } => {
            let mut terms_with_factors: Vec<(RealScalar, AstNode)> = vec![];

            for node in nodes.iter() {
                let (factor, term) = split_multiple_of_constant(node.clone());

                let mut found = false;
                for (existing_factor, existing_term) in terms_with_factors.iter_mut() {
                    if *existing_term == term {
                        // let existing_factor = existing_factor.clone();
                        *existing_factor = &*existing_factor + &factor;
                        found = true;
                        break;
                    }
                }

                if !found {
                    terms_with_factors.push((factor, term));
                }
            }

            return AstNode::add_seq(
                terms_with_factors
                    .into_iter()
                    .map(|(factor, term)| {
                        if factor.is_one() {
                            term
                        } else {
                            AstNode::mul_seq(vec![AstNode::constant(factor), term])
                        }
                    })
                    .collect(),
            );
        }
        _ => {}
    }

    node
}

fn expand_subtraction(node: AstNode) -> AstNode {
    use AstNode::*;
    match &node {
        Sub { lhs, rhs, .. } => {
            let lhs = expand_subtraction(*lhs.to_owned());
            let rhs = expand_subtraction(*rhs.to_owned());

            return AstNode::add(lhs, AstNode::negation(rhs));
        }
        _ => {}
    }

    node
}

fn flatten_commutative(node: AstNode) -> AstNode {
    use AstNode::*;

    fn flatten_commutative_inner<F>(nodes: &[AstNode], extract_func: F) -> Vec<AstNode>
    where
        F: Fn(&AstNode) -> Option<Vec<AstNode>>,
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
            return flatten_commutative(AstNode::add_seq(vec![*lhs.to_owned(), *rhs.to_owned()]));
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
                return AstNode::constant(RealScalar::zero());
            } else if flattened_nodes.len() == 1 {
                return flattened_nodes.pop().unwrap();
            } else {
                return AstNode::add_seq(flattened_nodes);
            }
        }
        Negation { arg, .. } => {
            return flatten_commutative(AstNode::mul_seq(vec![
                AstNode::constant(RealScalar::minus_one()),
                *arg.to_owned(),
            ]));
        }
        Mul { lhs, rhs, .. } => {
            return flatten_commutative(AstNode::mul_seq(vec![*lhs.to_owned(), *rhs.to_owned()]));
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
                return AstNode::constant(RealScalar::one());
            } else if flattened_nodes.len() == 1 {
                return flattened_nodes.pop().unwrap();
            } else {
                return AstNode::mul_seq(flattened_nodes);
            }
        }
        _ => {}
    }

    node
}

fn cannonical_order(node: AstNode) -> AstNode {
    use AstNode::*;

    match &node {
        AddSeq { nodes, .. } => {
            let mut sorted_nodes = nodes.clone();
            sorted_nodes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            return AstNode::add_seq(sorted_nodes);
        }
        MulSeq { nodes, .. } => {
            let mut sorted_nodes = nodes.clone();
            sorted_nodes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            return AstNode::mul_seq(sorted_nodes);
        }
        _ => {}
    }

    node
}

fn unflatten_commutative(node: AstNode) -> AstNode {
    use AstNode::*;

    let altered_node = match &node {
        AddSeq { nodes, .. } => Some(if nodes.len() == 1 {
            nodes[0].clone()
        } else if nodes.len() == 2 {
            AstNode::add(nodes[0].clone(), nodes[1].clone())
        } else {
            AstNode::add_seq(vec![
                AstNode::add(nodes[0].clone(), nodes[1].clone()),
                AstNode::add_seq(nodes[2..].to_vec()),
            ])
        }),
        MulSeq { nodes, .. } => Some(if nodes.len() == 1 {
            nodes[0].clone()
        } else if nodes.len() == 2 {
            AstNode::mul(nodes[0].clone(), nodes[1].clone())
        } else {
            AstNode::mul_seq(vec![
                AstNode::mul(nodes[0].clone(), nodes[1].clone()),
                AstNode::mul_seq(nodes[2..].to_vec()),
            ])
        }),
        _ => None,
    };

    altered_node.map_or_else(|| node, |n| n.map(unflatten_commutative))
}

fn simplify_add_neg_to_sub(node: AstNode) -> AstNode {
    // Maybe this can be generalized in some factorization step?

    use AstNode::*;
    match &node {
        Add { lhs, rhs, .. } => {
            let lhs = simplify_add_neg_to_sub(*lhs.clone());
            let rhs = simplify_add_neg_to_sub(*rhs.clone());

            if let (Negation { arg: neg_lhs, .. }, Negation { arg: neg_rhs, .. }) = (&lhs, &rhs) {
                return AstNode::negation(AstNode::add(*neg_lhs.to_owned(), *neg_rhs.to_owned()));
            } else if let Negation { arg: neg_rhs, .. } = rhs {
                return AstNode::sub(lhs, *neg_rhs);
            } else if let Negation { arg: neg_lhs, .. } = lhs {
                return AstNode::sub(rhs, *neg_lhs);
            }
        }
        Mul { lhs, rhs, .. } => {
            if let Constant { value, .. } = *lhs.clone() {
                if value < RealScalar::zero() {
                    return AstNode::negation(AstNode::mul(
                        AstNode::constant(-value.clone()),
                        simplify_add_neg_to_sub(*rhs.to_owned()),
                    ));
                }
            }
        }
        _ => {}
    }

    node
}

pub fn simplify_ast(mut tree: AstNode) -> AstNode {
    loop {
        let tree_iteration = tree
            .clone()
            .map(expand_subtraction)
            .map(flatten_commutative)
            .map(fold_constants)
            .map(gather_common_terms)
            .map(cannonical_order)
            .map(unflatten_commutative)
            .map(simplify_add_neg_to_sub);
        if tree_iteration == tree {
            break tree_iteration;
        }
        tree = tree_iteration;
    }
}
