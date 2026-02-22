use std::fmt::Debug;

use crate::{
    expr::Expr,
    matcher::{
        context::{MatchContext, MatchContextBindError},
        pattern_span::PatSpan,
    },
    pattern::{Pattern, PatternPredicate},
};

type CommutativePredicate<A> = Box<dyn Fn(&Expr<A>) -> bool>;

enum Task<'a, A> {
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
        next_choice_pos: usize,          // index into remaining vector
        seq_pat: Option<Pattern<'a, A>>, // for now: only support at most one sequence pattern
    },
}

struct ChoicePoint<'a, A> {
    pub todo_len: usize,
    pub undo_len: usize,
    pub resume: Task<'a, A>,
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
    is_commutative: Option<CommutativePredicate<A>>,
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
            is_commutative: None,
        }
    }

    pub fn commutative_if(self, f: CommutativePredicate<A>) -> Self {
        MatchIter {
            is_commutative: Some(f),
            ..self
        }
    }

    fn is_commutative_head(&self, head: &Expr<A>) -> bool {
        self.is_commutative
            .as_ref()
            .map(|f| f(head))
            .unwrap_or(false)
    }

    fn bind_one(&mut self, name: &'a str, expr: &'a Expr<A>) -> Result<(), MatchContextBindError> {
        self.ctx.bind_one(name, expr)?;
        self.bind_action_log.push(name);
        Ok(())
    }

    fn bind_seq(
        &mut self,
        name: &'a str,
        expr_arr: Vec<&'a Expr<A>>,
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
        if let Some(cp) = self.back_track.pop() {
            self.tasks.truncate(cp.todo_len);
            self.rollback_binds(cp.undo_len);
            self.tasks.push(cp.resume);

            true
        } else {
            false
        }
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

    fn match_blank_seq_or_blank_null_seq(
        &mut self,
        exprs: &'a [Expr<A>],
        k_min: usize,
        min_left: usize,
        rest_pats: PatSpan<'a, A>,
        bind_name: Option<&'a str>,
    ) -> Result<(), MatchFail> {
        if exprs.len() < k_min {
            return Err(MatchFail);
        }

        // A few exprs for remaining patterns
        if exprs.len() < k_min + min_left {
            return Err(MatchFail);
        }

        // At most k_max lements in BlankSeq pattern
        let k_max = exprs.len() - min_left;

        if k_min < k_max {
            let cp = ChoicePoint {
                todo_len: self.tasks.len(),
                undo_len: self.bind_action_log.len(),
                resume: Task::ResumeOrderedSplit {
                    seq_name: bind_name,
                    k_min,
                    k: k_min + 1,
                    rest_pats: rest_pats.clone(),
                    rest_exprs: exprs,
                },
            };

            self.back_track.push(cp);
        }

        if let Some(name) = bind_name {
            self.bind_seq(name, exprs[..k_min].iter().collect())
                .map_err(|_| MatchFail)?;
        }

        self.tasks.push(Task::MatchSeq {
            patterns: rest_pats,
            exprs: &exprs[k_min..],
        });
        Ok(())
    }

    fn min_required(pats: &PatSpan<'a, A>) -> usize {
        pats.as_slice()
            .iter()
            .map(|p| match p {
                Pattern::BlankNullSeq { .. } => 0,
                Pattern::BlankSeq { .. }
                | Pattern::Blank { .. }
                | Pattern::Compound { .. }
                | Pattern::Literal { .. } => 1,
            })
            .sum()
    }

    fn match_blank_seq(
        &mut self,
        exprs: &'a [Expr<A>],
        rest_pats: PatSpan<'a, A>,
        bind_name: Option<&'a str>,
    ) -> Result<(), MatchFail> {
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
    ) -> Result<(), MatchFail> {
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
            Pattern::Compound {
                head: pat_head,
                args,
                predicate,
            } => {
                if predicate.is_some() {
                    todo!()
                }

                if let Expr::Compound {
                    head: expr_head,
                    args: expr_args,
                    ..
                } = expr
                {
                    self.tasks.push(Task::MatchOne {
                        pattern: *pat_head,
                        expr: expr_head,
                    });

                    if self.is_commutative_head(expr_head.as_ref()) {
                        self.tasks.push(Task::MatchUnorderedSeq {
                            patterns: PatSpan::from(args),
                            exprs: expr_args,
                            remaining: (0..expr_args.len()).collect(),
                        });
                    } else {
                        self.tasks.push(Task::MatchSeq {
                            patterns: PatSpan::from(args),
                            exprs: expr_args,
                        });
                    }
                    Ok(())
                } else {
                    Err(MatchFail)
                }
            }
            Pattern::BlankSeq { .. } | Pattern::BlankNullSeq { .. } => Err(MatchFail),
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
            Pattern::BlankSeq {
                bind_name,
                match_head,
                predicate,
            } => {
                if match_head.is_some() || predicate.is_some() {
                    todo!()
                }

                self.match_blank_seq(exprs, patterns.clone().rest(), *bind_name)
            }
            Pattern::BlankNullSeq {
                bind_name,
                match_head,
                predicate,
            } => {
                if match_head.is_some() || predicate.is_some() {
                    todo!()
                }

                self.match_blank_null_seq(exprs, patterns.clone().rest(), *bind_name)
            }
            Pattern::Literal { .. } | Pattern::Compound { .. } | Pattern::Blank { .. } => {
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

    fn task_resume_ordered_split(
        &mut self,
        seq_name: Option<&'a str>,
        k_min: usize,
        k: usize,
        rest_pats: PatSpan<'a, A>,
        rest_exprs: &'a [Expr<A>],
    ) -> Result<(), MatchFail> {
        let min_left = Self::min_required(&rest_pats);
        let k_max = rest_exprs.len().saturating_sub(min_left);

        if k < k_min || k > k_max {
            return Err(MatchFail); // exhausted / invalid
        }

        // If there are further ks, save a choicepoint that will resume with k+1
        if k < k_max {
            let cp = ChoicePoint {
                todo_len: self.tasks.len(),
                undo_len: self.bind_action_log.len(),
                resume: Task::ResumeOrderedSplit {
                    seq_name,
                    k_min,
                    k: k + 1,
                    rest_pats: rest_pats.clone(),
                    rest_exprs,
                },
            };
            self.back_track.push(cp);
        }

        // Apply this k
        if let Some(name) = seq_name {
            self.bind_seq(name, rest_exprs[..k].iter().collect())
                .map_err(|_| MatchFail)?;
        }

        self.tasks.push(Task::MatchSeq {
            patterns: rest_pats,
            exprs: &rest_exprs[k..],
        });

        Ok(())
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
                Task::ResumeOrderedSplit {
                    seq_name,
                    k_min,
                    k,
                    rest_pats,
                    rest_exprs,
                } => self.task_resume_ordered_split(seq_name, k_min, k, rest_pats, rest_exprs),
                Task::MatchUnorderedSeq { .. } => todo!(),
                Task::ResumeUnorderedPick { .. } => todo!(),
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
