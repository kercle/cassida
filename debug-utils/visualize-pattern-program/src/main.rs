use std::collections::HashMap;

use parser::parse;
use symbolics::{
    expr::RawExpr,
    pattern::{
        merging::Merger,
        program::{ArgPlan, Compiler, InstrId, Instruction, PatternId, Program},
    },
};

struct InstructionGraph<'p> {
    nodes: HashMap<String, HashMap<&'static str, String>>,
    edges: Vec<(String, String)>,
    program: &'p Program,
}

impl<'p> InstructionGraph<'p> {
    pub fn new(program: &'p Program) -> Self {
        let mut ret = Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
            program,
        };

        ret.enter();

        ret
    }

    pub fn to_dot(&self) -> String {
        let mut output = String::new();
        output.push_str("digraph G {\n");

        for (node_id, opts) in self.nodes.iter() {
            output.push_str(&format!("{node_id}["));
            let opts_str: Vec<String> = opts
                .iter()
                .map(|(key, value)| format!(r#"{key}="{value}""#))
                .collect();
            output.push_str(&opts_str.join(", ").to_string());
            output.push_str("];\n");
        }

        for (a, b) in self.edges.iter() {
            output.push_str(&format!("{a} -> {b};\n"));
        }

        output.push('}');

        output
    }

    fn bind_to(&self, bind: Option<u32>) -> String {
        let Some(bind) = bind else {
            return "".to_string();
        };

        let bind = self.program.var(bind).unwrap();
        format!("{bind} := ")
    }

    fn enter(&mut self) {
        let mut node_opts = HashMap::new();
        let entry_node_id = "entry".to_string();

        node_opts.insert("shape", "circle".to_string());
        node_opts.insert("label", "Start".to_string());
        self.nodes.insert(entry_node_id.clone(), node_opts);

        self.walk_program(self.program.entry(), entry_node_id);
    }

    fn walk_program(&mut self, cur_instr: InstrId, prev_node: String) {
        use Instruction::*;

        let mut node_opts = HashMap::new();
        let mut current_node_id = format!("i{cur_instr}");

        match self.program.instruction(cur_instr).unwrap() {
            Literal { inner, bind } => {
                let bind = self.bind_to(*bind);
                node_opts.insert("label", format!("{bind}Literal: `{inner:?}`"));
            }
            Variadic {
                min_len,
                head_pattern,
                bind,
            } => {
                let bind = self.bind_to(*bind);
                let label = format!("{bind}Variadic({min_len})");

                node_opts.insert("shape", "box".to_string());
                node_opts.insert("label", label);
                node_opts.insert("style", "filled".to_string());
                node_opts.insert("fillcolor", "lightgreen".to_string());

                if let Some(pat) = head_pattern {
                    self.walk_program(*pat, format!("i{cur_instr}:h"));
                }
            }
            Wildcard { head_pattern, bind } => {
                let bind = self.bind_to(*bind);
                let label = format!("{bind}Wildcard");

                node_opts.insert("shape", "box".to_string());
                node_opts.insert("label", label);
                node_opts.insert("style", "filled".to_string());
                node_opts.insert("fillcolor", "lightgreen".to_string());

                if let Some(pat) = head_pattern {
                    self.walk_program(*pat, format!("i{cur_instr}:h"));
                }
            }
            Predicate {
                predicate,
                inner,
                bind,
            } => {
                let bind = self.bind_to(*bind);
                let label = format!("{bind}Predicate {predicate:?}");

                node_opts.insert("shape", "invhouse".to_string());
                node_opts.insert("label", label);
                node_opts.insert("style", "filled".to_string());
                node_opts.insert("fillcolor", "pink".to_string());

                self.walk_program(*inner, format!("i{cur_instr}:h"));
            }
            Node { head, plan, bind } => {
                let bind = self.bind_to(*bind);
                let mut label = format!("<n> {bind}Node ");
                current_node_id.push_str(":n");

                let args = match plan {
                    ArgPlan::Multiset(args) => {
                        label.push_str("Multiset");
                        args
                    }
                    ArgPlan::Sequence(args) => {
                        label.push_str("Sequence");
                        args
                    }
                };

                label.push_str(" | <h> Head");

                for (slot_index, &next_instr) in args.iter().enumerate() {
                    self.walk_program(next_instr, format!("i{cur_instr}:a{slot_index}"));
                    label.push_str(&format!(" | <a{slot_index}> Arg {slot_index}"));
                }

                node_opts.insert("shape", "record".to_string());
                node_opts.insert("label", label);
                node_opts.insert("style", "filled".to_string());
                node_opts.insert("fillcolor", "lightyellow".to_string());
                self.walk_program(*head, format!("i{cur_instr}:h"));
            }
            Alternatives { branches } => {
                let mut label = "<n> Alternatives ".to_string();
                current_node_id.push_str(":n");

                for (slot_index, (pattern_id, next_instr)) in branches.iter().enumerate() {
                    self.walk_program(*next_instr, format!("i{cur_instr}:a{slot_index}"));
                    label.push_str(&format!(
                        " | <a{slot_index}> Branch({pattern_id}) {slot_index}"
                    ));
                }

                node_opts.insert("shape", "record".to_string());
                node_opts.insert("label", label);
                node_opts.insert("style", "filled".to_string());
                node_opts.insert("fillcolor", "lightblue".to_string());
            }
            CheckCondition { inner, test_expr } => {
                let label = format!("Condition {test_expr:?}");

                node_opts.insert("shape", "house".to_string());
                node_opts.insert("label", label);
                node_opts.insert("style", "filled".to_string());
                node_opts.insert("fillcolor", "pink".to_string());

                self.walk_program(*inner, format!("i{cur_instr}"));
            }
            With { bind, value, next } => {
                let label = format!("With {}{value:?}", self.bind_to(Some(*bind)));

                node_opts.insert("shape", "parallelogram".to_string());
                node_opts.insert("label", label);
                node_opts.insert("style", "filled".to_string());
                node_opts.insert("fillcolor", "aliceblue".to_string());

                self.walk_program(*next, format!("i{cur_instr}"));
            }
        }

        self.nodes.insert(current_node_id.clone(), node_opts);
        self.edges.push((prev_node, current_node_id));
    }
}

fn main() {
    let mut programs = Vec::new();

    for (pat_id, arg) in std::env::args().skip(1).enumerate() {
        let ast = parse(&arg).unwrap_or_else(|_| panic!("Cannot compile pattern `{arg}`"));

        let expr: RawExpr = ast.into();
        let norm_expr = expr.normalize();
        let program = Compiler::default()
            .with_pattern_id(pat_id as PatternId)
            .compile(&norm_expr);

        programs.push(program);
    }

    let mut merged_program = programs.remove(0);
    for program in programs.into_iter() {
        merged_program = Merger::default().merge(merged_program, program);
    }

    let graph = InstructionGraph::new(&merged_program);
    println!("{}", graph.to_dot());
}
