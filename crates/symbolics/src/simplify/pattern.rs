use std::{collections::HashMap, ops};

use numbers::RealScalar;

use crate::parser::ast::AstNode;

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

impl AstPattern<'_> {
    pub fn matches<A: Clone>(&self, tree: &AstNode<A>) -> Option<HashMap<String, AstNode<A>>> {
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

pub struct PatternRewriteIter {
    flagged_ast: AstNode<bool>,
}

impl PatternRewriteIter {
    pub fn with_ast<A: Clone>(ast: AstNode<A>) -> Self {
        let flagged_ast = ast.map_annotation(&mut |_| false);
        Self { flagged_ast }
    }
}

#[cfg(test)]
mod tests {
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
}
