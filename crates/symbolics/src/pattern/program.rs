use std::collections::HashMap;

use crate::expr::Expr;

pub type InstrId = usize;
pub type VarId = u32;

pub struct Program<T> {
    pub entry: InstrId,
    pub instructions: Vec<Instruction<T>>,
    pub vars: Vec<String>,
}

pub enum Instruction<T> {
    Literal(T),
    VariadicOne { id: Option<VarId> },
    VariadicMany { id: Option<VarId>, min: usize },
    Compound { plan: ArgPlan<T> },
}

pub enum ArgPlan<T> {
    Sequence(Vec<InstrId>),
    Multiset(MultisetPlan<T>),
}

enum ArgOrder {
    Sequence,
    Multiset,
}

pub struct MultisetPlan<T> {
    pub literals: Vec<T>,
    pub fixed: Vec<InstrId>,
    pub rest: Vec<(VarId, usize)>,
}

pub struct Compiler<A> {
    instructions: Vec<Instruction<A>>,
    var_ids: HashMap<String, VarId>,
    vars: Vec<String>,
}

impl<A: Clone> Compiler<A> {
    fn emit(&mut self, instr: Instruction<A>) -> InstrId {
        let id = self.instructions.len();
        self.instructions.push(instr);
        id
    }

    fn var_id(&mut self, name: &str) -> VarId {
        if let Some(&id) = self.var_ids.get(name) {
            return id;
        }
        let id = self.vars.len() as VarId;
        self.vars.push(name.to_string());
        self.var_ids.insert(name.to_string(), id);
        id
    }

    pub fn compile(mut self, pat: &Expr<A>) -> Program<A> {
        let entry = self.compile_pat(pat);

        Program {
            entry,
            instructions: self.instructions,
            vars: self.vars,
        }
    }

    fn compile_pat(&mut self, pat_expr: &Expr<A>) -> InstrId {
        todo!()
    }

    fn compile_pat_var(&mut self, name: &str) -> InstrId {
        todo!()
    }

    fn compile_pat_node(&mut self, arg_order: ArgOrder, children: &[Expr<A>]) -> InstrId {
        let plan = match arg_order {
            ArgOrder::Sequence => {
                let pats = children
                    .iter()
                    .map(|c| self.compile_pat(c))
                    .collect();
                ArgPlan::Sequence(pats)
            }
            ArgOrder::Multiset => {
                let plan = self.compile_unordered(children);
                ArgPlan::Multiset(plan)
            }
        };

        self.emit(Instruction::Compound { plan })
    }

    fn compile_unordered(&mut self, children: &[Expr<A>]) -> MultisetPlan<A> {
        let mut literals = Vec::new();
        let mut fixed = Vec::new();
        let mut rest: Vec<(VarId, usize)> = vec![];

        for c in children {
            todo!()
        }

        if rest.len() > 1 {
            unimplemented!(
                "Matching unordered children with more than 1 variadic pattern not supported yet"
            )
        }

        MultisetPlan {
            literals,
            fixed,
            rest,
        }
    }
}