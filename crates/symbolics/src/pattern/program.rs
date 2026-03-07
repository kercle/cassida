use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

use crate::builtin::*;
use crate::expr::Expr;
use crate::expr::walk::ExprTopDownWalker;
use crate::pattern::{PatternPredicate, builtin::*};

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

impl<A: Clone + PartialEq> Instruction<A> {
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

#[derive(Debug)]
pub struct MultisetPlan<A: Clone + PartialEq> {
    pub literals: Vec<Expr<A>>,
    pub fixed: Vec<InstrId>,
    pub rest: Vec<(VarId, usize)>,
}

pub struct Compiler<A>
where
    A: Clone + PartialEq,
{
    instructions: Vec<Instruction<A>>,
    var_ids: HashMap<String, VarId>,
    vars: Vec<String>,
    is_multiset: fn(&Expr<A>) -> bool,
}

fn is_multiset_default<A>(expr: &Expr<A>) -> bool
where
    A: Clone + PartialEq,
{
    expr.has_head_symbol(ADD_HEAD) || expr.has_head_symbol(MUL_HEAD)
}

impl<A> Default for Compiler<A>
where
    A: Clone + PartialEq + Default,
{
    fn default() -> Self {
        Self {
            instructions: Vec::new(),
            var_ids: HashMap::new(),
            vars: Vec::new(),
            is_multiset: is_multiset_default,
        }
    }
}

impl<A> Compiler<A>
where
    A: Clone + PartialEq + Default,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_multiset_predicate(mut self, f: fn(&Expr<A>) -> bool) -> Self {
        self.is_multiset = f;
        self
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

    fn compile_pattern(&mut self, pat_expr: &Expr<A>, bind: Option<VarId>) -> InstrId {
        use Expr::*;
        match pat_expr {
            Atom { .. } => self.emit(Instruction::Literal {
                inner: pat_expr.clone(),
                bind,
            }),
            Node { args, .. } if Self::is_blank(pat_expr) => {
                self.compile_blank_with_head_constraint(Quantity::One, args.first(), bind)
            }
            Node { args, .. } if Self::is_blank_seq(pat_expr) => self
                .compile_blank_with_head_constraint(Quantity::Many { min: 1 }, args.first(), bind),
            Node { args, .. } if Self::is_blank_null_seq(pat_expr) => self
                .compile_blank_with_head_constraint(Quantity::Many { min: 0 }, args.first(), bind),
            Node { args, .. } if Self::is_pattern(pat_expr) => {
                let [lhs, rhs] = args.as_slice() else {
                    unreachable!()
                };

                // Unwrap is safe here: guaranteed by is_pattern
                let bind_var_name = lhs.get_symbol().unwrap();

                let var_id = self.bind_name_id(bind_var_name);
                self.compile_pattern(rhs, Some(var_id))
            }
            Node { head, args, .. } if Self::is_pattern_test(pat_expr) => {
                let [lhs, rhs] = args.as_slice() else {
                    unreachable!()
                };

                // Unwrap is safe here: guaranteed by is_pattern_test
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
            Node { head, args, .. } => {
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
        head_pattern: Option<&Expr<A>>,
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
        head: &Expr<A>,
        arg_order: ArgOrder,
        children: &[Expr<A>],
        bind: Option<VarId>,
    ) -> InstrId {
        let head = Self::compile_pattern(self, head, None);

        let pats = children
            .iter()
            .map(|c| self.compile_pattern(c, None))
            .collect();
        let plan = match arg_order {
            ArgOrder::Sequence => ArgPlan::Sequence(pats),
            ArgOrder::Multiset => ArgPlan::Multiset(pats),
        };

        self.emit(Instruction::Node { head, plan, bind })
    }

    fn arg_order(&self, expr: &Expr<A>) -> ArgOrder {
        if (self.is_multiset)(expr) {
            ArgOrder::Multiset
        } else {
            ArgOrder::Sequence
        }
    }

    fn is_blank(expr: &Expr<A>) -> bool {
        if let Expr::Node { head, args, .. } = expr {
            head.matches_symbol(HEAD_BLANK) && args.len() <= 1
        } else {
            false
        }
    }

    fn is_blank_seq(expr: &Expr<A>) -> bool {
        if let Expr::Node { head, args, .. } = expr {
            head.matches_symbol(HEAD_BLANK_SEQUENCE) && args.len() <= 1
        } else {
            false
        }
    }

    fn is_blank_null_seq(expr: &Expr<A>) -> bool {
        if let Expr::Node { head, args, .. } = expr {
            head.matches_symbol(HEAD_BLANK_NULL_SEQUENCE) && args.len() <= 1
        } else {
            false
        }
    }

    fn is_pattern(expr: &Expr<A>) -> bool {
        if !expr.is_application_of(HEAD_PATTERN, 2) {
            return false;
        }

        expr.get_arg(0).unwrap().is_symbol()
    }

    fn is_pattern_test(expr: &Expr<A>) -> bool {
        if !expr.is_application_of(HEAD_PATTERN_TEST, 2) {
            return false;
        }

        expr.get_arg(1).unwrap().is_symbol()
    }

    fn is_literal(&self, root: &Expr<A>) -> bool {
        for expr in ExprTopDownWalker::new(root) {
            if matches!(self.arg_order(expr), ArgOrder::Multiset) {
                // Since multisets can be ordered arbitrary
                // expressions can match, even if the don't
                // map 1:1 onto each other.

                return false;
            }

            if Self::is_blank(expr)
                || Self::is_blank_null_seq(expr)
                || Self::is_blank_seq(expr)
                || Self::is_pattern(expr)
                || Self::is_pattern_test(expr)
            {
                return false;
            }
        }

        true
    }
}
