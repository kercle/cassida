use std::{collections::HashMap, ops};

use numbers::RealScalar;

use crate::parser::ast::AstNode;

pub enum AstPattern<'a> {
    Any(&'a str),
    Number(&'a str),
    Constant(RealScalar),
    Add(Box<AstPattern<'a>>, Box<AstPattern<'a>>),
}

pub trait PatternRewrite
where
    Self: Sized,
{
    fn matches(&self, pattern: &AstPattern) -> Option<HashMap<String, Self>>;
    fn rewrite<F>(&self, pattern: &AstPattern, apply_func: F) -> Self
    where
        F: Fn(&Self) -> Self;
}

impl ops::Add for AstPattern<'_> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        AstPattern::Add(Box::new(self), Box::new(rhs))
    }
}

impl PatternRewrite for AstNode {
    fn matches(&self, pattern: &AstPattern) -> Option<HashMap<String, Self>> {
        let mut matches = HashMap::new();

        match pattern {
            AstPattern::Any(name) => {
                matches.insert(name.to_string(), self.clone());
            }
            AstPattern::Number(name) => {
                if let AstNode::Constant { .. } = self {
                    matches.insert(name.to_string(), self.clone());
                } else {
                    return None;
                }
            }
            AstPattern::Constant(pattern_value) => {
                if let AstNode::Constant { value, .. } = self {
                    if value != pattern_value {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            AstPattern::Add(left_pattern, right_pattern) => {
                if let AstNode::Add(left, right) = self {
                    let left_matches = left.matches(left_pattern);
                    let right_matches = right.matches(right_pattern);

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

    fn rewrite<F>(&self, _pattern: &AstPattern, _apply_func: F) -> Self
    where
        F: Fn(&Self) -> Self,
    {
        todo!()
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

        let matches = ast.matches(&pattern);
        assert!(matches.is_some());

        let matches = matches.unwrap();
        assert!(matches.contains_key("X"));
        assert!(matches.contains_key("Y"));

        let x = matches.get("X").unwrap();
        let y = matches.get("Y").unwrap();
        assert_eq!(ast, AstNode::Add(Box::new(x.clone()), Box::new(y.clone())));
    }
}
