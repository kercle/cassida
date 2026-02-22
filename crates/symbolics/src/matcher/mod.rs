pub mod context;
pub mod iter;

#[cfg(test)]
mod tests;

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
