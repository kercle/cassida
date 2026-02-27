use crate::pattern::program::Instruction;
use std::collections::HashMap;

use crate::{
    expr::Expr,
    pattern::program::{InstrId, Program, VarId},
};

enum Frame<'s, A: Clone + PartialEq> {
    Exec {
        instr: InstrId,
        subject: &'s Expr<A>,
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
    frames: Vec<Frame<'s, A>>,
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

    fn bind_one(&mut self, bind: &Option<VarId>, expr: &'s Expr<A>) -> bool {
        todo!()
    }

    fn step(&mut self, frame: Frame<'s, A>) -> bool {
        match frame {
            Frame::Exec { instr, subject } => self.step_exec(instr, subject),
        }
    }

    fn step_exec(&mut self, instr: InstrId, subject: &'s Expr<A>) -> bool {
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
            Node { head, plan, bind } => {
                self.frames.push(Frame::Exec {
                    instr: *head,
                    subject,
                });

                todo!()
            }
            Variadic { .. } => todo!(),
            Predicate { .. } => todo!(),
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
}
