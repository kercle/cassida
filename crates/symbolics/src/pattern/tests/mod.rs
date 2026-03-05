mod literals;
mod mixed;
mod multiset;
mod predicate;
mod program;
mod sequence;
mod wildcards;

use expr_macro::expr;

use super::*;
use crate::{
    atom::Atom,
    expr::{Expr, generator::*},
};

#[test]
fn test_dbg() {
    let e = blank_sequence(None);
    dbg!(&e);
    let p = Pattern::from_expr(&e);
    dbg!(&p);
}

#[test]
fn test_built_pattern_from_expr() {
    let expr = expr! {
        PatternTest[Blank[], IsSymbolQ]
    };
    let pattern = Pattern::from_expr(&expr);
    assert_eq!(
        format!("{pattern:?}"),
        r#"Blank{None, None, Some(IsSymbolQ)}"#
    );

    let expr = expr! {
        PatternTest[Pattern[x, Blank[]], IsSymbolQ]
    };
    let pattern = Pattern::from_expr(&expr);

    assert_eq!(
        format!("{pattern:?}"),
        r#"Blank{Some("x"), None, Some(IsSymbolQ)}"#
    );
}
