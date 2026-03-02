use crate::{
    dbg_matcher,
    pattern::{
        PatternPredicate,
        program::{ArgPlan, Instruction, Quantity},
    },
};
use std::{
    collections::{HashMap, HashSet, hash_map::Keys},
    fmt::Debug,
};

use crate::{
    expr::Expr,
    pattern::program::{InstrId, Program, VarId},
};

struct ChoicePoint {
    pub frame_stack_len: usize,
    pub bindings: HashSet<VarId>,
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

    fn bound_variables(&self) -> Keys<'_, u32, EnvBinding<'_, A>> {
        self.bindings.keys()
    }

    fn bind_one(&mut self, bind_var: VarId, subject: &'s Expr<A>) -> bool {
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
        match self.bindings.get(&bind_var) {
            Some(EnvBinding::Many(_bound_subject)) => todo!(),
            None => {
                self.bindings.insert(bind_var, EnvBinding::Many(subjects));
                true
            }
            _ => false,
        }
    }
}

pub struct Runtime<'p, 's, A: Clone + PartialEq> {
    program: &'p Program<A>,
    environment: Environment<'p, 's, A>,
    frame_stack: Vec<Frame<'p, 's, A>>,
    choice_points: Vec<ChoicePoint>,
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
        loop {
            let Some(frame) = self.frame_stack.pop() else {
                return Some(&self.environment);
            };

            if !self.step(frame) {
                // todo: choicepoints
                // fail for now
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
            Variadic {
                quantity: Quantity::One,
                head_pattern,
                bind,
            } => {
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
        if instrs.is_empty() || subjects.is_empty() {
            if instrs.is_empty() && subjects.is_empty() {
                return true;
            }

            return false;
        }

        if let Some(rest_start) = self.find_first_var_many(instrs) {
            let Some(rest_end) = self.find_last_var_many(instrs) else {
                return false;
            };

            let front_exact_len = rest_start;
            let back_exact_len = instrs.len() - rest_end - 1;

            if front_exact_len + back_exact_len > subjects.len() {
                return false;
            }

            let rest_match_result = self.match_sequence_rest(
                &instrs[rest_start..=rest_end],
                &subjects[rest_start..subjects.len() - back_exact_len],
            );

            let front_match_result =
                self.match_sequence_exact(&instrs[..front_exact_len], &subjects[..front_exact_len]);
            let back_match_result = self.match_sequence_exact(
                &instrs[rest_end + 1..],
                &subjects[subjects.len() - back_exact_len..],
            );

            front_match_result && back_match_result && rest_match_result
        } else {
            self.match_sequence_exact(instrs, subjects)
        }
    }

    pub fn match_sequence_exact(&mut self, instrs: &'p [InstrId], subjects: &'s [Expr<A>]) -> bool {
        if instrs.len() != subjects.len() {
            return false;
        }

        for (&instr, subject) in instrs.iter().zip(subjects) {
            self.frame_stack.push(Frame::Exec { instr, subject });
        }

        true
    }

    pub fn match_sequence_rest(&mut self, instrs: &'p [InstrId], subjects: &'s [Expr<A>]) -> bool {
        if instrs.len() == 1 {
            let &instr = instrs.first().unwrap();

            let Some(Instruction::Variadic {
                quantity: Quantity::Many { min },
                head_pattern,
                bind,
            }) = self.program.instructions.get(instr)
            else {
                unreachable!("Rest with only one instruction is required to be variadic many");
            };

            if subjects.len() < *min {
                return false;
            }

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
        } else if instrs.len() > 1 {
            todo!("Multiple many-variadics. Require backtracking.")
        } else {
            true
        }
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
                Some(Instruction::Variadic {
                    quantity: Quantity::Many { .. },
                    ..
                }) => return Some(pos),
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
        literals: &[Expr<A>],
        fixed: &[InstrId],
        rest: &[(VarId, usize)],
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

    fn push_choice_point(&mut self) {
        let mut choice_point = ChoicePoint {
            frame_stack_len: self.frame_stack.len(),
            bindings: HashSet::new(),
        };

        for &v_id in self.environment.bound_variables() {
            choice_point.bindings.insert(v_id);
        }

        self.choice_points.push(choice_point);
    }
}
