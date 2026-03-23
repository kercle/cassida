use std::fmt::{Debug, Error, Formatter};

use crate::pattern::{
    Pattern,
    environment::EnvBinding,
    program::{ArgPlan, Instruction, Program, Quantity, VarId},
};

impl<'a> Debug for Pattern<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        use Pattern::*;
        match self {
            Literal(e) => write!(f, "Literal{{{e:?}}}"),
            Blank {
                bind_name,
                match_head,
                predicate,
            } => write!(f, "Blank{{{bind_name:?}, {match_head:?}, {predicate:?}}}"),
            BlankSeq {
                bind_name,
                match_head,
                predicate,
            } => write!(
                f,
                "BlankSeq{{{bind_name:?}, {match_head:?}, {predicate:?}}}"
            ),
            BlankNullSeq {
                bind_name,
                match_head,
                predicate,
            } => write!(
                f,
                "BlankNullSeq{{{bind_name:?}, {match_head:?}, {predicate:?}}}"
            ),
            Node {
                head,
                args,
                predicate,
            } => write!(f, "Node{{{head:?}, {args:?}, {predicate:?}}}"),
        }
    }
}

impl Debug for Quantity {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Quantity::One => write!(f, "one"),
            Quantity::Many { min } => write!(f, "many({min})"),
        }
    }
}

impl Debug for ArgPlan {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
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

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        use Instruction::*;
        match self {
            Literal { inner, bind } => {
                write!(f, "literal{} {inner:?}", format_bind(bind))
            }
            Alternatives { branches } => {
                write!(f, "alternatives {branches:?}")
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
            CheckCondition { inner, test_expr } => {
                write!(f, "condition inner={inner:?} predicate={test_expr:?}")
            }
            With { bind, value, next } => {
                write!(
                    f,
                    "with {}:={value:?} goto {next}",
                    format_bind(&Some(*bind))
                )
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

impl Debug for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
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

impl<'a> Debug for EnvBinding<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
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
