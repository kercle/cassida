use crate::{
    builtin::{
        CANNONICAL_HEAD_COS, CANNONICAL_HEAD_DERIVATIVE, CANNONICAL_HEAD_SIN, CANNONICAL_HEAD_SQRT,
        CANNONICAL_HEAD_TAN, CANNONICAL_SYM_INDETERMINATE,
    },
    parser::ast::ParserAst,
};
use numbers::Number;

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

fn node_weight<A>(ast: &ParserAst<A>) -> Option<u32>
where
    A: Clone + PartialEq,
{
    match ast {
        ParserAst::Negation { .. } => Some(3),
        ParserAst::Add { .. } => Some(1),
        ParserAst::Sub { .. } => Some(1),
        ParserAst::Mul { .. } => Some(2),
        ParserAst::Div { .. } => Some(2),
        ParserAst::Pow { .. } => Some(4),
        _ => None,
    }
}

fn wrap_with_parentheses(
    sub_tree_str: String,
    weight: Option<u32>,
    parent_weight: Option<u32>,
) -> String {
    if parent_weight > weight {
        format!("\\left({}\\right)", sub_tree_str)
    } else {
        sub_tree_str
    }
}

pub fn ast_to_latex<A>(ast: &ParserAst<A>, parent_weight: Option<u32>) -> String
where
    A: Clone + PartialEq,
{
    let weight = node_weight(ast);

    use ParserAst::*;
    match ast {
        Constant { value, .. } => {
            if let Number::Rational(rational) = value {
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
        Symbol { name, .. } if name == CANNONICAL_SYM_INDETERMINATE => format!(r#"\text{{{name}}}"#),
        Symbol { name, .. } => greek_letter(name),
        Negation { arg, .. } => {
            format!("-{}", ast_to_latex(arg, weight))
        }
        Add { nodes, .. } => {
            let add_str = nodes
                .iter()
                .map(|node| ast_to_latex(node, weight))
                .collect::<Vec<_>>()
                .join(" + ");
            wrap_with_parentheses(add_str, weight, parent_weight)
        }
        Sub { lhs, rhs, .. } => wrap_with_parentheses(
            format!(
                "{} - {}",
                ast_to_latex(lhs, weight),
                ast_to_latex(rhs, weight)
            ),
            weight,
            parent_weight,
        ),
        Mul { nodes, .. } => {
            let mul_str = nodes
                .iter()
                .map(|node| ast_to_latex(node, weight))
                .collect::<Vec<_>>()
                .join(" \\cdot ");
            wrap_with_parentheses(mul_str, weight, parent_weight)
        }
        Div { lhs, rhs, .. } => {
            let frac_str = format!(
                "\\frac{{{}}}{{{}}}",
                ast_to_latex(lhs, None),
                ast_to_latex(rhs, None)
            );

            wrap_with_parentheses(frac_str, weight, parent_weight)
        }
        Pow { lhs, rhs, .. } => {
            let pow_str = format!(
                "{}^{{{}}}",
                ast_to_latex(lhs, weight),
                ast_to_latex(rhs, weight)
            );

            wrap_with_parentheses(pow_str, weight, parent_weight)
        }
        FunctionCall { name, args, .. } if name == CANNONICAL_HEAD_SIN && args.len() == 1 => {
            format!(
                "\\sin\\left({}\\right)",
                ast_to_latex(args.first().unwrap(), weight)
            )
        }
        FunctionCall { name, args, .. } if name == CANNONICAL_HEAD_COS && args.len() == 1 => {
            format!(
                "\\cos\\left({}\\right)",
                ast_to_latex(args.first().unwrap(), weight)
            )
        }
        FunctionCall { name, args, .. } if name == CANNONICAL_HEAD_TAN && args.len() == 1 => {
            format!(
                "\\tan\\left({}\\right)",
                ast_to_latex(args.first().unwrap(), weight)
            )
        }
        FunctionCall { name, args, .. } if name == CANNONICAL_HEAD_SQRT && args.len() == 1 => {
            format!("\\sqrt{{{}}}", ast_to_latex(args.first().unwrap(), weight))
        }
        FunctionCall { name, args, .. }
            if name == CANNONICAL_HEAD_DERIVATIVE && args.len() == 2 =>
        {
            let f = args.get(0).unwrap();
            let x = args.get(1).unwrap();

            let f_latex = ast_to_latex(f, weight);
            let x_latex = ast_to_latex(x, weight);

            if x.is_symbol() {
                format!(
                    "\\frac{{ \\text{{d}} }}{{ \\text{{d}} {x_latex} }}\\left({f_latex}\\right)"
                )
            } else {
                format!("\\text{{D}}\\left[{f_latex}, {x_latex}\\right]")
            }
        }
        FunctionCall { name, args, .. } => {
            let args_str = args
                .iter()
                .map(|arg| ast_to_latex(arg, None))
                .collect::<Vec<_>>()
                .join(", ");

            format!("\\text{{{name}}}\\left[{args_str}\\right]")
        }
        Block { nodes, .. } => {
            let mut block_str = String::new();
            for node in nodes {
                if !block_str.is_empty() {
                    block_str.push_str(" \\\\\n");
                }
                block_str.push_str(&ast_to_latex(node, parent_weight));
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
        assert_eq!(ast.to_latex(), "\\left(2 + 3\\right) \\cdot 6");
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
            r#"\frac{5 \cdot \pi^{2}}{4} \cdot \text{cos}\left[\frac{\pi \cdot x}{2}\right] \cdot \text{sin}\left[\frac{\pi \cdot y}{2}\right]"#
        );
    }
}
