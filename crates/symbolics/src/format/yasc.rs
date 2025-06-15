use crate::parser::ast::AstNode;
use numbers::RealScalar;

fn operator_precedence(ast: &AstNode) -> Option<u32> {
    match ast {
        AstNode::Negation { .. } => Some(3),
        AstNode::Reciprocal { .. } => Some(3),
        AstNode::Add { .. } => Some(1),
        AstNode::Sub { .. } => Some(1),
        AstNode::Mul { .. } => Some(2),
        AstNode::Div { .. } => Some(2),
        AstNode::Constant {
            value: RealScalar::Rational { .. },
            ..
        } => Some(2),
        AstNode::Pow { .. } => Some(4),
        _ => None,
    }
}

fn wrap_with_parentheses(
    sub_tree_str: String,
    precedence: Option<u32>,
    parent_precedence: Option<u32>,
) -> String {
    if parent_precedence > precedence {
        format!("({})", sub_tree_str)
    } else {
        sub_tree_str
    }
}

pub fn ast_to_yasc(ast: &AstNode, parent_precedence: Option<u32>) -> String {
    let precedence = operator_precedence(ast);

    use AstNode::*;
    match ast {
        Constant {
            value: RealScalar::Rational(value),
            ..
        } => wrap_with_parentheses(value.to_string(), precedence, parent_precedence),
        Constant { value, .. } => value.to_string(),
        NamedValue { name, .. } => name.to_string(),
        Negation { arg, .. } => {
            format!("-{}", ast_to_yasc(arg, precedence))
        }
        Reciprocal { arg, .. } => {
            format!("1/{}", ast_to_yasc(arg, precedence))
        }
        Sub { lhs, rhs, .. } => wrap_with_parentheses(
            format!(
                "{}-{}",
                ast_to_yasc(lhs, precedence),
                ast_to_yasc(rhs, precedence)
            ),
            precedence,
            parent_precedence,
        ),
        Add { lhs, rhs, .. }
        | Mul { lhs, rhs, .. }
        | Div { lhs, rhs, .. }
        | Pow { lhs, rhs, .. } => {
            let op = match ast {
                AstNode::Add { .. } => "+",
                AstNode::Mul { .. } => "*",
                AstNode::Div { .. } => "/",
                AstNode::Pow { .. } => "^",
                _ => unreachable!(),
            };
            wrap_with_parentheses(
                format!(
                    "{}{op}{}",
                    ast_to_yasc(lhs, precedence),
                    ast_to_yasc(rhs, precedence)
                ),
                precedence,
                parent_precedence,
            )
        }
        AddSeq { nodes, .. } | MulSeq { nodes, .. } => {
            let op = if matches!(ast, AstNode::AddSeq { .. }) {
                "+"
            } else {
                "*"
            };
            let add_str = nodes
                .iter()
                .map(|node| ast_to_yasc(node, precedence))
                .collect::<Vec<_>>()
                .join(op);
            wrap_with_parentheses(add_str, precedence, parent_precedence)
        }
        Sin { arg, .. } => {
            format!("sin[{}]", ast_to_yasc(arg, precedence))
        }
        Cos { arg, .. } => {
            format!("cos[{}]", ast_to_yasc(arg, precedence))
        }
        Tan { arg, .. } => {
            format!("tan[{}]", ast_to_yasc(arg, precedence))
        }
        Sqrt { arg, .. } => {
            format!("sqrt[{}]", ast_to_yasc(arg, precedence))
        }
        FunctionCall { name, args, .. } => {
            let args_str = args
                .iter()
                .map(|arg| ast_to_yasc(arg, None))
                .collect::<Vec<_>>()
                .join(", ");

            format!("{name}[{args_str}]")
        }
        Block(nodes) => {
            let mut block_str = Vec::new();
            for node in nodes {
                block_str.push(ast_to_yasc(node, parent_precedence));
            }
            block_str.join(";\n")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::format::MathDisplay;
    use crate::parser::parse;

    #[test]
    fn test_ast_to_yasc() {
        let ast = parse("2 + 3").unwrap();
        assert_eq!(ast.to_yasc(), "2+3");
    }

    #[test]
    fn test_ast_to_yasc_with_parenthesis() {
        let ast = parse("(2 + 3) *6").unwrap();
        assert_eq!(ast.to_yasc(), "(2+3)*6");
    }

    #[test]
    fn test_ast_to_yasc_multiple_adds() {
        let ast = parse("1+2+3+4").unwrap();
        assert_eq!(ast.to_yasc(), "1+2+3+4");
    }

    #[test]
    fn test_ast_to_yasc_with_unary_op() {
        let ast = parse("-2 + 3").unwrap();
        assert_eq!(ast.to_yasc(), "-2+3");
    }

    #[test]
    fn test_ast_to_yasc_with_function_call() {
        let ast = parse("5*pi^2/4*cos[pi*x/2]*sin[π*y/2]").unwrap();
        assert_eq!(ast.to_yasc(), "5*pi^2/4*cos[pi*x/2]*sin[π*y/2]");
    }
}
