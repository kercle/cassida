// mod naive;
pub mod macros;
pub mod normalize;
pub mod pattern;

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::LazyLock;
use std::{fmt, vec};

use crate::format::MathDisplay;
use crate::parser::ast::AstNode;
use crate::rw_rule;
use crate::simplify::pattern::{AstPattern, BindingType, PatternRewriteOnceIter};

type RwRule<'a> = (
    AstPattern<'a>,
    Box<dyn Fn(&BindingType) -> AstNode + Send + Sync>,
);

static RW_RULES: LazyLock<Vec<RwRule<'static>>> = LazyLock::new(|| {
    use AstPattern::*;

    vec![
        rw_rule!((Any("a") + Any("b")) + Any("c") => |a, b, c|
            a + (b + c)
        ),
        rw_rule!(Any("a") + Any("a") => |a|
            2 * a
        ),
        rw_rule!(Any("k") * Any("a") + Any("a") => |k, a| {
            if let Some(k_val) = k.value_from_constant() {
                AstNode::new_constant(k_val + 1) * a
            } else {
                (k + 1) * a
            }
        }),
        rw_rule!(Number("a") + Number("b") => |a, b| {
            let a_val = a.value_from_constant().unwrap();
            let b_val = b.value_from_constant().unwrap();

            AstNode::new_constant(a_val + b_val)
        }),
        rw_rule!((Any("a") * Any("b")) * Any("c") => |a, b, c|
            a * (b * c)
        ),
        rw_rule!(Number("a") * Number("b") => |a, b| {
            let a_val = a.value_from_constant().unwrap();
            let b_val = b.value_from_constant().unwrap();

            if let Some(result) = a_val * b_val {
                AstNode::new_constant(result)
            } else {
                a * b // mixed types, return as is
            }
        }),
    ]
});

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
        self.0.values().map(|entry| &entry.ast)
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

pub fn simplify_exhaustive(ast: AstNode) -> AstNode {
    let mut equivalent_asts = EquivalentAst::new();
    equivalent_asts.insert(ast.clone());

    loop {
        let equivalent_asts_len = equivalent_asts.len();

        for ast in equivalent_asts.clone().iter_asts() {
            for (pattern_id, (pattern, rewrite_rule)) in RW_RULES.iter().enumerate() {
                if equivalent_asts.pattern_already_applied(ast, &pattern_id) {
                    continue; // Skip if the pattern has already been applied to this AST
                }

                let mut rewrite_iter =
                    PatternRewriteOnceIter::new(ast.clone(), pattern, rewrite_rule);

                while let Some(new_ast) = rewrite_iter.next_pattern() {
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
        let ast = parse("f[x]+f[x]*2").unwrap();
        let simplified_ast = simplify_exhaustive(ast);
        println!("Simplified: {}", simplified_ast.to_yasc());
    }
}
