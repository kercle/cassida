use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

use crate::expr::Expr;
use crate::pattern::{PATTERN_HEAD, PATTERN_TEST_HEAD, PatternPredicate, builtin::*};

pub type InstrId = usize;
pub type VarId = u32;

pub struct Program<A: Clone + PartialEq> {
    pub(super) entry: InstrId,
    pub(super) instructions: Vec<Instruction<A>>,
    pub(super) vars: Vec<String>,
    pub(super) var_ids: HashMap<String, VarId>,
}

pub enum Quantity {
    One,
    Many { min: usize },
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
        predicate: PatternPredicate,
        inner: InstrId,
        bind: Option<VarId>,
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
pub enum ArgOrder {
    Sequence,
    Multiset,
}

#[derive(Debug)]
pub struct MultisetPlan<A: Clone + PartialEq> {
    pub literals: Vec<Expr<A>>,
    pub fixed: Vec<InstrId>,
    pub rest: Vec<(VarId, usize)>,
}

pub struct Compiler<A, F>
where
    A: Clone + PartialEq,
    F: Fn(&Expr<A>) -> ArgOrder,
{
    instructions: Vec<Instruction<A>>,
    var_ids: HashMap<String, VarId>,
    vars: Vec<String>,
    arg_order_predicate: F,
}

impl<A, F> Compiler<A, F>
where
    A: Clone + PartialEq + Default,
    F: Fn(&Expr<A>) -> ArgOrder,
{
    pub fn new(arg_order_predicate: F) -> Self {
        Compiler {
            instructions: Vec::new(),
            var_ids: HashMap::new(),
            vars: Vec::new(),
            arg_order_predicate,
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
        let entry = self.compile_pattern(pattern, None);

        Program {
            entry,
            instructions: self.instructions,
            vars: self.vars,
            var_ids: self.var_ids,
        }
    }

    fn compile_pattern(&mut self, pat_expr: &Expr<A>, bind: Option<VarId>) -> InstrId {
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
                    return self.compile_node(
                        head,
                        (self.arg_order_predicate)(pat_expr),
                        args,
                        bind,
                    );
                };

                let var_id = self.bind_name_id(bind_var_name);
                self.compile_pattern(rhs, Some(var_id))
            }
            Node { head, args, .. } if pat_expr.is_application_of(PATTERN_TEST_HEAD, 2) => {
                let [lhs, rhs] = args.as_slice() else {
                    unreachable!()
                };

                let Some(predicate_symbol) = rhs.get_symbol() else {
                    return self.compile_node(
                        head,
                        (self.arg_order_predicate)(pat_expr),
                        args,
                        bind,
                    );
                };

                let Ok(predicate) = PatternPredicate::from_str(predicate_symbol) else {
                    // Maybe error reporting instead?

                    return self.compile_node(
                        head,
                        (self.arg_order_predicate)(pat_expr),
                        args,
                        bind,
                    );
                };

                let inner = self.compile_pattern(lhs, None);

                self.emit(Instruction::Predicate {
                    predicate,
                    inner,
                    bind,
                })
            }
            Node { head, args, .. } => {
                self.compile_node(head, (self.arg_order_predicate)(pat_expr), args, bind)
            }
        }
    }

    fn compile_blank_with_head_constraint(
        &mut self,
        quantity: Quantity,
        head_pattern: Option<&Expr<A>>,
        bind: Option<VarId>,
    ) -> InstrId {
        let head_pattern = head_pattern.map(|e| self.compile_pattern(e, None));

        self.emit(Instruction::Variadic {
            quantity,
            head_pattern,
            bind,
        })
    }

    fn compile_node(
        &mut self,
        head: &Expr<A>,
        arg_order: ArgOrder,
        children: &[Expr<A>],
        bind: Option<VarId>,
    ) -> InstrId {
        let head = Self::compile_pattern(self, head, None);

        let plan = match arg_order {
            ArgOrder::Sequence => {
                let pats = children
                    .iter()
                    .map(|c| self.compile_pattern(c, None))
                    .collect();
                ArgPlan::Sequence(pats)
            }
            ArgOrder::Multiset => {
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
