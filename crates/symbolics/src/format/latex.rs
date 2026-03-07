use crate::builtin::*;
use crate::{
    atom::Atom,
    builtin::{
        CANNONICAL_HEAD_COS, CANNONICAL_HEAD_DERIVATIVE, CANNONICAL_HEAD_EXP,
        CANNONICAL_HEAD_INTEGRATE, CANNONICAL_HEAD_LOG, CANNONICAL_HEAD_SIN, CANNONICAL_HEAD_SQRT,
        CANNONICAL_HEAD_TAN, CANNONICAL_SYM_INDETERMINATE, CANNONICAL_SYM_PLUS_INFINITY,
    },
};
use numbers::Number;

use crate::expr::Expr;

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

fn node_weight(expr: &Expr) -> Option<u32> {
    if expr.is_application_of(NEG_HEAD, 1) {
        Some(3)
    } else if expr.has_head_symbol(ADD_HEAD) || expr.is_application_of(SUB_HEAD, 2) {
        Some(1)
    } else if expr.has_head_symbol(MUL_HEAD) || expr.is_application_of(DIV_HEAD, 2) {
        Some(2)
    } else if expr.is_application_of(POW_HEAD, 2) {
        Some(4)
    } else {
        None
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

pub fn expr_to_latex(expr: &Expr, parent_weight: Option<u32>) -> String {
    let weight = node_weight(expr);

    match expr {
        Expr::Atom {
            entry: Atom::Number(value),
            ..
        } => {
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
        Expr::Atom {
            entry: Atom::Symbol(name),
            ..
        } if name == CANNONICAL_SYM_INDETERMINATE => format!(r#"\text{{{name}}}"#),
        Expr::Atom {
            entry: Atom::Symbol(name),
            ..
        } if name == CANNONICAL_SYM_PLUS_INFINITY => r#"\infty"#.to_string(),
        Expr::Atom {
            entry: Atom::Symbol(name),
            ..
        } => greek_letter(name),
        Expr::Node { args, .. } if expr.is_application_of(NEG_HEAD, 1) => {
            format!("-{}", expr_to_latex(args.first().unwrap(), weight))
        }
        Expr::Node { args, .. } if expr.has_head_symbol(ADD_HEAD) => {
            let args_str = args
                .iter()
                .map(|arg| expr_to_latex(arg, weight))
                .collect::<Vec<_>>()
                .join(" + ");
            wrap_with_parentheses(args_str, weight, parent_weight)
        }
        Expr::Node { args, .. } if expr.is_application_of(SUB_HEAD, 2) => wrap_with_parentheses(
            format!(
                "{} - {}",
                expr_to_latex(args.first().unwrap(), weight),
                expr_to_latex(args.last().unwrap(), weight)
            ),
            weight,
            parent_weight,
        ),
        Expr::Node { args, .. } if expr.is_application_of(MUL_HEAD, 2) => {
            let args_str = args
                .iter()
                .map(|arg| expr_to_latex(arg, weight))
                .collect::<Vec<_>>()
                .join(" \\cdot ");
            wrap_with_parentheses(args_str, weight, parent_weight)
        }
        Expr::Node { args, .. } if expr.is_application_of(DIV_HEAD, 2) => wrap_with_parentheses(
            format!(
                "\\frac{{{}}}{{{}}}",
                expr_to_latex(args.first().unwrap(), weight),
                expr_to_latex(args.last().unwrap(), weight)
            ),
            weight,
            parent_weight,
        ),
        Expr::Node { args, .. } if expr.is_application_of(POW_HEAD, 2) => wrap_with_parentheses(
            format!(
                "{{{}}}^{{{}}}",
                expr_to_latex(args.first().unwrap(), weight),
                expr_to_latex(args.last().unwrap(), weight)
            ),
            weight,
            parent_weight,
        ),
        Expr::Node { args, .. } if expr.is_application_of(CANNONICAL_HEAD_EXP, 1) => {
            format!(
                "\\exp\\left({}\\right)",
                expr_to_latex(args.first().unwrap(), weight)
            )
        }
        Expr::Node { args, .. } if expr.is_application_of(CANNONICAL_HEAD_LOG, 1) => {
            format!(
                "\\log\\left({}\\right)",
                expr_to_latex(args.first().unwrap(), weight)
            )
        }
        Expr::Node { args, .. } if expr.is_application_of(CANNONICAL_HEAD_SIN, 1) => {
            format!(
                "\\sin\\left({}\\right)",
                expr_to_latex(args.first().unwrap(), weight)
            )
        }
        Expr::Node { args, .. } if expr.is_application_of(CANNONICAL_HEAD_COS, 1) => {
            format!(
                "\\cos\\left({}\\right)",
                expr_to_latex(args.first().unwrap(), weight)
            )
        }
        Expr::Node { args, .. } if expr.is_application_of(CANNONICAL_HEAD_TAN, 1) => {
            format!(
                "\\tan\\left({}\\right)",
                expr_to_latex(args.first().unwrap(), weight)
            )
        }
        Expr::Node { args, .. } if expr.is_application_of(CANNONICAL_HEAD_SQRT, 1) => {
            format!("\\sqrt{{{}}}", expr_to_latex(args.first().unwrap(), weight))
        }
        Expr::Node { args, .. } if expr.is_application_of(CANNONICAL_HEAD_DERIVATIVE, 2) => {
            let f = args.first().unwrap();
            let x = args.last().unwrap();

            let f_latex = expr_to_latex(f, weight);
            let x_latex = expr_to_latex(x, weight);

            if x.is_symbol() {
                format!(
                    "\\frac{{ \\text{{d}} }}{{ \\text{{d}} {x_latex} }}\\left({f_latex}\\right)"
                )
            } else {
                format!("\\text{{D}}\\left[{f_latex}, {x_latex}\\right]")
            }
        }
        Expr::Node { args, .. } if expr.is_application_of(CANNONICAL_HEAD_INTEGRATE, 2) => {
            let f = args.first().unwrap();
            let x = args.last().unwrap();

            let f_latex = expr_to_latex(f, weight);
            let x_latex = expr_to_latex(x, weight);

            if x.is_symbol() {
                format!("\\int {f_latex}\\,\\text{{d}}{x_latex}")
            } else {
                format!("\\text{{Integrate}}\\left[{f_latex}, {x_latex}\\right]")
            }
        }
        Expr::Node { head, args, .. } => {
            let Some(name) = head.get_symbol() else {
                unimplemented!()
            };

            let args_str = args
                .iter()
                .map(|arg| expr_to_latex(arg, None))
                .collect::<Vec<_>>()
                .join(", ");

            format!("\\text{{{name}}}\\left[{args_str}\\right]")
        }
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{expr::Expr, format::MathDisplay};
    use parser::parse;

    #[test]
    fn test_expr_to_latex() {
        let ast = parse("2 + 3").unwrap();
        let expr: Expr = Expr::<()>::from(ast);
        assert_eq!(expr.to_latex(), "2 + 3");
    }

    #[test]
    fn test_expr_to_latex_with_parenthesis() {
        let ast = parse("(2 + 3) * 6").unwrap();
        let expr: Expr = Expr::<()>::from(ast);
        assert_eq!(expr.to_latex(), "\\left(2 + 3\\right) \\cdot 6");
    }

    #[test]
    fn test_expr_to_latex_multiple_adds() {
        let ast = parse("1+2+3+4").unwrap();
        let expr: Expr = Expr::<()>::from(ast);
        assert_eq!(expr.to_latex(), "1 + 2 + 3 + 4");
    }

    #[test]
    fn test_expr_to_latex_with_unary_op() {
        let ast = parse("-2 + 3").unwrap();
        let expr: Expr = Expr::<()>::from(ast);
        dbg!(&expr);
        assert_eq!(expr.to_latex(), "-2 + 3");
    }

    #[test]
    fn test_expr_to_latex_with_pow() {
        let ast = parse("pi^2").unwrap();
        let expr: Expr = Expr::<()>::from(ast);
        dbg!(&expr);
        assert_eq!(expr.to_latex(), "{\\pi}^{2}");
    }

    #[test]
    fn test_expr_to_latex_with_function_call() {
        let ast = parse("5*pi^2/4*cos[pi*x/2]*sin[π*y/2]").unwrap();
        let expr: Expr = Expr::<()>::from(ast);
        assert_eq!(
            expr.to_latex(),
            "\\frac{5 \\cdot {\\pi}^{2}}{4} \\cdot \\text{cos}\\left[\\frac{\\pi \\cdot x}{2}\\right] \\cdot \\text{sin}\\left[\\frac{\\pi \\cdot y}{2}\\right]"
        );
    }
}
