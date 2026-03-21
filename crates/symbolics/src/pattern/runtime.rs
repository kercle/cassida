use std::{fmt::Debug, rc::Rc};

use crate::{
    expr::{ExprKind, NormExpr},
    pattern::{
        PatternPredicate,
        bit_mask::BitMaskArena,
        environment::Environment,
        program::{ArgPlan, InstrId, Instruction, Program, VarId},
        utils::MultisetMatchState,
    },
};

#[derive(Debug)]
struct ChoicePoint<'p, 's> {
    pub frame_stack: Rc<FrameStack<'p, 's>>,
    pub bind_stack_len: usize,
    pub resume_frame: Frame<'p, 's>,
}

#[derive(Debug)]
enum FrameStack<'p, 's> {
    Empty,
    More {
        frame: Frame<'p, 's>,
        next: Rc<FrameStack<'p, 's>>,
    },
}

#[derive(Debug)]
enum Frame<'p, 's> {
    Exec {
        instr: InstrId,
        subject: &'s NormExpr,
    },
    MatchBranches {
        instr: InstrId,
    },
    ResumeNextBranch {
        instr: InstrId,
        next_branch: usize,
    },
    MatchSequence {
        instrs: &'p [InstrId],
        subjects: &'s [NormExpr],
    },
    ResumeMatchSequence {
        instrs: &'p [InstrId],
        subjects: &'s [NormExpr],
        first_consume_count: usize,
        first_head_pattern: &'p Option<InstrId>,
        first_bind: &'p Option<VarId>,
    },
    MatchMultiset {
        instrs: &'p [InstrId],
        subjects: &'s [NormExpr],
        state: MultisetMatchState,
    },
    ResumeMatchMultiset {
        instrs: &'p [InstrId],
        subjects: &'s [NormExpr],
        state: MultisetMatchState,
        already_tried_count: usize,
    },
    BindOne {
        bind_var: VarId,
        subject: &'s NormExpr,
    },
    BindSeq {
        bind_var: VarId,
        subjects: Rc<Vec<&'s NormExpr>>,
    },
    TestPredicate {
        subject: &'s NormExpr,
        predicate: PatternPredicate,
    },
}

impl<'p, 's> Frame<'p, 's> {
    fn deep_clone(&self, bitmask_arena: &mut BitMaskArena) -> Self {
        match self {
            Frame::Exec { instr, subject } => Frame::Exec {
                instr: *instr,
                subject,
            },
            Frame::MatchBranches { instr } => Frame::MatchBranches { instr: *instr },
            Frame::ResumeNextBranch { instr, next_branch } => Frame::ResumeNextBranch {
                instr: *instr,
                next_branch: *next_branch,
            },
            Frame::MatchSequence { instrs, subjects } => Frame::MatchSequence { instrs, subjects },
            Frame::ResumeMatchSequence {
                instrs,
                subjects,
                first_consume_count,
                first_head_pattern,
                first_bind,
            } => Frame::ResumeMatchSequence {
                instrs,
                subjects,
                first_consume_count: *first_consume_count,
                first_head_pattern,
                first_bind,
            },
            Frame::MatchMultiset {
                instrs,
                subjects,
                state,
            } => Frame::MatchMultiset {
                instrs,
                subjects,
                state: state.deep_clone(bitmask_arena),
            },
            Frame::ResumeMatchMultiset {
                instrs,
                subjects,
                state,
                already_tried_count,
            } => Frame::ResumeMatchMultiset {
                instrs,
                subjects,
                state: state.deep_clone(bitmask_arena),
                already_tried_count: *already_tried_count,
            },
            Frame::BindOne { bind_var, subject } => Frame::BindOne {
                bind_var: *bind_var,
                subject,
            },
            Frame::BindSeq { bind_var, subjects } => Frame::BindSeq {
                bind_var: *bind_var,
                subjects: subjects.clone(),
            },
            Frame::TestPredicate { subject, predicate } => Frame::TestPredicate {
                subject,
                predicate: *predicate,
            },
        }
    }
}

pub struct Runtime<'p, 's> {
    program: &'p Program,
    environment: Environment<'p, 's>,
    frame_stack: Rc<FrameStack<'p, 's>>,
    choice_points: Vec<ChoicePoint<'p, 's>>,
    bind_stack: Vec<VarId>,
    bitmask_arena: BitMaskArena,
}

impl<'p, 's> Runtime<'p, 's> {
    pub fn new(program: &'p Program, expr: &'s NormExpr) -> Self {
        Runtime {
            program,
            environment: Environment::new(program),
            frame_stack: Rc::new(FrameStack::More {
                frame: Frame::Exec {
                    instr: program.entry,
                    subject: expr,
                },
                next: Rc::new(FrameStack::Empty),
            }),
            choice_points: Vec::new(),
            bind_stack: Vec::new(),
            bitmask_arena: BitMaskArena::new(),
        }
    }

    pub fn first_match(&mut self) -> Option<&Environment<'p, 's>> {
        self.next_match()
    }

    pub fn next_match(&mut self) -> Option<&Environment<'p, 's>> {
        if self.is_frame_stack_empty() && !self.backtrack() {
            return None;
        }

        loop {
            let Some(frame) = self.pop_frame() else {
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

    fn step(&mut self, frame: Frame<'p, 's>) -> bool {
        use Frame::*;
        match frame {
            Exec { instr, subject } => self.exec(instr, subject),
            MatchBranches { instr } => todo!(),
            ResumeNextBranch { instr, next_branch } => todo!(),
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
                state,
            } => self.match_multiset(instrs, subjects, state, 0),
            ResumeMatchMultiset {
                instrs,
                subjects,
                state,
                already_tried_count,
            } => self.match_multiset(instrs, subjects, state, already_tried_count),
            BindOne { bind_var, subject } => self.bind_one(bind_var, subject),
            BindSeq { bind_var, subjects } => self.bind_seq(bind_var, subjects),
            TestPredicate { subject, predicate } => predicate.check(subject),
        }
    }

    fn exec(&mut self, instr: InstrId, subject: &'s NormExpr) -> bool {
        let instr = self
            .program
            .instructions
            .get(instr)
            .expect("Program is corrupted. Trying to execute non-existent instruction");

        use Instruction::*;
        match instr {
            Literal { inner, bind } => self.match_literal(inner, subject, *bind),
            Alternatives { branches, bind } => {
                todo!()
            }
            Node { head, plan, .. } => {
                let ExprKind::Node {
                    head: subject_head,
                    args: subject_args,
                    ..
                } = subject.kind()
                else {
                    // subject is an Atom -> no match
                    return false;
                };

                self.schedule_bind_one_if_present(instr, subject);

                match plan {
                    ArgPlan::Sequence(pattern_args) => {
                        self.push_frame(Frame::MatchSequence {
                            instrs: pattern_args.as_slice(),
                            subjects: subject_args,
                        });
                    }
                    ArgPlan::Multiset(pattern_args) => {
                        let state = MultisetMatchState::new(
                            &mut self.bitmask_arena,
                            pattern_args.len(),
                            subject_args.len(),
                        );

                        self.push_frame(Frame::MatchMultiset {
                            instrs: pattern_args.as_slice(),
                            subjects: subject_args,
                            state,
                        });
                    }
                }

                self.push_frame(Frame::Exec {
                    instr: *head,
                    subject: subject_head,
                });

                true
            }
            Wildcard { head_pattern, .. } => {
                self.schedule_bind_one_if_present(instr, subject);

                if let Some(head_pattern_instr) = head_pattern {
                    self.stage_match_head_pattern(*head_pattern_instr, subject)
                } else {
                    true
                }
            }
            Variadic { .. } => false, // Dangling variadics don't match anything.
            Predicate {
                predicate, inner, ..
            } => {
                self.schedule_bind_one_if_present(instr, subject);

                self.push_frame(Frame::TestPredicate {
                    subject,
                    predicate: *predicate,
                });

                self.push_frame(Frame::Exec {
                    instr: *inner,
                    subject,
                });

                true
            }
        }
    }

    fn stage_match_head_pattern(&mut self, instr: InstrId, subject: &'s NormExpr) -> bool {
        let Some(head) = subject.head() else {
            // Subject is Atom
            return false;
        };

        self.push_frame(Frame::Exec {
            instr,
            subject: head,
        });

        true
    }

    fn schedule_bind_one_if_present(&mut self, instr: &Instruction, subject: &'s NormExpr) {
        if let Some(bind_var) = instr.bind() {
            self.push_frame(Frame::BindOne { bind_var, subject });
        }
    }

    // ---- Branching ----

    // ---- Literal Matching ----

    fn match_literal(
        &mut self,
        inner: &NormExpr,
        subject: &'s NormExpr,
        bind: Option<VarId>,
    ) -> bool {
        if !Self::expressions_equal(inner, subject) {
            return false;
        }

        if let Some(bind_var) = bind {
            self.bind_one(bind_var, subject)
        } else {
            true
        }
    }

    fn expressions_equal(inner: &NormExpr, subject: &'s NormExpr) -> bool {
        // fingerprint checking is implemented in PartialEq for NormExpr
        subject == inner
    }

    fn literal_instr_matches_expr(&self, instr: InstrId, subject: &'s NormExpr) -> bool {
        match self.program.instructions.get(instr) {
            Some(Instruction::Literal { inner, .. }) => Self::expressions_equal(inner, subject),
            _ => false,
        }
    }

    // ---- Sequence Matching ----

    fn match_sequence(&mut self, instrs: &'p [InstrId], subjects: &'s [NormExpr]) -> bool {
        if instrs.is_empty() {
            return subjects.is_empty();
        }

        let Some(first_variadic_pos) = self.position_first_variadic(instrs) else {
            return self.match_subsequence_only_literals_and_wildcards(instrs, subjects);
        };
        let Some(last_variadic_pos) = self.position_last_variadic(instrs) else {
            return false;
        };

        let front_exact_len = first_variadic_pos;
        let back_exact_len = instrs.len() - last_variadic_pos - 1;

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

            self.match_subsequence_with_variadic_start_and_end(
                &instrs[first_variadic_pos..=last_variadic_pos],
                &subjects[first_variadic_pos..subjects.len() - back_exact_len],
            )
        } else {
            // Defer matching variadics in sequence to after we matched all
            // deterministic match options.

            // queue matching of subsequence starting and ending in variadic.
            self.push_frame(Frame::MatchSequence {
                instrs: &instrs[first_variadic_pos..=last_variadic_pos],
                subjects: &subjects[first_variadic_pos..subjects.len() - back_exact_len],
            });

            // the rest is completely deterministically matchable.
            let front_match_result = self.match_subsequence_only_literals_and_wildcards(
                &instrs[..front_exact_len],
                &subjects[..front_exact_len],
            );

            let back_match_result = self.match_subsequence_only_literals_and_wildcards(
                &instrs[last_variadic_pos + 1..],
                &subjects[subjects.len() - back_exact_len..],
            );

            front_match_result && back_match_result
        }
    }

    fn match_subsequence_only_literals_and_wildcards(
        &mut self,
        instrs: &'p [InstrId],
        subjects: &'s [NormExpr],
    ) -> bool {
        if instrs.len() != subjects.len() {
            return false;
        }

        for (&instr, subject) in instrs.iter().zip(subjects).rev() {
            self.push_frame(Frame::Exec { instr, subject });
        }

        true
    }

    fn match_subsequence_with_variadic_start_and_end(
        &mut self,
        instrs: &'p [InstrId],
        subjects: &'s [NormExpr],
    ) -> bool {
        let Some(&instr) = instrs.first() else {
            // no instructions left. pattern matches only if also
            // subjects are exhausted.
            return subjects.is_empty();
        };

        let Some(Instruction::Variadic {
            min_len,
            head_pattern,
            bind,
        }) = self.program.instructions.get(instr)
        else {
            // Function assumes that instrs starts with variadic.
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

    fn match_single_variadic(
        &mut self,
        subjects: &'s [NormExpr],
        head_pattern: &Option<InstrId>,
        bind: &Option<VarId>,
    ) -> bool {
        // Push bind before, so when the stack is popped, this is
        // executed after the head pattern check succeeds.
        if let Some(&bind_var) = bind.as_ref() {
            self.push_frame(Frame::BindSeq {
                bind_var,
                subjects: Rc::new(subjects.iter().collect()),
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

    fn try_split_variadic_subsequence(
        &mut self,
        instrs: &'p [InstrId],
        subjects: &'s [NormExpr],
        first_variadic_len: usize,
        first_head_pattern: &'p Option<InstrId>,
        first_bind: &'p Option<VarId>,
    ) -> bool {
        debug_assert!(instrs.len() >= 2);

        let suffix_min = self.min_subjects_needed(&instrs[1..]);
        let required_min_len = first_variadic_len + suffix_min;

        if subjects.len() < required_min_len {
            return false;
        }

        if required_min_len < subjects.len() {
            // we can afford to add one more subject to first sequence
            self.push_choice_point(Frame::ResumeMatchSequence {
                instrs,
                subjects,
                first_consume_count: first_variadic_len + 1,
                first_head_pattern,
                first_bind,
            });
        }

        let (first_chunk, rest_subjects) = subjects.split_at(first_variadic_len);

        self.push_frame(Frame::MatchSequence {
            instrs: &instrs[1..],
            subjects: rest_subjects,
        });

        self.match_single_variadic(first_chunk, first_head_pattern, first_bind)
    }

    // ---- Multiset Matching ----

    fn match_multiset(
        &mut self,
        instrs: &'p [InstrId],
        subjects: &'s [NormExpr],
        state: MultisetMatchState,
        already_tried_count: usize,
    ) -> bool {
        // Get rid of all literals. If any literal in the pattern doesn't
        // match any subject we abort.

        for (instr_pos, &instr) in instrs.iter().enumerate() {
            if !self.is_literal(instr) || state.is_instruction_set(&self.bitmask_arena, instr_pos) {
                continue;
            }

            let Some(subject_pos) = ('find_subject: {
                for (subject_pos, subject) in subjects.iter().enumerate() {
                    if state.is_subject_set(&self.bitmask_arena, subject_pos) {
                        continue;
                    }

                    if self.literal_instr_matches_expr(instr, subject) {
                        break 'find_subject Some(subject_pos);
                    }
                }
                None
            }) else {
                // No subject found that matches given literal pattern: abort.
                return false;
            };

            state.set(&mut self.bitmask_arena, instr_pos, subject_pos);

            self.schedule_bind_one_if_present(
                self.program.instructions.get(instr).unwrap(),
                &subjects[subject_pos],
            );
        }

        if state.is_instructions_mask_full(&self.bitmask_arena) {
            // all instructions exhausted
            return state.is_subjects_mask_full(&self.bitmask_arena);
        }

        let Some(next_instr_pos) = state
            .instructions_index_iter(&self.bitmask_arena, true)
            .find(|pos| !self.is_variadic(*instrs.get(*pos).unwrap()))
        else {
            // There is possibly a variadic pattern left
            // handle here!
            return self.match_multiset_variadics(instrs, subjects, state, already_tried_count);
        };

        // among the unmatched subjects, take the one after `already_tried_count`
        // since they have already been tried in a previous choicepoint
        let Some(next_subject_pos) = state
            .subject_index_iter(&self.bitmask_arena, true)
            .nth(already_tried_count)
        else {
            return false;
        };

        // Optimization potential: check first if there are even
        // more subjects left to match before pushing choicepoint
        // For now, we just want to get it to work.
        if already_tried_count + 1 < state.count_unmatched_subjects(&self.bitmask_arena) {
            let state = state.deep_clone(&mut self.bitmask_arena);

            self.push_choice_point(Frame::ResumeMatchMultiset {
                instrs,
                subjects,
                state,
                already_tried_count: already_tried_count + 1,
            });
        }

        state.set(&mut self.bitmask_arena, next_instr_pos, next_subject_pos);

        self.push_frame(Frame::MatchMultiset {
            instrs,
            subjects,
            state,
        });

        self.push_frame(Frame::Exec {
            instr: instrs[next_instr_pos],
            subject: &subjects[next_subject_pos],
        });

        true
    }

    fn match_multiset_variadics(
        &mut self,
        instrs: &'p [InstrId],
        subjects: &'s [NormExpr],
        state: MultisetMatchState,
        _already_tried_count: usize, // for later use when implementing multiple variadics
    ) -> bool {
        let unmatched_instr_count = state.count_unmatched_instructions(&self.bitmask_arena);

        if unmatched_instr_count > 1 {
            todo!("More than one variadic pattern in Multiset is not supported yet.");
        }

        if unmatched_instr_count == 0 {
            return state.is_subjects_mask_full(&self.bitmask_arena);
        }

        let Some(variadic_pos) = state
            .instructions_index_iter(&self.bitmask_arena, true)
            .next()
        else {
            unreachable!()
        };

        let Some(&Instruction::Variadic {
            min_len,
            head_pattern,
            bind,
        }) = self.program.instructions.get(instrs[variadic_pos])
        else {
            unreachable!();
        };

        if head_pattern.is_some() {
            todo!("Head patterns for variadics in multisets not supported yet");
        }

        if state.count_unmatched_subjects(&self.bitmask_arena) < min_len {
            return false;
        }

        if let Some(bind_var) = bind {
            let rest = subjects
                .iter()
                .enumerate()
                .filter_map(|(p, e)| {
                    if !state.is_subject_set(&self.bitmask_arena, p) {
                        Some(e)
                    } else {
                        None
                    }
                })
                .collect();

            self.push_frame(Frame::BindSeq {
                bind_var,
                subjects: Rc::new(rest),
            });
        }

        true
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
                sum += *min_len;
            } else {
                sum += 1;
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

    fn bind_one(&mut self, bind_var: VarId, subject: &'s NormExpr) -> bool {
        match self.environment.bind_one(bind_var, subject) {
            Ok(true) => {
                self.bind_stack.push(bind_var);
                true
            }
            Ok(false) => true,
            Err(_) => false,
        }
    }

    fn bind_seq(&mut self, bind_var: VarId, subjects: Rc<Vec<&'s NormExpr>>) -> bool {
        match self.environment.bind_seq(bind_var, subjects) {
            Ok(true) => {
                self.bind_stack.push(bind_var);
                true
            }
            Ok(false) => true,
            Err(_) => false,
        }
    }

    fn push_frame(&mut self, frame: Frame<'p, 's>) {
        self.frame_stack = Rc::new(FrameStack::More {
            frame,
            next: self.frame_stack.clone(),
        })
    }

    fn pop_frame(&mut self) -> Option<Frame<'p, 's>> {
        use FrameStack::*;
        match self.frame_stack.as_ref() {
            Empty => None,
            More { frame, next } => {
                let frame = frame.deep_clone(&mut self.bitmask_arena);
                self.frame_stack = next.clone();
                Some(frame)
            }
        }
    }

    fn is_frame_stack_empty(&self) -> bool {
        matches!(*self.frame_stack, FrameStack::Empty)
    }

    fn push_choice_point(&mut self, resume_frame: Frame<'p, 's>) {
        let choice_point = ChoicePoint {
            frame_stack: self.frame_stack.clone(),
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
            self.environment.unbind(var);
        }

        self.frame_stack = choice_point.frame_stack;
        self.push_frame(choice_point.resume_frame);

        true
    }
}

impl<'p, 's> Iterator for Runtime<'p, 's> {
    type Item = Environment<'p, 's>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.next_match().cloned()
    }
}
