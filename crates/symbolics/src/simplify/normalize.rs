use numbers::RealScalar;

use crate::parser::ast::AstNode;

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

fn transform_inverses(node: AstNode) -> AstNode {
    use AstNode::*;
    match &node {
        Sub { lhs, rhs, .. } => {
            let lhs = transform_inverses(*lhs.to_owned());
            let rhs = transform_inverses(*rhs.to_owned());

            return AstNode::add(lhs, AstNode::negation(rhs));
        }
        Div { lhs, rhs, .. } => {
            let lhs = transform_inverses(*lhs.to_owned());
            let rhs = transform_inverses(*rhs.to_owned());

            return AstNode::mul(lhs, AstNode::reciprocal(rhs));
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

pub fn normalize_tree(tree: AstNode) -> AstNode {
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
