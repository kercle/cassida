use numbers::{RealScalar, rational::Rational};

use crate::parser::ast::AstNode;

fn simplify_constant_expression(node: AstNode) -> AstNode {
    use AstNode::*;
    match &node {
        Add(lhs, rhs) => {
            return simplify_constant_expression(AddSeq(vec![*lhs.to_owned(), *rhs.to_owned()]));
        }
        Sub(lhs, rhs) => {
            if let (Constant(l), Constant(r)) = (lhs.as_ref(), rhs.as_ref()) {
                return (l - r).map_or_else(|| node, |val| Constant(val));
            }
        }
        Mul(lhs, rhs) => {
            return simplify_constant_expression(MulSeq(vec![*lhs.to_owned(), *rhs.to_owned()]));
        }
        Div(lhs, rhs) => {
            if let (Constant(RealScalar::Integer(l)), Constant(RealScalar::Integer(r))) =
                (lhs.as_ref(), rhs.as_ref())
            {
                if r.is_zero() {
                    todo!("Handle division by zero");
                }
                let rational = Rational::new(l.clone().into(), r.clone().into())
                    .expect("todo: handle invalid rational");

                return Constant(RealScalar::Rational(rational));
            }
        }
        AddSeq(nodes) => {
            let mut sum = RealScalar::zero();
            let mut non_constant_nodes = vec![];

            for node in nodes.iter() {
                if let Constant(val) = node {
                    if let Some(value) = val + &sum {
                        sum = value;
                    } else {
                        non_constant_nodes.push(node.clone());
                    }
                } else {
                    non_constant_nodes.push(node.clone());
                }
            }

            if !sum.is_zero() {
                non_constant_nodes.insert(0, Constant(sum.clone()));
            }

            if non_constant_nodes.len() == 1 {
                return non_constant_nodes.pop().unwrap();
            } else if non_constant_nodes.len() == 2 {
                let rhs = non_constant_nodes.pop().unwrap();
                let lhs = non_constant_nodes.pop().unwrap();
                return Add(Box::new(lhs), Box::new(rhs));
            } else {
                return AddSeq(non_constant_nodes);
            }
        }
        MulSeq(nodes) => {
            let mut product = RealScalar::one();
            let mut new_nodes = vec![];

            for node in nodes.iter() {
                if let Constant(val) = node {
                    if let Some(value) = val * &product {
                        product = value;
                    } else {
                        new_nodes.push(node.clone());
                    }
                } else {
                    new_nodes.push(node.clone());
                }

                if product.is_zero() {
                    return Constant(RealScalar::zero());
                }
            }

            if !product.is_one() {
                new_nodes.insert(0, Constant(product.clone()));
            }

            if new_nodes.len() == 1 {
                return new_nodes.pop().unwrap();
            } else if new_nodes.len() == 2 {
                let rhs = new_nodes.pop().unwrap();
                let lhs = new_nodes.pop().unwrap();
                return Mul(Box::new(lhs), Box::new(rhs));
            } else {
                return MulSeq(new_nodes);
            }
        }
        _ => {}
    };

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
        Add(lhs, rhs) => {
            return flatten_commutative(AddSeq(vec![*lhs.to_owned(), *rhs.to_owned()]));
        }
        AddSeq(nodes) => {
            let mut flattened_nodes = flatten_commutative_inner(nodes, |node| {
                if let AddSeq(inner_nodes) = node {
                    Some(inner_nodes.clone())
                } else {
                    None
                }
            });

            if flattened_nodes.is_empty() {
                return Constant(RealScalar::zero());
            } else if flattened_nodes.len() == 1 {
                return flattened_nodes.pop().unwrap();
            } else if flattened_nodes.len() == 2 {
                let rhs = flattened_nodes.pop().unwrap();
                let lhs = flattened_nodes.pop().unwrap();
                return Add(Box::new(lhs), Box::new(rhs));
            } else {
                return AddSeq(flattened_nodes);
            }
        }
        Mul(lhs, rhs) => {
            return flatten_commutative(MulSeq(vec![*lhs.to_owned(), *rhs.to_owned()]));
        }
        MulSeq(nodes) => {
            let mut flattened_nodes = flatten_commutative_inner(nodes, |node| {
                if let MulSeq(inner_nodes) = node {
                    Some(inner_nodes.clone())
                } else {
                    None
                }
            });

            if flattened_nodes.is_empty() {
                return Constant(RealScalar::one());
            } else if flattened_nodes.len() == 1 {
                return flattened_nodes.pop().unwrap();
            } else if flattened_nodes.len() == 2 {
                let rhs = flattened_nodes.pop().unwrap();
                let lhs = flattened_nodes.pop().unwrap();
                return Mul(Box::new(lhs), Box::new(rhs));
            } else {
                return MulSeq(flattened_nodes);
            }
        }
        _ => {}
    }

    node
}

fn coalesce_constants(mut tree: AstNode) -> AstNode {
    loop {
        let new_tree = tree.clone().map(simplify_constant_expression);
        if new_tree == tree {
            break;
        }
        tree = new_tree;
    }

    tree
}

pub fn simplify_ast(tree: AstNode) -> AstNode {
    let tree = flatten_commutative(tree);
    coalesce_constants(tree)
}
