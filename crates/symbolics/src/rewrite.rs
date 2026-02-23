use std::fmt::Debug;

use crate::{
    expr::{Expr, NormalizedExpr},
    matcher::{CommutativePredicate, Matcher, context::MatchContext},
};

pub type RuleTransformer<A> = Box<dyn Fn(&mut MatchContext<'_, A>) -> Expr<A>>;

pub struct Rule<A>
where
    A: Clone + PartialEq,
{
    pub pattern: Expr<A>,
    pub transform: RuleTransformer<A>,
}

#[derive(Default)]
pub struct Rewriter<A>
where
    A: Clone + PartialEq,
{
    rules: Vec<Rule<A>>,
    is_commutative: Option<CommutativePredicate<A>>,
}

impl<A> Rewriter<A>
where
    A: Clone + PartialEq + Default,
{
    pub fn new() -> Self {
        Self::default()
    }
}

impl<A> Rewriter<A>
where
    A: Clone + PartialEq,
{
    pub fn with_rule<F>(mut self, pattern: NormalizedExpr<A>, transform: F) -> Self
    where
        F: Fn(&mut MatchContext<'_, A>) -> Expr<A> + 'static,
    {
        self.rules.push(Rule {
            pattern: pattern.take_expr(),
            transform: Box::new(transform),
        });
        self
    }

    pub fn with_rules<I, F>(mut self, rules: I) -> Self
    where
        I: IntoIterator<Item = (NormalizedExpr<A>, F)>,
        F: Fn(&mut MatchContext<'_, A>) -> Expr<A> + 'static,
    {
        for (p, t) in rules {
            self = self.with_rule(p, t);
        }
        self
    }

    pub fn commutative_if<F>(mut self, f: F) -> Self
    where
        F: Fn(&Expr<A>) -> bool + 'static,
    {
        self.is_commutative = Some(CommutativePredicate::new(f));
        self
    }
}

impl<A> Rewriter<A>
where
    A: Clone + PartialEq + Default + Debug,
{
    pub fn apply_first_match(self, expr: Expr<A>) -> Expr<A> {
        let patterns: Vec<(Matcher<A>, RuleTransformer<A>)> = self
            .rules
            .into_iter()
            .map(|r| {
                (
                    Matcher::new(r.pattern).with_commutative_predicate(self.is_commutative.clone()),
                    r.transform,
                )
            })
            .collect();

        expr.map_bottom_up(&|expr| {
            let mut res = expr;

            for (m, transform) in &patterns {
                if let Some(mut ctx) = m.first_match(&res) {
                    res = transform(&mut ctx);
                }
            }

            res
        })
    }
}
