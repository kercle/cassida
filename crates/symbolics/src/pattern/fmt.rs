use std::fmt::{Debug, Formatter};

use crate::pattern::program::{ArgPlan, Instruction, Program, VarId};

impl<A: Clone + PartialEq + Debug> Debug for ArgPlan<A> {
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
            Multiset(_) => todo!(),
        }
    }
}

fn format_bind(bind: &Option<VarId>) -> String {
    if let Some(bind) = bind {
        format!("{{{bind}}}")
    } else {
        String::new()
    }
}

impl<A: Clone + PartialEq + Debug> Debug for Instruction<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Instruction::*;
        match self {
            Literal { inner, bind } => {
                write!(f, "lit{} {inner:?}", format_bind(bind))
            }
            Variadic {
                quantity,
                head_pattern,
                bind,
            } => write!(
                f,
                "var{} quant={quantity:?} head={head_pattern:?}",
                format_bind(bind)
            ),
            Node { head, plan, bind } => {
                write!(f, "node{} head={head:?} plan={plan:?}", format_bind(bind))
            }
            _ => todo!(),
        }
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
