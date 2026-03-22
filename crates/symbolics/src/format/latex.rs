use crate::builtins;
use crate::builtins::traits::BuiltIn;
use crate::expr::{ExprKind, RawExpr};
use crate::{
    atom::Atom,
    builtin::{CANNONICAL_SYM_INDETERMINATE, CANNONICAL_SYM_PLUS_INFINITY},
};
use numbers::Number;

// Determines the placement of paranthesis, depending on
// which position the term is placed at.
#[derive(Clone, Copy)]
enum Position {
    Root,
    AddOperand, // Either operand of Add
    SubLhs,     // Left-hand side of Sub
    SubRhs,     // Right-hand side of Sub; needs parens for add/sub/neg
    MulOperand, // Any operand of Mul
    DivChild,   // Numerator or denominator of Div
    FactArg,    // Factorial argument
    PowBase,    // Base of Pow
    PowExp,     // Exponent of Pow
    NegOperand, // Operand of unary Neg
    FnArg,      // Argument of a function
}

fn needs_parens(expr: &RawExpr, pos: Position) -> bool {
    match pos {
        // No parens necessary for these:
        Position::Root
        | Position::DivChild
        | Position::PowExp
        | Position::FnArg
        | Position::AddOperand
        | Position::SubLhs => false,

        Position::SubRhs => {
            // a-(b+c),  a-(b-c),  a-(-c)
            expr.has_head_symbol(builtins::Add::head())
                || expr.is_application_of(builtins::Sub::head(), 2)
                || expr.is_application_of(builtins::Neg::head(), 1)
        }

        Position::NegOperand => {
            // -(a+b),  -(a-b)
            expr.has_head_symbol(builtins::Add::head())
                || expr.is_application_of(builtins::Sub::head(), 2)
        }

        Position::MulOperand => {
            // (a+b)*c
            expr.has_head_symbol(builtins::Add::head())
                || expr.is_application_of(builtins::Sub::head(), 2)
        }

        Position::PowBase => {
            // (a+b)^n,  (a*b)^n,  (a/b)^n,  (-a)^n
            expr.has_head_symbol(builtins::Add::head())
                || expr.is_application_of(builtins::Sub::head(), 2)
                || expr.has_head_symbol(builtins::Mul::head())
                || expr.is_application_of(builtins::Div::head(), 2)
                || expr.is_application_of(builtins::Neg::head(), 1)
        }

        Position::FactArg => {
            // (a+b)!,  (a*b)!,  (a/b)!,  (-a)!
            expr.has_head_symbol(builtins::Add::head())
                || expr.is_application_of(builtins::Sub::head(), 2)
                || expr.has_head_symbol(builtins::Mul::head())
                || expr.is_application_of(builtins::Div::head(), 2)
                || expr.is_application_of(builtins::Neg::head(), 1)
        }
    }
}

fn wrap(s: String, expr: &RawExpr, pos: Position) -> String {
    if needs_parens(expr, pos) {
        format!("\\left({}\\right)", s)
    } else {
        s
    }
}

fn greek_letter(name: &str) -> String {
    match name {
        "alpha" | "α" => "\\alpha",
        "beta" | "β" => "\\beta",
        "gamma" | "γ" => "\\gamma",
        "delta" | "δ" => "\\delta",
        "epsilon" | "ε" => "\\epsilon",
        "zeta" | "ζ" => "\\zeta",
        "eta" | "η" => "\\eta",
        "theta" | "θ" => "\\theta",
        "iota" | "ι" => "\\iota",
        "kappa" | "κ" => "\\kappa",
        "lambda" | "λ" => "\\lambda",
        "mu" | "μ" => "\\mu",
        "nu" | "ν" => "\\nu",
        "xi" | "ξ" => "\\xi",
        "omicron" | "ο" => "\\omicron",
        "pi" | "π" => "\\pi",
        "rho" | "ρ" => "\\rho",
        "sigma" | "σ" | "ς" => "\\sigma",
        "tau" | "τ" => "\\tau",
        "upsilon" | "υ" => "\\upsilon",
        "phi" | "φ" | "ϕ" => "\\phi",
        "chi" | "χ" => "\\chi",
        "psi" | "ψ" => "\\psi",
        "omega" | "ω" => "\\omega",
        _ => return name.to_string(),
    }
    .to_string()
}

fn render_text(value: &str) -> String {
    format!(r#"\text{{{value}}}"#)
}

// ---- Rendering ----

fn render_one_arg(cmd: &str, arg: &RawExpr) -> String {
    format!(
        "{}\\left({}\\right)",
        cmd,
        expr_to_latex_with_pos(arg, Position::FnArg)
    )
}

fn render_derivative(args: &[RawExpr]) -> String {
    let f = &args[0];
    let x = &args[1];
    let f_latex = expr_to_latex_with_pos(f, Position::FnArg);
    let x_latex = expr_to_latex_with_pos(x, Position::FnArg);
    if x.is_symbol() {
        format!("\\frac{{ \\partial }}{{ \\partial {x_latex} }}\\left({f_latex}\\right)")
    } else {
        format!(
            "\\text{{{}}}\\left[{f_latex}, {x_latex}\\right]",
            builtins::Derivative::head()
        )
    }
}

fn render_integrate(args: &[RawExpr]) -> String {
    let f = &args[0];
    let x = &args[1];
    let f_latex = expr_to_latex_with_pos(f, Position::FnArg);
    let x_latex = expr_to_latex_with_pos(x, Position::FnArg);
    if x.is_symbol() {
        format!("\\int {f_latex}\\,\\text{{d}}{x_latex}")
    } else {
        format!("\\text{{Integrate}}\\left[{f_latex}, {x_latex}\\right]")
    }
}

fn render_generic_node(head_name: &str, args: &[RawExpr]) -> String {
    let args_str = args
        .iter()
        .map(|arg| expr_to_latex_with_pos(arg, Position::FnArg))
        .collect::<Vec<_>>()
        .join(", ");
    format!("\\text{{{head_name}}}\\left[{args_str}\\right]")
}

fn expr_to_latex_with_pos(expr: &RawExpr, pos: Position) -> String {
    let latex = expr_to_latex_inner(expr);
    wrap(latex, expr, pos)
}

fn expr_to_latex_inner(expr: &RawExpr) -> String {
    match expr.kind() {
        ExprKind::Atom {
            entry: Atom::Number(Number::Integer(value)),
            ..
        } => value.to_string(),

        ExprKind::Atom {
            entry: Atom::Number(Number::Rational(_)),
            ..
        } => unimplemented!("Should have been resugared to Div"),

        ExprKind::Atom {
            entry: Atom::Symbol(name),
            ..
        } if name == CANNONICAL_SYM_INDETERMINATE => render_text(name),

        ExprKind::Atom {
            entry: Atom::Symbol(name),
            ..
        } if name == CANNONICAL_SYM_PLUS_INFINITY => r#"\infty"#.to_string(),

        ExprKind::Atom {
            entry: Atom::Symbol(name),
            ..
        } => greek_letter(name),

        ExprKind::Node { args, .. } if expr.is_application_of(builtins::Neg::head(), 1) => {
            format!(
                "-{}",
                expr_to_latex_with_pos(&args[0], Position::NegOperand)
            )
        }

        ExprKind::Node { args, .. } if expr.has_head_symbol(builtins::Add::head()) => args
            .iter()
            .map(|arg| expr_to_latex_with_pos(arg, Position::AddOperand))
            .collect::<Vec<_>>()
            .join(" + "),

        ExprKind::Node { args, .. } if expr.is_application_of(builtins::Sub::head(), 2) => {
            let lhs = expr_to_latex_with_pos(&args[0], Position::SubLhs);
            let rhs = expr_to_latex_with_pos(&args[1], Position::SubRhs);
            format!("{lhs} - {rhs}")
        }

        ExprKind::Node { args, .. } if expr.has_head_symbol(builtins::Mul::head()) => args
            .iter()
            .map(|arg| expr_to_latex_with_pos(arg, Position::MulOperand))
            .collect::<Vec<_>>()
            .join(" \\cdot "),

        ExprKind::Node { args, .. } if expr.is_application_of(builtins::Div::head(), 2) => format!(
            "\\frac{{{}}}{{{}}}",
            expr_to_latex_with_pos(&args[0], Position::DivChild),
            expr_to_latex_with_pos(&args[1], Position::DivChild),
        ),

        ExprKind::Node { args, .. } if expr.is_application_of(builtins::Pow::head(), 2) => {
            let base = expr_to_latex_with_pos(&args[0], Position::PowBase);
            let exp = expr_to_latex_with_pos(&args[1], Position::PowExp);
            format!("{{{base}}}^{{{exp}}}")
        }

        ExprKind::Node { args, .. } if expr.is_application_of(builtins::Factorial::HEAD, 1) => {
            format!("{}!", expr_to_latex_with_pos(&args[0], Position::FactArg))
        }

        ExprKind::Node { args, .. } if expr.is_application_of(builtins::Sqrt::head(), 1) => {
            format!(
                "\\sqrt{{{}}}",
                expr_to_latex_with_pos(&args[0], Position::FnArg)
            )
        }

        ExprKind::Node { args, .. } if expr.is_application_of(builtins::Exp::head(), 1) => {
            render_one_arg("\\exp", &args[0])
        }
        ExprKind::Node { args, .. } if expr.is_application_of(builtins::Log::head(), 1) => {
            render_one_arg("\\log", &args[0])
        }
        ExprKind::Node { args, .. } if expr.is_application_of(builtins::Sin::head(), 1) => {
            render_one_arg("\\sin", &args[0])
        }
        ExprKind::Node { args, .. } if expr.is_application_of(builtins::Cos::head(), 1) => {
            render_one_arg("\\cos", &args[0])
        }
        ExprKind::Node { args, .. } if expr.is_application_of(builtins::Tan::head(), 1) => {
            render_one_arg("\\tan", &args[0])
        }

        ExprKind::Node { args, .. } if expr.is_application_of(builtins::Derivative::head(), 2) => {
            render_derivative(args)
        }

        ExprKind::Node { args, .. } if expr.is_application_of(builtins::Integrate::head(), 2) => {
            render_integrate(args)
        }

        ExprKind::Node { head, args } => {
            let Some(name) = head.get_symbol() else {
                unimplemented!()
            };
            render_generic_node(name, args)
        }

        _ => todo!(),
    }
}

pub fn render(expr: &RawExpr) -> String {
    expr_to_latex_with_pos(expr, Position::Root)
}

#[cfg(test)]
mod tests {
    use super::render;
    use crate::expr::RawExpr;
    use parser::parse;

    #[test]
    fn test_render() {
        let expr = RawExpr::from(parse("2 + 3").unwrap());
        assert_eq!(render(&expr), "2 + 3");
    }

    #[test]
    fn test_render_with_parenthesis() {
        let expr = RawExpr::from(parse("(2 + 3) * 6").unwrap());
        assert_eq!(render(&expr), "\\left(2 + 3\\right) \\cdot 6");
    }

    #[test]
    fn test_render_multiple_adds() {
        let expr = RawExpr::from(parse("1+2+3+4").unwrap());
        assert_eq!(render(&expr), "1 + 2 + 3 + 4");
    }

    #[test]
    fn test_render_with_unary_op() {
        let expr = RawExpr::from(parse("-2 + 3").unwrap());
        assert_eq!(render(&expr), "-2 + 3");
    }

    #[test]
    fn test_render_with_pow() {
        let expr = RawExpr::from(parse("pi^2").unwrap());
        assert_eq!(render(&expr), "{\\pi}^{2}");
    }

    #[test]
    fn test_render_with_function_call() {
        let expr = RawExpr::from(parse("5*pi^2/4*Cos[pi*x/2]*Sin[π*y/2]").unwrap());
        assert_eq!(
            render(&expr),
            "\\frac{5 \\cdot {\\pi}^{2}}{4} \\cdot \\cos\\left(\\frac{\\pi \\cdot x}{2}\\right) \\cdot \\sin\\left(\\frac{\\pi \\cdot y}{2}\\right)"
        );
    }

    // New tests exercising position-sensitive parenthesization
    #[test]
    fn test_sub_rhs_parens() {
        let expr = RawExpr::from(parse("a - (b + c)").unwrap());
        assert_eq!(render(&expr), "a - \\left(b + c\\right)");
    }

    #[test]
    fn test_sub_rhs_sub_parens() {
        let expr = RawExpr::from(parse("a - (b - c)").unwrap());
        assert_eq!(render(&expr), "a - \\left(b - c\\right)");
    }

    #[test]
    fn test_pow_base_sum_parens() {
        let expr = RawExpr::from(parse("(a + b)^2").unwrap());
        assert_eq!(render(&expr), "{\\left(a + b\\right)}^{2}");
    }

    #[test]
    fn test_neg_sum_parens() {
        let expr = RawExpr::from(parse("-(a + b)").unwrap());
        assert_eq!(render(&expr), "-\\left(a + b\\right)");
    }
}
