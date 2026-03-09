use crate::builtin::*;
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
    DivLhs,     // Numerator of Div
    DivRhs,     // Denominator of Div
    PowBase,    // Base of Pow
    PowExp,     // Exponent of Pow
    NegOperand, // Operand of unary Neg
    FnArg,      // Argument of a function
}

fn needs_parens(expr: &RawExpr, pos: Position) -> bool {
    match pos {
        // No parens necessary for these:
        Position::Root | Position::FnArg | Position::AddOperand | Position::SubLhs => false,

        Position::SubRhs => {
            // a-(b+c),  a-(b-c),  a-(-c)
            expr.has_head_symbol(ADD_HEAD)
                || expr.is_application_of(SUB_HEAD, 2)
                || expr.is_application_of(NEG_HEAD, 1)
        }

        Position::NegOperand => {
            // -(a+b),  -(a-b)
            expr.has_head_symbol(ADD_HEAD) || expr.is_application_of(SUB_HEAD, 2)
        }

        Position::MulOperand => {
            // (a+b)*c
            expr.has_head_symbol(ADD_HEAD) || expr.is_application_of(SUB_HEAD, 2)
        }

        Position::DivLhs => {
            // (a+b)*c
            expr.has_head_symbol(ADD_HEAD) || expr.is_application_of(SUB_HEAD, 2)
        }

        Position::DivRhs => {
            // a/(a+b), a/(a-b), a/(a*b), a/(a/b), a/(-a)
            expr.has_head_symbol(ADD_HEAD)
                || expr.is_application_of(SUB_HEAD, 2)
                || expr.has_head_symbol(MUL_HEAD)
                || expr.has_head_symbol(DIV_HEAD)
                || expr.has_head_symbol(NEG_HEAD)
        }

        Position::PowBase => {
            // (a+b)^n,  (a*b)^n,  (a/b)^n,  (-a)^n
            expr.has_head_symbol(ADD_HEAD)
                || expr.is_application_of(SUB_HEAD, 2)
                || expr.has_head_symbol(MUL_HEAD)
                || expr.is_application_of(DIV_HEAD, 2)
                || expr.is_application_of(NEG_HEAD, 1)
        }

        Position::PowExp => {
            expr.has_head_symbol(ADD_HEAD)
                || expr.is_application_of(SUB_HEAD, 2)
                || expr.has_head_symbol(MUL_HEAD)
                || expr.is_application_of(DIV_HEAD, 2)
                || expr.is_application_of(NEG_HEAD, 1)
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
            entry: Atom::Number(Number::Rational(_)),
            ..
        } => unimplemented!("Should have been resugared to Div"),

        ExprKind::Atom {
            entry: Atom::StringLiteral(value),
            ..
        } => format!("\"{value}\""),

        ExprKind::Atom {
            entry: Atom::Symbol(name),
            ..
        } if name == CANNONICAL_SYM_INDETERMINATE => name.to_string(),

        ExprKind::Atom {
            entry: Atom::Symbol(name),
            ..
        } if name == CANNONICAL_SYM_PLUS_INFINITY => r#"\infty"#.to_string(),

        ExprKind::Atom {
            entry: Atom::Symbol(name),
            ..
        } => name.to_string(),

        ExprKind::Node { args, .. } if expr.is_application_of(NEG_HEAD, 1) => {
            format!(
                "-{}",
                expr_to_input_with_pos(&args[0], Position::NegOperand)
            )
        }

        ExprKind::Node { args, .. } if expr.has_head_symbol(ADD_HEAD) => args
            .iter()
            .map(|arg| expr_to_input_with_pos(arg, Position::AddOperand))
            .collect::<Vec<_>>()
            .join(" + "),

        ExprKind::Node { args, .. } if expr.is_application_of(SUB_HEAD, 2) => {
            let lhs = expr_to_input_with_pos(&args[0], Position::SubLhs);
            let rhs = expr_to_input_with_pos(&args[1], Position::SubRhs);
            format!("{lhs} - {rhs}")
        }

        ExprKind::Node { args, .. } if expr.has_head_symbol(MUL_HEAD) => args
            .iter()
            .map(|arg| expr_to_input_with_pos(arg, Position::MulOperand))
            .collect::<Vec<_>>()
            .join(" * "),

        ExprKind::Node { args, .. } if expr.is_application_of(DIV_HEAD, 2) => format!(
            "{}/{}",
            expr_to_input_with_pos(&args[0], Position::DivLhs),
            expr_to_input_with_pos(&args[1], Position::DivRhs),
        ),

        ExprKind::Node { args, .. } if expr.is_application_of(POW_HEAD, 2) => {
            let base = expr_to_input_with_pos(&args[0], Position::PowBase);
            let exp = expr_to_input_with_pos(&args[1], Position::PowExp);
            format!("{base}^{exp}")
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
}
