// --------------------------------------------------------------
// CONDITION TESTS
// --------------------------------------------------------------
//
//  Pattern             | Test Expr           | Expected Matches
//  --------------------|---------------------|------------------
//  1 + a_ /; True      | 1 + x               | 1
//  1 + a_ /; False     | 1 + x               | 0

use crate::norm_expr;
use crate::pattern::tests::utils::count_matches;

#[test]
fn test_pattern_condition_1() {
    let pattern = norm_expr! { 1 + a_ /; True };
    let subject = norm_expr! { 1 + x };

    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_pattern_condition_2() {
    let pattern = norm_expr! { 1 + a_ /; False };
    let subject = norm_expr! { 1 + x };

    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_pattern_condition_3() {
    let pattern = norm_expr! { 1 + a_ /; FreeOf[a, x] };
    let subject = norm_expr! { 1 + x };

    assert_eq!(
        count_matches(&pattern, &subject),
        0,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_pattern_condition_4() {
    let pattern = norm_expr! { 1 + a_ /; FreeOf[a, x] };
    let subject = norm_expr! { 1 + u };

    assert_eq!(
        count_matches(&pattern, &subject),
        1,
        "Number of found matched patterns unexpected"
    );
}
