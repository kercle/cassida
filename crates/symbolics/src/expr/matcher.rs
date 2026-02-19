use std::collections::HashMap;

use crate::expr::{Expr, pattern::Pattern};

pub enum Bound<'a, A> {
    One(&'a Expr<A>),
    Seq(&'a [Expr<A>]),
}

enum BindAction {
    Bind(String),
}

#[derive(Clone)]
pub struct Binding<'a, A> {
    expr: &'a Expr<A>,
    rc: u32,
}

impl<'a, A> Binding<'a, A> {
    pub fn new(expr: &'a Expr<A>) -> Self {
        Self { expr, rc: 1 }
    }

    pub fn inc_bindings(&mut self) {
        self.rc += 1;
    }

    pub fn dec_bindings(&mut self) -> Result<(), ()> {
        if self.rc > 0 {
            self.rc -= 1;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn has_no_bindings(&self) -> bool {
        self.rc == 0
    }
}

#[derive(Clone)]
pub struct MatchContext<'a, A> {
    bindings: HashMap<String, Binding<'a, A>>,
}

impl<'a, A> MatchContext<'a, A>
where
    A: PartialEq,
{
    pub fn new() -> Self {
        MatchContext {
            bindings: HashMap::new(),
        }
    }

    pub fn bind<T: AsRef<str>>(&mut self, name: T, expr: &'a Expr<A>) -> Result<(), ()> {
        if let Some(b) = self.bindings.get_mut(name.as_ref()) {
            if b.expr == expr {
                b.inc_bindings();
                Ok(())
            } else {
                Err(())
            }
        } else {
            self.bindings
                .insert(name.as_ref().to_string(), Binding::new(expr));
            Ok(())
        }
    }

    pub fn unbind<T: AsRef<str>>(&mut self, name: T) {
        if let Some(b) = self.bindings.get_mut(name.as_ref()) {
            let _ = b.dec_bindings();
            if b.has_no_bindings() {
                self.bindings.remove(name.as_ref());
            }
        }
    }
}

enum Task<'a, A> {
    Node {
        pattern: &'a Pattern<'a, A>,
        expr: &'a Expr<A>,
    },
    Args {
        patterns: &'a [Pattern<'a, A>],
        exprs: &'a [Expr<A>],
    },
}

struct ChoicePoint<'a, A> {
    todo_len: usize,
    undo_len: usize,

    // data needed to enumerate alternatives:
    seq_name: Option<&'a str>,
    k_next: usize, // next length to try
    // and what remains after the seq pattern:
    rest_pats: &'a [Pattern<'a, A>],
    rest_exprs: &'a [Expr<A>],
}

#[derive(Debug, Clone, Copy)]
struct MatchFail;

pub struct MatchIter<'a, A> {
    todo: Vec<Task<'a, A>>,
    ctx: MatchContext<'a, A>,
    back_track: Vec<ChoicePoint<'a, A>>,
    undo_log: Vec<BindAction>,
    done: bool,
}

impl<'a, A> MatchIter<'a, A>
where
    A: PartialEq + Clone,
{
    pub fn new(expr: &'a Expr<A>, pattern: &'a Pattern<'a, A>) -> Self {
        MatchIter {
            todo: vec![Task::Node { pattern, expr }],
            ctx: MatchContext::new(),
            back_track: Vec::new(),
            undo_log: Vec::new(),
            done: false,
        }
    }

    fn bind_one(&mut self, name: &str, expr: &'a Expr<A>) -> Result<(), ()> {
        self.undo_log.push(BindAction::Bind(name.to_string()));
        self.ctx.bind(name, expr)
    }

    fn rollback_to(&mut self, undo_len: usize) {
        while self.undo_log.len() > undo_len {
            match self.undo_log.pop().unwrap() {
                BindAction::Bind(name) => self.ctx.unbind(name),
            }
        }
    }

    fn backtrack(&mut self) -> bool {
        while let Some(cp) = self.back_track.pop() {
            self.todo.truncate(cp.todo_len);
            self.rollback_to(cp.undo_len);

            // recompute bounds for remaining choices
            let patterns = cp.rest_pats;
            let exprs = cp.rest_exprs;

            let min_left = patterns.len();
            let k_max = exprs.len().saturating_sub(min_left);
            let k = cp.k_next;
            if k < 1 || k > k_max {
                continue; // this choicepoint exhausted, keep popping
            }

            // if there are further ks, push updated choicepoint back
            if k + 1 <= k_max {
                self.back_track.push(ChoicePoint {
                    k_next: k + 1,
                    ..cp
                });
            }

            // apply this k
            if let Some(_name) = cp.seq_name {
                todo!("seq binding support");
            }

            self.todo.push(Task::Args {
                patterns,
                exprs: &exprs[k..],
            });
            return true;
        }
        false
    }

    fn queue_args(
        &mut self,
        patterns: &'a [Pattern<'a, A>],
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

                let k_min = 1;
                let k_max = exprs.len() - min_left;

                // Try the first split k_min now, but save choicepoint for k_min+1..=k_max
                if k_min + 1 <= k_max {
                    self.back_track.push(ChoicePoint {
                        todo_len: self.todo.len(),
                        undo_len: self.undo_log.len(),
                        seq_name: bind_name.as_deref(),
                        k_next: k_min + 1,
                        rest_pats: prest,
                        rest_exprs: exprs,
                    });
                }

                // Apply k = k_min
                // You don’t yet support seq bindings in MatchContext; for now:
                // either (a) extend Bindings to allow slices, or (b) delay and just implement __ without binding.
                // I'd recommend (a). But since your Pattern::BlankSeq example had no name, do (b) for now:
                if let Some(_name) = bind_name {
                    todo!("add seq binding support (slice) to MatchContext");
                }

                self.todo.push(Task::Args {
                    patterns: prest,
                    exprs: &exprs[k_min..],
                });
                Ok(())
            }

            _ => {
                // non-seq: need at least one expr
                let (e0, erest) = exprs.split_first().ok_or(MatchFail)?;
                self.todo.push(Task::Args {
                    patterns: prest,
                    exprs: erest,
                });
                self.todo.push(Task::Node {
                    pattern: p0,
                    expr: e0,
                });
                Ok(())
            }
        }
    }

    fn queue_node(
        &mut self,
        pattern: &'a Pattern<'a, A>,
        expr: &'a Expr<A>,
    ) -> Result<(), MatchFail> {
        match pattern {
            Pattern::Literal(p_expr) => {
                if p_expr == &expr {
                    Ok(())
                } else {
                    Err(MatchFail)
                }
            }
            Pattern::Blank {
                bind_name,
                match_head,
            } => {
                if match_head.is_some() {
                    // match head constraint
                    todo!();
                }

                let b = if let Some(n) = bind_name {
                    self.bind_one(n, expr)
                } else {
                    Ok(())
                };

                if b.is_ok() { Ok(()) } else { Err(MatchFail) }
            }
            Pattern::Compound { head, args } => {
                if let Expr::Compound {
                    head: ehead,
                    args: eargs,
                    ..
                } = expr
                {
                    self.todo.push(Task::Args {
                        patterns: args,
                        exprs: eargs,
                    });
                    self.todo.push(Task::Node {
                        pattern: head,
                        expr: ehead,
                    });
                    Ok(())
                } else {
                    Err(MatchFail)
                }
            }

            // no seq blanks here; see match_args
            Pattern::BlankSeq { .. } => Err(MatchFail),
        }
    }
}

impl<'a, A> Iterator for MatchIter<'a, A>
where
    A: PartialEq + Clone,
{
    type Item = MatchContext<'a, A>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        while let Some(task) = self.todo.pop() {
            let r = match task {
                Task::Node { pattern, expr } => self.queue_node(pattern, expr),
                Task::Args { patterns, exprs } => self.queue_args(patterns, exprs),
            };

            if r.is_err() && !self.backtrack() {
                self.done = true;
                return None;
            }
        }

        // todo empty => full match found!
        let out = self.ctx.clone();

        // IMPORTANT: to find more matches on next call, force backtracking now
        if !self.backtrack() {
            self.done = true;
        }
        return Some(out);
    }
}
