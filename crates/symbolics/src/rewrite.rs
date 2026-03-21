use crate::{
    builtin::CANNONICAL_HEAD_HOLD,
    expr::{ExprKind, NormExpr, RawExpr},
    pattern::{
        environment::Environment,
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
    rules: Vec<Rule>,
}

impl Rewriter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_rule<F>(mut self, pattern: NormExpr, transform: F) -> Self
    where
        F: Fn(&Environment<'_, '_>) -> RawExpr + Send + Sync + 'static,
    {
        // let matcher = Matcher::new(pattern.take_expr())
        //     .with_commutative_predicate(self.is_commutative.clone());
        // let program = Compiler::default().compile(&pattern.take_expr());

        self.rules.push(Rule {
            program: Compiler::default().compile(&pattern),
            transform: Box::new(transform),
        });
        self
    }

    pub fn with_rules<I, F>(mut self, rules: I) -> Self
    where
        I: IntoIterator<Item = (NormExpr, F)>,
        F: Fn(&Environment<'_, '_>) -> RawExpr + Send + Sync + 'static,
    {
        for (p, t) in rules {
            self = self.with_rule(p, t);
        }
        self
    }

    pub fn apply_first_match(&self, expr: NormExpr) -> NormExpr {
        expr.into_raw()
            .map_bottom_up(&|expr| {
                let norm_expr = expr.clone().normalize();

                for rule in &self.rules {
                    let mut runtime = Runtime::new(&rule.program, &norm_expr);
                    if let Some(env) = runtime.first_match() {
                        let f = &rule.transform;
                        return f(env).into_raw();
                    }
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
        let rw: Rewriter = Rewriter::new().with_rules(rules);
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
                if expr.is_application_of(CANNONICAL_HEAD_HOLD, 1) {
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
