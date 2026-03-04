use std::{collections::HashMap, fmt::Debug};

use crate::{
    expr::Expr,
    pattern::{
        PatternPredicate,
        bit_mask::BitMask,
        program::{self, ArgPlan, InstrId, Instruction, Program, VarId},
    },
};

#[derive(Debug)]
struct ChoicePoint<'p, 's, A: Clone + PartialEq> {
    pub frame_stack_len: usize,
    pub bind_stack_len: usize,
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
        instrs: &'p [InstrId],
        subjects: &'s [Expr<A>],
        instrs_mask: BitMask,
        subjects_mask: BitMask,
    },
    ResumeMatchMultiset {
        instrs: &'p [InstrId],
        subjects: &'s [Expr<A>],
        instrs_mask: BitMask,
        subjects_mask: BitMask,
        next_unmatched_subject_pos: usize,
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

struct ErrorBindCollision;

impl<'p, 's, A: Clone + PartialEq> Environment<'p, 's, A> {
    fn new(program: &'p Program<A>) -> Self {
        Self {
            bindings: HashMap::new(),
            program,
        }
    }

    fn bind_one(
        &mut self,
        bind_var: VarId,
        subject: &'s Expr<A>,
    ) -> Result<bool, ErrorBindCollision> {
        match self.bindings.get(&bind_var) {
            Some(EnvBinding::One(bound_subject)) => {
                if subject == *bound_subject {
                    Ok(false)
                } else {
                    Err(ErrorBindCollision)
                }
            }
            None => {
                self.bindings.insert(bind_var, EnvBinding::One(subject));
                Ok(true)
            }
            _ => Err(ErrorBindCollision),
        }
    }

    fn bind_seq(
        &mut self,
        bind_var: VarId,
        subjects: Vec<&'s Expr<A>>,
    ) -> Result<bool, ErrorBindCollision> {
        match self.bindings.get(&bind_var) {
            Some(EnvBinding::Many(bound_subjects)) => {
                if bound_subjects.len() != subjects.len() {
                    return Err(ErrorBindCollision);
                }

                let all_equal = bound_subjects.iter().zip(subjects).all(|(a, b)| *a == b);

                if all_equal {
                    Ok(false)
                } else {
                    Err(ErrorBindCollision)
                }
            }
            None => {
                self.bindings.insert(bind_var, EnvBinding::Many(subjects));
                Ok(true)
            }
            _ => Err(ErrorBindCollision),
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
    bind_stack: Vec<VarId>,
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
            bind_stack: Vec::new(),
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
        use Frame::*;
        match frame {
            Exec { instr, subject } => self.exec(instr, subject),
            MatchSequence { instrs, subjects } => self.match_sequence(instrs, subjects),
            ResumeMatchSequence {
                instrs,
                subjects,
                first_consume_count,
                first_head_pattern,
                first_bind,
            } => self.try_split_variadic_subsequence(
                instrs,
                subjects,
                first_consume_count,
                first_head_pattern,
                first_bind,
            ),
            MatchMultiset {
                instrs,
                subjects,
                instrs_mask,
                subjects_mask,
            } => self.match_multiset(instrs, subjects, instrs_mask, subjects_mask, 0),
            ResumeMatchMultiset {
                instrs,
                subjects,
                instrs_mask,
                subjects_mask,
                next_unmatched_subject_pos,
            } => self.match_multiset(
                instrs,
                subjects,
                instrs_mask,
                subjects_mask,
                next_unmatched_subject_pos,
            ),
            BindOne { bind_var, subject } => self.bind_one(bind_var, subject),
            BindSeq { bind_var, subjects } => self.bind_seq(bind_var, subjects),
            TestPredicate { subject, predicate } => self.test_predicate(subject, predicate),
        }
    }

    fn exec(&mut self, instr: InstrId, subject: &'s Expr<A>) -> bool {
        let instr = self
            .program
            .instructions
            .get(instr)
            .expect("Program is corrupted. Trying to execute non-existent instruction");

        use Instruction::*;
        match instr {
            Literal { inner, bind } => {
                // TODO: check hash from Merkle tree first once implemented

                if subject != inner {
                    return false;
                }

                if let Some(&bind_var) = bind.as_ref() {
                    self.bind_one(bind_var, subject)
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
                    ArgPlan::Multiset(pattern_args) => {
                        self.frame_stack.push(Frame::MatchMultiset {
                            instrs: pattern_args.as_slice(),
                            subjects: subject_args,
                            instrs_mask: BitMask::new(pattern_args.len()),
                            subjects_mask: BitMask::new(subject_args.len()),
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
                    self.stage_match_head_pattern(*head_pattern_instr, subject)
                } else {
                    true
                }
            }
            Variadic { .. } => unreachable!("Variadics handled in match_variadic_subsequence."),
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

    fn stage_match_head_pattern(&mut self, instr: InstrId, subject: &'s Expr<A>) -> bool {
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

    fn test_predicate(&self, subject: &'s Expr<A>, predicate: PatternPredicate) -> bool {
        use PatternPredicate::*;
        match predicate {
            IsNumberQ => subject.is_number(),
            IsSymbolQ => subject.is_symbol(),
        }
    }

    // ---- Sequence Matching ----

    fn match_sequence(&mut self, instrs: &'p [InstrId], subjects: &'s [Expr<A>]) -> bool {
        if instrs.is_empty() {
            return subjects.is_empty();
        }

        let Some(rest_start) = self.position_first_variadic(instrs) else {
            return self.match_subsequence_of_literals_and_wildcards(instrs, subjects);
        };
        let Some(rest_end) = self.position_last_variadic(instrs) else {
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

            self.match_variadic_subsequence(
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

            let front_match_result = self.match_subsequence_of_literals_and_wildcards(
                &instrs[..front_exact_len],
                &subjects[..front_exact_len],
            );

            let back_match_result = self.match_subsequence_of_literals_and_wildcards(
                &instrs[rest_end + 1..],
                &subjects[subjects.len() - back_exact_len..],
            );

            front_match_result && back_match_result
        }
    }

    fn match_subsequence_of_literals_and_wildcards(
        &mut self,
        instrs: &'p [InstrId],
        subjects: &'s [Expr<A>],
    ) -> bool {
        if instrs.len() != subjects.len() {
            return false;
        }

        for (&instr, subject) in instrs.iter().zip(subjects).rev() {
            self.frame_stack.push(Frame::Exec { instr, subject });
        }

        true
    }

    fn match_variadic_subsequence(
        &mut self,
        instrs: &'p [InstrId],
        subjects: &'s [Expr<A>],
    ) -> bool {
        if instrs.is_empty() {
            return subjects.is_empty();
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
            // Single variadics are deterministic -> no backtracking
            self.match_single_variadic(subjects, head_pattern, bind)
        } else {
            // Multiple variadics require backtracking
            self.try_split_variadic_subsequence(instrs, subjects, *min_len, head_pattern, bind)
        }
    }

    fn try_split_variadic_subsequence(
        &mut self,
        instrs: &'p [InstrId],
        subjects: &'s [Expr<A>],
        first_seq_len: usize,
        first_head_pattern: &'p Option<InstrId>,
        first_bind: &'p Option<VarId>,
    ) -> bool {
        debug_assert!(instrs.len() >= 2);

        let suffix_min = self.min_subjects_needed(&instrs[1..]);
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

        self.match_single_variadic(first_chunk, first_head_pattern, first_bind)
    }

    fn match_single_variadic(
        &mut self,
        subjects: &'s [Expr<A>],
        head_pattern: &Option<InstrId>,
        bind: &Option<VarId>,
    ) -> bool {
        // Push bind before, so when the stack is popped, this is
        // executed after the head pattern check succeeds.
        if let Some(&bind_var) = bind.as_ref() {
            self.frame_stack.push(Frame::BindSeq {
                bind_var,
                subjects: subjects.iter().collect(),
            });
        }

        // Push head pattern stack to top of frame stack.
        if let Some(head_pattern_instr) = head_pattern {
            for subject in subjects {
                if !self.stage_match_head_pattern(*head_pattern_instr, subject) {
                    return false;
                }
            }
        }

        true
    }

    // ---- Multiset Matching ----

    fn match_multiset(
        &mut self,
        instrs: &'p [InstrId],
        subjects: &'s [Expr<A>],
        mut instrs_mask: BitMask,
        mut subjects_mask: BitMask,
        next_unmatched_subject_pos: usize,
    ) -> bool {
        // Get rid of all literals. If any literal in the pattern does
        // not match any subject, the pattern does not match and we abort.

        for (instr_pos, instr) in instrs.iter().enumerate() {
            if !self.is_literal(*instr) {
                continue;
            }

            let mut found_match = false;
            for (subject_pos, subject) in subjects.iter().enumerate() {
                if self.exec(*instr, subject) {
                    found_match = true;
                    instrs_mask.set(instr_pos);
                    subjects_mask.set(subject_pos);
                    break;
                }
            }

            if !found_match {
                return false;
            }
        }

        if instrs_mask.is_full() {
            // all instructions exhausted

            if subjects_mask.is_full() {
                return true;
            } else {
                return false;
            }
        }

        // Iterate over all unmatched instructions (wildcards and variadics):

        let mut variadic_instr = None;

        let mut unmatched_counter = 0;
        let mut next_subject = None;
        for subj_pos in 0..subjects_mask.capacity() {
            if subjects_mask.is_set(subj_pos) {
                continue;
            }

            if unmatched_counter == next_unmatched_subject_pos {
                next_subject = Some(&subjects[subj_pos])
            }

            unmatched_counter += 1;
        }

        let Some(next_subject) = next_subject else {
            return false;
        };

        for (pos, instr) in instrs.iter().enumerate() {
            if instrs_mask.is_set(pos) {
                continue;
            }

            if self.is_variadic(*instr) {
                // we first match non-variadic and handle those later.
                variadic_instr = Some(self.program.instructions.get(*instr).unwrap());
                continue;
            }

            // unmatched instruction
            if next_unmatched_subject_pos < subjects_mask.count_unmatched() {
                self.push_choice_point(Frame::ResumeMatchMultiset {
                    instrs,
                    subjects,
                    instrs_mask: instrs_mask.clone(),
                    subjects_mask: subjects_mask.clone(),
                    next_unmatched_subject_pos: next_unmatched_subject_pos + 1,
                });
            }

            instrs_mask.set(pos);
            subjects_mask.set(pos);
            self.frame_stack.push(Frame::MatchMultiset {
                instrs,
                subjects,
                instrs_mask,
                subjects_mask,
            });

            self.frame_stack.push(Frame::Exec {
                instr: *instr,
                subject: next_subject,
            });

            dbg!(&self.frame_stack);
            return true;
        }

        match instrs_mask.count_unmatched() {
            0 => subjects_mask.is_full(), // everything matched
            1 if variadic_instr.is_some() => {
                // no predicates or head constraints for now
                // also: tricky to implement with how they are
                // handled now.

                let &Instruction::Variadic {
                    min_len,
                    head_pattern,
                    bind,
                } = variadic_instr.unwrap()
                else {
                    unreachable!();
                };

                if head_pattern.is_some() {
                    todo!("Head patterns for variadics in multisets not supported yet");
                }

                if subjects_mask.count_unmatched() < min_len {
                    return false;
                }

                if let Some(bind_var) = bind {
                    let rest = subjects
                        .iter()
                        .enumerate()
                        .filter_map(|(p, e)| {
                            if subjects_mask.is_set(p) {
                                None
                            } else {
                                Some(e)
                            }
                        })
                        .collect();

                    self.frame_stack.push(Frame::BindSeq {
                        bind_var,
                        subjects: rest,
                    });
                }

                true
            }
            _ => todo!("Multiple variadics in multisets are note supported yet."),
        }
    }

    // ---- Program Queries ----

    fn min_subjects_needed(&self, instrs: &'p [InstrId]) -> usize {
        use Instruction::*;

        let mut sum = 0usize;
        for &id in instrs {
            let instr = self
                .program
                .instructions
                .get(id)
                .expect("Referenced instruction does not exist in program.");

            if let Variadic { min_len, .. } = instr {
                sum = sum + *min_len;
            } else {
                sum = sum + 1;
            }
        }

        sum
    }

    fn position_first_variadic(&self, instrs: &'p [InstrId]) -> Option<usize> {
        instrs.iter().position(|&instr| self.is_variadic(instr))
    }

    fn position_last_variadic(&self, instrs: &'p [InstrId]) -> Option<usize> {
        instrs.iter().rposition(|&instr| self.is_variadic(instr))
    }

    fn is_variadic(&self, instr: InstrId) -> bool {
        matches!(
            self.program.instructions.get(instr),
            Some(Instruction::Variadic { .. })
        )
    }

    fn is_literal(&self, instr: InstrId) -> bool {
        matches!(
            self.program.instructions.get(instr),
            Some(Instruction::Literal { .. })
        )
    }

    // ---- Execution State ----

    fn bind_one(&mut self, bind_var: VarId, subject: &'s Expr<A>) -> bool {
        match self.environment.bind_one(bind_var, subject) {
            Ok(true) => {
                self.bind_stack.push(bind_var);
                true
            }
            Ok(false) => true,
            Err(_) => false,
        }
    }

    fn bind_seq(&mut self, bind_var: VarId, subjects: Vec<&'s Expr<A>>) -> bool {
        match self.environment.bind_seq(bind_var, subjects) {
            Ok(true) => {
                self.bind_stack.push(bind_var);
                true
            }
            Ok(false) => true,
            Err(_) => false,
        }
    }

    fn push_choice_point(&mut self, resume_frame: Frame<'p, 's, A>) {
        let choice_point = ChoicePoint {
            frame_stack_len: self.frame_stack.len(),
            bind_stack_len: self.bind_stack.len(),
            resume_frame,
        };

        self.choice_points.push(choice_point);
    }

    fn backtrack(&mut self) -> bool {
        let Some(choice_point) = self.choice_points.pop() else {
            return false;
        };

        while self.bind_stack.len() > choice_point.bind_stack_len {
            let var = self.bind_stack.pop().unwrap();
            self.environment.bindings.remove(&var);
        }

        self.frame_stack.truncate(choice_point.frame_stack_len);
        self.frame_stack.push(choice_point.resume_frame);

        true
    }
}
