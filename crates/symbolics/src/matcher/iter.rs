use std::{fmt::Debug, rc::Rc};

use crate::{
    expr::Expr,
    matcher::{context::MatchContext, pattern_span::PatSpan},
    pattern::{Pattern, PatternPredicate},
};

type PredicateFunction<A> = Box<dyn Fn(&Expr<A>) -> bool>;

#[derive(Clone)]
pub struct CommutativePredicate<A>(Rc<PredicateFunction<A>>);

impl<A> CommutativePredicate<A> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(&Expr<A>) -> bool + 'static,
    {
        Self(Rc::new(Box::new(f)))
    }

    pub fn eval(&self, arg: &Expr<A>) -> bool {
        self.0(arg)
    }
}

#[derive(Debug, Clone)]
enum Task<'a, A>
where
    A: Clone + PartialEq,
{
    MatchOne {
        pattern: Pattern<'a, A>,
        expr: &'a Expr<A>,
    },
    MatchSeq {
        patterns: PatSpan<'a, A>,
        exprs: &'a [Expr<A>],
    },
    ResumeOrderedSplit {
        seq_name: Option<&'a str>,
        k_min: usize,
        k: usize,
        rest_pats: PatSpan<'a, A>,
        rest_exprs: &'a [Expr<A>],
    },
    MatchUnorderedSeq {
        patterns: PatSpan<'a, A>,
        exprs: &'a [Expr<A>],
        remaining: Vec<usize>,
    },
    ResumeUnorderedPick {
        pat: Pattern<'a, A>,
        patterns_rest: Vec<Pattern<'a, A>>,
        exprs: &'a [Expr<A>],
        remaining: Vec<usize>,
        next_choice_pos: usize,
        seq_pat: Option<Pattern<'a, A>>,
    },
    MatchUnorderedRest {
        patterns_rest: Vec<Pattern<'a, A>>,
        exprs: &'a [Expr<A>],
        remaining: Vec<usize>,
        seq_pat: Option<Pattern<'a, A>>,
    },
    PopTaskFloor,
}

#[derive(Debug)]
struct ChoicePoint<'a, A>
where
    A: Clone + PartialEq,
{
    pub tasks_snapshot: Vec<Task<'a, A>>,
    pub undo_len: usize,
    pub floor_stack_snapshot: Vec<usize>,
    pub resume: Task<'a, A>,
}

#[derive(Debug, Clone, Copy)]
enum MatchError {
    MatchFail,
    BindFail,
}

pub struct MatchIter<'a, A>
where
    A: PartialEq + Clone,
{
    tasks: Vec<Task<'a, A>>,
    task_floor_stack: Vec<usize>,
    ctx: MatchContext<'a, A>,
    back_track: Vec<ChoicePoint<'a, A>>,
    bind_action_log: Vec<&'a str>,
    done: bool,
    is_commutative: Option<CommutativePredicate<A>>,
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
                Task::MatchSeq { patterns, exprs } => self.task_match_ordered_seq(patterns, exprs),
                Task::ResumeOrderedSplit {
                    seq_name,
                    k_min,
                    k,
                    rest_pats,
                    rest_exprs,
                } => self.task_resume_ordered_split(seq_name, k_min, k, rest_pats, rest_exprs),
                Task::MatchUnorderedSeq {
                    patterns,
                    exprs,
                    remaining,
                } => self.task_match_unordered_seq(patterns, exprs, remaining),
                Task::MatchUnorderedRest {
                    patterns_rest,
                    exprs,
                    remaining,
                    seq_pat,
                } => self.task_match_unordered_rest(patterns_rest, exprs, remaining, seq_pat),
                Task::ResumeUnorderedPick {
                    pat,
                    patterns_rest,
                    exprs,
                    remaining,
                    next_choice_pos,
                    seq_pat,
                } => self.task_resume_unordered_pick(
                    pat,
                    patterns_rest,
                    exprs,
                    remaining,
                    next_choice_pos,
                    seq_pat,
                ),
                Task::PopTaskFloor => {
                    self.task_floor_stack.pop();
                    Ok(())
                }
            };

            if r.is_err() && !self.backtrack() {
                self.done = true;
                return None;
            }
        }

        // need to take ctx before draining in backtrack.
        let out = self.ctx.clone();

        if !self.backtrack() {
            self.done = true;
        }

        Some(out)
    }
}

impl<'a, A> MatchIter<'a, A>
where
    A: Default + PartialEq + Clone + Debug,
{
    pub fn new(expr: &'a Expr<A>, pattern: Pattern<'a, A>) -> Self {
        MatchIter {
            tasks: vec![Task::MatchOne { pattern, expr }],
            task_floor_stack: vec![0],
            ctx: MatchContext::default(),
            back_track: Vec::new(),
            bind_action_log: Vec::new(),
            done: false,
            is_commutative: None,
        }
    }

    pub fn with_commutative_predicate(mut self, f: Option<CommutativePredicate<A>>) -> Self {
        self.is_commutative = f;
        self
    }

    pub fn commutative_if<F>(mut self, f: F) -> Self
    where
        F: Fn(&Expr<A>) -> bool + 'static,
    {
        self.is_commutative = Some(CommutativePredicate::new(f));
        self
    }

    fn is_commutative_head(&self, head: &Expr<A>) -> bool {
        self.is_commutative
            .as_ref()
            .map(|f| f.eval(head))
            .unwrap_or(false)
    }

    fn min_required(pats: &PatSpan<'a, A>) -> usize {
        pats.as_slice()
            .iter()
            .map(|p| match p {
                Pattern::BlankNullSeq { .. } => 0,
                Pattern::BlankSeq { .. }
                | Pattern::Blank { .. }
                | Pattern::Node { .. }
                | Pattern::Literal(_) => 1,
            })
            .sum()
    }

    fn bind_one(&mut self, name: &'a str, expr: &'a Expr<A>) -> Result<(), MatchError> {
        self.ctx
            .bind_one(name, expr)
            .map_err(|_| MatchError::BindFail)?;
        self.bind_action_log.push(name);

        Ok(())
    }

    fn bind_seq(&mut self, name: &'a str, expr_arr: Vec<&'a Expr<A>>) -> Result<(), MatchError> {
        self.ctx
            .bind_seq(name, expr_arr)
            .map_err(|_| MatchError::BindFail)?;
        self.bind_action_log.push(name);

        Ok(())
    }

    fn set_choice_point(&mut self, resume_task: Task<'a, A>) {
        let cp = ChoicePoint {
            tasks_snapshot: self.tasks.clone(),
            undo_len: self.bind_action_log.len(),
            floor_stack_snapshot: self.task_floor_stack.clone(),
            resume: resume_task,
        };
        self.back_track.push(cp);
    }

    fn rollback_binds(&mut self, undo_len: usize) {
        while self.bind_action_log.len() > undo_len {
            let name = self.bind_action_log.pop().unwrap();
            self.ctx.unbind(name);
        }

        debug_assert_eq!(self.bind_action_log.len(), undo_len);
    }

    fn backtrack(&mut self) -> bool {
        if let Some(cp) = self.back_track.pop() {
            self.tasks = cp.tasks_snapshot;
            self.task_floor_stack = cp.floor_stack_snapshot;

            self.rollback_binds(cp.undo_len);
            self.push_task(cp.resume);
            true
        } else {
            false
        }
    }

    fn push_task(&mut self, task: Task<'a, A>) {
        self.tasks.push(task);
    }

    fn match_blank(
        &mut self,
        expr: &'a Expr<A>,
        bind_name: Option<&'a str>,
        match_head: Option<&Expr<A>>,
        predicate: Option<PatternPredicate>,
    ) -> Result<(), MatchError> {
        if let Some(expected_head) = match_head {
            match expr {
                Expr::Node { head, .. } => {
                    if head.as_ref() != expected_head {
                        return Err(MatchError::MatchFail);
                    }
                }
                _ => {
                    return Err(MatchError::MatchFail);
                }
            }
        }

        if let Some(n) = bind_name {
            self.bind_one(n, expr)?
        }

        if predicate.as_ref().map(|p| p.check(expr)).unwrap_or(true) {
            Ok(())
        } else {
            Err(MatchError::MatchFail)
        }
    }

    fn match_blank_seq_or_blank_null_seq(
        &mut self,
        exprs: &'a [Expr<A>],
        k_min: usize,
        min_left: usize,
        rest_pats: PatSpan<'a, A>,
        bind_name: Option<&'a str>,
    ) -> Result<(), MatchError> {
        if exprs.len() < k_min {
            return Err(MatchError::MatchFail);
        }

        // A few exprs for remaining patterns
        if exprs.len() < k_min + min_left {
            return Err(MatchError::MatchFail);
        }

        // At most k_max lements in BlankSeq pattern
        let k_max = exprs.len() - min_left;

        if k_min < k_max {
            self.set_choice_point(Task::ResumeOrderedSplit {
                seq_name: bind_name,
                k_min,
                k: k_min + 1,
                rest_pats: rest_pats.clone(),
                rest_exprs: exprs,
            });
        }

        if let Some(name) = bind_name {
            self.bind_seq(name, exprs[..k_min].iter().collect())?;
        }

        self.push_task(Task::MatchSeq {
            patterns: rest_pats,
            exprs: &exprs[k_min..],
        });
        Ok(())
    }

    fn match_blank_seq(
        &mut self,
        exprs: &'a [Expr<A>],
        rest_pats: PatSpan<'a, A>,
        bind_name: Option<&'a str>,
    ) -> Result<(), MatchError> {
        self.match_blank_seq_or_blank_null_seq(
            exprs,
            1,
            Self::min_required(&rest_pats),
            rest_pats,
            bind_name,
        )
    }

    fn match_blank_null_seq(
        &mut self,
        exprs: &'a [Expr<A>],
        rest_pats: PatSpan<'a, A>,
        bind_name: Option<&'a str>,
    ) -> Result<(), MatchError> {
        self.match_blank_seq_or_blank_null_seq(
            exprs,
            0,
            Self::min_required(&rest_pats),
            rest_pats,
            bind_name,
        )
    }

    fn task_match_one(
        &mut self,
        pattern: Pattern<'a, A>,
        expr: &'a Expr<A>,
    ) -> Result<(), MatchError> {
        match pattern {
            Pattern::Literal(p_expr) => {
                if p_expr == expr {
                    Ok(())
                } else {
                    Err(MatchError::MatchFail)
                }
            }
            Pattern::Blank {
                bind_name,
                match_head,
                predicate,
            } => self.match_blank(expr, bind_name, match_head, predicate),
            Pattern::Node {
                head: pat_head,
                args,
                predicate,
            } => {
                if predicate.is_some() {
                    todo!("match_one: predicates not yet supported for Node.")
                }

                if let Expr::Node {
                    head: expr_head,
                    args: expr_args,
                    ..
                } = expr
                {
                    let floor = self.tasks.len();
                    self.task_floor_stack.push(floor);

                    self.push_task(Task::PopTaskFloor);
                    if self.is_commutative_head(expr_head.as_ref()) {
                        self.push_task(Task::MatchUnorderedSeq {
                            patterns: PatSpan::from(args),
                            exprs: expr_args,
                            remaining: (0..expr_args.len()).collect(),
                        });
                    } else {
                        self.push_task(Task::MatchSeq {
                            patterns: PatSpan::from(args),
                            exprs: expr_args,
                        });
                    }

                    self.push_task(Task::MatchOne {
                        pattern: *pat_head,
                        expr: expr_head,
                    });

                    Ok(())
                } else {
                    Err(MatchError::MatchFail)
                }
            }
            Pattern::BlankSeq { .. } | Pattern::BlankNullSeq { .. } => Err(MatchError::MatchFail),
        }
    }

    fn task_match_ordered_seq(
        &mut self,
        patterns: PatSpan<'a, A>,
        exprs: &'a [Expr<A>],
    ) -> Result<(), MatchError> {
        if patterns.is_empty() {
            return if exprs.is_empty() {
                Ok(())
            } else {
                Err(MatchError::MatchFail)
            };
        }

        match patterns.first().unwrap() {
            Pattern::BlankSeq {
                bind_name,
                match_head,
                predicate,
            } => {
                if match_head.is_some() || predicate.is_some() {
                    todo!(
                        "task_match_orderd_seq: head match and predicates not yet supported for BlankSeq"
                    )
                }

                self.match_blank_seq(exprs, patterns.clone().rest(), *bind_name)
            }
            Pattern::BlankNullSeq {
                bind_name,
                match_head,
                predicate,
            } => {
                if match_head.is_some() || predicate.is_some() {
                    todo!(
                        "task_match_orderd_seq: head match and predicates not yet supported for BlankNullSeq"
                    )
                }

                self.match_blank_null_seq(exprs, patterns.clone().rest(), *bind_name)
            }
            Pattern::Literal(_) | Pattern::Node { .. } | Pattern::Blank { .. } => {
                // non-seq: need at least one expr
                let (e0, erest) = exprs.split_first().ok_or(MatchError::MatchFail)?;
                self.push_task(Task::MatchSeq {
                    patterns: patterns.clone().rest(),
                    exprs: erest,
                });
                self.push_task(Task::MatchOne {
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

    fn task_resume_ordered_split(
        &mut self,
        seq_name: Option<&'a str>,
        k_min: usize,
        k: usize,
        rest_pats: PatSpan<'a, A>,
        rest_exprs: &'a [Expr<A>],
    ) -> Result<(), MatchError> {
        let min_left = Self::min_required(&rest_pats);
        let k_max = rest_exprs.len().saturating_sub(min_left);

        if k < k_min || k > k_max {
            return Err(MatchError::MatchFail); // exhausted / invalid
        }

        // If there are further ks, save a choicepoint that will resume with k+1
        if k < k_max {
            self.set_choice_point(Task::ResumeOrderedSplit {
                seq_name,
                k_min,
                k: k + 1,
                rest_pats: rest_pats.clone(),
                rest_exprs,
            });
        }

        // Apply this k
        if let Some(name) = seq_name {
            self.bind_seq(name, rest_exprs[..k].iter().collect())?;
        }

        self.push_task(Task::MatchSeq {
            patterns: rest_pats,
            exprs: &rest_exprs[k..],
        });

        Ok(())
    }

    fn task_match_unordered_seq(
        &mut self,
        patterns: PatSpan<'a, A>,
        exprs: &'a [Expr<A>],
        mut remaining: Vec<usize>,
    ) -> Result<(), MatchError> {
        // For now, we only support at most one sequence pattern in unordered seq.
        let mut literal_exprs: Vec<&'a Expr<A>> = Vec::new();
        let mut blanks: Vec<Pattern<'a, A>> = Vec::new();
        let mut other_nonseq: Vec<Pattern<'a, A>> = Vec::new();
        let mut seq_pat: Option<Pattern<'a, A>> = None;

        for p in patterns.as_slice().iter().cloned() {
            match p {
                Pattern::Literal(e) => literal_exprs.push(e),
                Pattern::Blank { .. } => blanks.push(p),
                Pattern::Node { .. } => other_nonseq.push(p),
                Pattern::BlankSeq { .. } | Pattern::BlankNullSeq { .. } => {
                    if seq_pat.is_some() {
                        todo!("unordered: more than one BlankSeq/BlankNullSeq not supported yet");
                    }
                    seq_pat = Some(p);
                }
            }
        }

        // Pick next from remaining literals
        for lit in literal_exprs {
            let pos = remaining
                .iter()
                .position(|&i| &exprs[i] == lit)
                .ok_or(MatchError::MatchFail)?;
            remaining.swap_remove(pos);
        }

        // Deligate remaining nonseq to next task
        let mut nonseq: Vec<Pattern<'a, A>> = Vec::new();
        nonseq.extend(blanks);
        nonseq.extend(other_nonseq);

        if nonseq.is_empty() {
            return self.finish_unordered_tail(seq_pat, exprs, remaining);
        }

        self.push_task(Task::MatchUnorderedRest {
            patterns_rest: nonseq,
            exprs,
            remaining,
            seq_pat,
        });

        Ok(())
    }

    fn finish_unordered_tail(
        &mut self,
        seq_pat: Option<Pattern<'a, A>>,
        exprs: &'a [Expr<A>],
        remaining: Vec<usize>,
    ) -> Result<(), MatchError> {
        match seq_pat {
            None => {
                if remaining.is_empty() {
                    Ok(())
                } else {
                    Err(MatchError::MatchFail)
                }
            }
            Some(Pattern::BlankSeq {
                bind_name,
                match_head,
                predicate,
            }) => {
                if match_head.is_some() || predicate.is_some() {
                    todo!("unordered BlankSeq with head/predicate not supported yet");
                }
                if remaining.is_empty() {
                    return Err(MatchError::MatchFail); // BlankSeq requires >= 1
                }
                if let Some(name) = bind_name {
                    self.bind_seq(name, remaining.iter().map(|&i| &exprs[i]).collect())?;
                }
                Ok(())
            }
            Some(Pattern::BlankNullSeq {
                bind_name,
                match_head,
                predicate,
            }) => {
                if match_head.is_some() || predicate.is_some() {
                    todo!("unordered BlankNullSeq with head/predicate not supported yet");
                }
                if let Some(name) = bind_name {
                    self.bind_seq(name, remaining.iter().map(|&i| &exprs[i]).collect())?;
                }
                Ok(())
            }
            Some(_) => unreachable!("seq_pat can only be BlankSeq / BlankNullSeq here"),
        }
    }

    fn task_match_unordered_rest(
        &mut self,
        mut patterns_rest: Vec<Pattern<'a, A>>,
        exprs: &'a [Expr<A>],
        mut remaining: Vec<usize>,
        seq_pat: Option<Pattern<'a, A>>,
    ) -> Result<(), MatchError> {
        if patterns_rest.is_empty() {
            return self.finish_unordered_tail(seq_pat, exprs, remaining);
        }

        if remaining.is_empty() {
            return Err(MatchError::MatchFail);
        }

        let pat0 = patterns_rest.remove(0);

        // Choicepoint for alternative expr choices
        if remaining.len() >= 2 {
            self.set_choice_point(Task::ResumeUnorderedPick {
                pat: pat0.clone(),
                patterns_rest: patterns_rest.clone(),
                exprs,
                remaining: remaining.clone(),
                next_choice_pos: 1,
                seq_pat: seq_pat.clone(),
            });
        }

        // Apply choice 0
        let chosen_idx = remaining.remove(0);

        self.push_task(Task::MatchUnorderedRest {
            patterns_rest,
            exprs,
            remaining,
            seq_pat,
        });

        self.push_task(Task::MatchOne {
            pattern: pat0,
            expr: &exprs[chosen_idx],
        });

        Ok(())
    }

    fn task_resume_unordered_pick(
        &mut self,
        pat: Pattern<'a, A>,
        patterns_rest: Vec<Pattern<'a, A>>,
        exprs: &'a [Expr<A>],
        mut remaining: Vec<usize>,
        next_choice_pos: usize,
        seq_pat: Option<Pattern<'a, A>>,
    ) -> Result<(), MatchError> {
        if next_choice_pos >= remaining.len() {
            return Err(MatchError::MatchFail);
        }

        // Save another choicepoint for the next candidate
        if next_choice_pos + 1 < remaining.len() {
            self.set_choice_point(Task::ResumeUnorderedPick {
                pat: pat.clone(),
                patterns_rest: patterns_rest.clone(),
                exprs,
                remaining: remaining.clone(),
                next_choice_pos: next_choice_pos + 1,
                seq_pat: seq_pat.clone(),
            });
        }

        // Apply this choice
        let chosen_idx = remaining.remove(next_choice_pos);

        self.push_task(Task::MatchUnorderedRest {
            patterns_rest,
            exprs,
            remaining,
            seq_pat,
        });

        self.push_task(Task::MatchOne {
            pattern: pat,
            expr: &exprs[chosen_idx],
        });

        Ok(())
    }
}
