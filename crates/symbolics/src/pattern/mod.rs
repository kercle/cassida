mod bit_mask;
mod builtin;
mod fmt;
mod utils;

pub mod environment;
pub mod merging;
pub mod predicates;
pub mod program;
pub mod runtime;

#[cfg(test)]
mod tests;

use std::{fmt::Debug, str::FromStr};

use crate::{
    builtins::{self, traits::BuiltIn},
    expr::{ExprKind, NormExpr, walk::ExprTopDownWalker},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PatternPredicate {
    IsSymbol,
    IsNumber,
    IsInteger,
    IsPositiveInteger,
    IsNegativeInteger,
    IsRational,
    IsPositive,
    IsNegative,
}

impl PatternPredicate {
    pub fn check(&self, expr: &NormExpr) -> bool {
        use PatternPredicate::*;
        match self {
            IsSymbol => predicates::is_symbol(expr),
            IsNumber => predicates::is_number(expr),
            IsInteger => predicates::is_integer(expr),
            IsPositiveInteger => predicates::is_integer(expr) && predicates::is_positive(expr),
            IsNegativeInteger => predicates::is_integer(expr) && predicates::is_negative(expr),
            IsRational => predicates::is_rational(expr),
            IsPositive => predicates::is_positive(expr),
            IsNegative => predicates::is_negative(expr),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePredicateError;

impl FromStr for PatternPredicate {
    type Err = ParsePredicateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IsSymbol" => Ok(PatternPredicate::IsSymbol),
            "IsNumber" => Ok(PatternPredicate::IsNumber),
            "IsInteger" => Ok(PatternPredicate::IsInteger),
            "IsPositiveInteger" => Ok(PatternPredicate::IsPositiveInteger),
            "IsNegativeInteger" => Ok(PatternPredicate::IsNegativeInteger),
            "IsRational" => Ok(PatternPredicate::IsRational),
            "IsPositive" => Ok(PatternPredicate::IsPositive),
            "IsNegative" => Ok(PatternPredicate::IsNegative),
            _ => Err(ParsePredicateError),
        }
    }
}

#[derive(Clone)]
pub enum Pattern<'a> {
    Literal(&'a NormExpr),
    Blank {
        bind_name: Option<&'a str>,
        match_head: Option<&'a NormExpr>,
        predicate: Option<PatternPredicate>,
    },
    BlankSeq {
        bind_name: Option<&'a str>,
        match_head: Option<&'a NormExpr>,
        predicate: Option<PatternPredicate>,
    },
    BlankNullSeq {
        bind_name: Option<&'a str>,
        match_head: Option<&'a NormExpr>,
        predicate: Option<PatternPredicate>,
    },
    Node {
        head: Box<Pattern<'a>>,
        args: Vec<Pattern<'a>>,
        predicate: Option<PatternPredicate>,
    },
}

impl<'a> Pattern<'a> {
    pub fn from_expr(expr: &'a NormExpr) -> Self {
        Self::from_expr_inner(expr)
    }

    fn from_pattern_node(expr: &'a NormExpr) -> Option<Pattern<'a>> {
        match expr.kind() {
            ExprKind::Node { head, args }
                if head.matches_symbol(builtins::PatternTest::head()) && args.len() == 2 =>
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
            ExprKind::Node { head, args }
                if head.matches_symbol(builtins::Pattern::head()) && args.len() == 2 =>
            {
                let e = args.last()?;
                let h = e.head()?;

                if h.matches_symbol(builtins::Blank::head())
                    || h.matches_symbol(builtins::BlankSeq::head())
                    || h.matches_symbol(builtins::BlankNullSeq::head())
                {
                    let bind_name = args.first()?.get_symbol()?;
                    Some(Self::from_pattern_node(e)?.with_bind_name(bind_name))
                } else {
                    unimplemented!()
                }
            }
            ExprKind::Node { head, args } if args.len() <= 1 => {
                if head.matches_symbol(builtins::Blank::head()) {
                    Some(Pattern::Blank {
                        bind_name: None,
                        match_head: args.first(),
                        predicate: None,
                    })
                } else if head.matches_symbol(builtins::BlankSeq::head()) {
                    Some(Pattern::BlankSeq {
                        bind_name: None,
                        match_head: args.first(),
                        predicate: None,
                    })
                } else if head.matches_symbol(builtins::BlankNullSeq::head()) {
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

    fn from_expr_inner(expr: &'a NormExpr) -> Self {
        let mut descend = false;
        for e in ExprTopDownWalker::new(expr) {
            if let ExprKind::Node { head, .. } = e.kind()
                && (head.matches_symbol(builtins::Pattern::head())
                    || head.matches_symbol(builtins::Blank::head())
                    || head.matches_symbol(builtins::BlankSeq::head())
                    || head.matches_symbol(builtins::BlankNullSeq::head()))
            {
                // There are possibly still non-literal patterns in tree -> descend
                descend = true;
                break;
            }
        }

        match expr.kind() {
            ExprKind::Atom { .. } => Pattern::Literal(expr),
            ExprKind::Node { head, args } if descend => {
                if let Some(p) = Self::from_pattern_node(expr) {
                    p
                } else {
                    Pattern::Node {
                        head: Box::new(Self::from_expr_inner(head.as_ref())),
                        args: args.iter().map(Self::from_expr_inner).collect(),
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
