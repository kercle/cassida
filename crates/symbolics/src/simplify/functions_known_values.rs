use expr_macro::expr;

use crate::{
    atom::Atom,
    builtin::{
        CANNONICAL_HEAD_COS, CANNONICAL_HEAD_EXP, CANNONICAL_HEAD_LOG, CANNONICAL_HEAD_SIN,
        CANNONICAL_HEAD_TAN, CANNONICAL_SYM_PLUS_INFINITY,
    },
    expr::Expr,
    matcher::MatchIter,
    parser::ast::NEG_HEAD,
    pattern::Pattern,
};

pub fn simplify_evaluation_at_zero(expr: Expr) -> Expr {
    let head_pattern = expr! {
        PatternTest[Pattern[h, Blank[]], IsSymbolQ]
    };
    let pattern_expr = Expr::new_node(head_pattern, vec![0.into()]).normalize();

    expr.map_bottom_up(&|e| {
        let pattern = Pattern::from_expr(&pattern_expr);

        if let Some(ctx) = MatchIter::new(&e, pattern).next() {
            let head_symbol = ctx.get_one("h").unwrap().get_symbol().unwrap();
            match head_symbol {
                CANNONICAL_HEAD_EXP => Expr::new_number(1),
                CANNONICAL_HEAD_LOG => Expr::new_node(
                    NEG_HEAD,
                    vec![Expr::new_symbol(CANNONICAL_SYM_PLUS_INFINITY)],
                ),
                CANNONICAL_HEAD_SIN => Expr::new_number(0),
                CANNONICAL_HEAD_COS => Expr::new_number(1),
                CANNONICAL_HEAD_TAN => Expr::new_number(0),
                _ => e,
            }
        } else {
            e
        }
    })
}
