use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

use crate::builtins;
use crate::builtins::traits::BuiltIn;
use crate::expr::walk::ExprTopDownWalker;
use crate::expr::{ExprKind, NormExpr, RawExpr};
use crate::pattern::runtime::Runtime;
use crate::pattern::{PatternPredicate, builtin::*};

pub type InstrId = usize;
pub type VarId = u32;
pub type PatternId = u32;

pub struct Program {
    pub(super) entry_pattern_id: PatternId,
    pub(super) entry: InstrId,
    pub(super) instructions: Vec<Instruction>,
    pub(super) vars: Vec<String>,
    pub(super) var_ids: HashMap<String, VarId>,
}

impl Program {
    pub fn entry(&self) -> InstrId {
        self.entry
    }

    pub fn pattern_id(&self) -> PatternId {
        self.entry_pattern_id
    }

    pub fn instruction(&self, instr_id: InstrId) -> Option<&Instruction> {
        self.instructions.get(instr_id)
    }

    pub fn var(&self, var_id: VarId) -> Option<&str> {
        self.vars.get(var_id as usize).map(|x| x.as_str())
    }

    pub fn run<'p, 's>(&'p self, subject: &'s NormExpr) -> Runtime<'p, 's> {
        Runtime::new(self, subject)
    }
}

pub enum Quantity {
    One,
    Many { min: usize },
}

#[derive(Clone)]
pub enum Instruction {
    Literal {
        inner: NormExpr,
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
    Alternatives {
        branches: Vec<(PatternId, InstrId)>,
    },
    With {
        bind: VarId,
        value: NormExpr,
        next: InstrId,
    },
    CheckCondition {
        inner: InstrId,
        test_expr: RawExpr,
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
            With { .. } => None,
            Alternatives { .. } => None,
            CheckCondition { .. } => None,
        }
    }
}

#[derive(Clone)]
pub enum ArgPlan {
    Sequence(Vec<InstrId>),
    Multiset(Vec<InstrId>),
}

impl ArgPlan {
    pub fn len(&self) -> usize {
        match self {
            ArgPlan::Sequence(a) => a.len(),
            ArgPlan::Multiset(a) => a.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ArgPlan::Sequence(a) => a.is_empty(),
            ArgPlan::Multiset(a) => a.is_empty(),
        }
    }

    pub fn iter(&self) -> core::slice::Iter<'_, InstrId> {
        match self {
            ArgPlan::Sequence(a) => a.iter(),
            ArgPlan::Multiset(a) => a.iter(),
        }
    }
}

impl PartialEq for ArgPlan {
    fn eq(&self, other: &ArgPlan) -> bool {
        use ArgPlan::*;

        match (self, other) {
            (Sequence(a), Sequence(b)) => a.len() == b.len(),
            (Multiset(a), Multiset(b)) => a.len() == b.len(),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ArgOrder {
    Sequence,
    Multiset,
}

#[derive(Debug)]
pub struct MultisetPlan {
    pub literals: Vec<NormExpr>,
    pub fixed: Vec<InstrId>,
    pub rest: Vec<(VarId, usize)>,
}

pub struct Compiler {
    instructions: Vec<Instruction>,
    var_ids: HashMap<String, VarId>,
    vars: Vec<String>,
    is_multiset: fn(&NormExpr) -> bool,
    optional_default: for<'s> fn(&NormExpr, &'s NormExpr) -> Option<(&'s str, NormExpr)>,
    pattern_id: PatternId,

    // used to keep track of nested hold patterns
    hold_pattern_counter: usize,
}

struct DefaultCallbacks;

impl DefaultCallbacks {
    fn is_multiset(expr: &NormExpr) -> bool {
        expr.has_head_symbol(builtins::Add::head()) || expr.has_head_symbol(builtins::Mul::head())
    }

    fn optional_default<'s>(
        parent_head: &NormExpr,
        opt_pattern: &'s NormExpr,
    ) -> Option<(&'s str, NormExpr)> {
        // For now, optional defaults only support patterns like x_.

        if !opt_pattern.is_application_of(builtins::Pattern::head(), 2) {
            return None;
        }

        if !opt_pattern
            .get_arg(1)?
            .is_application_of(builtins::Blank::head(), 0)
        {
            return None;
        }

        let bind_var = opt_pattern.get_arg(0)?.get_symbol()?;

        if parent_head.matches_symbol(builtins::Add::head()) {
            Some((bind_var, RawExpr::from_i64(0).normalize()))
        } else if parent_head.matches_symbol(builtins::Mul::head()) {
            Some((bind_var, RawExpr::from_i64(1).normalize()))
        } else {
            None
        }
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self {
            instructions: Vec::new(),
            var_ids: HashMap::new(),
            vars: Vec::new(),
            is_multiset: DefaultCallbacks::is_multiset,
            optional_default: DefaultCallbacks::optional_default,
            pattern_id: 0,
            hold_pattern_counter: 0,
        }
    }
}

impl Compiler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_multiset_predicate(mut self, f: fn(&NormExpr) -> bool) -> Self {
        self.is_multiset = f;
        self
    }

    pub fn with_pattern_id(mut self, pattern_id: PatternId) -> Self {
        self.pattern_id = pattern_id;
        self
    }

    // --- Program compiling --------------------------------------------------

    pub fn compile(mut self, pattern: &NormExpr) -> Program {
        let entry = self.compile_pattern(pattern, None);

        Program {
            entry_pattern_id: self.pattern_id,
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

    fn compile_pattern(&mut self, pat_expr: &NormExpr, bind: Option<VarId>) -> InstrId {
        use ExprKind::*;
        match pat_expr.kind() {
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
            Node { head, args } if Self::is_pattern_test(pat_expr) => {
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
            Node { args, .. } if pat_expr.is_application_of(builtins::Optional::HEAD, 1) => {
                let inner = args.first().unwrap();
                self.compile_pattern(inner, bind)
            }
            Node { args, .. } if pat_expr.is_application_of(builtins::Condition::HEAD, 2) => {
                let [pat_expr, test_expr] = args.as_slice() else {
                    unreachable!()
                };

                let inner = self.compile_pattern(pat_expr, None);
                self.emit(Instruction::CheckCondition {
                    inner,
                    test_expr: test_expr.clone().into_raw(),
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
        head_pattern: Option<&NormExpr>,
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
        head: &NormExpr,
        arg_order: ArgOrder,
        children: &[NormExpr],
        bind: Option<VarId>,
    ) -> InstrId {
        if head.matches_symbol(builtins::HoldPattern::HEAD) && children.len() == 1 {
            // HoldPattern is not compiled to an instruction.
            self.hold_pattern_counter += 1;
            let instr_id = self.compile_pattern(children.first().unwrap(), bind);
            self.hold_pattern_counter -= 1;

            return instr_id;
        }

        if let Some(optional_pos) = children
            .iter()
            .position(|expr| expr.is_application_of(builtins::Optional::HEAD, 1))
        {
            return self.compile_node_with_optional_child(head, children, optional_pos, bind);
        }

        let head = self.compile_pattern(head, None);

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

    fn compile_node_with_optional_child(
        &mut self,
        head: &NormExpr,
        children: &[NormExpr],
        optional_child_pos: usize,
        bind: Option<VarId>,
    ) -> InstrId {
        let mut children_raw: Vec<_> = children.iter().map(|e| e.clone().into_raw()).collect();

        let optional_inner = std::mem::replace(
            &mut children_raw[optional_child_pos],
            RawExpr::new_symbol(builtins::symbols::ABSENT),
        );
        let mut branch_absent =
            self.reduce_node_in_optional_branch(head, &children_raw, optional_child_pos, bind);

        if let Some((var_name, default_value)) =
            (self.optional_default)(head, children[optional_child_pos].get_arg(0).unwrap())
        {
            let bind = self.bind_name_id(var_name);

            branch_absent = self.emit(Instruction::With {
                bind,
                value: default_value,
                next: branch_absent,
            })
        }

        let ExprKind::Node { mut args, .. } = optional_inner.into_kind() else {
            unreachable!()
        };

        children_raw[optional_child_pos] = args.pop().unwrap();
        let branch_present =
            self.reduce_node_in_optional_branch(head, &children_raw, optional_child_pos, bind);

        self.emit(Instruction::Alternatives {
            branches: vec![
                (self.pattern_id, branch_present),
                (self.pattern_id, branch_absent),
            ],
        })
    }

    fn reduce_node_in_optional_branch(
        &mut self,
        head: &NormExpr,
        children: &[RawExpr],
        optional_child_pos: usize,
        bind: Option<VarId>,
    ) -> InstrId {
        let children = if self.hold_pattern_counter > 0 {
            let wrap_with_hold_pattern = |i: usize, e: &RawExpr| {
                let raw = e.clone().into_raw();
                if i == optional_child_pos {
                    raw
                } else {
                    RawExpr::new_unary_node(builtins::HoldPattern::HEAD, raw)
                }
            };

            children
                .iter()
                .enumerate()
                .map(|(i, e)| wrap_with_hold_pattern(i, e))
                .collect()
        } else {
            children.to_vec()
        };

        let alternative_node = RawExpr::new_node(head.clone().into_raw(), children).normalize();
        self.compile_pattern(&alternative_node, bind)
    }

    fn arg_order(&self, expr: &NormExpr) -> ArgOrder {
        if (self.is_multiset)(expr) {
            ArgOrder::Multiset
        } else {
            ArgOrder::Sequence
        }
    }

    fn is_blank(expr: &NormExpr) -> bool {
        if let ExprKind::Node { head, args } = expr.kind() {
            head.matches_symbol(HEAD_BLANK) && args.len() <= 1
        } else {
            false
        }
    }

    fn is_blank_seq(expr: &NormExpr) -> bool {
        if let ExprKind::Node { head, args } = expr.kind() {
            head.matches_symbol(HEAD_BLANK_SEQUENCE) && args.len() <= 1
        } else {
            false
        }
    }

    fn is_blank_null_seq(expr: &NormExpr) -> bool {
        if let ExprKind::Node { head, args } = expr.kind() {
            head.matches_symbol(HEAD_BLANK_NULL_SEQUENCE) && args.len() <= 1
        } else {
            false
        }
    }

    fn is_pattern(expr: &NormExpr) -> bool {
        if !expr.is_application_of(HEAD_PATTERN, 2) {
            return false;
        }

        expr.get_arg(0).unwrap().is_symbol()
    }

    fn is_pattern_test(expr: &NormExpr) -> bool {
        if !expr.is_application_of(HEAD_PATTERN_TEST, 2) {
            return false;
        }

        expr.get_arg(1).unwrap().is_symbol()
    }

    fn is_literal(&self, root: &NormExpr) -> bool {
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

    // --- Program merging ----------------------------------------------------

    pub fn merge(mut self, program_a: Program, program_b: Program) -> Program {
        let entry = self.merge_inner(&program_a, program_a.entry(), &program_b, program_b.entry());

        Program {
            entry_pattern_id: self.pattern_id,
            entry,
            instructions: self.instructions,
            vars: self.vars,
            var_ids: self.var_ids,
        }
    }

    fn merge_inner(
        &mut self,
        program_a: &Program,
        instr_a: InstrId,
        program_b: &Program,
        instr_b: InstrId,
    ) -> InstrId {
        use Instruction::*;

        match (
            program_a.instruction(instr_a).unwrap(),
            program_b.instruction(instr_b).unwrap(),
        ) {
            (Literal { .. }, Literal { .. }) => {
                self.merge_literals(program_a, instr_a, program_b, instr_b)
            }
            (Variadic { .. }, Variadic { .. }) => {
                self.merge_variadic(program_a, instr_a, program_b, instr_b)
            }
            (Wildcard { .. }, Wildcard { .. }) => {
                self.merge_wildcard(program_a, instr_a, program_b, instr_b)
            }
            (Predicate { .. }, Predicate { .. }) => {
                self.merge_predicates(program_a, instr_a, program_b, instr_b)
            }
            (Alternatives { .. }, Alternatives { .. }) => {
                self.merge_alternatives(program_a, instr_a, program_b, instr_b)
            }
            (Node { .. }, Node { .. }) => self.merge_nodes(program_a, instr_a, program_b, instr_b),
            (CheckCondition { .. }, CheckCondition { .. }) => {
                self.merge_check_condition(program_a, instr_a, program_b, instr_b)
            }
            (With { .. }, With { .. }) => self.merge_with(program_a, instr_a, program_b, instr_b),
            _ => self.branch(program_a, instr_a, program_b, instr_b),
        }
    }

    fn merge_with(
        &mut self,
        program_a: &Program,
        instr_a: InstrId,
        program_b: &Program,
        instr_b: InstrId,
    ) -> InstrId {
        let Some(Instruction::With {
            bind: bind_a,
            value: value_a,
            next: next_a,
        }) = program_a.instructions.get(instr_a)
        else {
            unreachable!();
        };

        let Some(Instruction::With {
            bind: bind_b,
            value: value_b,
            next: next_b,
        }) = program_b.instructions.get(instr_b)
        else {
            unreachable!();
        };

        if !Self::same_bind(program_a, Some(*bind_a), program_b, Some(*bind_b))
            || value_a != value_b
        {
            return self.branch(program_a, instr_a, program_b, instr_b);
        }

        let bind = self.bind_name_id(program_a.var(*bind_a).unwrap());
        let next = self.merge_inner(program_a, *next_a, program_b, *next_b);

        self.emit(Instruction::With {
            bind,
            value: value_a.clone(),
            next,
        })
    }

    fn merge_check_condition(
        &mut self,
        program_a: &Program,
        instr_a: InstrId,
        program_b: &Program,
        instr_b: InstrId,
    ) -> InstrId {
        let Some(Instruction::CheckCondition {
            inner: inner_a,
            test_expr: test_expr_a,
        }) = program_a.instructions.get(instr_a)
        else {
            unreachable!();
        };

        let Some(Instruction::CheckCondition {
            inner: inner_b,
            test_expr: test_expr_b,
        }) = program_b.instructions.get(instr_b)
        else {
            unreachable!();
        };

        if test_expr_a != test_expr_b {
            self.branch(program_a, instr_a, program_b, instr_b)
        } else {
            let inner = self.merge_inner(program_a, *inner_a, program_b, *inner_b);
            self.emit(Instruction::CheckCondition {
                inner,
                test_expr: test_expr_a.clone(),
            })
        }
    }

    fn merge_alternatives(
        &mut self,
        program_a: &Program,
        instr_a: InstrId,
        program_b: &Program,
        instr_b: InstrId,
    ) -> InstrId {
        let Some(Instruction::Alternatives {
            branches: branches_a,
        }) = program_a.instructions.get(instr_a)
        else {
            unreachable!();
        };

        let Some(Instruction::Alternatives {
            branches: branches_b,
        }) = program_b.instructions.get(instr_b)
        else {
            unreachable!();
        };

        let mut branches = branches_a.clone();
        branches.extend(branches_b);

        self.emit(Instruction::Alternatives { branches })
    }

    fn merge_wildcard(
        &mut self,
        program_a: &Program,
        instr_a: InstrId,
        program_b: &Program,
        instr_b: InstrId,
    ) -> InstrId {
        let Some(Instruction::Wildcard {
            head_pattern: head_pattern_a,
            bind: bind_a,
        }) = program_a.instructions.get(instr_a)
        else {
            unreachable!();
        };

        let Some(Instruction::Wildcard {
            head_pattern: head_pattern_b,
            bind: bind_b,
        }) = program_b.instructions.get(instr_b)
        else {
            unreachable!();
        };

        if !Self::same_bind(program_a, *bind_a, program_b, *bind_b) {
            return self.branch(program_a, instr_a, program_b, instr_b);
        }

        let bind = bind_a.map(|bind_a| self.bind_name_id(program_a.var(bind_a).unwrap()));

        match (head_pattern_a, head_pattern_b) {
            (Some(head_instr_a), Some(head_instr_b)) => {
                let head_pattern =
                    self.merge_inner(program_a, *head_instr_a, program_b, *head_instr_b);

                self.emit(Instruction::Wildcard {
                    head_pattern: Some(head_pattern),
                    bind,
                })
            }
            (None, None) => self.emit(Instruction::Wildcard {
                head_pattern: None,
                bind,
            }),
            _ => self.branch(program_a, instr_a, program_b, instr_b),
        }
    }

    fn merge_variadic(
        &mut self,
        program_a: &Program,
        instr_a: InstrId,
        program_b: &Program,
        instr_b: InstrId,
    ) -> InstrId {
        let Some(Instruction::Variadic {
            min_len: min_len_a,
            head_pattern: head_pattern_a,
            bind: bind_a,
        }) = program_a.instructions.get(instr_a)
        else {
            unreachable!();
        };

        let Some(Instruction::Variadic {
            min_len: min_len_b,
            head_pattern: head_pattern_b,
            bind: bind_b,
        }) = program_b.instructions.get(instr_b)
        else {
            unreachable!();
        };

        if min_len_a != min_len_b || !Self::same_bind(program_a, *bind_a, program_b, *bind_b) {
            self.branch(program_a, instr_a, program_b, instr_b)
        } else {
            let bind = bind_a.map(|bind_a| self.bind_name_id(program_a.var(bind_a).unwrap()));

            let (Some(head_instr_a), Some(head_instr_b)) = (head_pattern_a, head_pattern_b) else {
                return self.branch(program_a, instr_a, program_b, instr_b);
            };

            let head_pattern = self.merge_inner(program_a, *head_instr_a, program_b, *head_instr_b);

            self.emit(Instruction::Variadic {
                min_len: *min_len_a,
                head_pattern: Some(head_pattern),
                bind,
            })
        }
    }

    fn merge_predicates(
        &mut self,
        program_a: &Program,
        instr_a: InstrId,
        program_b: &Program,
        instr_b: InstrId,
    ) -> InstrId {
        let Some(Instruction::Predicate {
            predicate: predicate_a,
            inner: inner_a,
            bind: bind_a,
        }) = program_a.instructions.get(instr_a)
        else {
            unreachable!();
        };

        let Some(Instruction::Predicate {
            predicate: predicate_b,
            inner: inner_b,
            bind: bind_b,
        }) = program_b.instructions.get(instr_b)
        else {
            unreachable!();
        };

        if predicate_a != predicate_b || !Self::same_bind(program_a, *bind_a, program_b, *bind_b) {
            self.branch(program_a, instr_a, program_b, instr_b)
        } else {
            self.merge_inner(program_a, *inner_a, program_b, *inner_b)
        }
    }

    fn merge_literals(
        &mut self,
        program_a: &Program,
        instr_a: InstrId,
        program_b: &Program,
        instr_b: InstrId,
    ) -> InstrId {
        let Some(Instruction::Literal {
            inner: inner_a,
            bind: bind_a,
        }) = program_a.instructions.get(instr_a)
        else {
            unreachable!();
        };

        let Some(Instruction::Literal {
            inner: inner_b,
            bind: bind_b,
        }) = program_b.instructions.get(instr_b)
        else {
            unreachable!();
        };

        if inner_a != inner_b || !Self::same_bind(program_a, *bind_a, program_b, *bind_b) {
            self.branch(program_a, instr_a, program_b, instr_b)
        } else {
            if let Some(bind_a) = bind_a {
                self.bind_name_id(program_a.var(*bind_a).unwrap());
            }

            self.import_sub_program(program_a, instr_a)
        }
    }

    fn merge_nodes(
        &mut self,
        program_a: &Program,
        instr_a: InstrId,
        program_b: &Program,
        instr_b: InstrId,
    ) -> InstrId {
        let Some(Instruction::Node {
            head: head_a,
            plan: plan_a,
            bind: bind_a,
        }) = program_a.instructions.get(instr_a)
        else {
            unreachable!();
        };

        let Some(Instruction::Node {
            head: head_b,
            plan: plan_b,
            bind: bind_b,
        }) = program_b.instructions.get(instr_b)
        else {
            unreachable!();
        };

        if plan_a != plan_b || !Self::same_bind(program_a, *bind_a, program_b, *bind_b) {
            return self.branch(program_a, instr_a, program_b, instr_b);
        }

        let instructions_checkpoint = self.instructions.len();

        let merged_head_instr = self.merge_inner(program_a, *head_a, program_b, *head_b);

        let mut branch_count = if matches!(
            self.instructions[merged_head_instr],
            Instruction::Alternatives { .. }
        ) {
            1
        } else {
            0
        };

        debug_assert!(plan_a.len() == plan_b.len());

        let mut merged_plan_instrs = Vec::with_capacity(plan_a.len());
        for (arg_instr_a, arg_instr_b) in plan_a.iter().zip(plan_b.iter()) {
            let instr = self.merge_inner(program_a, *arg_instr_a, program_b, *arg_instr_b);

            if matches!(self.instructions[instr], Instruction::Alternatives { .. }) {
                if branch_count == 1 {
                    self.instructions.truncate(instructions_checkpoint);
                    return self.branch(program_a, instr_a, program_b, instr_b);
                }

                branch_count += 1;
            }

            merged_plan_instrs.push(instr);
        }

        let plan = match plan_a {
            ArgPlan::Multiset { .. } => ArgPlan::Multiset(merged_plan_instrs),
            ArgPlan::Sequence { .. } => ArgPlan::Sequence(merged_plan_instrs),
        };

        if let Some(bind_a) = bind_a {
            self.bind_name_id(program_a.var(*bind_a).unwrap());
        }

        self.emit(Instruction::Node {
            head: merged_head_instr,
            plan,
            bind: *bind_a,
        })
    }

    fn same_bind(
        program_a: &Program,
        bind_a: Option<VarId>,
        program_b: &Program,
        bind_b: Option<VarId>,
    ) -> bool {
        // Check if two (optional) bindings are identical in terms
        // of their symbols.

        if let (Some(var_a), Some(var_b)) = (bind_a, bind_b) {
            program_a.var(var_a) == program_b.var(var_b)
        } else {
            bind_a == bind_b
        }
    }

    fn branch(
        &mut self,
        program_a: &Program,
        instr_a: InstrId,
        program_b: &Program,
        instr_b: InstrId,
    ) -> InstrId {
        // This function creates a new alternative. If one of its
        // branches is an alternative already, the branches are
        // absorbed into the new instruction.

        let mut branches = Vec::new();

        self.collect_branches(&mut branches, program_a, instr_a);
        self.collect_branches(&mut branches, program_b, instr_b);

        self.emit(Instruction::Alternatives { branches })
    }

    fn collect_branches(
        &mut self,
        branches: &mut Vec<(PatternId, InstrId)>,
        program: &Program,
        instr: InstrId,
    ) {
        // Replays a program to create a new branch. If the
        // `instr` points to an alternative instruction, the
        // branches are absorbed in the new list.

        if let Instruction::Alternatives {
            branches: sub_branches,
        } = &program.instructions[instr]
        {
            for (pat_idx, branch) in sub_branches {
                let branch = self.import_sub_program(program, *branch);

                branches.push((*pat_idx, branch));
            }
        } else {
            let branch = self.import_sub_program(program, instr);
            branches.push((program.pattern_id(), branch));
        }
    }

    fn import_sub_program(&mut self, program: &Program, instr_pos: InstrId) -> InstrId {
        // This function imports the instructions from `program`
        // into the program under construction. Note that the
        // instruction ids are different in general, so the program
        // has to be "replayed".

        use Instruction::*;

        match program.instruction(instr_pos).unwrap() {
            Literal { inner, bind } => {
                let bind = bind.map(|b| self.bind_name_id(program.var(b).unwrap()));
                self.emit(Literal {
                    inner: inner.clone(),
                    bind,
                })
            }
            Node { head, plan, bind } => {
                let head = self.import_sub_program(program, *head);

                let bind = bind.map(|b| self.bind_name_id(program.var(b).unwrap()));

                let plan_instrs = plan
                    .iter()
                    .map(|instr| self.import_sub_program(program, *instr))
                    .collect();

                let plan = match plan {
                    ArgPlan::Multiset { .. } => ArgPlan::Multiset(plan_instrs),
                    ArgPlan::Sequence { .. } => ArgPlan::Sequence(plan_instrs),
                };

                self.emit(Node { head, plan, bind })
            }
            Wildcard { head_pattern, bind } => {
                let bind = bind.map(|b| self.bind_name_id(program.var(b).unwrap()));
                let head_pattern = head_pattern.map(|p| self.import_sub_program(program, p));

                self.emit(Wildcard { head_pattern, bind })
            }
            Variadic {
                min_len,
                head_pattern,
                bind,
            } => {
                let bind = bind.map(|b| self.bind_name_id(program.var(b).unwrap()));
                let head_pattern = head_pattern.map(|p| self.import_sub_program(program, p));

                self.emit(Variadic {
                    min_len: *min_len,
                    head_pattern,
                    bind,
                })
            }
            Alternatives { branches } => {
                let branches = branches
                    .iter()
                    .map(|(pat_id, instr)| (*pat_id, self.import_sub_program(program, *instr)))
                    .collect();

                self.emit(Alternatives { branches })
            }
            Predicate {
                predicate,
                inner,
                bind,
            } => {
                let bind = bind.map(|b| self.bind_name_id(program.var(b).unwrap()));
                let inner = self.import_sub_program(program, *inner);

                self.emit(Predicate {
                    predicate: *predicate,
                    inner,
                    bind,
                })
            }
            CheckCondition { inner, test_expr } => {
                let inner = self.import_sub_program(program, *inner);

                self.emit(CheckCondition {
                    inner,
                    test_expr: test_expr.clone(),
                })
            }
            With { bind, value, next } => {
                let bind_new = self.bind_name_id(program.var(*bind).unwrap());
                let next = self.import_sub_program(program, *next);

                self.emit(With {
                    bind: bind_new,
                    value: value.clone(),
                    next,
                })
            }
        }
    }
}
