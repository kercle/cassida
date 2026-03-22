use crate::atom::Atom;
use crate::builtins;
use crate::builtins::traits::BuiltIn;
use crate::expr::{ExprKind, RawExpr};
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
    DivLhs,     // Numerator of Div
    DivRhs,     // Denominator of Div
    PowBase,    // Base of Pow
    PowExp,     // Exponent of Pow
    FactArg,    // Factorial argument
    NegOperand, // Operand of unary Neg
    FnArg,      // Argument of a function
}

fn needs_parens(expr: &RawExpr, pos: Position) -> bool {
    match pos {
        // No parens necessary for these:
        Position::Root | Position::FnArg | Position::AddOperand | Position::SubLhs => false,

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

        Position::DivLhs => {
            // (a+b)*c
            expr.has_head_symbol(builtins::Add::head())
                || expr.is_application_of(builtins::Sub::head(), 2)
        }

        Position::DivRhs => {
            // a/(a+b), a/(a-b), a/(a*b), a/(a/b), a/(-a)
            expr.has_head_symbol(builtins::Add::head())
                || expr.is_application_of(builtins::Sub::head(), 2)
                || expr.has_head_symbol(builtins::Mul::head())
                || expr.has_head_symbol(builtins::Div::head())
                || expr.has_head_symbol(builtins::Neg::head())
        }

        Position::PowBase => {
            // (a+b)^n,  (a*b)^n,  (a/b)^n,  (-a)^n
            expr.has_head_symbol(builtins::Add::head())
                || expr.is_application_of(builtins::Sub::head(), 2)
                || expr.has_head_symbol(builtins::Mul::head())
                || expr.is_application_of(builtins::Div::head(), 2)
                || expr.is_application_of(builtins::Neg::head(), 1)
        }

        Position::PowExp => {
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
                || expr.is_application_of(builtins::Pow::head(), 2)
        }
    }
}

fn wrap(s: String, expr: &RawExpr, pos: Position) -> String {
    if needs_parens(expr, pos) {
        format!("({})", s)
    } else {
        s
    }
}

// ---- Rendering ----

fn expr_to_input_with_pos(expr: &RawExpr, pos: Position) -> String {
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
            entry: Atom::Number(Number::Rational(r)),
            ..
        } => unimplemented!("{r} should have been resugared to Div"),

        ExprKind::Atom {
            entry: Atom::StringLiteral(value),
            ..
        } => format!("\"{value}\""),

        ExprKind::Atom {
            entry: Atom::Symbol(name),
            ..
        } if name == builtins::symbols::INDETERMINATE => name.to_string(),

        ExprKind::Atom {
            entry: Atom::Symbol(name),
            ..
        } if name == builtins::symbols::INFINITY => r#"\infty"#.to_string(),

        ExprKind::Atom {
            entry: Atom::Symbol(name),
            ..
        } => name.to_string(),

        ExprKind::Node { args, .. } if expr.is_application_of(builtins::Neg::head(), 1) => {
            format!(
                "-{}",
                expr_to_input_with_pos(&args[0], Position::NegOperand)
            )
        }

        ExprKind::Node { args, .. } if expr.has_head_symbol(builtins::Add::head()) => {
            if args.is_empty() {
                format!("{}[]", builtins::Add::head())
            } else if args.len() == 1 {
                format!(
                    "{}[{}]",
                    builtins::Add::head(),
                    expr_to_input_with_pos(args.first().unwrap(), Position::FnArg)
                )
            } else {
                args.iter()
                    .map(|arg| expr_to_input_with_pos(arg, Position::AddOperand))
                    .collect::<Vec<_>>()
                    .join(" + ")
            }
        }

        ExprKind::Node { args, .. } if expr.is_application_of(builtins::Sub::head(), 2) => {
            let lhs = expr_to_input_with_pos(&args[0], Position::SubLhs);
            let rhs = expr_to_input_with_pos(&args[1], Position::SubRhs);
            format!("{lhs} - {rhs}")
        }

        ExprKind::Node { args, .. } if expr.has_head_symbol(builtins::Mul::head()) => {
            if args.is_empty() {
                format!("{}[]", builtins::Mul::head())
            } else if args.len() == 1 {
                format!(
                    "{}[{}]",
                    builtins::Mul::head(),
                    expr_to_input_with_pos(args.first().unwrap(), Position::FnArg)
                )
            } else {
                args.iter()
                    .map(|arg| expr_to_input_with_pos(arg, Position::MulOperand))
                    .collect::<Vec<_>>()
                    .join(" * ")
            }
        }

        ExprKind::Node { args, .. } if expr.is_application_of(builtins::Div::head(), 2) => format!(
            "{}/{}",
            expr_to_input_with_pos(&args[0], Position::DivLhs),
            expr_to_input_with_pos(&args[1], Position::DivRhs),
        ),

        ExprKind::Node { args, .. } if expr.is_application_of(builtins::Pow::head(), 2) => {
            let base = expr_to_input_with_pos(&args[0], Position::PowBase);
            let exp = expr_to_input_with_pos(&args[1], Position::PowExp);
            format!("{base}^{exp}")
        }

        ExprKind::Node { args, .. } if expr.is_application_of(builtins::Factorial::HEAD, 1) => {
            format!("{}!", expr_to_input_with_pos(&args[0], Position::FactArg))
        }

        ExprKind::Node { head, args } => {
            let Some(name) = head.get_symbol() else {
                unimplemented!()
            };
            let args_str = args
                .iter()
                .map(|arg| expr_to_input_with_pos(arg, Position::FnArg))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{name}[{args_str}]")
        }
    }
}

pub fn render(expr: &RawExpr) -> String {
    expr_to_input_with_pos(expr, Position::Root)
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
        assert_eq!(render(&expr), "(2 + 3) * 6");
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
        assert_eq!(render(&expr), "pi^2");
    }

    #[test]
    fn test_render_with_function_call() {
        let expr = RawExpr::from(parse("5*pi^2/4*Cos[pi*x/2]*Sin[π*y/2]").unwrap());
        assert_eq!(render(&expr), "5 * pi^2/4 * Cos[pi * x/2] * Sin[π * y/2]");
    }

    // New tests exercising position-sensitive parenthesization
    #[test]
    fn test_sub_rhs_parens() {
        let expr = RawExpr::from(parse("a - (b + c)").unwrap());
        assert_eq!(render(&expr), "a - (b + c)");
    }

    #[test]
    fn test_sub_rhs_sub_parens() {
        let expr = RawExpr::from(parse("a - (b - c)").unwrap());
        assert_eq!(render(&expr), "a - (b - c)");
    }

    #[test]
    fn test_pow_base_sum_parens() {
        let expr = RawExpr::from(parse("(a + b)^2").unwrap());
        assert_eq!(render(&expr), "(a + b)^2");
    }

    #[test]
    fn test_neg_sum_parens() {
        let expr = RawExpr::from(parse("-(a + b)").unwrap());
        assert_eq!(render(&expr), "-(a + b)");
    }

    #[test]
    fn test_hold_pattern_with_add() {
        let expr = RawExpr::from(parse("HoldPattern[Add[x__]]").unwrap()).normalize();
        assert_eq!(
            render(&expr.into_raw()),
            "HoldPattern[Add[Pattern[x, BlankSeq[]]]]"
        );
    }
}
