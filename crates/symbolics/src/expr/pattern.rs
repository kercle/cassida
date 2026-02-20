use crate::expr::{Expr, ExprTopDownWalker};

pub const PATTERN_HEAD: &str = "Pattern";
pub const BLANK_ONE_HEAD: &str = "Blank";
pub const BLANK_SEQ_HEAD: &str = "BlankSeq";

#[derive(Debug)]
pub enum PatternPredicate {
    IsSymbolQ,
}

pub enum Pattern<'a, A> {
    Literal(&'a Expr<A>),
    Blank {
        bind_name: Option<String>,
        match_head: Option<&'a Expr<A>>,
        predicate: Option<PatternPredicate>,
    },
    BlankSeq {
        bind_name: Option<String>,
        match_head: Option<&'a Expr<A>>,
    },
    Compound {
        head: Box<Pattern<'a, A>>,
        args: Vec<Pattern<'a, A>>,
    },
}

impl<'a, A> Pattern<'a, A>
where
    A: PartialEq + Clone,
{
    pub fn from_expr(expr: &'a Expr<A>) -> Self {
        Self::from_expr_inner(expr)
    }

    fn from_pattern_compound(expr: &'a Expr<A>) -> Option<Pattern<'a, A>> {
        match expr {
            Expr::Compound { head, args, .. } if head.matches_symbol(PATTERN_HEAD) => {
                if args.len() != 2 {
                    return None;
                }

                let e = args.get(1)?;
                let h = e.head()?;

                if h.matches_symbol(BLANK_ONE_HEAD) || h.matches_symbol(BLANK_SEQ_HEAD) {
                    let bind_name = args.get(0)?.get_symbol()?;
                    Some(Self::from_pattern_compound(e)?.with_bind_name(bind_name))
                } else {
                    unimplemented!()
                }
            }
            Expr::Compound { head, args, .. } => {
                if args.len() > 1 {
                    return None;
                }

                if head.matches_symbol(BLANK_ONE_HEAD) {
                    Some(Pattern::Blank {
                        bind_name: None,
                        match_head: args.get(0),
                        predicate: None,
                    })
                } else if head.matches_symbol(BLANK_SEQ_HEAD) {
                    Some(Pattern::BlankSeq {
                        bind_name: None,
                        match_head: args.get(0),
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
            if let Expr::Compound { head, .. } = e {
                if head.matches_symbol(PATTERN_HEAD) | head.matches_symbol(BLANK_ONE_HEAD)
                    || head.matches_symbol(BLANK_SEQ_HEAD)
                {
                    // There are possibly still non-literal patterns in tree -> descend
                    descend = true;
                    break;
                }
            }
        }

        match expr {
            Expr::Atom { .. } => Pattern::Literal(expr),
            Expr::Compound { head, args, .. } if descend => {
                if let Some(p) = Self::from_pattern_compound(expr) {
                    p
                } else {
                    Pattern::Compound {
                        head: Box::new(Self::from_expr_inner(head.as_ref())),
                        args: args.iter().map(|e| Self::from_expr_inner(e)).collect(),
                    }
                }
            }
            _ => Pattern::Literal(expr),
        }
    }

    pub fn with_bind_name(self, name: &str) -> Self {
        use Pattern::*;
        match self {
            Blank { match_head, .. } => Blank {
                bind_name: Some(name.to_string()),
                match_head,
                predicate: None,
            },
            BlankSeq { match_head, .. } => BlankSeq {
                bind_name: Some(name.to_string()),
                match_head,
            },
            _ => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::{generator::*, pattern::Pattern};

    #[test]
    fn test_dbg() {
        let e = blank_sequence(None);
        dbg!(&e);
        let p = Pattern::from_expr(&e);
        dbg!(&p);
    }
}
