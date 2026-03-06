// --------------------------------------------------------------
// LITERAL TESTS
// --------------------------------------------------------------
//
//  Pattern             | Test Expr           | Expected Matches
//  --------------------|---------------------|------------------
//  5                   | 5                   | 1
//  5                   | 6                   | 0
//  foo                 | foo                 | 0
//  f[1, 2, 3]          | f[1, 2, 3]          | 1
//  f[1, 2, 3]          | f[1, 2, 4]          | 0
//  f[1, 2, 3]          | f[1, 2]             | 0
//  f[1, 2, 3]          | f[3, 2, 1]          | 0
//  f[g[1, 2], h[3, 4]] | f[g[1, 2], h[3, 4]] | 1
//  f[g[1, 2], h[3, 4]] | f[g[1, 2], h[3, 5]] | 0
//  f[g[h[1]]]          | f[g[h[1]]]          | 1
//  f[g[h[1]]]          | f[g[h[2]]]          | 0
//  f[]                 | f[]                 | 1
//  f[]                 | f[1]                | 0
//  f[]                 | g[]                 | 0

use crate::atom::Atom;
use crate::expr::Expr;
use crate::pattern::tests::utils::count_matches;
use expr_macro::expr;

#[test]
fn test_pattern_literal_1() {
    let pattern = expr! { 5 };
    let subject = expr! { 5 };
    let expected_count = 1usize;

    assert_eq!(
        count_matches(&pattern, &subject),
        expected_count,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_pattern_literal_2() {
    let pattern = expr! { 5 };
    let subject = expr! { 6 };
    let expected_count = 0usize;

    assert_eq!(
        count_matches(&pattern, &subject),
        expected_count,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_pattern_literal_3() {
    let pattern = expr! { foo };
    let subject = expr! { foo };
    let expected_count = 1usize;

    assert_eq!(
        count_matches(&pattern, &subject),
        expected_count,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_pattern_literal_4() {
    let pattern = expr! { f[1, 2, 3] };
    let subject = expr! { f[1, 2, 3] };
    let expected_count = 1usize;

    assert_eq!(
        count_matches(&pattern, &subject),
        expected_count,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_pattern_literal_5() {
    let pattern = expr! { f[1, 2, 3] };
    let subject = expr! { f[1, 2, 4] };
    let expected_count = 0usize;

    assert_eq!(
        count_matches(&pattern, &subject),
        expected_count,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_pattern_literal_6() {
    let pattern = expr! { f[1, 2, 3] };
    let subject = expr! { f[1, 2] };
    let expected_count = 0usize;

    assert_eq!(
        count_matches(&pattern, &subject),
        expected_count,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_pattern_literal_7() {
    let pattern = expr! { f[1, 2, 3] };
    let subject = expr! { f[3, 2, 1] };
    let expected_count = 0usize;

    assert_eq!(
        count_matches(&pattern, &subject),
        expected_count,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_pattern_literal_8() {
    let pattern = expr! { f[g[1, 2], h[3, 4]] };
    let subject = expr! { f[g[1, 2], h[3, 4]] };
    let expected_count = 1usize;

    assert_eq!(
        count_matches(&pattern, &subject),
        expected_count,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_pattern_literal_9() {
    let pattern = expr! { f[g[1, 2], h[3, 4]] };
    let subject = expr! { f[g[1, 2], h[3, 5]] };
    let expected_count = 0usize;

    assert_eq!(
        count_matches(&pattern, &subject),
        expected_count,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_pattern_literal_10() {
    let pattern = expr! { f[g[h[1]]] };
    let subject = expr! { f[g[h[1]]] };
    let expected_count = 1usize;

    assert_eq!(
        count_matches(&pattern, &subject),
        expected_count,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_pattern_literal_11() {
    let pattern = expr! { f[g[h[1]]] };
    let subject = expr! { f[g[h[2]]] };
    let expected_count = 0usize;

    assert_eq!(
        count_matches(&pattern, &subject),
        expected_count,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_pattern_literal_12() {
    let pattern = expr! { f[] };
    let subject = expr! { f[] };
    let expected_count = 1usize;

    assert_eq!(
        count_matches(&pattern, &subject),
        expected_count,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_pattern_literal_13() {
    let pattern = expr! { f[] };
    let subject = expr! { f[1] };
    let expected_count = 0usize;

    assert_eq!(
        count_matches(&pattern, &subject),
        expected_count,
        "Number of found matched patterns unexpected"
    );
}

#[test]
fn test_pattern_literal_14() {
    let pattern = expr! { f[] };
    let subject = expr! { g[] };
    let expected_count = 0usize;

    assert_eq!(
        count_matches(&pattern, &subject),
        expected_count,
        "Number of found matched patterns unexpected"
    );
}
