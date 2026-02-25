use std::{fmt::Debug, str::FromStr};

use crate::expr::{Expr, walk::ExprTopDownWalker};

pub const PATTERN_HEAD: &str = "Pattern";
pub const PATTERN_TEST_HEAD: &str = "PatternTest";
pub const BLANK_ONE_HEAD: &str = "Blank";
pub const BLANK_SEQ_HEAD: &str = "BlankSeq";
pub const BLANK_NULL_SEQ_HEAD: &str = "BlankNullSeq";

#[derive(Debug, Clone, Copy)]
pub enum PatternPredicate {
    IsSymbolQ,
    IsNumberQ,
}

impl PatternPredicate {
    pub fn check<A: Clone + Debug + PartialEq>(&self, expr: &Expr<A>) -> bool {
        use PatternPredicate::*;
        match self {
            IsSymbolQ => expr.is_symbol(),
            IsNumberQ => expr.is_number(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePredicateError;

impl FromStr for PatternPredicate {
    type Err = ParsePredicateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IsSymbolQ" => Ok(PatternPredicate::IsSymbolQ),
            "IsNumberQ" => Ok(PatternPredicate::IsNumberQ),
            _ => Err(ParsePredicateError),
        }
    }
}

#[derive(Clone)]
pub enum Pattern<'a, A> {
    Literal(&'a Expr<A>),
    Blank {
        bind_name: Option<&'a str>,
        match_head: Option<&'a Expr<A>>,
        predicate: Option<PatternPredicate>,
    },
    BlankSeq {
        bind_name: Option<&'a str>,
        match_head: Option<&'a Expr<A>>,
        predicate: Option<PatternPredicate>,
    },
    BlankNullSeq {
        bind_name: Option<&'a str>,
        match_head: Option<&'a Expr<A>>,
        predicate: Option<PatternPredicate>,
    },
    Node {
        head: Box<Pattern<'a, A>>,
        args: Vec<Pattern<'a, A>>,
        predicate: Option<PatternPredicate>,
    },
}

impl<'a, A> Pattern<'a, A>
where
    A: PartialEq + Clone,
{
    pub fn from_expr(expr: &'a Expr<A>) -> Self {
        Self::from_expr_inner(expr)
    }

    fn from_pattern_node(expr: &'a Expr<A>) -> Option<Pattern<'a, A>> {
        match expr {
            Expr::Node { head, args, .. }
                if head.matches_symbol(PATTERN_TEST_HEAD) && args.len() == 2 =>
            {
                let pat = args.first()?;
                let pred = args
                    .get(1)?
                    .get_symbol()?
                    .parse::<PatternPredicate>()
                    .ok()?;

                match Self::from_pattern_node(pat) {
                    Some(Pattern::Blank {
                        bind_name,
                        match_head,
                        predicate: None,
                    }) => Some(Pattern::Blank {
                        bind_name,
                        match_head,
                        predicate: Some(pred),
                    }),
                    _ => {
                        todo!()
                    }
                }
            }
            Expr::Node { head, args, .. }
                if head.matches_symbol(PATTERN_HEAD) && args.len() == 2 =>
            {
                let e = args.last()?;
                let h = e.head()?;

                if h.matches_symbol(BLANK_ONE_HEAD)
                    || h.matches_symbol(BLANK_SEQ_HEAD)
                    || h.matches_symbol(BLANK_NULL_SEQ_HEAD)
                {
                    let bind_name = args.first()?.get_symbol()?;
                    Some(Self::from_pattern_node(e)?.with_bind_name(bind_name))
                } else {
                    unimplemented!()
                }
            }
            Expr::Node { head, args, .. } if args.len() <= 1 => {
                if head.matches_symbol(BLANK_ONE_HEAD) {
                    Some(Pattern::Blank {
                        bind_name: None,
                        match_head: args.first(),
                        predicate: None,
                    })
                } else if head.matches_symbol(BLANK_SEQ_HEAD) {
                    Some(Pattern::BlankSeq {
                        bind_name: None,
                        match_head: args.first(),
                        predicate: None,
                    })
                } else if head.matches_symbol(BLANK_NULL_SEQ_HEAD) {
                    Some(Pattern::BlankNullSeq {
                        bind_name: None,
                        match_head: args.first(),
                        predicate: None,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn from_expr_inner(expr: &'a Expr<A>) -> Self {
        let mut descend = false;
        for e in ExprTopDownWalker::new(expr) {
            if let Expr::Node { head, .. } = e
                && (head.matches_symbol(PATTERN_HEAD)
                    || head.matches_symbol(BLANK_ONE_HEAD)
                    || head.matches_symbol(BLANK_SEQ_HEAD)
                    || head.matches_symbol(BLANK_NULL_SEQ_HEAD))
            {
                // There are possibly still non-literal patterns in tree -> descend
                descend = true;
                break;
            }
        }

        match expr {
            Expr::Atom { .. } => Pattern::Literal(expr),
            Expr::Node { head, args, .. } if descend => {
                if let Some(p) = Self::from_pattern_node(expr) {
                    p
                } else {
                    Pattern::Node {
                        head: Box::new(Self::from_expr_inner(head.as_ref())),
                        args: args.iter().map(|e| Self::from_expr_inner(e)).collect(),
                        predicate: None,
                    }
                }
            }
            _ => Pattern::Literal(expr),
        }
    }

    pub fn with_bind_name(self, name: &'a str) -> Self {
        use Pattern::*;
        match self {
            Literal(_) => self,
            Node { .. } => self,
            Blank {
                match_head,
                predicate,
                ..
            } => Blank {
                bind_name: Some(name),
                match_head,
                predicate,
            },
            BlankSeq {
                match_head,
                predicate,
                ..
            } => BlankSeq {
                bind_name: Some(name),
                match_head,
                predicate,
            },
            BlankNullSeq {
                match_head,
                predicate,
                ..
            } => BlankNullSeq {
                bind_name: Some(name),
                match_head,
                predicate,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use expr_macro::expr;

    use super::*;
    use crate::{
        atom::Atom,
        expr::{Expr, generator::*},
    };

    #[test]
    fn test_dbg() {
        let e = blank_sequence(None);
        dbg!(&e);
        let p = Pattern::from_expr(&e);
        dbg!(&p);
    }

    #[test]
    fn test_built_pattern_from_expr() {
        let expr = expr! {
            PatternTest[Blank[], IsSymbolQ]
        };
        let pattern = Pattern::from_expr(&expr);
        assert_eq!(
            format!("{pattern:?}"),
            r#"Blank{None, None, Some(IsSymbolQ)}"#
        );

        let expr = expr! {
            PatternTest[Pattern[x, Blank[]], IsSymbolQ]
        };
        let pattern = Pattern::from_expr(&expr);

        assert_eq!(
            format!("{pattern:?}"),
            r#"Blank{Some("x"), None, Some(IsSymbolQ)}"#
        );
    }
}
