use std::{
    collections::{HashMap, hash_map::Iter as HashMapIter},
    fmt::{Debug, Formatter},
};

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

#[derive(Clone, Debug)]
pub struct BindingDecError;

impl<'a, A: Clone + Debug + PartialEq> Debug for Binding<'a, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", *self.expr)
    }
}

impl<'a, A> Binding<'a, A> {
    pub fn new(expr: &'a Expr<A>) -> Self {
        Self { expr, rc: 1 }
    }

    pub fn inc_bindings(&mut self) {
        self.rc += 1;
    }

    pub fn dec_bindings(&mut self) -> Result<(), BindingDecError> {
        if self.rc > 0 {
            self.rc -= 1;
            Ok(())
        } else {
            Err(BindingDecError)
        }
    }

    pub fn has_no_bindings(&self) -> bool {
        self.rc == 0
    }

    pub fn get_expr(&self) -> &'a Expr<A> {
        self.expr
    }
}

#[derive(Clone, Debug, Default)]
pub struct MatchContext<'a, A>
where
    A: PartialEq + Clone,
{
    bindings: HashMap<String, Binding<'a, A>>,
}

#[derive(Clone, Debug)]
pub struct MatchContextBindError;

impl<'a, A> MatchContext<'a, A>
where
    A: PartialEq + Clone + Debug,
{
    pub fn bind<T: AsRef<str>>(
        &mut self,
        name: T,
        expr: &'a Expr<A>,
    ) -> Result<(), MatchContextBindError> {
        if let Some(b) = self.bindings.get_mut(name.as_ref()) {
            if b.expr == expr {
                b.inc_bindings();
                Ok(())
            } else {
                Err(MatchContextBindError)
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

    pub fn get<T: AsRef<str>>(&self, name: T) -> Option<&'a Expr<A>> {
        if let Some(e) = self.bindings.get(name.as_ref()) {
            Some(e.expr)
        } else {
            None
        }
    }

    pub fn iter(&self) -> HashMapIter<'_, String, Binding<'_, A>> {
        // self.bindings.iter().map(|(k,b)| (k,b))
        self.bindings.iter()
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

pub struct MatchIter<'a, A>
where
    A: PartialEq + Clone,
{
    todo: Vec<Task<'a, A>>,
    ctx: MatchContext<'a, A>,
    back_track: Vec<ChoicePoint<'a, A>>,
    undo_log: Vec<BindAction>,
    done: bool,
}

impl<'a, A> MatchIter<'a, A>
where
    A: Default + PartialEq + Clone + Debug,
{
    pub fn new(expr: &'a Expr<A>, pattern: &'a Pattern<'a, A>) -> Self {
        MatchIter {
            todo: vec![Task::Node { pattern, expr }],
            ctx: MatchContext::default(),
            back_track: Vec::new(),
            undo_log: Vec::new(),
            done: false,
        }
    }

    fn bind_one(&mut self, name: &str, expr: &'a Expr<A>) -> Result<(), MatchContextBindError> {
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
            if k < k_max {
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
                if k_min < k_max {
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
                predicate,
            } => {
                if match_head.is_some() {
                    // match head constraint
                    todo!();
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
    A: Default + PartialEq + Clone + Debug,
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

        Some(out)
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

    fn collect<'a>(expr: &'a Expr<()>, pat: &'a Pattern<'a, ()>) -> Vec<MatchContext<'a, ()>> {
        MatchIter::new(expr, pat).collect()
    }

    fn pat<'a>(e: &'a Expr<()>) -> Pattern<'a, ()> {
        Pattern::from_expr(e)
    }

    fn one<'a>(expr: &'a Expr<()>, pat: &'a Pattern<'a, ()>) -> MatchContext<'a, ()> {
        let m = collect(expr, pat);
        assert_eq!(m.len(), 1, "expected exactly 1 match, got {}", m.len());
        m.into_iter().next().unwrap()
    }

    #[test]
    fn test_expr_matching() {
        let x = symbol!("x");

        let expr1 = x + 1;
        let expr2 = blank(None) + pattern("a", blank(None));
        let pat = Pattern::from_expr(&expr2);
        let matches: Vec<MatchContext<'_, ()>> = MatchIter::new(&expr1, &pat).into_iter().collect();

        assert!(matches.len() == 1);
        assert_eq!(matches.first().unwrap().get("a"), Some(1.into()).as_ref());
    }

    #[test]
    fn literal_and_structure() {
        let x = symbol!("x");
        let e = x + 1;

        // equals
        let p1e = x + 1;
        let p1 = pat(&p1e);
        assert_eq!(collect(&e, &p1).len(), 1);

        // literal mismatch
        let p2e = x + 2;
        let p2 = pat(&p2e);
        assert_eq!(collect(&e, &p2).len(), 0);

        // head mismatch
        let p3e = x * 1;
        let p3 = pat(&p3e);
        assert_eq!(collect(&e, &p3).len(), 0);

        // arity mismatch
        let p4e = x + 1 + 2;
        let p4 = pat(&p4e);
        assert_eq!(collect(&e, &p4).len(), 0);
    }

    #[test]
    fn blanks_and_named_blanks() {
        let x = symbol!("x");

        let e = x + 1;
        let pe = blank(None) + pattern("a", blank(None));
        let p = pat(&pe);
        let m = one(&e, &p);
        assert_eq!(m.get("a"), Some(1.into()).as_ref());

        let pe2 = blank(None) + blank(None);
        let p2 = pat(&pe2);
        let m2 = one(&e, &p2);
        assert!(m2.get("a").is_none());

        let e31 = Expr::from_i64(1) + 1;
        let pe3 = pattern("a", blank(None)) + pattern("a", blank(None));
        let p3 = pat(&pe3);
        let m31 = one(&e31, &p3);
        assert_eq!(m31.get("a"), Some(1.into()).as_ref());

        let e32 = Expr::from_i64(1) + 2;
        assert_eq!(collect(&e32, &p3).len(), 0);

        let y = symbol!("y");
        let e41 = h(x, x);
        let e42 = h(x, y);
        let pe4 = h(pattern("a", blank(None)), pattern("a", blank(None)));
        let p4 = pat(&pe4);
        assert_eq!(collect(&e41, &p4).len(), 1);
        assert_eq!(collect(&e42, &p4).len(), 0);
    }

    #[test]
    fn blank_seq_unnamed_basic() {
        // Add[1, __, 3] against 1+2+3 -> 1 match
        let e1 = flatten(Expr::from_i64(1) + 2 + 3);
        let pe = flatten(1 + blank_sequence(None) + 3);
        let p = pat(&pe);
        assert_eq!(collect(&e1, &p).len(), 1);

        // Add[1, __, 4, __] against 1+2+4+3+4+5 -> 2 matches
        let e2 = flatten(Expr::from_i64(1) + 2 + 4 + 3 + 4 + 5);
        let pe2 = flatten(1 + blank_sequence(None) + 4 + blank_sequence(None));
        let p2 = pat(&pe2);
        assert_eq!(collect(&e2, &p2).len(), 2);

        // should not match 1+3
        let e3 = Expr::from_i64(1) + 3;
        assert_eq!(collect(&e3, &p).len(), 0);
    }
}
