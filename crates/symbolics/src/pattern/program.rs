use std::collections::HashMap;
use std::fmt::Debug;

use crate::expr::Expr;
use crate::pattern::{PATTERN_HEAD, builtin::*};

pub type InstrId = usize;
pub type VarId = u32;

pub struct Program<A: Clone + PartialEq> {
    pub entry: InstrId,
    pub instructions: Vec<Instruction<A>>,
    pub vars: Vec<String>,
}

#[derive(Debug)]
pub enum Quantity {
    One,
    Many { min: usize },
}

#[derive(Debug)]
pub enum Predicate {
    IsNumberQ,
    IsSymbolQ,
}

#[derive(Debug)]
pub enum Instruction<A: Clone + PartialEq> {
    Literal(Expr<A>),
    Variadic {
        quantity: Quantity,
        head_constraint: Option<Vec<Expr<A>>>,
    },
    Bind(VarId),
    Predicate {
        pred_id: Predicate,
        inner: InstrId,
    },
    Node {
        head: InstrId,
        plan: ArgPlan<A>,
    },
}

#[derive(Debug)]
pub enum ArgPlan<A: Clone + PartialEq> {
    Sequence(Vec<InstrId>),
    Multiset(MultisetPlan<A>),
}

#[derive(Debug)]
enum ArgOrder {
    Sequence,
    Multiset,
}

#[derive(Debug)]
pub struct MultisetPlan<A: Clone + PartialEq> {
    pub literals: Vec<Expr<A>>,
    pub fixed: Vec<InstrId>,
    pub rest: Vec<(VarId, usize)>,
}

pub struct Compiler<A: Clone + PartialEq> {
    instructions: Vec<Instruction<A>>,
    var_ids: HashMap<String, VarId>,
    vars: Vec<String>,
}

impl<A: Clone + PartialEq + Default> Compiler<A> {
    fn emit(&mut self, instr: Instruction<A>) -> InstrId {
        let id = self.instructions.len();
        self.instructions.push(instr);
        id
    }

    fn bind_name_id(&mut self, name: &str) -> VarId {
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
        use Expr::*;
        match pat_expr {
            Atom { .. } => self.emit(Instruction::Literal(pat_expr.clone())),
            Node { head, args, .. } if head.matches_symbol(HEAD_BLANK) => {
                self.emit(Instruction::Variadic {
                    quantity: Quantity::One,
                    head_constraint: None,
                })
            }
            Node { head, args, .. } if head.matches_symbol(HEAD_BLANK_SEQUENCE) => {
                self.emit(Instruction::Variadic {
                    quantity: Quantity::Many { min: 1 },
                    head_constraint: None,
                })
            }
            Node { head, args, .. } if head.matches_symbol(HEAD_BLANK_NULL_SEQUENCE) => {
                self.emit(Instruction::Variadic {
                    quantity: Quantity::Many { min: 0 },
                    head_constraint: None,
                })
            }
            Node { .. } => self.emit(Self::nested_expr_to_instr(pat_expr)),
        }
    }

    fn nested_expr_to_instr(pat_expr: &Expr<A>) -> Instruction<A> {
        use Expr::*;
        match pat_expr {
            Node { head, args, .. } if pat_expr.is_application_of(PATTERN_HEAD, 2) => {
                todo!()
            }
            _ => Instruction::Literal(pat_expr.clone()),
        }
    }

    fn compile_pat_var(&mut self, name: Option<&str>) -> InstrId {
        todo!()
    }

    fn compile_pat_node(
        &mut self,
        head: &Expr<A>,
        arg_order: ArgOrder,
        children: &[Expr<A>],
    ) -> InstrId {
        let head = Self::compile_pat(self, head);

        let plan = match arg_order {
            ArgOrder::Sequence => {
                let pats = children.iter().map(|c| self.compile_pat(c)).collect();
                ArgPlan::Sequence(pats)
            }
            ArgOrder::Multiset => {
                let plan = self.compile_unordered(children);
                ArgPlan::Multiset(plan)
            }
        };

        self.emit(Instruction::Node { head, plan })
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
