use crate::eval::program::{EvalProgram, Instruction};

type Stack = Vec<f64>;

pub struct EvalRuntime<'p> {
    program: &'p EvalProgram,
    bindings: Vec<Option<f64>>,
}

#[derive(Debug)]
pub enum EvalRuntimeError {
    UnknownVariable,
    IndeterminateProgram,
    CorruptedProgram,
}

impl<'p> EvalRuntime<'p> {
    pub fn new(program: &'p EvalProgram) -> Self {
        let var_count = program.vars.len();
        Self {
            program,
            bindings: vec![None; var_count],
        }
    }

    pub fn bind_var<T: AsRef<str>>(&mut self, name: T, value: f64) -> Result<(), EvalRuntimeError> {
        let &id = self
            .program
            .var_ids
            .get(name.as_ref())
            .ok_or(EvalRuntimeError::UnknownVariable)?;
        self.bindings[id as usize] = Some(value);
        Ok(())
    }

    pub fn has_unbound_vars(&self) -> bool {
        self.bindings.iter().any(|b| b.is_none()) && !self.bindings.is_empty()
    }

    pub fn execute(&self) -> Result<f64, EvalRuntimeError> {
        if self.has_unbound_vars() {
            return Err(EvalRuntimeError::IndeterminateProgram);
        }

        let mut stack = Stack::new();

        for instr in self.program.instructions.iter() {
            use Instruction::*;
            match instr {
                PushConstant(x) => stack.push(*x),
                PushVar(i) => self.push_var(&mut stack, *i)?,
                AddMany(count) => Self::add_many(&mut stack, *count)?,
                MulMany(count) => Self::mul_many(&mut stack, *count)?,
                Pow => Self::pow(&mut stack)?,
                _ => Self::unary_op(&mut stack, instr)?,
            }
        }

        Self::pop(&mut stack)
    }

    fn pop(stack: &mut Stack) -> Result<f64, EvalRuntimeError> {
        let Some(val) = stack.pop() else {
            return Err(EvalRuntimeError::CorruptedProgram);
        };

        Ok(val)
    }

    fn push_var(&self, stack: &mut Stack, i: u32) -> Result<(), EvalRuntimeError> {
        let Some(Some(val)) = self.bindings.get(i as usize) else {
            unreachable!()
        };
        stack.push(*val);
        Ok(())
    }

    fn add_many(stack: &mut Stack, count: usize) -> Result<(), EvalRuntimeError> {
        let mut res = 0.0;

        for _ in 0..count {
            res += Self::pop(stack)?;
        }

        stack.push(res);
        Ok(())
    }

    fn mul_many(stack: &mut Stack, count: usize) -> Result<(), EvalRuntimeError> {
        let mut res = 1.0;

        for _ in 0..count {
            res *= Self::pop(stack)?;
        }

        stack.push(res);

        Ok(())
    }

    fn pow(stack: &mut Stack) -> Result<(), EvalRuntimeError> {
        let exponent = Self::pop(stack)?;
        let base = Self::pop(stack)?;

        stack.push(base.powf(exponent));
        Ok(())
    }

    fn unary_op(stack: &mut Stack, instr: &Instruction) -> Result<(), EvalRuntimeError> {
        use Instruction::*;

        let arg = Self::pop(stack)?;
        let res = match instr {
            Exp => arg.exp(),
            Log => arg.ln(),
            Sin => arg.sin(),
            Cos => arg.cos(),
            _ => unimplemented!(),
        };

        stack.push(res);
        Ok(())
    }
}
