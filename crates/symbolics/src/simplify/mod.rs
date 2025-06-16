// mod naive;
pub mod normalize;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::{fmt, vec};

use numbers::RealScalar;
use numbers::integer::BigInteger;

use crate::format::MathDisplay;
use crate::parser::ast::AstNode;
use crate::simplify::pattern::{AstPattern, BindingType, PatternRewriteOnceIter};
pub mod pattern;

#[derive(Debug, Clone)]
struct EquivalentAstEntry<PatternId> {
    ast: AstNode,
    metric: usize,
    matched_patterns: HashSet<PatternId>,
}

#[derive(Clone)]
struct EquivalentAst<PatternId>(HashMap<String, EquivalentAstEntry<PatternId>>);

impl<PatternId: Eq + Hash + Debug> EquivalentAst<PatternId> {
    fn new() -> Self {
        EquivalentAst(HashMap::new())
    }

    fn insert(&mut self, ast: AstNode) {
        let ast_repr = ast.to_yasc();
        let metric = ast_repr.len();
        self.0.insert(
            ast_repr,
            EquivalentAstEntry {
                ast,
                metric,
                matched_patterns: HashSet::new(),
            },
        );
    }

    fn pattern_already_applied(&self, ast: &AstNode, pattern_id: &PatternId) -> bool {
        let ast_repr = ast.to_yasc();
        if let Some(entry) = self.0.get(&ast_repr) {
            entry.matched_patterns.contains(pattern_id)
        } else {
            false
        }
    }

    fn add_pattern(&mut self, ast: AstNode, pattern_id: PatternId) {
        let ast_repr = ast.to_yasc();
        if let Some(entry) = self.0.get_mut(&ast_repr) {
            entry.matched_patterns.insert(pattern_id);
        } else {
            // If the AST is not found, we can insert it
            self.insert(ast);
            self.0
                .get_mut(&ast_repr)
                .unwrap()
                .matched_patterns
                .insert(pattern_id);
        }
    }

    fn iter_asts(&self) -> impl Iterator<Item = &AstNode> {
        self.0.iter().map(|(_, entry)| &entry.ast)
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn shortest_ast(&self) -> Option<&AstNode> {
        self.0
            .values()
            .min_by_key(|entry| entry.metric)
            .map(|entry| &entry.ast)
    }
}

impl<T: Debug> fmt::Debug for EquivalentAst<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EquivalentAst[")?;
        for (repr, entry) in &self.0 {
            write!(f, "{}:{:?}, ", repr, entry.matched_patterns)?;
        }
        write!(f, "]")
    }
}

pub fn simplify_ast(ast: AstNode) -> AstNode {
    use AstPattern::*;
    let rules: Vec<(AstPattern, Box<dyn Fn(&BindingType) -> AstNode>)> = vec![
        (
            Any("A") + Any("B"),
            Box::new(|bind: &BindingType| {
                let a = bind.get("A").unwrap().clone();
                let b = bind.get("B").unwrap().clone();
                AstNode::add(b, a)
            }),
        ),
        (
            (Any("A") + Any("B")) + Any("C"),
            Box::new(|bind: &BindingType| {
                let a = bind.get("A").unwrap().clone();
                let b = bind.get("B").unwrap().clone();
                let c = bind.get("C").unwrap().clone();
                AstNode::add(AstNode::add(a, c), b)
            }),
        ),
        (
            Any("A") + Any("A"),
            Box::new(|bind: &BindingType| {
                let a = bind.get("A").unwrap().clone();
                AstNode::mul(
                    AstNode::constant(RealScalar::Integer(BigInteger::from_u64(2))),
                    a,
                )
            }),
        ),
        (
            Number("A") + Number("B"),
            Box::new(|bind: &BindingType| {
                let a = bind.get("A").unwrap().clone();
                let b = bind.get("B").unwrap().clone();
                if let (
                    AstNode::Constant { value: a_val, .. },
                    AstNode::Constant { value: b_val, .. },
                ) = (&a, &b)
                {
                    AstNode::constant(a_val + b_val)
                } else {
                    AstNode::add(a, b)
                }
            }),
        ),
    ];

    let mut equivalent_asts = EquivalentAst::new();
    equivalent_asts.insert(ast.clone());

    loop {
        let equivalent_asts_len = equivalent_asts.len();

        for ast in equivalent_asts.clone().iter_asts() {
            for (pattern_id, (pattern, rewrite_rule)) in rules.iter().enumerate() {
                if equivalent_asts.pattern_already_applied(ast, &pattern_id) {
                    continue; // Skip if the pattern has already been applied to this AST
                }

                let mut rewrite_iter =
                    PatternRewriteOnceIter::new(ast.clone(), pattern, rewrite_rule);

                while let Some(new_ast) = rewrite_iter.next() {
                    equivalent_asts.add_pattern(ast.clone(), pattern_id);
                    equivalent_asts.insert(new_ast.clone());
                }
            }
        }

        if equivalent_asts.len() == equivalent_asts_len {
            break; // No new ASTs were generated, we can stop
        }
    }

    equivalent_asts.shortest_ast().cloned().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn test_simplify_ast() {
        let ast = parse("x+x+x").unwrap();
        let simplified_ast = simplify_ast(ast);
        println!("Simplified: {}", simplified_ast.to_yasc());
    }
}
