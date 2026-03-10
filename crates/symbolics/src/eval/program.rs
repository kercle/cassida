use std::collections::HashMap;

use numbers::Number;

use crate::{
    atom::Atom,
    builtin::{
        ADD_HEAD, CANNONICAL_HEAD_COS, CANNONICAL_HEAD_EXP, CANNONICAL_HEAD_LOG,
        CANNONICAL_HEAD_SIN, MUL_HEAD, POW_HEAD,
    },
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
    IntegerOverflow,
    UnevaluableHead(&'static str),
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
                let num = num.to_f64().ok_or(CompileError::IntegerOverflow)?;
                self.instructions.push(Instruction::PushConstant(num));
            }
            ExprKind::Atom {
                entry: Atom::Symbol(name),
            } => {
                let id = self.new_var(name);
                self.instructions.push(Instruction::PushVar(id))
            }
            ExprKind::Atom {
                entry: Atom::StringLiteral(_),
            } => unimplemented!(),
            ExprKind::Node { .. } => return self.compile_node(expr),
        }

        Ok(())
    }

    fn new_var(&mut self, name: &str) -> VarId {
        if let Some(id) = self.var_ids.get(name) {
            return *id;
        }

        let idx = self.vars.len() as VarId;
        self.vars.push(name.to_string());
        self.var_ids.insert(name.to_string(), idx);
        self.instructions.push(Instruction::PushVar(idx));
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

        if head.matches_symbol(ADD_HEAD) {
            self.instructions.push(Instruction::AddMany(args.len()));
        } else if head.matches_symbol(MUL_HEAD) {
            self.instructions.push(Instruction::MulMany(args.len()));
        } else if head.matches_symbol(POW_HEAD) {
            self.instructions.push(Instruction::Pow);
        } else if head.matches_symbol(CANNONICAL_HEAD_EXP) {
            self.instructions.push(Instruction::Exp);
        } else if head.matches_symbol(CANNONICAL_HEAD_LOG) {
            self.instructions.push(Instruction::Log);
        } else if head.matches_symbol(CANNONICAL_HEAD_SIN) {
            self.instructions.push(Instruction::Sin);
        } else if head.matches_symbol(CANNONICAL_HEAD_COS) {
            self.instructions.push(Instruction::Cos);
        }

        Ok(())
    }
}
