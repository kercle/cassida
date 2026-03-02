use crate::atom::Atom;
use crate::expr::Expr;
use crate::pattern::program::{ArgOrder, Compiler};
use crate::pattern::runtime::Runtime;
use expr_macro::expr;

#[test]
fn test_program_compilation() {
    let pattern = expr! {
        Pattern[y, f[Pattern[x, BlankSeq[]]]]+g[Blank[]]
    };

    let program = Compiler::new(|_| ArgOrder::Sequence).compile(&pattern);

    dbg!(program);
}

#[test]
fn test_program_executation_no_blanks_no_multiset() {
    let pattern = expr! {
        f[1, g[Pattern[x, a], b]]
    };

    let program = Compiler::new(|_| ArgOrder::Sequence).compile(&pattern);

    let subject = expr! {
        f[1, g[a, b]]
    };
    let mut runtime = Runtime::new(&program, &subject);
    dbg!(runtime.next_match());
}

#[test]
fn test_program_executation_one_blank_no_multiset() {
    let pattern = expr! {
        f[g[Pattern[x, Blank[]], b], Pattern[y, BlankSeq[]], Pattern[z, Blank[]]]
    };

    let program = Compiler::new(|_| ArgOrder::Sequence).compile(&pattern);
    dbg!(&program);

    let subject = expr! {
        f[g[h[2, 4, 8], b], 10, x, 5]
    };
    let mut runtime = Runtime::new(&program, &subject);
    dbg!(runtime.next_match());
}

#[test]
fn test_predicate_matching() {
    let pattern = expr! { f[PatternTest[Pattern[x, Blank[]], IsNumberQ]] };
    let program = Compiler::new(|_| ArgOrder::Sequence).compile(&pattern);

    dbg!(&program);

    let subject = expr! { f[42] };
    let mut runtime = Runtime::new(&program, &subject);
    let m = runtime.next_match();

    dbg!(&m);

    assert!(m.is_some());
}
