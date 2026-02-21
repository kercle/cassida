use expr_macro::raw_expr;

use crate::{
    builtin::{
        CANNONICAL_HEAD_COS, CANNONICAL_HEAD_EXP, CANNONICAL_HEAD_LOG, CANNONICAL_HEAD_SIN,
        CANNONICAL_HEAD_TAN, CANNONICAL_SYM_PLUS_INFINITY,
    },
    expr::{Expr, atom::Atom, matcher::MatchIter, pattern::Pattern},
    parser::ast::NEG_HEAD,
};

pub fn simplify_evaluation_at_zero(expr: Expr) -> Expr {
    let head_pattern = raw_expr! {
        PatternTest[Pattern[h, Blank[]], IsSymbolQ]
    };
    let pattern_expr = Expr::new_compound(head_pattern, vec![0.into()]).normalize();
    let pattern = Pattern::from_expr(&pattern_expr);

    expr.map_bottom_up(&|e| {
        if let Some(ctx) = MatchIter::new(&e, &pattern).next() {
            let head_symbol = ctx.get("h").unwrap().get_symbol().unwrap();
            match head_symbol {
                CANNONICAL_HEAD_EXP => Expr::new_number_one(),
                CANNONICAL_HEAD_LOG => Expr::new_compound(
                    NEG_HEAD,
                    vec![Expr::new_symbol(CANNONICAL_SYM_PLUS_INFINITY)],
                ),
                CANNONICAL_HEAD_SIN => Expr::new_number_zero(),
                CANNONICAL_HEAD_COS => Expr::new_number_one(),
                CANNONICAL_HEAD_TAN => Expr::new_number_zero(),
                _ => e,
            }
        } else {
            e
        }
    })
}
