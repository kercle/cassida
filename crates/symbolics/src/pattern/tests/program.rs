use crate::norm_expr;
use crate::pattern::program::Compiler;
use crate::pattern::runtime::Runtime;

#[test]
fn test_program_compilation() {
    let pattern = norm_expr! {
        Pattern[y, f[Pattern[x, BlankSeq[]]]]+g[Blank[]]
    };

    let program = Compiler::new().compile(&pattern);

    dbg!(program);
}

#[test]
fn test_program_executation_no_blanks_no_multiset() {
    let pattern = norm_expr! {
        f[1, g[Pattern[x, a], b]]
    };

    let program = Compiler::new().compile(&pattern);

    let subject = norm_expr! {
        f[1, g[a, b]]
    };
    let mut runtime = Runtime::new(&program, &subject);
    dbg!(runtime.next_match());
}

#[test]
fn test_program_executation_one_blank_no_multiset() {
    let pattern = norm_expr! {
        f[g[Pattern[x, Blank[]], b], Pattern[y, BlankSeq[]], Pattern[z, Blank[]]]
    };

    let program = Compiler::new().compile(&pattern);
    dbg!(&program);

    let subject = norm_expr! {
        f[g[h[2, 4, 8], b], 10, x, 5]
    };
    let mut runtime = Runtime::new(&program, &subject);
    dbg!(runtime.next_match());
}

#[test]
fn test_predicate_matching() {
    let pattern = norm_expr! { f[x_?IsNumber] };
    let program = Compiler::new().compile(&pattern);

    dbg!(&program);

    let subject = norm_expr! { f[42] };
    let mut runtime = Runtime::new(&program, &subject);
    let m = runtime.next_match();

    dbg!(&m);

    assert!(m.is_some());
}

#[test]
fn test_multiple_blank_sequences() {
    let pattern = norm_expr! { f[Pattern[x, BlankSeq[]], Blank[], Pattern[z, BlankNullSeq[]], Pattern[w, BlankNullSeq[]]] };
    let program = Compiler::new().compile(&pattern);

    let subject = norm_expr! { f[a,b,c,d,e] };
    let mut runtime = Runtime::new(&program, &subject);

    let mut counter = 0;
    while runtime.next_match().is_some() {
        counter += 1;
    }

    assert_eq!(
        counter, 10,
        "f[x__,_,z___,w___] matched against f[a,b,c,d,e] should produce 10 pattern."
    );
}

#[test]
fn test_variadic_empty_instrs_nonempty_subjects() {
    let pattern = norm_expr! { f[] };
    let program = Compiler::new().compile(&pattern);
    let subject = norm_expr! { f[a, b] };
    let mut runtime = Runtime::new(&program, &subject);
    assert!(
        runtime.next_match().is_none(),
        "f[] should not match f[a,b]"
    );
}

#[test]
fn test_no_spurious_match_after_length_mismatch() {
    let pattern = norm_expr! { f[a, Pattern[x, BlankNullSeq[]], b, c] };
    let program = Compiler::new().compile(&pattern);
    let subject = norm_expr! { f[a, b] };
    let mut runtime = Runtime::new(&program, &subject);
    assert!(
        runtime.next_match().is_none(),
        "Should not match when back literals cannot be satisfied"
    );
}

#[test]
fn test_confirming_rebind_not_wiped_on_backtrack() {
    let pattern =
        norm_expr! { f[Pattern[x, Blank[]], Pattern[x, Blank[]], Pattern[y, BlankNullSeq[]]] };
    let program = Compiler::new().compile(&pattern);
    let subject = norm_expr! { f[a, a, b] };
    let mut runtime = Runtime::new(&program, &subject);

    let env = runtime.next_match().expect("should match");
    assert_eq!(env.get_one("x"), Some(&norm_expr! { a }));

    assert!(runtime.next_match().is_none());
}

#[test]
fn test_simple_multiset_matching_only_literals() {
    let pattern = norm_expr! { Add[4, 2, 3, 1] };
    let program = Compiler::new().compile(&pattern);
    let subject = norm_expr! {
        Add[1, 2, 3, 4]
    };
    dbg!(&program);
    let mut runtime = Runtime::new(&program, &subject);

    runtime.next_match().expect("should match");
}

#[test]
fn test_simple_multiset_matching() {
    let pattern = norm_expr! { Add[g[a], b, Pattern[x, Blank[]], Pattern[y, Blank[]], Pattern[z, BlankSeq[]]] };

    let program = Compiler::new().compile(&pattern);
    let subject = norm_expr! {
        Add[c,d,g[a],e,f,b]
    };

    let mut runtime = Runtime::new(&program, &subject);

    runtime.next_match().expect("should match");
}
