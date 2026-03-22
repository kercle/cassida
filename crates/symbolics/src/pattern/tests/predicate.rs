// ------------------------------------------------------------
// PREDICATE TESTS
// ------------------------------------------------------------
//
//  Pattern                     | Test Expr | Expected Matches
//  ----------------------------|-----------|------------------
//  _?IsNumber                  | 5         | 1
//  _?IsNumber                  | foo       | 0
//  _?IsNumber                  | f[1]      | 0
//  _?IsSymbol                  | foo       | 1
//  _?IsSymbol                  | 5         | 0
//  f[_?IsNumber]               | f[3]      | 1
//  f[_?IsNumber]               | f[x]      | 0
//  f[_?IsNumber, _?IsSymbol]   | f[1, x]   | 1
//  f[_?IsNumber, _?IsSymbol]   | f[x, 1]   | 0
//  f[x_?IsNumber]              | f[3]      | 1
//  f[x_?IsNumber]              | f[foo]    | 0
//  Add[_?IsNumber, _?IsSymbol] | Add[x, 1] | 1
//  Add[_?IsNumber, _?IsSymbol] | Add[1, 2] | 0

use crate::norm_expr;
use crate::pattern::tests::utils::count_matches;

#[test]
fn test_predicate_1() {
    let pattern = norm_expr! { _?IsNumber };
    let subject = norm_expr! { 5 };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_2() {
    let pattern = norm_expr! { _?IsNumber };
    let subject = norm_expr! { foo };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_3() {
    let pattern = norm_expr! { _?IsNumber };
    let subject = norm_expr! { f[1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_4() {
    let pattern = norm_expr! { _?IsSymbol };
    let subject = norm_expr! { foo };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_5() {
    let pattern = norm_expr! { _?IsSymbol };
    let subject = norm_expr! { 5 };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_6() {
    let pattern = norm_expr! { f[_?IsNumber] };
    let subject = norm_expr! { f[3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_7() {
    let pattern = norm_expr! { f[_?IsNumber] };
    let subject = norm_expr! { f[x] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_8() {
    let pattern = norm_expr! { f[_?IsNumber, _?IsSymbol] };
    let subject = norm_expr! { f[1, x] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_9() {
    let pattern = norm_expr! { f[_?IsNumber, _?IsSymbol] };
    let subject = norm_expr! { f[x, 1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_10() {
    let pattern = norm_expr! { f[x_?IsNumber] };
    let subject = norm_expr! { f[3] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_11() {
    let pattern = norm_expr! { f[x_?IsNumber] };
    let subject = norm_expr! { f[foo] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_12() {
    let pattern =
        norm_expr! { Add[_?IsNumber, _?IsSymbol] };
    let subject = norm_expr! { Add[x, 1] };
    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_predicate_13() {
    let pattern =
        norm_expr! { Add[_?IsNumber, _?IsSymbol] };
    let subject = norm_expr! { Add[1, 2] };
    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}
