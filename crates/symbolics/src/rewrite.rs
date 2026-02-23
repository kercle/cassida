use crate::{
    expr::Expr,
    matcher::{CommutativePredicate, context::MatchContext},
};

pub type RuleTransformer<A> = Box<dyn Fn(&MatchContext<'_, A>) -> Expr<A>>;

pub struct Rule<A>
where
    A: Clone + PartialEq,
{
    pub pattern: Expr<A>,
    pub transform: RuleTransformer<A>,
}

pub struct Rewriter<A>
where
    A: Clone + PartialEq,
{
    rules: Vec<Rule<A>>,
    is_commutative: Option<CommutativePredicate<A>>,
}

impl<A> Rewriter<A>
where
    A: Clone + PartialEq,
{
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            is_commutative: None,
        }
    }

    pub fn with_rule(mut self, pattern: Expr<A>, transform: RuleTransformer<A>) -> Self {
        self.rules.push(Rule { pattern, transform });
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
    A: Clone + PartialEq + Default,
{
    pub fn apply_all(expr: Expr<A>) -> Expr<A> {
        expr.map_bottom_up(&|_e| {
            todo!()
        })
    }
}
