use crate::pattern::program::{ArgPlan, BuildContext, Instruction, Program};

pub type InstrId = usize;
pub type VarId = u32;
pub type PatternId = u32;

#[derive(Default)]
pub struct Merger {
    build_context: BuildContext,
}

impl Merger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn merge(mut self, program_a: Program, program_b: Program) -> Program {
        let entry = self.merge_inner(&program_a, program_a.entry(), &program_b, program_b.entry());

        Program {
            entry_pattern_id: program_a.pattern_id(),
            entry,
            instructions: self.build_context.instructions,
            vars: self.build_context.vars,
            var_ids: self.build_context.var_ids,
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

        let bind = self
            .build_context
            .bind_name_id(program_a.var(*bind_a).unwrap());
        let next = self.merge_inner(program_a, *next_a, program_b, *next_b);

        self.build_context.emit(Instruction::With {
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
            self.build_context.emit(Instruction::CheckCondition {
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

        self.build_context
            .emit(Instruction::Alternatives { branches })
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

        let bind = bind_a.map(|bind_a| {
            self.build_context
                .bind_name_id(program_a.var(bind_a).unwrap())
        });

        match (head_pattern_a, head_pattern_b) {
            (Some(head_instr_a), Some(head_instr_b)) => {
                let head_pattern =
                    self.merge_inner(program_a, *head_instr_a, program_b, *head_instr_b);

                self.build_context.emit(Instruction::Wildcard {
                    head_pattern: Some(head_pattern),
                    bind,
                })
            }
            (None, None) => self.build_context.emit(Instruction::Wildcard {
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
            let bind = bind_a.map(|bind_a| {
                self.build_context
                    .bind_name_id(program_a.var(bind_a).unwrap())
            });

            let (Some(head_instr_a), Some(head_instr_b)) = (head_pattern_a, head_pattern_b) else {
                return self.branch(program_a, instr_a, program_b, instr_b);
            };

            let head_pattern = self.merge_inner(program_a, *head_instr_a, program_b, *head_instr_b);

            self.build_context.emit(Instruction::Variadic {
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
                self.build_context
                    .bind_name_id(program_a.var(*bind_a).unwrap());
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

        // TODO: Integrate checkpoint mechanism into BuildContext
        let instructions_checkpoint = self.build_context.instructions.len();

        let merged_head_instr = self.merge_inner(program_a, *head_a, program_b, *head_b);

        let mut branch_count = if matches!(
            self.build_context.instructions[merged_head_instr],
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

            if matches!(
                self.build_context.instructions[instr],
                Instruction::Alternatives { .. }
            ) {
                if branch_count == 1 {
                    self.build_context
                        .instructions
                        .truncate(instructions_checkpoint);
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
            self.build_context
                .bind_name_id(program_a.var(*bind_a).unwrap());
        }

        self.build_context.emit(Instruction::Node {
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

        self.build_context
            .emit(Instruction::Alternatives { branches })
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
                let bind = bind.map(|b| self.build_context.bind_name_id(program.var(b).unwrap()));
                self.build_context.emit(Literal {
                    inner: inner.clone(),
                    bind,
                })
            }
            Node { head, plan, bind } => {
                let head = self.import_sub_program(program, *head);

                let bind = bind.map(|b| self.build_context.bind_name_id(program.var(b).unwrap()));

                let plan_instrs = plan
                    .iter()
                    .map(|instr| self.import_sub_program(program, *instr))
                    .collect();

                let plan = match plan {
                    ArgPlan::Multiset { .. } => ArgPlan::Multiset(plan_instrs),
                    ArgPlan::Sequence { .. } => ArgPlan::Sequence(plan_instrs),
                };

                self.build_context.emit(Node { head, plan, bind })
            }
            Wildcard { head_pattern, bind } => {
                let bind = bind.map(|b| self.build_context.bind_name_id(program.var(b).unwrap()));
                let head_pattern = head_pattern.map(|p| self.import_sub_program(program, p));

                self.build_context.emit(Wildcard { head_pattern, bind })
            }
            Variadic {
                min_len,
                head_pattern,
                bind,
            } => {
                let bind = bind.map(|b| self.build_context.bind_name_id(program.var(b).unwrap()));
                let head_pattern = head_pattern.map(|p| self.import_sub_program(program, p));

                self.build_context.emit(Variadic {
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

                self.build_context.emit(Alternatives { branches })
            }
            Predicate {
                predicate,
                inner,
                bind,
            } => {
                let bind = bind.map(|b| self.build_context.bind_name_id(program.var(b).unwrap()));
                let inner = self.import_sub_program(program, *inner);

                self.build_context.emit(Predicate {
                    predicate: *predicate,
                    inner,
                    bind,
                })
            }
            CheckCondition { inner, test_expr } => {
                let inner = self.import_sub_program(program, *inner);

                self.build_context.emit(CheckCondition {
                    inner,
                    test_expr: test_expr.clone(),
                })
            }
            With { bind, value, next } => {
                let bind_new = self.build_context.bind_name_id(program.var(*bind).unwrap());
                let next = self.import_sub_program(program, *next);

                self.build_context.emit(With {
                    bind: bind_new,
                    value: value.clone(),
                    next,
                })
            }
        }
    }
}
