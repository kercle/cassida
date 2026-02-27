use crate::atom::Atom;
use crate::expr::Expr;
use crate::pattern::program::{ArgOrder, Compiler};
use expr_macro::expr;

#[test]
fn test_program_compilation() {
    let pattern = expr! {
        Pattern[y, f[Pattern[x, BlankSeq[]]]]+g[Blank[]]
    };

    let program = Compiler::new(|_| ArgOrder::Sequence).compile(&pattern);

    dbg!(program);
}
