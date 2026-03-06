use std::{collections::HashMap, fmt::Debug, rc::Rc};

use crate::{
    expr::Expr,
    pattern::{
        PatternPredicate,
        program::{ArgPlan, InstrId, Instruction, Program, VarId},
        utils::MultisetMatchState,
    },
};

#[derive(Debug)]
struct ChoicePoint<'p, 's, A: Clone + PartialEq> {
    // pub frame_stack_snapshot: Vec<Frame<'p, 's, A>>,
    pub frame_stack: Rc<FrameStack<'p, 's, A>>,
    pub bind_stack_len: usize,
    pub resume_frame: Frame<'p, 's, A>,
}

#[derive(Debug)]
enum FrameStack<'p, 's, A: Clone + PartialEq> {
    Empty,
    More {
        frame: Frame<'p, 's, A>,
        next: Rc<FrameStack<'p, 's, A>>,
    },
}

#[derive(Debug, Clone)]
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
        state: MultisetMatchState,
    },
    ResumeMatchMultiset {
        instrs: &'p [InstrId],
        subjects: &'s [Expr<A>],
        state: MultisetMatchState,
        already_tried_count: usize,
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

#[derive(Clone)]
pub(super) enum EnvBinding<'s, A: Clone + PartialEq> {
    One(&'s Expr<A>),
    Many(Vec<&'s Expr<A>>),
}

#[derive(Debug, Clone)]
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
    // frame_stack: Vec<Frame<'p, 's, A>>,
    frame_stack: Rc<FrameStack<'p, 's, A>>,
    choice_points: Vec<ChoicePoint<'p, 's, A>>,
    bind_stack: Vec<VarId>,
}

impl<'p, 's, A: Clone + PartialEq + Debug> Runtime<'p, 's, A> {
    pub fn new(program: &'p Program<A>, expr: &'s Expr<A>) -> Self {
        Runtime {
            program,
            environment: Environment::new(program),
            // frame_stack: vec![Frame::Exec {
            //     instr: program.entry,
            //     subject: expr,
            // }],
            frame_stack: Rc::new(FrameStack::More {
                frame: Frame::Exec {
                    instr: program.entry,
                    subject: expr,
                },
                next: Rc::new(FrameStack::Empty),
            }),
            choice_points: Vec::new(),
            bind_stack: Vec::new(),
        }
    }

    pub fn first_match(&mut self) -> Option<&Environment<'p, 's, A>> {
        self.next_match()
    }

    pub fn next_match(&mut self) -> Option<&Environment<'p, 's, A>> {
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
            Literal { inner, bind } => self.match_literal(inner, subject, *bind),
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
                    self.push_frame(Frame::BindOne { bind_var, subject });
                }

                match plan {
                    ArgPlan::Sequence(pattern_args) => {
                        self.push_frame(Frame::MatchSequence {
                            instrs: pattern_args.as_slice(),
                            subjects: subject_args,
                        });
                    }
                    ArgPlan::Multiset(pattern_args) => {
                        self.push_frame(Frame::MatchMultiset {
                            instrs: pattern_args.as_slice(),
                            subjects: subject_args,
                            state: MultisetMatchState::new(pattern_args.len(), subject_args.len()),
                        });
                    }
                }

                self.push_frame(Frame::Exec {
                    instr: *head,
                    subject: subject_head,
                });

                true
            }
            Wildcard { head_pattern, bind } => {
                if let Some(&bind_var) = bind.as_ref() {
                    self.push_frame(Frame::BindOne { bind_var, subject });
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
                    self.push_frame(Frame::BindOne { bind_var, subject });
                }

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

    fn stage_match_head_pattern(&mut self, instr: InstrId, subject: &'s Expr<A>) -> bool {
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

    fn test_predicate(&self, subject: &'s Expr<A>, predicate: PatternPredicate) -> bool {
        use PatternPredicate::*;
        match predicate {
            IsNumberQ => subject.is_number(),
            IsSymbolQ => subject.is_symbol(),
        }
    }

    // ---- Literal Matching ----

    fn match_literal(
        &mut self,
        inner: &Expr<A>,
        subject: &'s Expr<A>,
        bind: Option<VarId>,
    ) -> bool {
        // TODO: check hash from Merkle tree first once implemented

        if subject != inner {
            return false;
        }

        if let Some(bind_var) = bind {
            self.bind_one(bind_var, subject)
        } else {
            true
        }
    }

    // ---- Sequence Matching ----

    fn match_sequence(&mut self, instrs: &'p [InstrId], subjects: &'s [Expr<A>]) -> bool {
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
        subjects: &'s [Expr<A>],
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
        subjects: &'s [Expr<A>],
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
        subjects: &'s [Expr<A>],
        head_pattern: &Option<InstrId>,
        bind: &Option<VarId>,
    ) -> bool {
        // Push bind before, so when the stack is popped, this is
        // executed after the head pattern check succeeds.
        if let Some(&bind_var) = bind.as_ref() {
            self.push_frame(Frame::BindSeq {
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

    fn try_split_variadic_subsequence(
        &mut self,
        instrs: &'p [InstrId],
        subjects: &'s [Expr<A>],
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
        subjects: &'s [Expr<A>],
        mut state: MultisetMatchState,
        already_tried_count: usize,
    ) -> bool {
        // Get rid of all literals. If any literal in the pattern doesn't
        // match any subject we abort.

        for (instr_pos, &instr) in instrs.iter().enumerate() {
            if !self.is_literal(instr) || state.is_instruction_set(instr_pos) {
                continue;
            }

            let Some(subject_pos) = ('find_subject: {
                for (subject_pos, subject) in subjects.iter().enumerate() {
                    if !state.is_subject_set(subject_pos) && self.exec(instr, subject) {
                        break 'find_subject Some(subject_pos);
                    }
                }
                None
            }) else {
                // No subject found that matches given literal pattern: abort.
                return false;
            };

            state.set(instr_pos, subject_pos);
        }

        if state.is_instructions_mask_full() {
            // all instructions exhausted
            return state.is_subjects_mask_full();
        }

        let Some(next_instr_pos) = state
            .instructions_index_iter(true)
            .find(|pos| !self.is_variadic(*instrs.get(*pos).unwrap()))
        else {
            // There is possibly a variadic pattern left
            // handle here!
            return self.match_multiset_variadics(instrs, subjects, state, already_tried_count);
        };

        // among the unmatched subjects, take the one after `already_tried_count`
        // since they have already been tried in a previous choicepoint
        let Some(next_subject_pos) = state.subject_index_iter(true).nth(already_tried_count) else {
            return false;
        };

        // Optimization potential: check first if there are even
        // more subjects left to match before pushing choicepoint
        // For now, we just want to get it to work.
        if already_tried_count + 1 < state.count_unmatched_subjects() {
            self.push_choice_point(Frame::ResumeMatchMultiset {
                instrs,
                subjects,
                state: state.clone(),
                already_tried_count: already_tried_count + 1,
            });
        }

        state.set(next_instr_pos, next_subject_pos);

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
        subjects: &'s [Expr<A>],
        state: MultisetMatchState,
        _already_tried_count: usize, // for later use when implementing multiple variadics
    ) -> bool {
        let unmatched_instr_count = state.count_unmatched_instructions();

        if unmatched_instr_count > 1 {
            todo!("More than one variadic pattern in Multiset is not supported yet.");
        }

        if unmatched_instr_count == 0 {
            return state.is_subjects_mask_full();
        }

        let Some(variadic_pos) = state.instructions_index_iter(true).next() else {
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

        if state.count_unmatched_subjects() < min_len {
            return false;
        }

        if let Some(bind_var) = bind {
            let rest = subjects
                .iter()
                .enumerate()
                .filter_map(|(p, e)| {
                    if !state.is_subject_set(p) {
                        Some(e)
                    } else {
                        None
                    }
                })
                .collect();

            self.push_frame(Frame::BindSeq {
                bind_var,
                subjects: rest,
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

    fn push_frame(&mut self, frame: Frame<'p, 's, A>) {
        self.frame_stack = Rc::new(FrameStack::More {
            frame,
            next: self.frame_stack.clone(),
        })
    }

    fn pop_frame(&mut self) -> Option<Frame<'p, 's, A>> {
        use FrameStack::*;
        match self.frame_stack.as_ref() {
            Empty => None,
            More { frame, next } => {
                let frame = frame.clone();
                self.frame_stack = next.clone();
                Some(frame)
            }
        }
    }

    fn is_frame_stack_empty(&self) -> bool {
        matches!(*self.frame_stack, FrameStack::Empty)
    }

    fn push_choice_point(&mut self, resume_frame: Frame<'p, 's, A>) {
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
            self.environment.bindings.remove(&var);
        }

        self.frame_stack = choice_point.frame_stack;
        self.push_frame(choice_point.resume_frame);

        true
    }
}

impl<'p, 's, A> Iterator for Runtime<'p, 's, A>
where
    A: PartialEq + Clone + Debug,
{
    type Item = Environment<'p, 's, A>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.next_match().cloned()
    }
}
