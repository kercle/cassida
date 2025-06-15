use std::{collections::HashMap, ops};

use numbers::RealScalar;

use crate::parser::ast::AstNode;

#[derive(Debug, Clone)]
pub enum AstPattern<'a> {
    Any(&'a str),
    Number(&'a str),
    Constant(RealScalar),
    Add(Box<AstPattern<'a>>, Box<AstPattern<'a>>),
}

impl ops::Add for AstPattern<'_> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        AstPattern::Add(Box::new(self), Box::new(rhs))
    }
}

pub type BindingType<A = ()> = HashMap<String, AstNode<A>>;

impl AstPattern<'_> {
    pub fn matches<A: Clone>(&self, tree: &AstNode<A>) -> Option<BindingType<A>> {
        let mut matches = HashMap::new();

        match self {
            AstPattern::Any(name) => {
                matches.insert(name.to_string(), tree.clone());
            }
            AstPattern::Number(name) => {
                if let AstNode::Constant { .. } = tree {
                    matches.insert(name.to_string(), tree.clone());
                } else {
                    return None;
                }
            }
            AstPattern::Constant(pattern_value) => {
                if let AstNode::Constant { value, .. } = tree {
                    if value != pattern_value {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            AstPattern::Add(left_pattern, right_pattern) => {
                if let AstNode::Add { lhs, rhs, .. } = tree {
                    let left_matches = left_pattern.matches(&lhs);
                    let right_matches = right_pattern.matches(&rhs);

                    if left_matches.is_none() || right_matches.is_none() {
                        return None;
                    }

                    matches.extend(left_matches.unwrap());
                    matches.extend(right_matches.unwrap());
                } else {
                    return None;
                }
            }
        }

        Some(matches)
    }
}

pub struct PatternRewriteOnceIter<F>
where
    F: FnMut(&BindingType) -> AstNode,
{
    annotated_ast: AstNode<Option<usize>>,
    bindings: Vec<BindingType>,
    mapping: F,
    iter_index: usize,
}

impl<'a, F> PatternRewriteOnceIter<F>
where
    F: FnMut(&BindingType) -> AstNode,
{
    pub fn new<A: Clone>(ast: AstNode<A>, pattern: &AstPattern<'a>, mapping: F) -> Self {
        let mut bindings = Vec::new();
        let annotated_ast =
            Self::mark_matches(&pattern, ast.map_annotation(&mut |_| None), &mut bindings);

        Self {
            annotated_ast,
            bindings,
            mapping,
            iter_index: 0,
        }
    }

    fn mark_matches(
        pattern: &AstPattern<'a>,
        ast: AstNode<Option<usize>>,
        bindings: &mut Vec<BindingType>,
    ) -> AstNode<Option<usize>> {
        ast.map(|node| {
            let node = node.with_annotation(Some(0));
            if let Some(node_bindings) = pattern.matches(&node) {
                let current_index = bindings.len();

                let node_bindings = node_bindings
                    .into_iter()
                    .map(|(k, v)| (k, v.drop_annotation()))
                    .collect();

                bindings.push(node_bindings);
                node.with_annotation(Some(current_index))
            } else {
                node.with_annotation(None)
            }
        })
    }

    pub fn next(&mut self) -> Option<AstNode> {
        if self.iter_index >= self.bindings.len() {
            return None;
        }

        let f = &mut self.mapping;

        let rewritten_ast = self
            .annotated_ast
            .clone()
            .map(&mut |node: AstNode<Option<usize>>| {
                if let Some(Some(index)) = node.annotation() {
                    if index == &self.iter_index {
                        f(&self.bindings[*index]).map_annotation(&mut |_| None)
                    } else {
                        node
                    }
                } else {
                    node
                }
            });

        self.iter_index += 1;
        Some(rewritten_ast.drop_annotation())
    }
}

#[cfg(test)]
mod tests {
    use numbers::integer::BigInteger;

    use super::*;
    use crate::parser::parse;

    #[test]
    fn test_pattern_matching() {
        let ast = parse("2 * cos[1 + x] + 3").unwrap();

        use AstPattern::*;
        let pattern = Any("X") + Any("Y");

        let matches = pattern.matches(&ast);
        assert!(matches.is_some());

        let matches = matches.unwrap();
        assert!(matches.contains_key("X"));
        assert!(matches.contains_key("Y"));

        let x = matches.get("X").unwrap();
        let y = matches.get("Y").unwrap();
        assert_eq!(ast, AstNode::add(x.clone(), y.clone()));
    }

    #[test]
    fn test_rewrite_iter() {
        let ast = parse("2 * cos[1 + x] + 3").unwrap();

        use AstPattern::*;

        // Implements commutative addition pattern
        let pattern = Any("X") + Any("Y");
        let mut iter = PatternRewriteOnceIter::new(ast, &pattern, |bindings| {
            let x = bindings.get("X").unwrap();
            let y = bindings.get("Y").unwrap();

            AstNode::add(y.clone(), x.clone())
        });

        assert_eq!(
            iter.next().unwrap(),
            AstNode::add(
                AstNode::mul(
                    AstNode::constant(RealScalar::Integer(BigInteger::from_u64(2))),
                    AstNode::cos(AstNode::add(
                        AstNode::named_value("x".to_string()),
                        AstNode::constant(RealScalar::Integer(BigInteger::from_u64(1))),
                    ))
                ),
                AstNode::constant(RealScalar::Integer(BigInteger::from_u64(3)))
            )
        );

        assert_eq!(
            iter.next().unwrap(),
            AstNode::add(
                AstNode::constant(RealScalar::Integer(BigInteger::from_u64(3))),
                AstNode::mul(
                    AstNode::constant(RealScalar::Integer(BigInteger::from_u64(2))),
                    AstNode::cos(AstNode::add(
                        AstNode::constant(RealScalar::Integer(BigInteger::from_u64(1))),
                        AstNode::named_value("x".to_string()),
                    ))
                )
            )
        );

        assert!(iter.next().is_none());
    }
}
