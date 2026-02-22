pub mod context;
pub mod iter;

use std::fmt::Debug;

pub use iter::*;

use crate::{expr::Expr, matcher::context::MatchContext, pattern::Pattern};

pub struct Matcher<A> {
    pattern_expr: Expr<A>,
}

impl<A> Matcher<A>
where
    A: PartialEq + Clone + Default + Debug,
{
    pub fn new(pattern_expr: Expr<A>) -> Self {
        Matcher { pattern_expr }
    }

    pub fn iter_matches<'a>(&'a self, expr: &'a Expr<A>) -> MatchIter<'a, A> {
        MatchIter::new(expr, Pattern::from_expr(&self.pattern_expr))
    }

    pub fn first_match<'a>(&'a self, expr: &'a Expr<A>) -> Option<MatchContext<'a, A>> {
        self.iter_matches(expr).next()
    }
}

#[cfg(test)]
mod tests {
    use expr_macro::raw_expr;

    use super::*;
    use crate::atom::Atom;

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
    fn match_two_blankseq_backtracking_count() {
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
}
