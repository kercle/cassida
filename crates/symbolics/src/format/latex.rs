use crate::parser::ast::AstNode;
use numbers::RealScalar;

fn greek_letter(name: &str) -> String {
    match name {
        "alpha" | "α" => "\\alpha".to_string(),
        "beta" | "β" => "\\beta".to_string(),
        "gamma" | "γ" => "\\gamma".to_string(),
        "delta" | "δ" => "\\delta".to_string(),
        "epsilon" | "ε" => "\\epsilon".to_string(),
        "zeta" | "ζ" => "\\zeta".to_string(),
        "eta" | "η" => "\\eta".to_string(),
        "theta" | "θ" => "\\theta".to_string(),
        "iota" | "ι" => "\\iota".to_string(),
        "kappa" | "κ" => "\\kappa".to_string(),
        "lambda" | "λ" => "\\lambda".to_string(),
        "mu" | "μ" => "\\mu".to_string(),
        "nu" | "ν" => "\\nu".to_string(),
        "xi" | "ξ" => "\\xi".to_string(),
        "omicron" | "ο" => "\\omicron".to_string(),
        "pi" | "π" => "\\pi".to_string(),
        "rho" | "ρ" => "\\rho".to_string(),
        "sigma" | "σ" | "ς" => "\\sigma".to_string(),
        "tau" | "τ" => "\\tau".to_string(),
        "upsilon" | "υ" => "\\upsilon".to_string(),
        "phi" | "φ" | "ϕ" => "\\phi".to_string(),
        "chi" | "χ" => "\\chi".to_string(),
        "psi" | "ψ" => "\\psi".to_string(),
        "omega" | "ω" => "\\omega".to_string(),
        _ => name.to_string(),
    }
}

fn operator_precedence(ast: &AstNode) -> Option<u32> {
    match ast {
        AstNode::Negation { .. } => Some(3),
        AstNode::Reciprocal { .. } => Some(3),
        AstNode::Add { .. } => Some(1),
        AstNode::Sub { .. } => Some(1),
        AstNode::Mul { .. } => Some(2),
        AstNode::Div { .. } => Some(2),
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
        format!("\\left({}\\right)", sub_tree_str)
    } else {
        sub_tree_str
    }
}

pub fn ast_to_latex(ast: &AstNode, parent_precedence: Option<u32>) -> String {
    let precedence = operator_precedence(ast);

    use AstNode::*;
    match ast {
        Constant { value, .. } => {
            if let RealScalar::Rational(rational) = value {
                if rational.is_zero() {
                    "0".to_string()
                } else if rational.denominator().is_one() {
                    rational.numerator().to_string()
                } else {
                    format!(
                        "\\frac{{{}}}{{{}}}",
                        rational.numerator(),
                        rational.denominator()
                    )
                }
            } else {
                greek_letter(&value.to_string())
            }
        }
        NamedValue { name, .. } => greek_letter(name),
        Negation { arg, .. } => {
            format!("-{}", ast_to_latex(arg, precedence))
        }
        Reciprocal { arg, .. } => {
            format!("\\frac{{1}}{{{}}}", ast_to_latex(arg, precedence))
        }
        Add { lhs, rhs, .. } => wrap_with_parentheses(
            format!(
                "{} + {}",
                ast_to_latex(lhs, precedence),
                ast_to_latex(rhs, precedence)
            ),
            precedence,
            parent_precedence,
        ),
        AddSeq { nodes, .. } => {
            let add_str = nodes
                .iter()
                .map(|node| ast_to_latex(node, precedence))
                .collect::<Vec<_>>()
                .join(" + ");
            wrap_with_parentheses(add_str, precedence, parent_precedence)
        }
        Sub { lhs, rhs, .. } => wrap_with_parentheses(
            format!(
                "{} - {}",
                ast_to_latex(lhs, precedence),
                ast_to_latex(rhs, precedence)
            ),
            precedence,
            parent_precedence,
        ),
        Mul { lhs, rhs, .. } => {
            let lhs_str = ast_to_latex(lhs, precedence);
            let rhs_str = ast_to_latex(rhs, precedence);
            let mul_str = if lhs.is_constant() && rhs.is_constant() {
                format!("{} \\cdot {}", lhs_str, rhs_str)
            } else {
                format!("{} {}", lhs_str, rhs_str)
            };

            wrap_with_parentheses(mul_str, precedence, parent_precedence)
        }
        MulSeq { nodes, .. } => {
            let mul_str = nodes
                .iter()
                .map(|node| ast_to_latex(node, precedence))
                .collect::<Vec<_>>()
                .join(" \\cdot ");
            wrap_with_parentheses(mul_str, precedence, parent_precedence)
        }
        Div { lhs, rhs, .. } => {
            let frac_str = format!(
                "\\frac{{{}}}{{{}}}",
                ast_to_latex(lhs, None),
                ast_to_latex(rhs, None)
            );

            wrap_with_parentheses(frac_str, precedence, parent_precedence)
        }
        Pow { lhs, rhs, .. } => {
            let pow_str = format!(
                "{}^{{{}}}",
                ast_to_latex(lhs, precedence),
                ast_to_latex(rhs, precedence)
            );

            wrap_with_parentheses(pow_str, precedence, parent_precedence)
        }
        Sin { arg, .. } => {
            format!("\\sin\\left({}\\right)", ast_to_latex(arg, precedence))
        }
        Cos { arg, .. } => {
            format!("\\cos\\left({}\\right)", ast_to_latex(arg, precedence))
        }
        Tan { arg, .. } => {
            format!("\\tan\\left({}\\right)", ast_to_latex(arg, precedence))
        }
        Sqrt { arg, .. } => {
            format!("\\sqrt{{{}}}", ast_to_latex(arg, precedence))
        }
        FunctionCall { name, args, .. } => {
            let args_str = args
                .iter()
                .map(|arg| ast_to_latex(arg, None))
                .collect::<Vec<_>>()
                .join(", ");

            format!("{name}\\left[{args_str}\\right]")
        }
        Block(nodes) => {
            let mut block_str = String::new();
            for node in nodes {
                if !block_str.is_empty() {
                    block_str.push_str(" \\\\\n");
                }
                block_str.push_str(&ast_to_latex(node, parent_precedence));
            }
            block_str
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::format::MathDisplay;
    use crate::parser::parse;

    #[test]
    fn test_ast_to_latex() {
        let ast = parse("2 + 3").unwrap();
        assert_eq!(ast.to_latex(), "2 + 3");
    }

    #[test]
    fn test_ast_to_latex_with_parenthesis() {
        let ast = parse("(2 + 3) * 6").unwrap();
        assert_eq!(ast.to_latex(), "\\left(2 + 3\\right) 6");
    }

    #[test]
    fn test_ast_to_latex_multiple_adds() {
        let ast = parse("1+2+3+4").unwrap();
        assert_eq!(ast.to_latex(), "1 + 2 + 3 + 4");
    }

    #[test]
    fn test_ast_to_latex_with_unary_op() {
        let ast = parse("-2 + 3").unwrap();
        assert_eq!(ast.to_latex(), "-2 + 3");
    }

    #[test]
    fn test_ast_to_latex_with_function_call() {
        let ast = parse("5*pi^2/4*cos[pi*x/2]*sin[π*y/2]").unwrap();
        assert_eq!(
            ast.to_latex(),
            "\\frac{5 \\pi^{2}}{4} \\cos\\left(\\frac{\\pi x}{2}\\right) \\sin\\left(\\frac{\\pi y}{2}\\right)"
        );
    }
}
