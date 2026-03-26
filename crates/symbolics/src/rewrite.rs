use crate::{
    builtins::{self, traits::BuiltIn},
    expr::{ExprKind, NormExpr, RawExpr},
    pattern::{
        environment::Environment,
        merging::Merger,
        program::{Compiler, Program},
        runtime::Runtime,
    },
};

pub type RuleTransformer = Box<dyn Fn(&Environment<'_, '_>) -> RawExpr + Send + Sync>;

pub struct Rule {
    pub program: Program,
    pub transform: RuleTransformer,
}

#[derive(Default)]
pub struct Rewriter {
    program: Option<Program>,
    transformers: Vec<RuleTransformer>,
}

impl Rewriter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_native_rule<F>(self, pattern: NormExpr, transform: F) -> Self
    where
        F: Fn(&Environment<'_, '_>) -> RawExpr + Send + Sync + 'static,
    {
        let next_program = Compiler::default()
            .with_pattern_id(self.transformers.len() as u32)
            .compile(&pattern);

        let merged_program = if let Some(current_program) = self.program {
            Merger::default().merge(current_program, next_program)
        } else {
            next_program
        };

        let mut transformers = self.transformers;
        transformers.push(Box::new(transform));

        Rewriter {
            program: Some(merged_program),
            transformers,
        }
    }

    pub fn with_rules_from_tuples<I, F>(mut self, rules: I) -> Self
    where
        I: IntoIterator<Item = (NormExpr, F)>,
        F: Fn(&Environment<'_, '_>) -> RawExpr + Send + Sync + 'static,
    {
        for (p, t) in rules {
            self = self.with_native_rule(p, t);
        }
        self
    }

    pub fn with_rule(self, rule: &NormExpr) -> Self {
        if !rule.is_application_of(builtins::RuleDelayed::head(), 2) {
            return self;
        }

        let pattern = rule.get_arg(0).unwrap().clone();
        let replacement = rule.get_arg(1).unwrap().clone().into_raw();

        self.with_native_rule(pattern, move |ctx: &Environment<'_, '_>| {
            ctx.fill(replacement.clone())
        })
    }

    pub fn with_rules(mut self, rules: &NormExpr) -> Self {
        if rules.has_head_symbol(builtins::RuleDelayed::head()) {
            return self.with_rule(rules);
        }

        if !rules.has_head_symbol(builtins::Compound::head()) {
            return self;
        }

        for rule in rules.iter_args().unwrap() {
            self = self.with_rule(rule);
        }

        dbg!(&self.program);

        self
    }

    pub fn apply_first_match(&self, expr: NormExpr) -> NormExpr {
        let Some(program) = &self.program else {
            return expr;
        };

        expr.into_raw()
            .map_bottom_up(&|expr| {
                let norm_expr = expr.clone().normalize();

                let mut runtime = Runtime::new(program, &norm_expr);
                if let Some(env) = runtime.first_match() {
                    let f = &self.transformers[env.pattern_id() as usize];
                    return f(env).into_raw();
                }

                expr
            })
            .normalize()
    }
}

impl NormExpr {
    pub fn apply_rules_until_fixed_point<F, I>(self, rules: I, limit_guard: u32) -> NormExpr
    where
        I: IntoIterator<Item = (NormExpr, F)>,
        F: Fn(&Environment<'_, '_>) -> RawExpr + Send + Sync + 'static,
    {
        let rw: Rewriter = Rewriter::new().with_rules_from_tuples(rules);
        self.rewrite_all(&rw, limit_guard)
    }

    pub fn rewrite_all(self, rw: &Rewriter, limit_guard: u32) -> NormExpr {
        let mut expr = self;

        for _ in 0..limit_guard {
            let expr_next_iter = rw.apply_first_match(expr.clone());

            if expr != expr_next_iter {
                expr = expr_next_iter;
            } else {
                return expr;
            }
        }

        expr
    }

    pub fn release_all_holds(self) -> Self {
        self.into_raw()
            .map_bottom_up(&|expr| {
                if expr.is_application_of(builtins::Hold::HEAD, 1) {
                    let ExprKind::Node { mut args, .. } = expr.into_kind() else {
                        unreachable!()
                    };

                    args.pop().unwrap()
                } else {
                    expr
                }
            })
            .normalize()
    }
}
