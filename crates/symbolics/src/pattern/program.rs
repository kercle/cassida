use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

use crate::builtin::*;
use crate::builtins::patterns::hold_pattern::HOLD_PATTERN_HEAD;
use crate::builtins::patterns::optional::OPTIONAL_HEAD;
use crate::expr::walk::ExprTopDownWalker;
use crate::expr::{ExprKind, NormExpr};
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
            Alternatives { .. } => None,
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

#[derive(Debug)]
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
    pattern_id: PatternId,
}

fn is_multiset_default(expr: &NormExpr) -> bool {
    expr.has_head_symbol(ADD_HEAD) || expr.has_head_symbol(MUL_HEAD)
}

impl Default for Compiler {
    fn default() -> Self {
        Self {
            instructions: Vec::new(),
            var_ids: HashMap::new(),
            vars: Vec::new(),
            is_multiset: is_multiset_default,
            pattern_id: 0,
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
            Node { args: _args, .. } if pat_expr.is_application_of(OPTIONAL_HEAD, 1) => {
                todo!()
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
        if head.matches_symbol(HOLD_PATTERN_HEAD) && children.len() == 1 {
            // HoldPattern is not compiled to an instruction.
            return self.compile_pattern(children.first().unwrap(), bind);
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
            (
                Variadic {
                    min_len: min_len_a,
                    head_pattern: head_pattern_a,
                    bind: bind_a,
                },
                Variadic {
                    min_len: min_len_b,
                    head_pattern: head_pattern_b,
                    bind: bind_b,
                },
            ) => {
                if min_len_a != min_len_b
                    || !Self::same_bind(program_a, *bind_a, program_b, *bind_b)
                {
                    self.branch(program_a, instr_a, program_b, instr_b)
                } else {
                    let bind = if let Some(bind_a) = bind_a {
                        Some(self.bind_name_id(program_a.var(*bind_a).unwrap()))
                    } else {
                        None
                    };

                    let (Some(head_instr_a), Some(head_instr_b)) = (head_pattern_a, head_pattern_b)
                    else {
                        return self.branch(program_a, instr_a, program_b, instr_b);
                    };

                    let head_pattern =
                        self.merge_inner(program_a, *head_instr_a, program_b, *head_instr_b);

                    self.emit(Instruction::Variadic {
                        min_len: *min_len_a,
                        head_pattern: Some(head_pattern),
                        bind,
                    })
                }
            }
            (
                Wildcard {
                    head_pattern: head_pattern_a,
                    bind: bind_a,
                },
                Wildcard {
                    head_pattern: head_pattern_b,
                    bind: bind_b,
                },
            ) => {
                if !Self::same_bind(program_a, *bind_a, program_b, *bind_b) {
                    return self.branch(program_a, instr_a, program_b, instr_b);
                }

                let bind = if let Some(bind_a) = bind_a {
                    Some(self.bind_name_id(program_a.var(*bind_a).unwrap()))
                } else {
                    None
                };

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
            (Predicate { .. }, Predicate { .. }) => todo!(),
            (Node { .. }, Node { .. }) => self.merge_nodes(program_a, instr_a, program_b, instr_b),
            (Alternatives { .. }, Alternatives { .. }) => todo!(),
            _ => self.branch(program_a, instr_a, program_b, instr_b),
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
            Predicate { .. } => todo!(),
        }
    }
}
