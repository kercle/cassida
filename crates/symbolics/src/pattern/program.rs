use std::collections::HashMap;
use std::fmt::Debug;

use crate::expr::Expr;
use crate::pattern::{PATTERN_HEAD, builtin::*};

pub type InstrId = usize;
pub type VarId = u32;

pub struct Program<A: Clone + PartialEq> {
    pub(super) entry: InstrId,
    pub(super) instructions: Vec<Instruction<A>>,
    pub(super) vars: Vec<String>,
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

pub enum Instruction<A: Clone + PartialEq> {
    Literal {
        inner: Expr<A>,
        bind: Option<VarId>,
    },
    Variadic {
        quantity: Quantity,
        head_pattern: Option<InstrId>,
        bind: Option<VarId>,
    },
    Predicate {
        predicate: Predicate,
        inner: InstrId,
    },
    Node {
        head: InstrId,
        plan: ArgPlan<A>,
        bind: Option<VarId>,
    },
}

pub enum ArgPlan<A: Clone + PartialEq> {
    Sequence(Vec<InstrId>),
    Multiset(MultisetPlan<A>),
}

#[derive(Debug)]
enum ArgOrder {
    Sequence,
    _Multiset,
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
    pub fn new() -> Self {
        Compiler {
            instructions: Vec::new(),
            var_ids: HashMap::new(),
            vars: Vec::new(),
        }
    }

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

    pub fn compile(mut self, pattern: &Expr<A>) -> Program<A> {
        let entry = self.compile_pat(pattern, None);

        Program {
            entry,
            instructions: self.instructions,
            vars: self.vars,
        }
    }

    fn compile_pat(&mut self, pat_expr: &Expr<A>, bind: Option<VarId>) -> InstrId {
        use Expr::*;
        match pat_expr {
            Atom { .. } => self.emit(Instruction::Literal {
                inner: pat_expr.clone(),
                bind,
            }),
            Node { head, args, .. } if head.matches_symbol(HEAD_BLANK) && args.len() <= 1 => {
                self.compile_blank_with_head_constraint(Quantity::One, args.first(), bind)
            }
            Node { head, args, .. }
                if head.matches_symbol(HEAD_BLANK_SEQUENCE) && args.len() <= 1 =>
            {
                self.compile_blank_with_head_constraint(
                    Quantity::Many { min: 1 },
                    args.first(),
                    bind,
                )
            }
            Node { head, args, .. }
                if head.matches_symbol(HEAD_BLANK_NULL_SEQUENCE) && args.len() <= 1 =>
            {
                self.compile_blank_with_head_constraint(
                    Quantity::Many { min: 0 },
                    args.first(),
                    bind,
                )
            }
            Node { head, args, .. } if pat_expr.is_application_of(PATTERN_HEAD, 2) => {
                let [lhs, rhs] = args.as_slice() else {
                    unreachable!()
                };

                let Some(bind_var_name) = lhs.get_symbol() else {
                    return self.compile_pat_node(head, ArgOrder::Sequence, &args, bind);
                };

                let var_id = self.bind_name_id(bind_var_name);
                self.compile_pat(rhs, Some(var_id))
            }
            Node { head, args, .. } => self.compile_pat_node(head, ArgOrder::Sequence, &args, bind),
        }
    }

    fn compile_blank_with_head_constraint(
        &mut self,
        quantity: Quantity,
        head_pattern: Option<&Expr<A>>,
        bind: Option<VarId>,
    ) -> InstrId {
        let head_pattern = if let Some(e) = head_pattern {
            Some(self.compile_pat(e, None))
        } else {
            None
        };

        self.emit(Instruction::Variadic {
            quantity,
            head_pattern,
            bind,
        })
    }

    fn compile_pat_node(
        &mut self,
        head: &Expr<A>,
        arg_order: ArgOrder,
        children: &[Expr<A>],
        bind: Option<VarId>,
    ) -> InstrId {
        let head = Self::compile_pat(self, head, None);

        let plan = match arg_order {
            ArgOrder::Sequence => {
                let pats = children.iter().map(|c| self.compile_pat(c, bind)).collect();
                ArgPlan::Sequence(pats)
            }
            ArgOrder::_Multiset => {
                let plan = self.compile_unordered(children);
                ArgPlan::Multiset(plan)
            }
        };

        self.emit(Instruction::Node { head, plan, bind })
    }

    fn compile_unordered(&mut self, _children: &[Expr<A>]) -> MultisetPlan<A> {
        todo!()
        // let mut literals = Vec::new();
        // let mut fixed = Vec::new();
        // let mut rest: Vec<(VarId, usize)> = vec![];

        // for c in children {
        //     todo!()
        // }

        // if rest.len() > 1 {
        //     unimplemented!(
        //         "Matching unordered children with more than 1 variadic pattern not supported yet"
        //     )
        // }

        // MultisetPlan {
        //     literals,
        //     fixed,
        //     rest,
        // }
    }
}
