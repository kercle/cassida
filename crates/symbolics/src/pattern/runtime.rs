use crate::{
    dbg_matcher,
    pattern::program::{ArgPlan, Instruction},
};
use std::collections::HashMap;

use crate::{
    expr::Expr,
    pattern::program::{InstrId, Program, VarId},
};

enum Frame<'p, 's, A: Clone + PartialEq> {
    Exec {
        instr: InstrId,
        subject: &'s Expr<A>,
    },
    MatchSequence {
        instrs: &'p [InstrId],
        subjects: &'s [Expr<A>],
        pattern_index: usize,
        subject_index: usize,
    },
    MatchMultiset {
        instrs: Vec<InstrId>,
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
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }
}

pub struct Runtime<'p, 's, A: Clone + PartialEq> {
    program: &'p Program<A>,
    environment: Environment<'s, A>,
    frames: Vec<Frame<'p, 's, A>>,
    // todo: stacks, choicepoints, etc.
}

impl<'p, 's, A: Clone + PartialEq> Runtime<'p, 's, A> {
    pub fn new(program: &'p Program<A>, expr: &'s Expr<A>) -> Self {
        Runtime {
            program,
            environment: Environment::new(),
            frames: vec![Frame::Exec {
                instr: program.entry,
                subject: expr,
            }],
        }
    }

    pub fn next_match(&mut self) -> Option<&Environment<'s, A>> {
        loop {
            let Some(frame) = self.frames.pop() else {
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
            Frame::MatchSequence {
                instrs,
                subjects,
                pattern_index,
                subject_index,
            } => self.match_sequence(instrs, subjects, pattern_index, subject_index),
            Frame::MatchMultiset { .. } => todo!(),
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

                if subject == inner {
                    self.bind_one(bind, subject)
                } else {
                    false
                }
            }
            Node {
                head,
                plan,
                bind: _bind,
            } => {
                let Expr::Node {
                    head: subject_head,
                    args: subject_args,
                    ..
                } = subject
                else {
                    // subject is an Atom -> no match
                    return false;
                };

                match plan {
                    ArgPlan::Sequence(pattern_args) => {
                        self.frames.push(Frame::MatchSequence {
                            instrs: pattern_args.as_slice(),
                            subjects: subject_args,
                            pattern_index: 0,
                            subject_index: 0,
                        });
                    }
                    ArgPlan::Multiset(_) => todo!(),
                }

                self.frames.push(Frame::Exec {
                    instr: *head,
                    subject: subject_head,
                });

                true
            }
            Variadic { .. } => todo!(),
            Predicate { .. } => todo!(),
        }
    }

    fn match_sequence(
        &mut self,
        instrs: &[InstrId],
        subjects: &[Expr<A>],
        pattern_index: usize,
        subject_index: usize,
    ) -> bool {
        todo!()
    }

    fn bind_one(&mut self, bind: &Option<VarId>, expr: &'s Expr<A>) -> bool {
        todo!()
    }
}
