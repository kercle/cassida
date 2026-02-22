use std::fmt::Debug;

use crate::{
    expr::Expr,
    matcher::context::{MatchContext, MatchContextBindError},
    pattern::Pattern,
};

enum BindAction {
    Bind(String),
}

enum Task<'a, A> {
    Node {
        pattern: Pattern<'a, A>,
        expr: &'a Expr<A>,
    },
    OrderedList {
        patterns: Vec<Pattern<'a, A>>,
        exprs: &'a [Expr<A>],
    },
    UnorderedList {
        patterns: Vec<Pattern<'a, A>>,
        exprs: &'a [Expr<A>],
    },
}

struct ChoicePoint<'a, A> {
    todo_len: usize,
    undo_len: usize,

    // data needed to enumerate alternatives:
    seq_name: Option<String>,
    k_next: usize, // next length to try
    // and what remains after the seq pattern:
    rest_pats: Vec<Pattern<'a, A>>,
    rest_exprs: &'a [Expr<A>],
}

#[derive(Debug, Clone, Copy)]
struct MatchFail;

pub struct MatchIter<'a, A>
where
    A: PartialEq + Clone,
{
    tasks: Vec<Task<'a, A>>,
    ctx: MatchContext<'a, A>,
    back_track: Vec<ChoicePoint<'a, A>>,
    undo_log: Vec<BindAction>,
    done: bool,
}

impl<'a, A> MatchIter<'a, A>
where
    A: Default + PartialEq + Clone + Debug,
{
    pub fn new(expr: &'a Expr<A>, pattern: Pattern<'a, A>) -> Self {
        MatchIter {
            tasks: vec![Task::Node { pattern, expr }],
            ctx: MatchContext::default(),
            back_track: Vec::new(),
            undo_log: Vec::new(),
            done: false,
        }
    }

    fn bind_one(&mut self, name: &str, expr: &'a Expr<A>) -> Result<(), MatchContextBindError> {
        self.ctx.bind_one(name, expr)?;
        self.undo_log.push(BindAction::Bind(name.to_string()));
        Ok(())
    }

    fn bind_seq(
        &mut self,
        name: &str,
        expr_arr: &'a [Expr<A>],
    ) -> Result<(), MatchContextBindError> {
        self.ctx.bind_seq(name, expr_arr)?;
        self.undo_log.push(BindAction::Bind(name.to_string()));
        Ok(())
    }

    fn rollback_binds(&mut self, undo_len: usize) {
        while self.undo_log.len() > undo_len {
            match self.undo_log.pop().unwrap() {
                BindAction::Bind(name) => self.ctx.unbind(name),
            }
        }
    }

    fn backtrack(&mut self) -> bool {
        while let Some(cp) = self.back_track.pop() {
            self.tasks.truncate(cp.todo_len);
            self.rollback_binds(cp.undo_len);

            let min_left = cp.rest_pats.len();
            let k_max = cp.rest_exprs.len().saturating_sub(min_left);
            let k = cp.k_next;

            if k < 1 || k > k_max {
                continue; // choicepoint exhausted
            }

            // if there are further ks, push updated choicepoint back
            if k < k_max {
                self.back_track.push(ChoicePoint {
                    k_next: k + 1,
                    rest_pats: cp.rest_pats.clone(),
                    seq_name: cp.seq_name.clone(),
                    ..cp
                });
            }

            // apply this k
            if let Some(name) = cp.seq_name.as_ref()
                && self
                    .bind_seq(&name, &cp.rest_exprs[..k])
                    .map_err(|_| MatchFail)
                    .is_err()
            {
                continue;
            }

            self.tasks.push(Task::OrderedList {
                patterns: cp.rest_pats,
                exprs: &cp.rest_exprs[k..],
            });
            return true;
        }
        false
    }

    fn queue_node(&mut self, pattern: Pattern<'a, A>, expr: &'a Expr<A>) -> Result<(), MatchFail> {
        match pattern {
            Pattern::Literal(p_expr) => {
                if p_expr == expr {
                    Ok(())
                } else {
                    Err(MatchFail)
                }
            }
            Pattern::Blank {
                bind_name,
                match_head,
                predicate,
            } => {
                if let Some(expected_head) = match_head {
                    match expr {
                        Expr::Compound { head, .. } => {
                            if head.as_ref() != expected_head {
                                return Err(MatchFail);
                            }
                        }
                        _ => {
                            return Err(MatchFail);
                        }
                    }
                }

                let bind_success = bind_name
                    .as_ref()
                    .map(|n| self.bind_one(n, expr).is_ok())
                    .unwrap_or(true);
                let pred_check_success = predicate.as_ref().map(|p| p.check(expr)).unwrap_or(true);

                if bind_success && pred_check_success {
                    Ok(())
                } else {
                    Err(MatchFail)
                }
            }
            Pattern::Compound { head, args } => {
                if let Expr::Compound {
                    head: ehead,
                    args: eargs,
                    ..
                } = expr
                {
                    self.tasks.push(Task::OrderedList {
                        patterns: args,
                        exprs: eargs,
                    });
                    self.tasks.push(Task::Node {
                        pattern: *head,
                        expr: ehead,
                    });
                    Ok(())
                } else {
                    Err(MatchFail)
                }
            }
            Pattern::BlankSeq { .. } => Err(MatchFail),
        }
    }

    fn queue_ordered_list(
        &mut self,
        patterns: Vec<Pattern<'a, A>>,
        exprs: &'a [Expr<A>],
    ) -> Result<(), MatchFail> {
        // both empty => ok
        if patterns.is_empty() {
            return if exprs.is_empty() {
                Ok(())
            } else {
                Err(MatchFail)
            };
        }

        let (p0, prest) = patterns.split_first().unwrap();

        match p0 {
            Pattern::BlankSeq { bind_name, .. } => {
                if exprs.is_empty() {
                    return Err(MatchFail);
                }

                // We need to leave a few exprs for remaining patterns
                let min_left = prest.len();
                if exprs.len() < 1 + min_left {
                    return Err(MatchFail);
                }

                let k_min = 1; // At least one element in BlankSeq pattern
                let k_max = exprs.len() - min_left; // At most k_max lements in BlankSeq pattern

                // Try the first split k_min now, but save choicepoint for k_min+1..=k_max
                if k_min < k_max {
                    self.back_track.push(ChoicePoint {
                        todo_len: self.tasks.len(),
                        undo_len: self.undo_log.len(),
                        seq_name: bind_name.clone(),
                        k_next: k_min + 1,
                        rest_pats: prest.to_vec(),
                        rest_exprs: exprs,
                    });
                }

                if let Some(name) = bind_name {
                    self.bind_seq(name, &exprs[..k_min])
                        .map_err(|_| MatchFail)?;
                }

                self.tasks.push(Task::OrderedList {
                    patterns: prest.to_vec(),
                    exprs: &exprs[k_min..],
                });
                Ok(())
            }
            _ => {
                // non-seq: need at least one expr
                let (e0, erest) = exprs.split_first().ok_or(MatchFail)?;
                self.tasks.push(Task::OrderedList {
                    patterns: prest.to_vec(),
                    exprs: erest,
                });
                self.tasks.push(Task::Node {
                    pattern: p0.clone(),
                    expr: e0,
                });
                Ok(())
            }
        }
    }

    fn queue_unordered_list(
        &mut self,
        _patterns: Vec<Pattern<'a, A>>,
        _exprs: &'a [Expr<A>],
    ) -> Result<(), MatchFail> {
        todo!()
    }
}

impl<'a, A> Iterator for MatchIter<'a, A>
where
    A: Default + PartialEq + Clone + Debug,
{
    type Item = MatchContext<'a, A>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        while let Some(task) = self.tasks.pop() {
            let r = match task {
                Task::Node { pattern, expr } => self.queue_node(pattern, expr),
                Task::OrderedList { patterns, exprs } => self.queue_ordered_list(patterns, exprs),
                Task::UnorderedList { patterns, exprs } => {
                    self.queue_unordered_list(patterns, exprs)
                }
            };

            if r.is_err() && !self.backtrack() {
                self.done = true;
                return None;
            }
        }

        if !self.backtrack() {
            self.done = true;
        }

        Some(self.ctx.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        expr::generator::*,
        parser::ast::{ADD_HEAD, MUL_HEAD},
        symbol,
    };

    fn flatten(e: Expr) -> Expr {
        e.flatten(|e: &Expr| e.matches_symbol(ADD_HEAD) || e.matches_symbol(MUL_HEAD))
    }

    fn collect<'a>(expr: &'a Expr<()>, pat: Pattern<'a, ()>) -> Vec<MatchContext<'a, ()>> {
        MatchIter::new(expr, pat).collect()
    }

    fn pat<'a>(e: &'a Expr<()>) -> Pattern<'a, ()> {
        Pattern::from_expr(e)
    }

    fn one<'a>(expr: &'a Expr<()>, pat: &'a Pattern<'a, ()>) -> MatchContext<'a, ()> {
        let m = collect(expr, pat.clone());
        assert_eq!(m.len(), 1, "expected exactly 1 match, got {}", m.len());
        m.into_iter().next().unwrap()
    }

    #[test]
    fn test_expr_matching() {
        let x = symbol!("x");

        let expr1 = x + 1;
        let expr2 = blank(None) + pattern("a", blank(None));
        let pat = Pattern::from_expr(&expr2);
        let matches: Vec<MatchContext<'_, ()>> = MatchIter::new(&expr1, pat).into_iter().collect();

        assert!(matches.len() == 1);
        assert_eq!(
            matches.first().unwrap().get_one("a"),
            Some(1.into()).as_ref()
        );
    }

    #[test]
    fn literal_and_structure() {
        let x = symbol!("x");
        let e = x + 1;

        // equals
        let p1e = x + 1;
        let p1 = Pattern::from_expr(&p1e);
        assert_eq!(collect(&e, p1).len(), 1);

        // literal mismatch
        let p2e = x + 2;
        let p2 = pat(&p2e);
        assert_eq!(collect(&e, p2).len(), 0);

        // head mismatch
        let p3e = x * 1;
        let p3 = pat(&p3e);
        assert_eq!(collect(&e, p3).len(), 0);

        // arity mismatch
        let p4e = x + 1 + 2;
        let p4 = pat(&p4e);
        assert_eq!(collect(&e, p4).len(), 0);
    }

    #[test]
    fn blanks_and_named_blanks() {
        let x = symbol!("x");

        let e = x + 1;
        let pe = blank(None) + pattern("a", blank(None));
        let p = pat(&pe);
        let m = one(&e, &p);
        assert_eq!(m.get_one("a"), Some(1.into()).as_ref());

        let pe2 = blank(None) + blank(None);
        let p2 = pat(&pe2);
        let m2 = one(&e, &p2);
        assert!(m2.get_one("a").is_none());

        let e31 = Expr::from_i64(1) + 1;
        let pe3 = pattern("a", blank(None)) + pattern("a", blank(None));
        let p3 = pat(&pe3);
        let m31 = one(&e31, &p3);
        assert_eq!(m31.get_one("a"), Some(1.into()).as_ref());

        let e32 = Expr::from_i64(1) + 2;
        assert_eq!(collect(&e32, p3).len(), 0);

        let y = symbol!("y");
        let e41 = h(x, x);
        let e42 = h(x, y);
        let pe4 = h(pattern("a", blank(None)), pattern("a", blank(None)));
        let p4 = pat(&pe4);
        assert_eq!(collect(&e41, p4.clone()).len(), 1);
        assert_eq!(collect(&e42, p4).len(), 0);
    }

    #[test]
    fn blank_seq_unnamed_basic() {
        // Add[1, __, 3] against 1+2+3 -> 1 match
        let e1 = flatten(Expr::from_i64(1) + 2 + 3);
        let pe = flatten(1 + blank_sequence(None) + 3);
        let p = pat(&pe);
        assert_eq!(collect(&e1, p.clone()).len(), 1);

        // Add[1, __, 4, __] against 1+2+4+3+4+5 -> 2 matches
        let e2 = flatten(Expr::from_i64(1) + 2 + 4 + 3 + 4 + 5);
        let pe2 = flatten(1 + blank_sequence(None) + 4 + blank_sequence(None));
        let p2 = pat(&pe2);
        assert_eq!(collect(&e2, p2).len(), 2);

        // should not match 1+3
        let e3 = Expr::from_i64(1) + 3;
        assert_eq!(collect(&e3, p).len(), 0);
    }
}
