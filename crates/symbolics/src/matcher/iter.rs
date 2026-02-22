use std::fmt::Debug;
use std::rc::Rc;

use crate::{
    expr::Expr,
    matcher::context::{MatchContext, MatchContextBindError},
    pattern::{Pattern, PatternPredicate},
};

#[derive(Clone)]
struct PatSpan<'a, A> {
    reference: Rc<[Pattern<'a, A>]>,
    start: usize,
}

impl<'a, A> PatSpan<'a, A> {
    pub fn from(arr: Vec<Pattern<'a, A>>) -> Self {
        PatSpan {
            reference: Rc::from(arr),
            start: 0,
        }
    }

    pub fn rest(mut self) -> Self {
        self.start += 1;
        self
    }

    pub fn len(&self) -> usize {
        self.reference.len().saturating_sub(self.start)
    }

    pub fn is_empty(&self) -> bool {
        self.start >= self.reference.len()
    }

    pub fn first(&self) -> Option<&Pattern<'a, A>> {
        self.reference.get(self.start)
    }
}

enum Task<'a, A> {
    MatchOne {
        pattern: Pattern<'a, A>,
        expr: &'a Expr<A>,
    },
    MatchSeq {
        patterns: PatSpan<'a, A>,
        exprs: &'a [Expr<A>],
    },
}

struct ChoicePoint<'a, A> {
    todo_len: usize,
    undo_len: usize,

    seq_name: Option<&'a str>,
    k_next: usize,

    rest_pats: PatSpan<'a, A>,
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
    bind_action_log: Vec<&'a str>,
    done: bool,
}

impl<'a, A> MatchIter<'a, A>
where
    A: Default + PartialEq + Clone + Debug,
{
    pub fn new(expr: &'a Expr<A>, pattern: Pattern<'a, A>) -> Self {
        MatchIter {
            tasks: vec![Task::MatchOne { pattern, expr }],
            ctx: MatchContext::default(),
            back_track: Vec::new(),
            bind_action_log: Vec::new(),
            done: false,
        }
    }

    fn bind_one(&mut self, name: &'a str, expr: &'a Expr<A>) -> Result<(), MatchContextBindError> {
        self.ctx.bind_one(name, expr)?;
        self.bind_action_log.push(name);
        Ok(())
    }

    fn bind_seq(
        &mut self,
        name: &'a str,
        expr_arr: &'a [Expr<A>],
    ) -> Result<(), MatchContextBindError> {
        self.ctx.bind_seq(name, expr_arr)?;
        self.bind_action_log.push(name);
        Ok(())
    }

    fn rollback_binds(&mut self, undo_len: usize) {
        while self.bind_action_log.len() > undo_len {
            self.ctx.unbind(self.bind_action_log.pop().unwrap());
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
            if let Some(&name) = cp.seq_name.as_ref()
                && self
                    .bind_seq(name, &cp.rest_exprs[..k])
                    .map_err(|_| MatchFail)
                    .is_err()
            {
                continue;
            }

            self.tasks.push(Task::MatchSeq {
                patterns: cp.rest_pats,
                exprs: &cp.rest_exprs[k..],
            });
            return true;
        }
        false
    }

    fn match_blank(
        &mut self,
        expr: &'a Expr<A>,
        bind_name: Option<&'a str>,
        match_head: Option<&Expr<A>>,
        predicate: Option<PatternPredicate>,
    ) -> Result<(), MatchFail> {
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

    fn match_blank_seq(
        &mut self,
        exprs: &'a [Expr<A>],
        min_left: usize,
        rest_pats: PatSpan<'a, A>,
        bind_name: Option<&'a str>,
    ) -> Result<(), MatchFail> {
        if exprs.is_empty() {
            return Err(MatchFail);
        }

        // We need to leave a few exprs for remaining patterns
        if exprs.len() < 1 + min_left {
            return Err(MatchFail);
        }

        let k_min = 1; // At least one element in BlankSeq pattern
        let k_max = exprs.len() - min_left; // At most k_max lements in BlankSeq pattern

        // Try the first split k_min now, but save choicepoint for k_min+1..=k_max
        if k_min < k_max {
            self.back_track.push(ChoicePoint {
                todo_len: self.tasks.len(),
                undo_len: self.bind_action_log.len(),
                seq_name: bind_name,
                k_next: k_min + 1,
                rest_pats: rest_pats.clone(),
                rest_exprs: exprs,
            });
        }

        if let Some(name) = bind_name {
            self.bind_seq(name, &exprs[..k_min])
                .map_err(|_| MatchFail)?;
        }

        self.tasks.push(Task::MatchSeq {
            patterns: rest_pats,
            exprs: &exprs[k_min..],
        });
        Ok(())
    }

    fn task_match_one(
        &mut self,
        pattern: Pattern<'a, A>,
        expr: &'a Expr<A>,
    ) -> Result<(), MatchFail> {
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
            } => self.match_blank(expr, bind_name, match_head, predicate),
            Pattern::Compound { head, args } => {
                if let Expr::Compound {
                    head: ehead,
                    args: eargs,
                    ..
                } = expr
                {
                    self.tasks.push(Task::MatchSeq {
                        patterns: PatSpan::from(args),
                        exprs: eargs,
                    });
                    self.tasks.push(Task::MatchOne {
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

    fn task_match_seq(
        &mut self,
        patterns: PatSpan<'a, A>,
        exprs: &'a [Expr<A>],
    ) -> Result<(), MatchFail> {
        if patterns.is_empty() {
            return if exprs.is_empty() {
                Ok(())
            } else {
                Err(MatchFail)
            };
        }

        match patterns.first().unwrap() {
            Pattern::BlankSeq { bind_name, .. } => self.match_blank_seq(
                exprs,
                patterns.len() - 1,
                patterns.clone().rest(),
                *bind_name,
            ),
            _ => {
                // non-seq: need at least one expr
                let (e0, erest) = exprs.split_first().ok_or(MatchFail)?;
                self.tasks.push(Task::MatchSeq {
                    patterns: patterns.clone().rest(),
                    exprs: erest,
                });
                self.tasks.push(Task::MatchOne {
                    // can we get rid of this clone?
                    // patterns are mostly pointers and relatively shallow
                    // -> shouldn't be a problem in general.
                    pattern: patterns.first().unwrap().clone(),
                    expr: e0,
                });
                Ok(())
            }
        }
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
                Task::MatchOne { pattern, expr } => self.task_match_one(pattern, expr),
                Task::MatchSeq { patterns, exprs } => self.task_match_seq(patterns, exprs),
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
