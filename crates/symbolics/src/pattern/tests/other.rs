use expr_macro::expr;

use crate::{
    atom::Atom,
    expr::Expr,
    pattern::{
        program::Compiler,
        runtime::Runtime,
        tests::utils::{count_matches, first_match},
    },
};

// ---- Literal Tests ----

#[test]
fn match_literal_success() {
    let program = Compiler::new().compile(&expr! { f[1, 2, 3] });
    assert!(first_match(&program, &expr! { f[1, 2, 3] }).is_some());
}

#[test]
fn match_literal_failure() {
    let program = Compiler::new().compile(&expr! { f[1, 2, 3] });
    assert!(first_match(&program, &expr! { f[1, 2, 4] }).is_none());
}

// ---- Blank Tests ----

#[test]
fn match_blank_any_single_arg() {
    let program = Compiler::new().compile(&expr! { f[Blank[], 2, 3] });
    assert!(first_match(&program, &expr! { f[x, 2, 3] }).is_some());
}

#[test]
fn match_blank_in_middle() {
    let program = Compiler::new().compile(&expr! { f[1, Blank[], 3] });
    assert!(first_match(&program, &expr! { f[1, 2, 3] }).is_some());
}

#[test]
fn match_blank_requires_same_head() {
    let program = Compiler::new().compile(&expr! { f[Blank[], 2, 3] });
    assert!(first_match(&program, &expr! { g[1, 2, 3] }).is_none());
}

#[test]
fn match_nested_blank() {
    let program = Compiler::new().compile(&expr! { f[g[Blank[]], 2] });
    assert!(first_match(&program, &expr! { f[g[99], 2] }).is_some());
}

#[test]
fn match_named_blank_binds_value() {
    let program = Compiler::new().compile(&expr! { f[Pattern[x, Blank[]], 2, 3] });
    let subject = expr! { f[1, 2, 3] };
    let ctx = first_match(&program, &subject).expect("should match");
    assert_eq!(ctx.get_one("x"), Some(&expr! { 1 }));
}

#[test]
fn match_repeated_named_blank_must_be_equal_success() {
    let program = Compiler::new().compile(&expr! { f[Pattern[x, Blank[]], Pattern[x, Blank[]]] });
    assert!(first_match(&program, &expr! { f[1, 1] }).is_some());
}

#[test]
fn match_repeated_named_blank_must_be_equal_failure() {
    let program = Compiler::new().compile(&expr! { f[Pattern[x, Blank[]], Pattern[x, Blank[]]] });
    assert!(first_match(&program, &expr! { f[1, 2] }).is_none());
}

// ---- BlankSeq Tests ----

#[test]
fn match_blankseq_one_or_more_success() {
    let program = Compiler::new().compile(&expr! { f[BlankSeq[]] });
    assert!(first_match(&program, &expr! { f[1] }).is_some());
    assert!(first_match(&program, &expr! { f[1, 2, 3] }).is_some());
}

#[test]
fn match_blankseq_one_or_more_failure_on_empty() {
    let program = Compiler::new().compile(&expr! { f[BlankSeq[]] });
    assert!(first_match(&program, &expr! { f[] }).is_none());
}

#[test]
fn match_fixed_then_blankseq_then_fixed_success() {
    let program = Compiler::new().compile(&expr! { f[1, BlankSeq[], 4] });
    assert!(first_match(&program, &expr! { f[1, 2, 3, 4] }).is_some());
}

#[test]
fn match_fixed_then_blankseq_then_fixed_failure_too_short() {
    let program = Compiler::new().compile(&expr! { f[1, BlankSeq[], 4] });
    assert!(first_match(&program, &expr! { f[1, 4] }).is_none());
}

#[test]
fn match_two_ordered_blankseq_backtracking_count() {
    // f[a__, b__] against 4 args
    // splits: a={1}, b={2,3,4} | a={1,2}, b={3,4} | a={1,2,3}, b={4} => 3
    assert_eq!(
        count_matches(
            &expr! { f[Pattern[a, BlankSeq[]], Pattern[b, BlankSeq[]]] },
            &expr! { f[1, 2, 3, 4] }
        ),
        3
    );
}

#[test]
fn match_blankseq_with_tail_literal() {
    let program = Compiler::new().compile(&expr! { f[Pattern[x, BlankSeq[]], 4] });
    assert!(first_match(&program, &expr! { f[1, 2, 3, 4] }).is_some());
}

#[test]
fn match_blankseq_with_head_literal() {
    let program = Compiler::new().compile(&expr! { f[1, Pattern[x, BlankSeq[]]] });
    assert!(first_match(&program, &expr! { f[1, 2, 3] }).is_some());
}

#[test]
fn match_blankseq_binding_slice_lengths() {
    let program = Compiler::new().compile(&expr! { f[Pattern[x, BlankSeq[]]] });
    let subject = expr! { f[1, 2, 3] };
    let ctx = first_match(&program, &subject).expect("should match");
    assert!(ctx.get_seq("x").is_some());
}

#[test]
fn match_nested_node_and_sequence() {
    let program = Compiler::new().compile(&expr! { f[g[Pattern[x, BlankSeq[]]], 9] });
    assert!(first_match(&program, &expr! { f[g[1, 2, 3], 9] }).is_some());
}

// ---- Head Constraint Tests ----

#[test]
fn match_head_restricted_blank_success() {
    let program = Compiler::new().compile(&expr! { f[Blank[g], 2] });
    assert!(first_match(&program, &expr! { f[g[1], 2] }).is_some());
}

#[test]
fn match_head_restricted_blank_failure() {
    let program = Compiler::new().compile(&expr! { f[Blank[g], 2] });
    assert!(first_match(&program, &expr! { f[h[1], 2] }).is_none());
}

// ---- Arg Count Tests ----

#[test]
fn match_fail_on_extra_args_ordered_list() {
    let program = Compiler::new().compile(&expr! { f[Blank[], 2, 3] });
    assert!(first_match(&program, &expr! { f[1, 2, 3, 4] }).is_none());
}

#[test]
fn match_fail_on_missing_args_ordered_list() {
    let program = Compiler::new().compile(&expr! { f[Blank[], 2, 3] });
    assert!(first_match(&program, &expr! { f[1, 2] }).is_none());
}

// ---- BlankNullSeq Tests ----

#[test]
fn match_blanknullseq_allows_empty_single() {
    let program = Compiler::new().compile(&expr! { f[BlankNullSeq[]] });
    assert!(first_match(&program, &expr! { f[] }).is_some());
}

#[test]
fn match_blanknullseq_matches_nonempty_single() {
    let program = Compiler::new().compile(&expr! { f[BlankNullSeq[]] });
    assert!(first_match(&program, &expr! { f[1, 2, 3] }).is_some());
}

#[test]
fn match_blanknullseq_prefix_suffix_empty_middle() {
    let program = Compiler::new().compile(&expr! { f[Blank[], BlankNullSeq[], Blank[]] });
    assert!(first_match(&program, &expr! { f[1, 2] }).is_some());
}

#[test]
fn match_blanknullseq_prefix_suffix_nonempty_middle() {
    let program = Compiler::new().compile(&expr! { f[Blank[], BlankNullSeq[], Blank[]] });
    assert!(first_match(&program, &expr! { f[1, 9, 8, 2] }).is_some());
}

#[test]
fn match_blanknullseq_fails_if_fixed_args_missing() {
    let program = Compiler::new().compile(&expr! { f[1, BlankNullSeq[], 3] });
    assert!(first_match(&program, &expr! { f[1] }).is_none());
}

#[test]
fn match_blanknullseq_vs_blankseq_distinguish_empty() {
    let program_seq = Compiler::new().compile(&expr! { f[BlankSeq[]] });
    assert!(first_match(&program_seq, &expr! { f[] }).is_none());

    let program_null_seq = Compiler::new().compile(&expr! { f[BlankNullSeq[]] });
    assert!(first_match(&program_null_seq, &expr! { f[] }).is_some());
}

#[test]
fn match_two_unordered_blanknullseq_backtracking_count_len4() {
    // f[a___, b___] against 4 args => n+1 = 5 splits
    assert_eq!(
        count_matches(
            &expr! { f[Pattern[a, BlankNullSeq[]], Pattern[b, BlankNullSeq[]]] },
            &expr! { f[1, 2, 3, 4] }
        ),
        5
    );
}

#[test]
fn match_blankseq_then_blanknullseq_backtracking_count_len4() {
    // f[a__, b___] against 4 args => a takes 1..4 => 4 solutions
    assert_eq!(
        count_matches(
            &expr! { f[Pattern[a, BlankSeq[]], Pattern[b, BlankNullSeq[]]] },
            &expr! { f[1, 2, 3, 4] }
        ),
        4
    );
}

#[test]
fn match_blanknullseq_then_blankseq_backtracking_count_len4() {
    // Add[a___, b__] against 4 args => b takes 1..4 => 4 solutions
    assert_eq!(
        count_matches(
            &expr! { f[Pattern[a, BlankNullSeq[]], Pattern[b, BlankSeq[]]] },
            &expr! { f[1, 2, 3, 4] }
        ),
        4
    );
}

#[test]
fn match_three_blanknullseq_count_len2() {
    // f[a___, b___, c___] against 2 args => C(4,2) = 6
    assert_eq!(
        count_matches(
            &expr! { f[Pattern[a, BlankNullSeq[]], Pattern[b, BlankNullSeq[]], Pattern[c, BlankNullSeq[]]] },
            &expr! { f[1, 2] }
        ),
        6
    );
}

// ---- Multiset / Unordered Tests ----

#[test]
fn unordered_two_blanks_count_len3() {
    // Add[a_, b_] against 3 args => no seq to absorb extra => 0
    assert_eq!(
        count_matches(
            &expr! { Add[Pattern[a, Blank[]], Pattern[b, Blank[]]] },
            &expr! { Add[1, 2, 3] }
        ),
        0
    );
}

#[test]
fn unordered_two_blanks_count_len2() {
    // Add[a_, b_] against 2 args unordered => 2 solutions
    assert_eq!(
        count_matches(
            &expr! { Add[Pattern[a, Blank[]], Pattern[b, Blank[]]] },
            &expr! { Add[1, 2] }
        ),
        2
    );
}

#[test]
fn unordered_three_blanks_count_len3() {
    // Add[a_, b_, c_] against 3 args => 3! = 6 solutions
    assert_eq!(
        count_matches(
            &expr! { Add[Pattern[a, Blank[]], Pattern[b, Blank[]], Pattern[c, Blank[]]] },
            &expr! { Add[1, 2, 3] }
        ),
        6
    );
}

#[test]
fn unordered_literal_plus_blank_count_len3() {
    // Add[1, a_] against Add[1,2,3] => no seq for leftover => 0
    assert_eq!(
        count_matches(
            &expr! { Add[1, Pattern[a, Blank[]]] },
            &expr! { Add[1, 2, 3] }
        ),
        0
    );
}

#[test]
fn unordered_literal_plus_blank_count_len2() {
    // Add[1, a_] against Add[1,2] => 1 solution
    assert_eq!(
        count_matches(&expr! { Add[1, Pattern[a, Blank[]]] }, &expr! { Add[1, 2] }),
        1
    );
}

#[test]
fn unordered_two_equal_literals_multiset_consumption() {
    // Add[1, 1, a_] against Add[1,1,2] => 1 solution
    assert_eq!(
        count_matches(
            &expr! { Add[1, 1, Pattern[a, Blank[]]] },
            &expr! { Add[1, 1, 2] }
        ),
        1
    );
}

#[test]
fn unordered_literal_fails_if_not_enough_occurrences() {
    // Add[1, 1, a_] against Add[1,2,3] => only one 1 => 0
    assert_eq!(
        count_matches(
            &expr! { Add[1, 1, Pattern[a, Blank[]]] },
            &expr! { Add[1, 2, 3] }
        ),
        0
    );
}

#[test]
fn unordered_blankseq_soaks_up_remainder_len3() {
    // Add[a_, b__] against 3 args => pick a: 3 choices, b gets rest => 3
    assert_eq!(
        count_matches(
            &expr! { Add[Pattern[a, Blank[]], Pattern[b, BlankSeq[]]] },
            &expr! { Add[1, 2, 3] }
        ),
        3
    );
}

#[test]
fn unordered_blanknullseq_can_be_empty_len1() {
    // Add[a___] against Add[1] => 1 solution
    assert_eq!(
        count_matches(
            &expr! { Add[Pattern[a, BlankNullSeq[]]] },
            &expr! { Add[1] }
        ),
        1
    );
}

#[test]
fn unordered_blankseq_requires_nonempty_len0() {
    // Add[a__] against Add[] => 0
    assert_eq!(
        count_matches(&expr! { Add[Pattern[a, BlankSeq[]]] }, &expr! { Add[] }),
        0
    );
}

#[test]
fn unordered_literal_plus_blankseq_len4() {
    // Add[1, xs__] against Add[1,2,3,4] => 1 solution
    assert_eq!(
        count_matches(
            &expr! { f[1, Pattern[xs, BlankSeq[]]] },
            &expr! { f[1, 2, 3, 4] }
        ),
        1
    );
}

#[test]
fn unordered_two_blanks_plus_blankseq_len4() {
    // Add[a_, b_, xs__] against 4 args => pick a: 4, pick b: 3, xs gets rest => 12
    assert_eq!(
        count_matches(
            &expr! { Add[Pattern[a, Blank[]], Pattern[b, Blank[]], Pattern[xs, BlankSeq[]]] },
            &expr! { Add[1, 2, 3, 4] }
        ),
        12
    );
}

#[test]
fn unordered_two_blanks_plus_blankseq_len7() {
    let program = Compiler::new().compile(&expr! {
        Add[
            Cos[Pattern[a, Blank[]]]^2,
            Sin[Pattern[a, Blank[]]]^2,
            Pattern[rest, BlankSeq[]]
        ]
    });
    let subject = expr! {
        Add[x, Cos[phi / 8]^2, y, Sin[phi / 8]^2, 1, 2, Exp[Log[x+1]]]
    };

    let ctx = Runtime::new(&program, &subject).next();
    assert!(ctx.is_some());

    let ctx = ctx.unwrap();
    dbg!(&ctx);
    assert_eq!(ctx.get_one("a"), Some(&expr! { phi / 8 }));
    assert!(ctx.get_seq("rest").is_some());
}

#[test]
fn unordered_two_blanks_plus_blanknullseq() {
    let program = Compiler::new().compile(&expr! {
        Add[
            Cos[Pattern[a, Blank[]]]^2,
            Sin[Pattern[a, Blank[]]]^2,
            Pattern[rest, BlankNullSeq[]]
        ]
    });
    let subject = expr! { Add[Cos[phi / 8]^2, Sin[phi / 8]^2] };

    let ctx = Runtime::new(&program, &subject).next();
    assert!(ctx.is_some());
}

#[test]
fn unordered_factorize_pattern_without_rest() {
    let program = Compiler::new().compile(&expr! {
        Add[
            Mul[Pattern[z, Blank[]], Pattern[x, Blank[]]],
            Mul[Pattern[z, Blank[]], Pattern[y, Blank[]]]
        ]
    });
    let subject = expr! { Add[Mul[a, Add[1, x]], Mul[Add[1, x], b]] };

    let ctx = Runtime::new(&program, &subject).next().unwrap();
    dbg!(&ctx);
    assert_eq!(ctx.get_one("x"), Some(&expr!(a)));
    assert_eq!(ctx.get_one("y"), Some(&expr!(b)));
    assert_eq!(ctx.get_one("z"), Some(&expr!(Add[1, x])));
}

#[test]
fn unordered_factorize_pattern_with_rest() {
    let program = Compiler::new().compile(&expr! {
        Add[
            Pattern[r, BlankNullSeq[]],
            Mul[Pattern[z, Blank[]], Pattern[x, Blank[]]],
            Mul[Pattern[z, Blank[]], Pattern[y, Blank[]]]
        ]
    });
    let subject = expr! { Add[Mul[a, Add[1, x]], Mul[b, Add[1, x]]] };

    let ctx = Runtime::new(&program, &subject).next().unwrap();
    assert_eq!(ctx.get_one("x"), Some(&expr!(a)));
    assert_eq!(ctx.get_one("y"), Some(&expr!(b)));
    assert_eq!(ctx.get_one("z"), Some(&expr!(Add[1, x])));
    assert_eq!(ctx.get_seq("r"), Some([].as_slice()));
}
