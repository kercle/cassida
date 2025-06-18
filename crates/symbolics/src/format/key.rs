use crate::parser::ast::AstNode;
use numbers::RealScalar;

pub fn ast_to_key_string<A>(ast: &AstNode<A>) -> String
where
    A: Clone + PartialEq,
{
    use AstNode::*;
    match ast {
        Constant {
            value: RealScalar::Rational(value),
            ..
        } => value.to_hex_string(),
        Constant { value, .. } => value.to_hex_string(),
        NamedValue { name, .. } => name.to_string(),
        Negation { arg, .. } => {
            format!("neg[{}]", ast_to_key_string(arg))
        }
        Reciprocal { arg, .. } => {
            format!("/ 1 {}", ast_to_key_string(arg))
        }
        Add { lhs, rhs, .. }
        | Sub { lhs, rhs, .. }
        | Mul { lhs, rhs, .. }
        | Div { lhs, rhs, .. }
        | Pow { lhs, rhs, .. } => {
            let op = match ast {
                AstNode::Add { .. } => "+",
                AstNode::Sub { .. } => "+",
                AstNode::Mul { .. } => "*",
                AstNode::Div { .. } => "/",
                AstNode::Pow { .. } => "^",
                _ => unreachable!(),
            };
            format!(
                "{}[{},{}]",
                op,
                ast_to_key_string(lhs),
                ast_to_key_string(rhs)
            )
        }
        AddSeq { nodes, .. } | MulSeq { nodes, .. } => {
            let mut out = "+ ".repeat(nodes.len() - 1);
            out.push_str(
                &nodes
                    .iter()
                    .map(|node| ast_to_key_string(node))
                    .collect::<Vec<_>>()
                    .join(" "),
            );
            out
        }
        Sin { arg, .. } => {
            format!("sin[{}]", ast_to_key_string(arg))
        }
        Cos { arg, .. } => {
            format!("cos[{}]", ast_to_key_string(arg))
        }
        Tan { arg, .. } => {
            format!("tan[{}]", ast_to_key_string(arg))
        }
        Sqrt { arg, .. } => {
            format!("sqrt[{}]", ast_to_key_string(arg))
        }
        FunctionCall { name, args, .. } => {
            let args_str = args
                .iter()
                .map(|arg| ast_to_key_string(arg))
                .collect::<Vec<_>>()
                .join(",");

            format!("{name}[{args_str}]")
        }
        Block(nodes) => {
            let mut block_str = Vec::new();
            for node in nodes {
                block_str.push(ast_to_key_string(node));
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
    fn test_ast_to_key_string() {
        let ast = parse("2 + 3").unwrap();
        assert_eq!(ast.to_key_string(), "+[0x2,0x3]");
    }

    #[test]
    fn test_ast_to_key_string_with_parenthesis() {
        let ast = parse("(2 + 3) *60").unwrap();
        assert_eq!(ast.to_key_string(), "*[+[0x2,0x3],0x3c]");
    }

    #[test]
    fn test_ast_to_key_string_multiple_adds() {
        let ast = parse("1+2+3+4").unwrap();
        assert_eq!(ast.to_key_string(), "+[+[+[0x1,0x2],0x3],0x4]");
    }

    #[test]
    fn test_ast_to_key_string_with_unary_op() {
        let ast = parse("-2+3").unwrap();
        assert_eq!(ast.to_key_string(), "+[neg[0x2],0x3]");
    }

    #[test]
    fn test_ast_to_key_string_with_function_call() {
        let ast = parse("5*pi^2/4*cos[pi*x/2]*sin[π*y/2]").unwrap();
        assert_eq!(
            ast.to_key_string(),
            "*[*[/[*[0x5,^[pi,0x2]],0x4],cos[/[*[pi,x],0x2]]],sin[/[*[π,y],0x2]]]"
        );
    }
}
