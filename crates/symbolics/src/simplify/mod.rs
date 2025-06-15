// mod naive;
pub mod normalize;

use std::collections::HashMap;
use std::vec;

use crate::format::MathDisplay;
use crate::parser::ast::AstNode;
use crate::simplify::pattern::{AstPattern, BindingType, PatternRewriteOnceIter};
pub mod pattern;

pub fn simplify_ast(ast: AstNode) -> AstNode {
    let initial_ast_repr_len = ast.to_yasc().len();
    let mut equivalent_asts = vec![(ast.clone(), initial_ast_repr_len)];
    let mut rewrite_tracker: HashMap<String, Vec<usize>> = HashMap::new();

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
        // (
        //     Any("A") + Constant(RealScalar::zero()),
        //     Box::new(|bind: &BindingType| bind.get("A").unwrap().clone()),
        // ),
        (
            Number("A") + Number("B"),
            Box::new(|bind: &BindingType| {
                let a = bind.get("A").unwrap().clone();
                let b = bind.get("B").unwrap().clone();
                dbg!("Pattern matches number: {}, {}", a.to_yasc(), b.to_yasc());
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

    loop {
        // TODO: Flatten nested loops for better readability

        let num_equivalent_asts = equivalent_asts.len();

        for (pattern_idx, (pattern, rewrite_rule)) in rules.iter().enumerate() {
            let mut more_equivalent_asts = Vec::new();

            for (ast, _) in equivalent_asts.iter() {
                let ast_repr = ast.to_yasc();

                println!("Applying pattern {} to {}", pattern_idx, ast_repr);
                println!("Rewrite tracker: {:?}", rewrite_tracker);

                if let Some(applied_patterns) = rewrite_tracker.get(&ast_repr) {
                    println!(" -- Already applied patterns: {:?}", applied_patterns);
                    if applied_patterns.contains(&pattern_idx) {
                        continue; // Skip already applied patterns
                    }
                }

                let mut rewrite_iter =
                    PatternRewriteOnceIter::new(ast.clone(), pattern, rewrite_rule);

                while let Some(new_ast) = rewrite_iter.next() {
                    println!(
                        " -- Applying pattern {}: {} -> {}",
                        pattern_idx,
                        ast.to_yasc(),
                        new_ast.to_yasc()
                    );

                    let new_ast_repr = new_ast.to_yasc();
                    if rewrite_tracker.contains_key(&new_ast_repr) {
                        continue; // Skip if the new AST is already in the list
                    }

                    more_equivalent_asts.push((new_ast, new_ast_repr.len()));
                }

                let ast_repr = ast.to_yasc();
                if let Some(entry) = rewrite_tracker.get_mut(&ast_repr) {
                    println!(
                        "Adding pattern {} to existing entry for AST: {}",
                        pattern_idx, ast_repr
                    );
                    entry.push(pattern_idx);
                } else {
                    rewrite_tracker.insert(ast_repr, vec![pattern_idx]);
                }
            }

            equivalent_asts.extend(more_equivalent_asts);
        }

        if equivalent_asts.len() == num_equivalent_asts {
            break; // No more rewrites possible
        }
    }

    let mut shortest_ast = ast;
    let shortest_ast_len = initial_ast_repr_len;

    for (equiv_ast, equiv_ast_len) in equivalent_asts {
        println!(
            "Equivalent AST (length: {}): {}",
            equiv_ast_len,
            equiv_ast.to_yasc(),
        );
        if equiv_ast_len < shortest_ast_len {
            // Update the shortest AST if a shorter one is found
            shortest_ast = equiv_ast;
        }
    }

    shortest_ast
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn test_simplify_ast() {
        let ast = parse("2+x+3").unwrap();
        let simplified_ast = simplify_ast(ast);

        println!("Simplified AST: {}", simplified_ast.to_yasc());
    }
}
