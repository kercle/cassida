use std::{collections::HashMap, fmt::Debug, ops};

use numbers::RealScalar;

use crate::parser::ast::AstNode;

#[derive(Debug, Clone)]
pub enum AstPattern<'a> {
    Any(&'a str),
    Number(&'a str),
    Constant(RealScalar),
    Add(Box<AstPattern<'a>>, Box<AstPattern<'a>>),
    Mul(Box<AstPattern<'a>>, Box<AstPattern<'a>>),
}

impl ops::Add for AstPattern<'_> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        AstPattern::Add(Box::new(self), Box::new(rhs))
    }
}

impl ops::Mul for AstPattern<'_> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        AstPattern::Mul(Box::new(self), Box::new(rhs))
    }
}

pub type BindingType<A = ()> = HashMap<String, AstNode<A>>;

impl AstPattern<'_> {
    pub fn matches<A>(&self, tree: &AstNode<A>) -> Option<BindingType<A>>
    where
        A: Clone + Debug + PartialEq + Default,
    {
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
            AstPattern::Add(left_pattern, right_pattern)
            | AstPattern::Mul(left_pattern, right_pattern) => {
                let (lhs, rhs) = match tree {
                    AstNode::Add { lhs, rhs, .. } if matches!(self, AstPattern::Add(..)) => {
                        (lhs, rhs)
                    }
                    AstNode::Mul { lhs, rhs, .. } if matches!(self, AstPattern::Mul(..)) => {
                        (lhs, rhs)
                    }
                    _ => return None,
                };

                let (matches_a, matches_b) = if let (Some(lm), Some(rm)) =
                    (left_pattern.matches(lhs), right_pattern.matches(rhs))
                {
                    (lm, rm)
                } else if let (Some(rm), Some(lm)) =
                    (right_pattern.matches(lhs), left_pattern.matches(rhs))
                {
                    // Operations are commutative, so also test the reverse order
                    (rm, lm)
                } else {
                    return None; // No valid match found
                };

                matches = matches_a;
                for (k, v) in matches_b {
                    if let Some(existing) = matches.get_mut(&k) {
                        if existing != &v {
                            return None; // Conflict in bindings
                        }
                    } else {
                        matches.insert(k, v);
                    }
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
    pub fn new<A: Clone + PartialEq + Debug>(
        ast: AstNode<A>,
        pattern: &AstPattern<'a>,
        mapping: F,
    ) -> Self {
        let mut bindings = Vec::new();
        let annotated_ast =
            Self::mark_matches(pattern, ast.map_annotation(&mut |_| None), &mut bindings);

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

    pub fn next_pattern(&mut self) -> Option<AstNode> {
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
        assert_eq!(ast, AstNode::new_add(x.clone(), y.clone()));
    }

    #[test]
    fn test_repeated_term_matching() {
        let ast = parse("x+x").unwrap();

        use AstPattern::*;
        let pattern = Any("X") + Any("X");
        let matches = pattern.matches(&ast);

        assert!(matches.is_some());
    }

    #[test]
    fn test_rewrite_x_plus_x() {
        let ast = parse("x+x").unwrap();

        use AstPattern::*;

        // Implements commutative addition pattern
        let pattern = Any("X") + Any("X");
        let mut iter = PatternRewriteOnceIter::new(ast, &pattern, |bindings| {
            let x = bindings.get("X").unwrap();
            AstNode::new_mul(
                AstNode::new_constant(RealScalar::Integer(BigInteger::from_u64(2))),
                x.clone(),
            )
        });

        assert_eq!(
            iter.next_pattern().unwrap(),
            AstNode::new_mul(
                AstNode::new_constant(RealScalar::Integer(BigInteger::from_u64(2))),
                AstNode::new_named_value("x".to_string()),
            )
        );

        assert!(iter.next_pattern().is_none());
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

            AstNode::new_add(y.clone(), x.clone())
        });

        assert_eq!(
            iter.next_pattern().unwrap(),
            AstNode::new_add(
                AstNode::new_mul(
                    AstNode::new_constant(RealScalar::Integer(BigInteger::from_u64(2))),
                    AstNode::new_cos(AstNode::new_add(
                        AstNode::new_named_value("x".to_string()),
                        AstNode::new_constant(RealScalar::Integer(BigInteger::from_u64(1))),
                    ))
                ),
                AstNode::new_constant(RealScalar::Integer(BigInteger::from_u64(3)))
            )
        );

        assert_eq!(
            iter.next_pattern().unwrap(),
            AstNode::new_add(
                AstNode::new_constant(RealScalar::Integer(BigInteger::from_u64(3))),
                AstNode::new_mul(
                    AstNode::new_constant(RealScalar::Integer(BigInteger::from_u64(2))),
                    AstNode::new_cos(AstNode::new_add(
                        AstNode::new_constant(RealScalar::Integer(BigInteger::from_u64(1))),
                        AstNode::new_named_value("x".to_string()),
                    ))
                )
            )
        );

        assert!(iter.next_pattern().is_none());
    }

    // #[test]
    // fn test_rewrite_addseq() {
    //     let ast = normalize_tree(parse("x+a+x+b+x").unwrap());

    //     dbg!(&ast);

    //     use AstPattern::*;
    //     let (pattern, rewrite_rule) = rw_rule!(
    //         AddSeq(|node| {
    //             if matches!(node, AstNode::Constant { .. }) {
    //                 "c".to_string()
    //             } else {
    //                 "rest".to_string()
    //             }
    //         }) =>
    //         |c, rest| {
    //             let c = bindings.get("c").unwrap();
    //             let rest = bindings.get("rest").unwrap();
    //             AstNode::new_add(c.clone(), rest.clone())
    //         }
    //     );

    //     assert!(matches.is_some());
    // }
}
