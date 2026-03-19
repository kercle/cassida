use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

use crate::builtin::*;
use crate::expr::pool::{ExprPool, ExprView, NormArgsHandle, NormExprHandle};
use crate::expr::walk::ExprHandleTopDownWalker;
use crate::pattern::{PatternPredicate, builtin::*};

pub type InstrId = usize;
pub type VarId = u32;

pub struct Program {
    pub(super) entry: InstrId,
    pub(super) instructions: Vec<Instruction>,
    pub(super) vars: Vec<String>,
    pub(super) var_ids: HashMap<String, VarId>,
}

pub enum Quantity {
    One,
    Many { min: usize },
}

pub enum Instruction {
    Literal {
        inner: NormExprHandle,
        bind: Option<VarId>,
    },
    Variadic {
        min_len: usize,
        head_pattern: Option<InstrId>,
        bind: Option<VarId>,
    },
    Wildcard {
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
        plan: ArgPlan,
        bind: Option<VarId>,
    },
}

impl Instruction {
    pub fn bind(&self) -> Option<VarId> {
        use Instruction::*;
        match self {
            Literal { bind, .. } => *bind,
            Variadic { bind, .. } => *bind,
            Wildcard { bind, .. } => *bind,
            Predicate { bind, .. } => *bind,
            Node { bind, .. } => *bind,
        }
    }
}

pub enum ArgPlan {
    Sequence(Vec<InstrId>),
    Multiset(Vec<InstrId>),
}

#[derive(Debug)]
pub enum ArgOrder {
    Sequence,
    Multiset,
}

pub struct MultisetPlan {
    pub literals: Vec<NormExprHandle>,
    pub fixed: Vec<InstrId>,
    pub rest: Vec<(VarId, usize)>,
}

pub struct Compiler<'ep> {
    instructions: Vec<Instruction>,
    var_ids: HashMap<String, VarId>,
    vars: Vec<String>,
    is_multiset: fn(pool: &ExprPool, expr: NormExprHandle) -> bool,
    pool: &'ep ExprPool,
}

fn is_multiset_default(pool: &ExprPool, expr: NormExprHandle) -> bool {
    expr.view(pool).is_node(pool, ADD_HEAD, None) || expr.view(pool).is_node(pool, MUL_HEAD, None)
}

impl<'ep> Compiler<'ep> {
    pub fn new(pool: &'ep ExprPool) -> Self {
        Self {
            instructions: Vec::new(),
            var_ids: HashMap::new(),
            vars: Vec::new(),
            is_multiset: is_multiset_default,
            pool,
        }
    }

    pub fn with_multiset_predicate(
        mut self,
        f: fn(pool: &ExprPool, expr: NormExprHandle) -> bool,
    ) -> Self {
        self.is_multiset = f;
        self
    }

    pub fn compile(mut self, pattern: NormExprHandle) -> Program {
        let entry = self.compile_pattern(pattern, None);

        Program {
            entry,
            instructions: self.instructions,
            vars: self.vars,
            var_ids: self.var_ids,
        }
    }

    fn emit(&mut self, instr: Instruction) -> InstrId {
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

    fn compile_pattern(&mut self, pat_expr: NormExprHandle, bind: Option<VarId>) -> InstrId {
        use ExprView::*;

        match pat_expr.view(self.pool) {
            Atom { .. } => self.emit(Instruction::Literal {
                inner: pat_expr.clone(),
                bind,
            }),
            Node { args, .. } if self.is_blank(pat_expr) => {
                self.compile_blank_with_head_constraint(Quantity::One, args.get(self.pool, 0), bind)
            }
            Node { args, .. } if self.is_blank_seq(pat_expr) => self
                .compile_blank_with_head_constraint(
                    Quantity::Many { min: 1 },
                    args.get(self.pool, 0),
                    bind,
                ),
            Node { args, .. } if self.is_blank_null_seq(pat_expr) => self
                .compile_blank_with_head_constraint(
                    Quantity::Many { min: 0 },
                    args.get(self.pool, 0),
                    bind,
                ),
            Node { args, .. } if self.is_pattern(pat_expr) => {
                let lhs = args.get(self.pool, 0).unwrap();
                let rhs = args.get(self.pool, 1).unwrap();

                // Unwrap is safe here: guaranteed by is_pattern
                let lhs = lhs.view(self.pool);
                let bind_var_name = lhs.get_symbol().unwrap();

                let var_id = self.bind_name_id(bind_var_name);
                self.compile_pattern(rhs, Some(var_id))
            }
            Node { head, args } if self.is_pattern_test(pat_expr) => {
                let lhs = args.get(self.pool, 0).unwrap();
                let rhs = args.get(self.pool, 1).unwrap();

                // Unwrap is safe here: guaranteed by is_pattern_test
                let rhs = rhs.view(self.pool);
                let predicate_symbol = rhs.get_symbol().unwrap();

                let Ok(predicate) = PatternPredicate::from_str(predicate_symbol) else {
                    // Maybe error reporting instead?

                    return self.compile_node(head, self.arg_order(pat_expr), args, bind);
                };

                let inner = self.compile_pattern(lhs, None);

                self.emit(Instruction::Predicate {
                    predicate,
                    inner,
                    bind,
                })
            }
            Node { head, args } => {
                if self.is_literal(pat_expr) {
                    self.emit(Instruction::Literal {
                        inner: pat_expr.clone(),
                        bind,
                    })
                } else {
                    self.compile_node(head, self.arg_order(pat_expr), args, bind)
                }
            }
        }
    }

    fn compile_blank_with_head_constraint(
        &mut self,
        quantity: Quantity,
        head_pattern: Option<NormExprHandle>,
        bind: Option<VarId>,
    ) -> InstrId {
        let head_pattern = head_pattern.map(|e| self.compile_pattern(e, None));

        match quantity {
            Quantity::Many { min } => self.emit(Instruction::Variadic {
                min_len: min,
                head_pattern,
                bind,
            }),
            Quantity::One => self.emit(Instruction::Wildcard { head_pattern, bind }),
        }
    }

    fn compile_node(
        &mut self,
        head: NormExprHandle,
        arg_order: ArgOrder,
        children: NormArgsHandle,
        bind: Option<VarId>,
    ) -> InstrId {
        let head = Self::compile_pattern(self, head, None);

        let pats = children
            .iter(self.pool)
            .map(|c| self.compile_pattern(c, None))
            .collect();
        let plan = match arg_order {
            ArgOrder::Sequence => ArgPlan::Sequence(pats),
            ArgOrder::Multiset => ArgPlan::Multiset(pats),
        };

        self.emit(Instruction::Node { head, plan, bind })
    }

    fn arg_order(&self, expr: NormExprHandle) -> ArgOrder {
        if (self.is_multiset)(self.pool, expr) {
            ArgOrder::Multiset
        } else {
            ArgOrder::Sequence
        }
    }

    fn is_blank(&self, expr: NormExprHandle) -> bool {
        if let ExprView::Node { head, args } = expr.view(self.pool) {
            head.view(self.pool).is_symbol(HEAD_BLANK) && args.len(self.pool) <= 1
        } else {
            false
        }
    }

    fn is_blank_seq(&self, expr: NormExprHandle) -> bool {
        if let ExprView::Node { head, args } = expr.view(self.pool) {
            head.view(self.pool).is_symbol(HEAD_BLANK_SEQUENCE) && args.len(self.pool) <= 1
        } else {
            false
        }
    }

    fn is_blank_null_seq(&self, expr: NormExprHandle) -> bool {
        if let ExprView::Node { head, args } = expr.view(self.pool) {
            head.view(self.pool).is_symbol(HEAD_BLANK_NULL_SEQUENCE) && args.len(self.pool) <= 1
        } else {
            false
        }
    }

    fn is_pattern(&self, expr: NormExprHandle) -> bool {
        if let ExprView::Node { head, args } = expr.view(self.pool) {
            head.view(self.pool).is_symbol(HEAD_PATTERN)
                && args.len(self.pool) == 2
                && args
                    .get(self.pool, 1)
                    .unwrap()
                    .view(self.pool)
                    .get_symbol()
                    .is_some()
        } else {
            false
        }
    }

    fn is_pattern_test(&self, expr: NormExprHandle) -> bool {
        if let ExprView::Node { head, args } = expr.view(self.pool) {
            head.view(self.pool).is_symbol(HEAD_PATTERN_TEST)
                && args.len(self.pool) == 2
                && args
                    .get(self.pool, 1)
                    .unwrap()
                    .view(self.pool)
                    .get_symbol()
                    .is_some()
        } else {
            false
        }
    }

    fn is_literal(&self, root: NormExprHandle) -> bool {
        for expr in ExprHandleTopDownWalker::new(&self.pool, root) {
            if matches!(self.arg_order(expr), ArgOrder::Multiset) {
                // Since multisets can be ordered arbitrary
                // expressions can match, even if the don't
                // map 1:1 onto each other.

                return false;
            }

            if self.is_blank(expr)
                || self.is_blank_null_seq(expr)
                || self.is_blank_seq(expr)
                || self.is_pattern(expr)
                || self.is_pattern_test(expr)
            {
                return false;
            }
        }

        true
    }
}
