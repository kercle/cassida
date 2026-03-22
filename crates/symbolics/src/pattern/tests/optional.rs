// -----------------------------------------------------------------------------------
// OPTIONAL TESTS
// -----------------------------------------------------------------------------------
//
//  Pattern               | Subject            | Matches | Notes
//  ----------------------|--------------------|---------|----------------------------
//  f[x_., y_]            | f[a, b]            | 1       | both present
//  f[x_., y_]            | f[b]               | 1       | x absent, binds to default
//  f[x_., y_]            | f[]                | 0       | y required
//  f[x_, y_.]            | f[a, b]            | 1       | both present
//  f[x_, y_.]            | f[a]               | 1       | y absent
//  f[x_, y_.]            | f[]                | 0       | x required
//  Mul[x_., Add[a__]^m_] | Mul[2, Add[a,b]^3] | 1       | x present
//  Mul[x_., Add[a__]^m_] | Add[a,b]^3         | 1       | x absent (Mul collapses)
//  Add[x_., y_^2]        | Add[3, a^2]        | 1       | x present
//  Add[x_., y_^2]        | a^2                | 1       | x absent (Add collapses)
//  Add[x_., y_^2]        | b^3                | 0       | y^2 doesn't match
//  Mul[x_., y_.]         | Mul[a, b]          | 1       | both present
//  Mul[x_., y_.]         | a                  | 1       | both absent
//  Power[x_, m_.]        | Power[a, 3]        | 1       | m present
//  Power[x_, m_.]        | a                  | 1       | m absent (Power collapses)
//  Power[x_, m_.]        | 5                  | 1       | m absent, x binds to number

use crate::norm_expr;
use crate::pattern::tests::utils::count_matches;

#[test]
fn test_optional_1() {
    // both args present
    let pattern = norm_expr! { f[x_., y_] };
    let subject = norm_expr! { f[a, b] };
    assert_eq!(count_matches(&pattern, &subject), 1);
}

#[test]
fn test_optional_2() {
    // x absent, f collapses to f[b]
    let pattern = norm_expr! { f[x_., y_] };
    let subject = norm_expr! { f[b] };
    assert_eq!(count_matches(&pattern, &subject), 1);
}

#[test]
fn test_optional_3() {
    // y is required, f[] has no subject for y
    let pattern = norm_expr! { f[x_., y_] };
    let subject = norm_expr! { f[] };
    assert_eq!(count_matches(&pattern, &subject), 0);
}

#[test]
fn test_optional_4() {
    // both present, y absent branch
    let pattern = norm_expr! { f[x_, y_.] };
    let subject = norm_expr! { f[a, b] };
    assert_eq!(count_matches(&pattern, &subject), 1);
}

#[test]
fn test_optional_5() {
    // y absent
    let pattern = norm_expr! { f[x_, y_.] };
    let subject = norm_expr! { f[a] };
    assert_eq!(count_matches(&pattern, &subject), 1);
}

#[test]
fn test_optional_6() {
    // x required, nothing to match it
    let pattern = norm_expr! { f[x_, y_.] };
    let subject = norm_expr! { f[] };
    assert_eq!(count_matches(&pattern, &subject), 0);
}

#[test]
fn test_optional_7() {
    // Mul with x present
    let pattern = norm_expr! { Mul[x_., Add[a__]^m_] };
    let subject = norm_expr! { Mul[2, Add[a, b]^3] };
    assert_eq!(count_matches(&pattern, &subject), 1);
}

#[test]
fn test_optional_8() {
    // Mul with x absent — Mul[Add[a,b]^3] collapses to Add[a,b]^3
    let pattern = norm_expr! { Mul[x_., Add[a__]^m_] };
    let subject = norm_expr! { Add[a, b]^3 };
    assert_eq!(count_matches(&pattern, &subject), 1);
}

#[test]
fn test_optional_9() {
    // Mul doesn't match a plain symbol
    let pattern = norm_expr! { Mul[x_., Add[a__]^m_] };
    let subject = norm_expr! { foo };
    assert_eq!(count_matches(&pattern, &subject), 0);
}

#[test]
fn test_optional_10() {
    // Add with x present
    let pattern = norm_expr! { Add[x_., y_^2] };
    let subject = norm_expr! { Add[3, a^2] };
    assert_eq!(count_matches(&pattern, &subject), 1);
}

#[test]
fn test_optional_11() {
    // Add with x absent — Add[a^2] collapses to a^2
    let pattern = norm_expr! { Add[x_., y_^2] };
    let subject = norm_expr! { a^2 };
    assert_eq!(count_matches(&pattern, &subject), 1);
}

#[test]
fn test_optional_12() {
    // y^2 required but b^3 doesn't match y^2
    let pattern = norm_expr! { Add[x_., y_^2] };
    let subject = norm_expr! { b^3 };
    assert_eq!(count_matches(&pattern, &subject), 0);
}

// #[test]
// fn test_optional_13() {
//     // Power: m present
//     let pattern = norm_expr! { Power[x_, m_.] };
//     let subject = norm_expr! { Power[a, 3] };
//     assert_eq!(count_matches(&pattern, &subject), 1);
// }

// #[test]
// fn test_optional_14() {
//     // Power: m absent — Power[a] collapses to a
//     let pattern = norm_expr! { Power[x_, m_.] };
//     let subject = norm_expr! { a };
//     assert_eq!(count_matches(&pattern, &subject), 1);
// }

// #[test]
// fn test_optional_15() {
//     // Power: m absent, x matches a number
//     let pattern = norm_expr! { Power[x_, m_.] };
//     let subject = norm_expr! { 5 };
//     assert_eq!(count_matches(&pattern, &subject), 1);
// }

// #[test]
// fn test_optional_16() {
//     // Both optionals absent in Mul — collapses to 1 (Mul identity)
//     let pattern = norm_expr! { Mul[x_., y_.] };
//     let subject = norm_expr! { Mul[a, b] };
//     assert_eq!(count_matches(&pattern, &subject), 1);
// }

// #[test]
// fn test_optional_17() {
//     // Both optionals absent — Mul[] collapses to 1, subject is 1
//     let pattern = norm_expr! { Mul[x_., y_.] };
//     let subject = norm_expr! { 1 };
//     assert_eq!(count_matches(&pattern, &subject), 1);
// }

#[test]
fn test_optional_18() {
    // HoldPattern prevents inner normalization but Mul still collapses at optional level
    let pattern = norm_expr! { HoldPattern[Mul[x_., Add[a__]^m_]] };
    let subject = norm_expr! { Add[a, b]^3 };
    assert_eq!(count_matches(&pattern, &subject), 1);
}

#[test]
fn test_optional_19() {
    // HoldPattern: both present
    let pattern = norm_expr! { HoldPattern[Mul[x_., Add[a__]^m_]] };
    let subject = norm_expr! { Mul[2, Add[a, b]^3] };
    assert_eq!(count_matches(&pattern, &subject), 1);
}

#[test]
fn test_optional_20() {
    // optional does not match across incompatible heads
    let pattern = norm_expr! { Add[x_., y_^2] };
    let subject = norm_expr! { Mul[3, a^2] };
    assert_eq!(count_matches(&pattern, &subject), 0);
}
