use crate::{
    dbg_matcher,
    pattern::program::{ArgPlan, Instruction},
};
use std::collections::{HashMap, HashSet, hash_map::Keys};

use crate::{
    expr::Expr,
    pattern::program::{InstrId, Program, VarId},
};

struct ChoicePoint {
    pub frame_stack_len: usize,
    pub bindings: HashSet<VarId>,
}

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
}

enum Binding<'s, A: Clone + PartialEq> {
    One(&'s Expr<A>),
    Many(Vec<&'s Expr<A>>),
}

pub struct Environment<'s, A: Clone + PartialEq> {
    bindings: HashMap<VarId, Binding<'s, A>>,
}

impl<'s, A: Clone + PartialEq> Environment<'s, A> {
    fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    fn bound_variables(&self) -> Keys<'_, u32, Binding<'_, A>> {
        self.bindings.keys()
    }

    fn bind_one(&mut self, bind_var: VarId, subject: &'s Expr<A>) -> bool {
        match self.bindings.get(&bind_var) {
            Some(Binding::One(_bound_subject)) => todo!(),
            None => {
                self.bindings.insert(bind_var, Binding::One(subject));
                true
            }
            _ => false,
        }
    }

    fn bind_seq(&mut self, bind_var: VarId, subjects: Vec<&'s Expr<A>>) -> bool {
        match self.bindings.get(&bind_var) {
            Some(Binding::Many(_bound_subject)) => todo!(),
            None => {
                self.bindings.insert(bind_var, Binding::Many(subjects));
                true
            }
            _ => false,
        }
    }
}

pub struct Runtime<'p, 's, A: Clone + PartialEq> {
    program: &'p Program<A>,
    environment: Environment<'s, A>,
    frame_stack: Vec<Frame<'p, 's, A>>,
    choice_points: Vec<ChoicePoint>,
}

impl<'p, 's, A: Clone + PartialEq> Runtime<'p, 's, A> {
    pub fn new(program: &'p Program<A>, expr: &'s Expr<A>) -> Self {
        Runtime {
            program,
            environment: Environment::new(),
            frame_stack: vec![Frame::Exec {
                instr: program.entry,
                subject: expr,
            }],
            choice_points: Vec::new(),
        }
    }

    pub fn next_match(&mut self) -> Option<&Environment<'s, A>> {
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
                if subject.to_hash() != inner.to_hash() {
                    return false;
                }

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
            Variadic { .. } => todo!(),
            Predicate { .. } => todo!(),
        }
    }

    fn match_sequence(&mut self, instrs: &'p [InstrId], subjects: &'s [Expr<A>]) -> bool {
        if instrs.is_empty() || subjects.is_empty() {
            if instrs.is_empty() && subjects.is_empty() {
                return true;
            }

            return false;
        }

        // Continue matching rest...
        self.frame_stack.push(Frame::MatchSequence {
            instrs: &instrs[1..],
            subjects: &subjects[1..],
        });

        // ...after matching first.
        self.frame_stack.push(Frame::Exec {
            instr: *instrs.first().unwrap(),
            subject: subjects.first().unwrap(),
        });

        true
    }

    fn match_multiset(
        &mut self,
        literals: &[Expr<A>],
        fixed: &[InstrId],
        rest: &[(VarId, usize)],
    ) -> bool {
        todo!()
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
