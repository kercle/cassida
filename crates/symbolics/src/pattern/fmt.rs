use std::fmt::{Debug, Formatter};

use crate::pattern::{
    environment::EnvBinding,
    program::{ArgPlan, Instruction, Program, Quantity, VarId},
};

impl Debug for Quantity {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Quantity::One => write!(f, "one"),
            Quantity::Many { min } => write!(f, "many({min})"),
        }
    }
}

impl Debug for ArgPlan {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        use ArgPlan::*;
        match self {
            Sequence(instructions) => {
                let mut leading_char = '[';
                for instr in instructions.iter() {
                    write!(f, "{leading_char}{instr}")?;
                    leading_char = ',';
                }
                if leading_char == '[' {
                    write!(f, "[]")
                } else {
                    write!(f, "]")
                }
            }
            Multiset(instructions) => {
                let mut leading_char = '{';
                for instr in instructions.iter() {
                    write!(f, "{leading_char}{instr}")?;
                    leading_char = ',';
                }
                if leading_char == '{' {
                    write!(f, "{{}}")
                } else {
                    write!(f, "}}")
                }
            }
        }
    }
}

impl<A: Clone + PartialEq + Debug> Debug for Instruction<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Instruction::*;
        match self {
            Literal { inner, bind } => {
                write!(f, "literal{} {inner:?}", format_bind(bind))
            }
            Variadic {
                min_len,
                head_pattern,
                bind,
            } => write!(
                f,
                "variadic{} {min_len} head={head_pattern:?}",
                format_bind(bind)
            ),
            Wildcard { head_pattern, bind } => {
                write!(f, "wildcard{} head={head_pattern:?}", format_bind(bind))
            }
            Node { head, plan, bind } => {
                write!(f, "node{} head={head:?} plan={plan:?}", format_bind(bind))
            }
            Predicate {
                predicate,
                inner,
                bind,
            } => {
                write!(f, "predicate{} {predicate:?} {inner:?}", format_bind(bind))
            }
        }
    }
}

fn format_bind(bind: &Option<VarId>) -> String {
    if let Some(bind) = bind {
        format!("{{bind:{bind}}}")
    } else {
        String::new()
    }
}

impl<A: Clone + PartialEq + Debug> Debug for Program<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "Program:\nEntry point: {:02}", self.entry)?;
        writeln!(f, "Vars:")?;
        for (idx, name) in self.vars.iter().enumerate() {
            writeln!(f, "  [{:02}] {}", idx, name)?;
        }
        writeln!(f, "Instructions:")?;
        for (idx, instr) in self.instructions.iter().enumerate() {
            writeln!(f, "  [{:02}] {:?}", idx, instr)?;
        }

        Ok(())
    }
}

impl<'a, A: Clone + PartialEq + Debug> Debug for EnvBinding<'a, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            EnvBinding::One(s) => write!(f, "{s:?}"),
            EnvBinding::Many(subjects) => {
                let mut leading_char = '[';
                for s in subjects.iter() {
                    write!(f, "{leading_char}{s:?}")?;
                    leading_char = ',';
                }
                if leading_char == '[' {
                    write!(f, "[]")
                } else {
                    write!(f, "]")
                }
            }
        }
    }
}
