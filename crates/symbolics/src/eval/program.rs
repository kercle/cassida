use std::collections::HashMap;

use crate::builtins;
use crate::builtins::traits::BuiltIn;
use crate::{
    atom::Atom,
    expr::{Expr, ExprKind},
};

pub type VarId = u32;

#[derive(Clone, Debug)]
pub enum Instruction {
    PushVar(VarId),
    PushConstant(f64),
    AddMany(usize),
    MulMany(usize),
    Pow,
    Exp,
    Log,
    Sin,
    Cos,
}

pub enum CompileError {
    NumberConversionFailed,
    UnevaluableHead,
}

#[derive(Debug)]
pub struct EvalProgram {
    pub(super) instructions: Vec<Instruction>,
    pub(super) vars: Vec<String>,
    pub(super) var_ids: HashMap<String, VarId>,
}

impl EvalProgram {
    pub fn compile<S>(expr: &Expr<S>) -> Result<Self, CompileError> {
        let mut program = EvalProgram {
            instructions: Vec::new(),
            vars: Vec::new(),
            var_ids: HashMap::new(),
        };

        program.compile_inner(expr).map(|_| program)
    }

    fn compile_inner<S>(&mut self, expr: &Expr<S>) -> Result<(), CompileError> {
        match expr.kind() {
            ExprKind::Atom {
                entry: Atom::Number(num),
            } => {
                let num = num.to_f64().ok_or(CompileError::NumberConversionFailed)?;
                self.instructions.push(Instruction::PushConstant(num));
            }
            ExprKind::Atom {
                entry: Atom::Symbol(name),
            } => {
                let id = self.get_or_create_var_id(name);
                self.instructions.push(Instruction::PushVar(id))
            }
            ExprKind::Atom {
                entry: Atom::StringLiteral(_),
            } => unimplemented!(),
            ExprKind::Node { .. } => return self.compile_node(expr),
        }

        Ok(())
    }

    fn get_or_create_var_id(&mut self, name: &str) -> VarId {
        if let Some(id) = self.var_ids.get(name) {
            return *id;
        }

        let idx = self.vars.len() as VarId;
        self.vars.push(name.to_string());
        self.var_ids.insert(name.to_string(), idx);
        idx
    }

    fn compile_node<S>(&mut self, expr: &Expr<S>) -> Result<(), CompileError> {
        let Some(args) = expr.args() else {
            unreachable!();
        };

        for arg in args {
            self.compile_inner(arg)?;
        }

        let Some(head) = expr.head() else {
            unreachable!();
        };

        if head.matches_symbol(builtins::Add::head()) {
            self.instructions.push(Instruction::AddMany(args.len()));
        } else if head.matches_symbol(builtins::Mul::head()) {
            self.instructions.push(Instruction::MulMany(args.len()));
        } else if head.matches_symbol(builtins::Pow::head()) {
            self.instructions.push(Instruction::Pow);
        } else if head.matches_symbol(builtins::Exp::head()) {
            self.instructions.push(Instruction::Exp);
        } else if head.matches_symbol(builtins::Log::head()) {
            self.instructions.push(Instruction::Log);
        } else if head.matches_symbol(builtins::Sin::head()) {
            self.instructions.push(Instruction::Sin);
        } else if head.matches_symbol(builtins::Cos::head()) {
            self.instructions.push(Instruction::Cos);
        } else {
            return Err(CompileError::UnevaluableHead);
        }

        Ok(())
    }
}
