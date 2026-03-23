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
//  2 + x_.               | 2                  | 1       | x bound to additive neutral el.
//  2 * x_.               | 2                  | 1       | x bound to multipl. neutral el.

use crate::norm_expr;
use crate::pattern::program::Compiler;
use crate::pattern::tests::utils::count_matches;

#[test]
fn test_optional_1() {
    let pattern = norm_expr! { f[x_., y_] };
    let subject = norm_expr! { f[a, b] };
    assert_eq!(count_matches(&pattern, &subject), 1);
}

#[test]
fn test_optional_2() {
    let pattern = norm_expr! { f[x_., y_] };
    let subject = norm_expr! { f[b] };
    assert_eq!(count_matches(&pattern, &subject), 1);
}

#[test]
fn test_optional_3() {
    let pattern = norm_expr! { f[x_., y_] };
    let subject = norm_expr! { f[] };
    assert_eq!(count_matches(&pattern, &subject), 0);
}

#[test]
fn test_optional_4() {
    let pattern = norm_expr! { f[x_, y_.] };
    let subject = norm_expr! { f[a, b] };
    assert_eq!(count_matches(&pattern, &subject), 1);
}

#[test]
fn test_optional_5() {
    let pattern = norm_expr! { f[x_, y_.] };
    let subject = norm_expr! { f[a] };
    assert_eq!(count_matches(&pattern, &subject), 1);
}

#[test]
fn test_optional_6() {
    let pattern = norm_expr! { f[x_, y_.] };
    let subject = norm_expr! { f[] };
    assert_eq!(count_matches(&pattern, &subject), 0);
}

#[test]
fn test_optional_7() {
    let pattern = norm_expr! { Mul[x_., Add[a__]^m_] };
    let subject = norm_expr! { Mul[2, Add[a, b]^3] };
    assert_eq!(count_matches(&pattern, &subject), 1);
}

#[test]
fn test_optional_8() {
    let pattern = norm_expr! { Mul[x_., Add[a__]^m_] };
    let subject = norm_expr! { Add[a, b]^3 };
    assert_eq!(count_matches(&pattern, &subject), 1);
}

#[test]
fn test_optional_9() {
    let pattern = norm_expr! { Mul[x_., Add[a__]^m_] };
    let subject = norm_expr! { foo };
    assert_eq!(count_matches(&pattern, &subject), 0);
}

#[test]
fn test_optional_10() {
    let pattern = norm_expr! { Add[x_., y_^2] };
    let subject = norm_expr! { Add[3, a^2] };
    assert_eq!(count_matches(&pattern, &subject), 1);
}

#[test]
fn test_optional_11() {
    let pattern = norm_expr! { Add[x_., y_^2] };
    let subject = norm_expr! { a^2 };
    assert_eq!(count_matches(&pattern, &subject), 1);
}

#[test]
fn test_optional_12() {
    let pattern = norm_expr! { Add[x_., y_^2] };
    let subject = norm_expr! { b^3 };
    assert_eq!(count_matches(&pattern, &subject), 0);
}

#[test]
fn test_optional_13() {
    let pattern = norm_expr! { Pow[x_, m_.] };
    let subject = norm_expr! { Pow[a, 3] };
    assert_eq!(count_matches(&pattern, &subject), 2);
}

#[test]
fn test_optional_14() {
    let pattern = norm_expr! { Pow[x_, m_.] };
    let subject = norm_expr! { a };
    assert_eq!(count_matches(&pattern, &subject), 1);
}

#[test]
fn test_optional_15() {
    let pattern = norm_expr! { Pow[x_, m_.] };
    let subject = norm_expr! { 5 };
    assert_eq!(count_matches(&pattern, &subject), 1);
}

#[test]
fn test_optional_16() {
    let pattern = norm_expr! { Mul[x_., y_.] };
    let subject = norm_expr! { Mul[a, b] };
    assert_eq!(count_matches(&pattern, &subject), 4);
}

#[test]
fn test_optional_17() {
    let pattern = norm_expr! { Mul[x_., y_.] };
    let subject = norm_expr! { 1 };
    assert_eq!(count_matches(&pattern, &subject), 2);
}

#[test]
fn test_optional_18() {
    let pattern = norm_expr! { HoldPattern[Mul[x_., Add[a__]^m_]] };
    let subject = norm_expr! { Add[a, b]^3 };
    assert_eq!(count_matches(&pattern, &subject), 1);
}

#[test]
fn test_optional_19() {
    let pattern = norm_expr! { HoldPattern[Mul[x_., Add[a__]^m_]] };
    let subject = norm_expr! { Mul[2, Add[a, b]^3] };
    assert_eq!(count_matches(&pattern, &subject), 1);
}

#[test]
fn test_optional_20() {
    let pattern = norm_expr! { Add[x_., y_^2] };
    let subject = norm_expr! { Mul[3, a^2] };
    assert_eq!(count_matches(&pattern, &subject), 0);
}

#[test]
fn test_optional_21() {
    let pattern = norm_expr! { 2 + x_. };
    let subject = norm_expr! { 2 };

    let program = Compiler::default().compile(&pattern);
    let env = program.run(&subject).next().unwrap();

    assert_eq!(env.get_one("x"), Some(&norm_expr!(0)));
}

#[test]
fn test_optional_22() {
    let pattern = norm_expr! { 2 * x_. };
    let subject = norm_expr! { 2 };

    let program = Compiler::default().compile(&pattern);
    let env = program.run(&subject).next().unwrap();

    assert_eq!(env.get_one("x"), Some(&norm_expr!(1)));
}
