use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use crate::{
    dbg_matcher,
    expr::Expr,
    pattern::program::{InstrId, Program, VarId},
    pattern::{
        PatternPredicate,
        program::{ArgPlan, Instruction, Quantity},
    },
};

#[derive(Debug)]
struct ChoicePoint<'p, 's, A: Clone + PartialEq> {
    pub frame_stack_len: usize,
    pub bindings: HashSet<VarId>,
    pub resume_frame: Frame<'p, 's, A>,
}

#[derive(Debug)]
enum Frame<'p, 's, A: Clone + PartialEq> {
    Exec {
        instr: InstrId,
        subject: &'s Expr<A>,
    },
    MatchSequence {
        instrs: &'p [InstrId],
        subjects: &'s [Expr<A>],
    },
    ResumeMatchSequence {
        instrs: &'p [InstrId],
        subjects: &'s [Expr<A>],
        first_consume_count: usize,
        first_head_pattern: &'p Option<InstrId>,
        first_bind: &'p Option<VarId>,
    },
    MatchMultiset {
        literals: &'p [Expr<A>],
        fixed: &'p [InstrId],
        rest: &'p [(VarId, usize)],
    },
    BindOne {
        bind_var: VarId,
        subject: &'s Expr<A>,
    },
    BindSeq {
        bind_var: VarId,
        subjects: Vec<&'s Expr<A>>,
    },
    TestPredicate {
        subject: &'s Expr<A>,
        predicate: PatternPredicate,
    },
}

pub enum EnvBinding<'s, A: Clone + PartialEq> {
    One(&'s Expr<A>),
    Many(Vec<&'s Expr<A>>),
}

#[derive(Debug)]
pub struct Environment<'p, 's, A: Clone + PartialEq> {
    bindings: HashMap<VarId, EnvBinding<'s, A>>,
    program: &'p Program<A>,
}

impl<'p, 's, A: Clone + PartialEq> Environment<'p, 's, A> {
    fn new(program: &'p Program<A>) -> Self {
        Self {
            bindings: HashMap::new(),
            program,
        }
    }

    fn bind_one(&mut self, bind_var: VarId, subject: &'s Expr<A>) -> bool {
        // eprintln!("BIND: {bind_var:?}");
        match self.bindings.get(&bind_var) {
            Some(EnvBinding::One(_bound_subject)) => todo!(),
            None => {
                self.bindings.insert(bind_var, EnvBinding::One(subject));
                true
            }
            _ => false,
        }
    }

    fn bind_seq(&mut self, bind_var: VarId, subjects: Vec<&'s Expr<A>>) -> bool {
        // eprintln!("BIND: {bind_var:?}");
        match self.bindings.get(&bind_var) {
            Some(EnvBinding::Many(_bound_subject)) => {
                todo!()
            }
            None => {
                self.bindings.insert(bind_var, EnvBinding::Many(subjects));
                true
            }
            _ => false,
        }
    }

    fn var_id_from_name<T: AsRef<str>>(&self, name: T) -> Option<VarId> {
        self.program.var_ids.get(name.as_ref()).cloned()
    }

    pub fn get_one<T: AsRef<str>>(&self, name: T) -> Option<&'s Expr<A>> {
        use EnvBinding::*;

        let var_id = self.var_id_from_name(name.as_ref())?;

        match self.bindings.get(&var_id)? {
            One(val) => Some(val),
            Many(_) => None,
        }
    }

    pub fn get_seq<T: AsRef<str>>(&self, name: T) -> Option<&[&'s Expr<A>]> {
        use EnvBinding::*;

        let var_id = self.var_id_from_name(name.as_ref())?;

        match self.bindings.get(&var_id)? {
            One(_) => None,
            Many(val) => Some(val.as_slice()),
        }
    }
}

impl<'p, 's, A: Clone + PartialEq + Debug> Environment<'p, 's, A> {
    pub fn dbg_print_bindings(&self) {
        let mut keys: Vec<&VarId> = self.bindings.keys().collect();
        keys.sort();

        for k in keys {
            let v = self.bindings.get(k).unwrap();
            let name = self.program.vars.get(*k as usize).unwrap();
            eprintln!("{name}: {v:?}");
        }
    }
}

pub struct Runtime<'p, 's, A: Clone + PartialEq> {
    program: &'p Program<A>,
    environment: Environment<'p, 's, A>,
    frame_stack: Vec<Frame<'p, 's, A>>,
    choice_points: Vec<ChoicePoint<'p, 's, A>>,
}

impl<'p, 's, A: Clone + PartialEq + Debug> Runtime<'p, 's, A> {
    pub fn new(program: &'p Program<A>, expr: &'s Expr<A>) -> Self {
        Runtime {
            program,
            environment: Environment::new(program),
            frame_stack: vec![Frame::Exec {
                instr: program.entry,
                subject: expr,
            }],
            choice_points: Vec::new(),
        }
    }

    pub fn next_match(&mut self) -> Option<&Environment<'p, 's, A>> {
        if self.frame_stack.is_empty() && !self.backtrack() {
            return None;
        }

        loop {
            let Some(frame) = self.frame_stack.pop() else {
                return Some(&self.environment);
            };

            if self.step(frame) {
                continue;
            }

            if !self.backtrack() {
                return None;
            }
        }
    }

    fn step(&mut self, frame: Frame<'p, 's, A>) -> bool {
        match frame {
            Frame::Exec { instr, subject } => self.exec(instr, subject),
            Frame::MatchSequence { instrs, subjects } => self.match_sequence(instrs, subjects),
            Frame::MatchMultiset {
                literals,
                fixed,
                rest,
            } => self.match_multiset(literals, fixed, rest),
            Frame::BindOne { bind_var, subject } => self.environment.bind_one(bind_var, subject),
            Frame::BindSeq { bind_var, subjects } => self.environment.bind_seq(bind_var, subjects),
            Frame::TestPredicate { subject, predicate } => self.test_predicate(subject, predicate),
            Frame::ResumeMatchSequence {
                instrs,
                subjects,
                first_consume_count,
                first_head_pattern,
                first_bind,
            } => self.match_sequence_multiple_variadic(
                instrs,
                subjects,
                first_consume_count,
                first_head_pattern,
                first_bind,
            ),
        }
    }

    fn exec(&mut self, instr: InstrId, subject: &'s Expr<A>) -> bool {
        dbg_matcher!("exec {instr:02} subject={subject:?}");

        let Some(instr) = self.program.instructions.get(instr) else {
            return false;
        };

        use Instruction::*;
        match instr {
            Literal { inner, bind } => {
                // TODO: check hash from Merkle tree first once implemented

                if subject != inner {
                    return false;
                }

                if let Some(&bind_var) = bind.as_ref() {
                    self.environment.bind_one(bind_var, subject)
                } else {
                    true
                }
            }
            Node { head, plan, bind } => {
                let Expr::Node {
                    head: subject_head,
                    args: subject_args,
                    ..
                } = subject
                else {
                    // subject is an Atom -> no match
                    return false;
                };

                if let Some(&bind_var) = bind.as_ref() {
                    self.frame_stack.push(Frame::BindOne { bind_var, subject });
                }

                match plan {
                    ArgPlan::Sequence(pattern_args) => {
                        self.frame_stack.push(Frame::MatchSequence {
                            instrs: pattern_args.as_slice(),
                            subjects: subject_args,
                        });
                    }
                    ArgPlan::Multiset(plan) => {
                        self.frame_stack.push(Frame::MatchMultiset {
                            literals: plan.literals.as_slice(),
                            fixed: plan.fixed.as_slice(),
                            rest: plan.rest.as_slice(),
                        });
                    }
                }

                self.frame_stack.push(Frame::Exec {
                    instr: *head,
                    subject: subject_head,
                });

                true
            }
            Wildcard { head_pattern, bind } => {
                if let Some(&bind_var) = bind.as_ref() {
                    self.frame_stack.push(Frame::BindOne { bind_var, subject });
                }

                if let Some(head_pattern_instr) = head_pattern {
                    self.match_head_pattern(*head_pattern_instr, subject)
                } else {
                    true
                }
            }
            Variadic { .. } => unreachable!("Variadics with quantity Many not handled here."),
            Predicate {
                predicate,
                inner,
                bind,
            } => {
                if let Some(&bind_var) = bind.as_ref() {
                    self.frame_stack.push(Frame::BindOne { bind_var, subject });
                }

                self.frame_stack.push(Frame::TestPredicate {
                    subject,
                    predicate: *predicate,
                });

                self.frame_stack.push(Frame::Exec {
                    instr: *inner,
                    subject,
                });

                true
            }
        }
    }

    fn match_head_pattern(&mut self, instr: InstrId, subject: &'s Expr<A>) -> bool {
        let Some(head) = subject.head() else {
            // Subject is Atom
            return false;
        };

        self.frame_stack.push(Frame::Exec {
            instr,
            subject: head,
        });

        true
    }

    fn match_sequence(&mut self, instrs: &'p [InstrId], subjects: &'s [Expr<A>]) -> bool {
        if instrs.is_empty() {
            return true;
        }

        let Some(rest_start) = self.find_first_var_many(instrs) else {
            return self.match_sequence_exact(instrs, subjects);
        };
        let Some(rest_end) = self.find_last_var_many(instrs) else {
            return false;
        };

        let front_exact_len = rest_start;
        let back_exact_len = instrs.len() - rest_end - 1;

        if front_exact_len + back_exact_len > subjects.len() {
            return false;
        }

        if front_exact_len == 0 && back_exact_len == 0 {
            // There are no patterns at the start or the end that are either
            // literals or wildcards. Thus sequence starts and ends with
            // variadic pattern. We needs to work through all possible remaining
            // choices.
            // This guarantees that the front and back is already matcheds before
            // we enter backtracking, which improves performance and makes
            // sure that all obvious bindings are in place before pushing
            // choicepoint.

            self.match_sequence_rest(
                &instrs[rest_start..=rest_end],
                &subjects[rest_start..subjects.len() - back_exact_len],
            )
        } else {
            // Defer matching rest of the sequence before we match all
            // deterministic match options.
            // The rest starts and ends with a variadic pattern.

            self.frame_stack.push(Frame::MatchSequence {
                instrs: &instrs[rest_start..=rest_end],
                subjects: &subjects[rest_start..subjects.len() - back_exact_len],
            });

            let front_match_result =
                self.match_sequence_exact(&instrs[..front_exact_len], &subjects[..front_exact_len]);
            let back_match_result = self.match_sequence_exact(
                &instrs[rest_end + 1..],
                &subjects[subjects.len() - back_exact_len..],
            );

            front_match_result && back_match_result
        }
    }

    fn match_sequence_exact(&mut self, instrs: &'p [InstrId], subjects: &'s [Expr<A>]) -> bool {
        if instrs.len() != subjects.len() {
            return false;
        }

        for (&instr, subject) in instrs.iter().zip(subjects) {
            self.frame_stack.push(Frame::Exec { instr, subject });
        }

        true
    }

    fn match_sequence_rest(&mut self, instrs: &'p [InstrId], subjects: &'s [Expr<A>]) -> bool {
        if instrs.is_empty() {
            return true;
        }

        let &instr = instrs.first().unwrap();

        let Some(Instruction::Variadic {
            min_len,
            head_pattern,
            bind,
        }) = self.program.instructions.get(instr)
        else {
            unreachable!("Rest with only one instruction is required to be variadic many");
        };

        if subjects.len() < *min_len {
            return false;
        }

        if instrs.len() == 1 {
            self.match_sequence_single_variadic(subjects, head_pattern, bind)
        } else {
            self.match_sequence_multiple_variadic(instrs, subjects, *min_len, head_pattern, bind)
        }
    }

    fn match_sequence_single_variadic(
        &mut self,
        subjects: &'s [Expr<A>],
        head_pattern: &Option<InstrId>,
        bind: &Option<VarId>,
    ) -> bool {
        if let Some(&bind_var) = bind.as_ref() {
            self.frame_stack.push(Frame::BindSeq {
                bind_var,
                subjects: subjects.iter().collect(),
            });
        }

        if let Some(head_pattern_instr) = head_pattern {
            for subject in subjects {
                if !self.match_head_pattern(*head_pattern_instr, subject) {
                    return false;
                }
            }
        }

        true
    }

    fn match_sequence_multiple_variadic(
        &mut self,
        instrs: &'p [InstrId],
        subjects: &'s [Expr<A>],
        first_seq_len: usize,
        first_head_pattern: &'p Option<InstrId>,
        first_bind: &'p Option<VarId>,
    ) -> bool {
        debug_assert!(instrs.len() >= 2);

        let Some(suffix_min) = self.min_subjects_needed(&instrs[1..]) else {
            return false;
        };

        let required_min_len = first_seq_len + suffix_min;

        if subjects.len() < required_min_len {
            return false;
        }

        if required_min_len < subjects.len() {
            // we can afford to add one more subject to first sequence
            self.push_choice_point(Frame::ResumeMatchSequence {
                instrs,
                subjects,
                first_consume_count: first_seq_len + 1,
                first_head_pattern,
                first_bind,
            });
        }

        let (first_chunk, rest_subjects) = subjects.split_at(first_seq_len);

        self.frame_stack.push(Frame::MatchSequence {
            instrs: &instrs[1..],
            subjects: rest_subjects,
        });

        self.match_sequence_single_variadic(first_chunk, first_head_pattern, first_bind)
    }

    fn min_subjects_needed(&self, instrs: &'p [InstrId]) -> Option<usize> {
        use Instruction::*;

        let mut sum = 0usize;
        for &id in instrs {
            let instr = self.program.instructions.get(id)?;

            if let Variadic { min_len, .. } = instr {
                sum = sum.checked_add(*min_len)?;
            } else {
                sum = sum.checked_add(1)?;
            }
        }
        Some(sum)
    }

    fn find_first_var_many(&mut self, instrs: &'p [InstrId]) -> Option<usize> {
        self.find_var_many(instrs, 0, 1)
    }

    fn find_last_var_many(&mut self, instrs: &'p [InstrId]) -> Option<usize> {
        self.find_var_many(instrs, instrs.len() - 1, -1)
    }

    fn find_var_many(
        &mut self,
        instrs: &'p [InstrId],
        mut pos: usize,
        delta: isize,
    ) -> Option<usize> {
        // As long as we don't encounter a variadic pattern with more
        // that can match multiple subjects, the front matching of
        // the sequence is fully deterministic.

        assert!(
            !instrs.is_empty(),
            "Empty instrs should be handled in match_sequence"
        );

        loop {
            if pos >= instrs.len() {
                return None;
            }

            let instr = instrs[pos];

            match self.program.instructions.get(instr) {
                None => return None,
                Some(Instruction::Variadic { .. }) => return Some(pos),
                _ => {}
            }

            if pos == 0 && delta < 0 {
                return None;
            } else {
                pos = pos.saturating_add_signed(delta);
            }
        }
    }

    fn match_multiset(
        &mut self,
        _literals: &[Expr<A>],
        _fixed: &[InstrId],
        _rest: &[(VarId, usize)],
    ) -> bool {
        todo!()
    }

    fn test_predicate(&self, subject: &'s Expr<A>, predicate: PatternPredicate) -> bool {
        use PatternPredicate::*;
        match predicate {
            IsNumberQ => subject.is_number(),
            IsSymbolQ => subject.is_symbol(),
        }
    }

    fn push_choice_point(&mut self, resume_frame: Frame<'p, 's, A>) {
        let mut choice_point = ChoicePoint {
            frame_stack_len: self.frame_stack.len(),
            bindings: HashSet::new(),
            resume_frame,
        };

        // eprint!("PUSH CHOICEPOINT:");
        for &v_id in self.environment.bindings.keys() {
            // eprint!(" {v_id:?}");
            choice_point.bindings.insert(v_id);
        }
        // eprintln!("");

        self.choice_points.push(choice_point);
    }

    fn backtrack(&mut self) -> bool {
        // eprintln!("BACKTRACK");
        let Some(choice_point) = self.choice_points.pop() else {
            return false;
        };

        let keys: Vec<VarId> = self.environment.bindings.keys().cloned().collect();

        for k in keys {
            if !choice_point.bindings.contains(&k) {
                self.environment.bindings.remove(&k);
            }
        }

        self.frame_stack.truncate(choice_point.frame_stack_len);
        self.frame_stack.push(choice_point.resume_frame);

        true
    }
}
