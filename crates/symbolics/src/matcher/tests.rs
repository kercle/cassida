use expr_macro::raw_expr;

use super::*;
use crate::{atom::Atom, parser::ast::ADD_HEAD};

#[test]
fn match_literal_success() {
    assert!(
        Matcher::new(raw_expr! { f[1, 2, 3] })
            .first_match(&raw_expr! { f[1, 2, 3] })
            .is_some()
    );
}

#[test]
fn match_literal_failure() {
    assert!(
        Matcher::new(raw_expr! { f[1, 2, 3] })
            .first_match(&raw_expr! { f[1, 2, 4] })
            .is_none()
    );
}

#[test]
fn match_blank_any_single_arg() {
    assert!(
        Matcher::new(raw_expr! { f[Blank[], 2, 3] })
            .first_match(&raw_expr! { f[x, 2, 3] })
            .is_some()
    );
}

#[test]
fn match_blank_in_middle() {
    assert!(
        Matcher::new(raw_expr! { f[1, Blank[], 3] })
            .first_match(&raw_expr! { f[1, 2, 3] })
            .is_some()
    );
}

#[test]
fn match_blank_requires_same_head() {
    // Pattern expects f[...]
    assert!(
        Matcher::new(raw_expr! { f[Blank[], 2, 3] })
            .first_match(&raw_expr! { g[1, 2, 3] })
            .is_none()
    );
}

#[test]
fn match_nested_blank() {
    assert!(
        Matcher::new(raw_expr! { f[g[Blank[]], 2] })
            .first_match(&raw_expr! { f[g[99], 2] })
            .is_some()
    );
}

#[test]
fn match_named_blank_binds_value() {
    let e = raw_expr! { f[1, 2, 3] };

    let matcher = Matcher::new(raw_expr! { f[Pattern[x, Blank[]], 2, 3] });
    let ctx = matcher.first_match(&e).expect("should match");

    assert_eq!(ctx.get_one("x"), Some(&raw_expr! {1}));
}

#[test]
fn match_repeated_named_blank_must_be_equal_success() {
    // x_ appears twice: must match same expr both times
    assert!(
        Matcher::new(raw_expr! { f[Pattern[x, Blank[]], Pattern[x, Blank[]]] })
            .first_match(&raw_expr! { f[1, 1] })
            .is_some()
    );
}

#[test]
fn match_repeated_named_blank_must_be_equal_failure() {
    assert!(
        Matcher::new(raw_expr! { f[Pattern[x, Blank[]], Pattern[x, Blank[]]] })
            .first_match(&raw_expr! { f[1, 2] })
            .is_none()
    );
}

#[test]
fn match_blankseq_one_or_more_success() {
    assert!(
        Matcher::new(raw_expr! { f[BlankSeq[]] })
            .first_match(&raw_expr! { f[1] })
            .is_some()
    );
    assert!(
        Matcher::new(raw_expr! { f[BlankSeq[]] })
            .first_match(&raw_expr! { f[1, 2, 3] })
            .is_some()
    );
}

#[test]
fn match_blankseq_one_or_more_failure_on_empty() {
    assert!(
        Matcher::new(raw_expr! { f[BlankSeq[]] })
            .first_match(&raw_expr! { f[] })
            .is_none()
    );
}

#[test]
fn match_fixed_then_blankseq_then_fixed_success() {
    assert!(
        Matcher::new(raw_expr! { f[1, BlankSeq[], 4] })
            .first_match(&raw_expr! { f[1, 2, 3, 4] })
            .is_some()
    );
}

#[test]
fn match_fixed_then_blankseq_then_fixed_failure_too_short() {
    assert!(
        Matcher::new(raw_expr! { f[1, BlankSeq[], 4] })
            .first_match(&raw_expr! { f[1, 4] })
            .is_none()
    );
}

#[test]
fn match_two_ordered_blankseq_backtracking_count() {
    // This is the classic backtracking stressor: f[a__, b__] against 4 args
    // Solutions are splits:
    // a={1}, b={2,3,4}
    // a={1,2}, b={3,4}
    // a={1,2,3}, b={4}
    //
    // So expected count = 3
    let expr = raw_expr! { f[1, 2, 3, 4] };
    let matcher = Matcher::new(raw_expr! { f[Pattern[a, BlankSeq[]], Pattern[b, BlankSeq[]]] });
    let mut it = matcher.iter_matches(&expr);

    let count = it.by_ref().count();
    assert_eq!(count, 3);
}

#[test]
fn match_blankseq_with_tail_literal() {
    // f[x__, 4] should match f[1,2,3,4]
    assert!(
        Matcher::new(raw_expr! { f[Pattern[x, BlankSeq[]], 4] })
            .first_match(&raw_expr! { f[1, 2, 3, 4] })
            .is_some()
    );
}

#[test]
fn match_blankseq_with_head_literal() {
    // f[1, x__] should match f[1,2,3]
    assert!(
        Matcher::new(raw_expr! { f[1, Pattern[x, BlankSeq[]]] })
            .first_match(&raw_expr! { f[1, 2, 3] })
            .is_some()
    );
}

#[test]
fn match_blankseq_binding_slice_lengths() {
    let expr = raw_expr! { f[1, 2, 3] };
    let matcher = Matcher::new(raw_expr! { f[Pattern[x, BlankSeq[]]] });
    let ctx = matcher.first_match(&expr).expect("should match");

    assert!(ctx.get_seq("x").is_some());
}

#[test]
fn match_nested_compound_and_sequence() {
    assert!(
        Matcher::new(raw_expr! { f[g[Pattern[x, BlankSeq[]]], 9] })
            .first_match(&raw_expr! { f[g[1, 2, 3], 9] })
            .is_some()
    );
}

#[test]
fn match_head_restricted_blank_success() {
    // If you encode `_g` as Blank[match_head=g] in raw_expr!
    assert!(
        Matcher::new(raw_expr! { f[Blank[g], 2] }) // f[_g,2]
            .first_match(&raw_expr! { f[g[1], 2] })
            .is_some()
    );
}

#[test]
fn match_head_restricted_blank_failure() {
    assert!(
        Matcher::new(raw_expr! { f[Blank[g], 2] }) // f[_g,2]
            .first_match(&raw_expr! { f[h[1], 2] })
            .is_none()
    );
}

#[test]
fn match_compound_head_as_pattern() {
    // Compound { head: Box<Pattern>, args: Vec<Pattern> }
    // If your pattern allows matching the head itself:
    let p = Expr::new_compound(
        Expr::new_blank(),
        vec![Expr::new_number(1), Expr::new_number(2)],
    );
    assert!(
        Matcher::new(p) // _[1,2] matches f[1,2]
            .first_match(&raw_expr! { f[1, 2] })
            .is_some()
    );
}

#[test]
fn match_fail_on_extra_args_ordered_list() {
    // Pattern has 3 args, expr has 4
    assert!(
        Matcher::new(raw_expr! { f[Blank[], 2, 3] })
            .first_match(&raw_expr! { f[1, 2, 3, 4] })
            .is_none()
    );
}

#[test]
fn match_fail_on_missing_args_ordered_list() {
    // Pattern has 3 args, expr has 2
    assert!(
        Matcher::new(raw_expr! { f[Blank[], 2, 3] })
            .first_match(&raw_expr! { f[1, 2] })
            .is_none()
    );
}

#[test]
fn match_two_unordered_blankseq_backtracking_count() {
    // This is the classic backtracking stressor: f[a__, b__] against 4 args
    // Solutions are splits:
    // a={1}, b={2,3,4}
    // a={1,2}, b={3,4}
    // a={1,2,3}, b={4}
    //
    // So expected count = 3
    let expr = raw_expr! { Add[1, 2, 3, 4] };
    let matcher = Matcher::new(raw_expr! { Add[Pattern[a, BlankSeq[]], Pattern[b, BlankSeq[]]] });
    let mut it = matcher.iter_matches(&expr);

    let count = it.by_ref().count();
    assert_eq!(count, 3);
}

#[test]
fn match_blanknullseq_allows_empty_single() {
    // f[___] should match f[]
    let expr = raw_expr! { f[] };
    let matcher = Matcher::new(raw_expr! { f[BlankNullSeq[]] });

    assert!(matcher.first_match(&expr).is_some());
}

#[test]
fn match_blanknullseq_matches_nonempty_single() {
    // f[___] should match f[1,2,3]
    let expr = raw_expr! { f[1, 2, 3] };
    let matcher = Matcher::new(raw_expr! { f[BlankNullSeq[]] });

    assert!(matcher.first_match(&expr).is_some());
}

#[test]
fn match_blanknullseq_prefix_suffix_empty_middle() {
    // f[x_, ___, y_] should match f[1,2] with ___ = {}
    let expr = raw_expr! { f[1, 2] };
    let matcher = Matcher::new(raw_expr! { f[Blank[], BlankNullSeq[], Blank[]] });

    assert!(matcher.first_match(&expr).is_some());
}

#[test]
fn match_blanknullseq_prefix_suffix_nonempty_middle() {
    // f[x_, ___, y_] should match f[1, 9, 8, 2] with ___ = {9,8}
    let expr = raw_expr! { f[1, 9, 8, 2] };
    let matcher = Matcher::new(raw_expr! { f[Blank[], BlankNullSeq[], Blank[]] });

    assert!(matcher.first_match(&expr).is_some());
}

#[test]
fn match_blanknullseq_fails_if_fixed_args_missing() {
    // f[1, ___, 3] should NOT match f[1]
    // because trailing 3 missing
    let expr = raw_expr! { f[1] };
    let matcher = Matcher::new(raw_expr! { f[1, BlankNullSeq[], 3] });

    assert!(matcher.first_match(&expr).is_none());
}

#[test]
fn match_blanknullseq_vs_blankseq_distinguish_empty() {
    // f[__] should NOT match f[]
    let expr = raw_expr! { f[] };
    let matcher = Matcher::new(raw_expr! { f[BlankSeq[]] });

    assert!(matcher.first_match(&expr).is_none());

    // but f[___] SHOULD match f[]
    let matcher2 = Matcher::new(raw_expr! { f[BlankNullSeq[]] });
    assert!(matcher2.first_match(&expr).is_some());
}

#[test]
fn match_two_unordered_blanknullseq_backtracking_count_len4() {
    // f[a___, b___] against 4 args
    // Number of splits = n+1 = 5:
    // k=0..4 for a, rest to b
    let expr = raw_expr! { Add[1, 2, 3, 4] };
    let matcher = Matcher::new(raw_expr! {
        Add[
            Pattern[a, BlankNullSeq[]],
            Pattern[b, BlankNullSeq[]]
        ]
    });

    let count = matcher.iter_matches(&expr).count();
    assert_eq!(count, 5);
}

#[test]
fn match_blankseq_then_blanknullseq_backtracking_count_len4() {
    // f[a__, b___] against 4 args
    // a must be at least 1 => k=1..4 -> 4 solutions
    let expr = raw_expr! { Add[1, 2, 3, 4] };
    let matcher = Matcher::new(raw_expr! {
        Add[
            Pattern[a, BlankSeq[]],
            Pattern[b, BlankNullSeq[]]
        ]
    });

    let count = matcher.iter_matches(&expr).count();
    assert_eq!(count, 4);
}

#[test]
fn match_blanknullseq_then_blankseq_backtracking_count_len4() {
    // f[a___, b__] against 4 args
    // b must be at least 1, so a can take 0..3 => 4 solutions
    let expr = raw_expr! { Add[1, 2, 3, 4] };
    let matcher = Matcher::new(raw_expr! {
        Add[
            Pattern[a, BlankNullSeq[]],
            Pattern[b, BlankSeq[]]
        ]
    });

    let count = matcher.iter_matches(&expr).count();
    assert_eq!(count, 4);
}

#[test]
fn match_three_blanknullseq_count_len2() {
    // f[a___, b___, c___] against 2 args
    // Number of weak compositions of 2 into 3 parts = C(2+3-1, 3-1) = C(4,2)=6
    let expr = raw_expr! { f[1, 2] };
    let matcher = Matcher::new(raw_expr! {
        f[
            Pattern[a, BlankNullSeq[]],
            Pattern[b, BlankNullSeq[]],
            Pattern[c, BlankNullSeq[]]
        ]
    });

    let count = matcher.iter_matches(&expr).count();
    assert_eq!(count, 6);
}

#[test]
fn unordered_two_blanks_count_len3() {
    // Add[a_, b_] against 3 args in unordered mode:
    // no seq to soak up the extra arg => impossible
    let expr = raw_expr! { Add[1, 2, 3] };
    let matcher = Matcher::new(raw_expr! {
        Add[
            Pattern[a, Blank[]],
            Pattern[b, Blank[]]
        ]
    });

    let count = matcher
        .iter_matches(&expr)
        .commutative_if(|head| head.matches_symbol(ADD_HEAD))
        .count();
    assert_eq!(count, 0);
}

#[test]
fn unordered_two_blanks_count_len2() {
    // Add[a_, b_] against 2 args unordered:
    // a can pick either expr, b gets the other => 2 solutions
    let expr = raw_expr! { Add[1, 2] };
    let matcher = Matcher::new(raw_expr! {
        Add[
            Pattern[a, Blank[]],
            Pattern[b, Blank[]]
        ]
    });

    let count = matcher
        .iter_matches(&expr)
        .commutative_if(|head| head.matches_symbol(ADD_HEAD))
        .count();
    assert_eq!(count, 2);
}

#[test]
fn unordered_three_blanks_count_len3() {
    // Add[a_, b_, c_] against 3 args unordered:
    // number of bijections = 3! = 6 solutions
    let expr = raw_expr! { Add[1, 2, 3] };
    let matcher = Matcher::new(raw_expr! {
        Add[
            Pattern[a, Blank[]],
            Pattern[b, Blank[]],
            Pattern[c, Blank[]]
        ]
    });

    let count = matcher
        .iter_matches(&expr)
        .commutative_if(|head| head.matches_symbol(ADD_HEAD))
        .count();
    assert_eq!(count, 6);
}

#[test]
fn unordered_literal_plus_blank_count_len3() {
    // Add[1, a_] against Add[1,2,3] unordered:
    // literal 1 consumed; a_ can bind to either 2 or 3 BUT leftover expr must be empty (no seq)
    // so no solutions.
    let expr = raw_expr! { Add[1, 2, 3] };
    let matcher = Matcher::new(raw_expr! {
        Add[
            1,
            Pattern[a, Blank[]]
        ]
    });

    let count = matcher.iter_matches(&expr).count();
    assert_eq!(count, 0);
}

#[test]
fn unordered_literal_plus_blank_count_len2() {
    // Add[1, a_] against Add[1,2] unordered:
    // literal 1 consumed; a_ must match 2 => 1 solution
    let expr = raw_expr! { Add[1, 2] };
    let matcher = Matcher::new(raw_expr! {
        Add[
            1,
            Pattern[a, Blank[]]
        ]
    });

    let count = matcher
        .iter_matches(&expr)
        .commutative_if(|head| head.matches_symbol(ADD_HEAD))
        .count();
    assert_eq!(count, 1);
}

#[test]
fn unordered_two_equal_literals_multiset_consumption() {
    // Add[1,1,a_] against Add[1,1,2] unordered:
    // literals consume two 1s, a binds to 2 => 1 solution
    let expr = raw_expr! { Add[1, 1, 2] };
    let matcher = Matcher::new(raw_expr! {
        Add[
            1,
            1,
            Pattern[a, Blank[]]
        ]
    });

    let count = matcher
        .iter_matches(&expr)
        .commutative_if(|head| head.matches_symbol(ADD_HEAD))
        .count();
    assert_eq!(count, 1);
}

#[test]
fn unordered_literal_fails_if_not_enough_occurrences() {
    // Add[1,1,a_] against Add[1,2,3] unordered:
    // only one '1' exists => 0 solutions
    let expr = raw_expr! { Add[1, 2, 3] };
    let matcher = Matcher::new(raw_expr! {
        Add[
            1,
            1,
            Pattern[a, Blank[]]
        ]
    });

    let count = matcher
        .iter_matches(&expr)
        .commutative_if(|head| head.matches_symbol(ADD_HEAD))
        .count();
    assert_eq!(count, 0);
}

#[test]
fn unordered_blankseq_soaks_up_remainder_len3() {
    // Add[a_, b__] against 3 args unordered.
    // Match a_ to one expr (3 choices),
    // then b__ binds to all remaining (2 exprs) => 3 solutions.
    let expr = raw_expr! { Add[1, 2, 3] };
    let matcher = Matcher::new(raw_expr! {
        Add[
            Pattern[a, Blank[]],
            Pattern[b, BlankSeq[]]
        ]
    });

    let count = matcher
        .iter_matches(&expr)
        .commutative_if(|head| head.matches_symbol(ADD_HEAD))
        .count();
    assert_eq!(count, 3);
}

#[test]
fn unordered_blanknullseq_can_be_empty_len1() {
    // Add[a___] against Add[1] unordered:
    // a___ binds to [1] => 1 solution
    let expr = raw_expr! { Add[1] };
    let matcher = Matcher::new(raw_expr! {
        Add[
            Pattern[a, BlankNullSeq[]]
        ]
    });

    let count = matcher
        .iter_matches(&expr)
        .commutative_if(|head| head.matches_symbol(ADD_HEAD))
        .count();
    assert_eq!(count, 1);
}

#[test]
fn unordered_blankseq_requires_nonempty_len0() {
    // Add[a__] against Add[] unordered:
    // BlankSeq requires >=1 => 0 solutions
    let expr = raw_expr! { Add[] };
    let matcher = Matcher::new(raw_expr! {
        Add[
            Pattern[a, BlankSeq[]]
        ]
    });

    let count = matcher
        .iter_matches(&expr)
        .commutative_if(|head| head.matches_symbol(ADD_HEAD))
        .count();
    assert_eq!(count, 0);
}

#[test]
fn unordered_literal_plus_blankseq_len4() {
    // Add[1, xs__] against Add[1,2,3,4] unordered:
    // literal 1 consumed; xs__ binds to [2,3,4] => 1 solution
    let expr = raw_expr! { Add[1, 2, 3, 4] };
    let matcher = Matcher::new(raw_expr! {
        Add[
            1,
            Pattern[xs, BlankSeq[]]
        ]
    });

    let count = matcher
        .iter_matches(&expr)
        .commutative_if(|head| head.matches_symbol(ADD_HEAD))
        .count();
    assert_eq!(count, 1);
}

#[test]
fn unordered_two_blanks_plus_blankseq_len4() {
    // Add[a_, b_, xs__] against 4 args unordered:
    // pick a: 4 choices, pick b: 3 choices => 12,
    // xs__ binds to remaining 2 => 12 solutions
    let expr = raw_expr! { Add[1, 2, 3, 4] };
    let matcher = Matcher::new(raw_expr! {
        Add[
            Pattern[a, Blank[]],
            Pattern[b, Blank[]],
            Pattern[xs, BlankSeq[]]
        ]
    });

    let count = matcher
        .iter_matches(&expr)
        .commutative_if(|head| head.matches_symbol(ADD_HEAD))
        .count();
    assert_eq!(count, 12);
}

#[test]
fn unordered_two_blanks_plus_blankseq_len7() {
    // Add[a_, b_, xs__] against 4 args unordered:
    // pick a: 4 choices, pick b: 3 choices => 12,
    // xs__ binds to remaining 2 => 12 solutions
    let expr = raw_expr! {
        Add[x, Cos[phi / 8]^2, y, Sin[phi / 8]^2, 1, 2, Exp[Log[x+1]]]
    };
    let matcher = Matcher::new(raw_expr! {
        Add[
            Cos[Pattern[a, Blank[]]]^2,
            Sin[Pattern[b, Blank[]]]^2,
            Pattern[xs, BlankSeq[]]
        ]
    })
    .commutative_if(|head| head.matches_symbol(ADD_HEAD));

    let ctx = matcher.first_match(&expr);
    assert!(ctx.is_some());

    dbg!(ctx);
}
