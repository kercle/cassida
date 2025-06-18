use numbers::{RealScalar, integer::BigInteger, rational::Rational};

use crate::parser::ast::AstNode;

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
