use std::sync::{Arc, Mutex};

use numbers::{Number, alg::binomial::BinomialGenerator};

use crate::{
    atom::Atom,
    builtin::{ADD_HEAD, MUL_HEAD, POW_HEAD},
    builtins::traits::{BuiltIn, PatternDoc},
    expr::{ExprKind, NormExpr, RawExpr},
    norm_expr,
    pattern::environment::Environment,
    raw_expr,
    rewrite::Rewriter,
};

pub const EXPAND_HEAD: &'static str = "Expand";

pub struct Expand {
    pattern_doc: Vec<PatternDoc>,
    rewriter: Rewriter,
}

impl Expand {
    pub fn new(binomial_generator: Arc<Mutex<BinomialGenerator>>) -> Self {
        Self {
            pattern_doc: vec![PatternDoc::new("Expand[t_]", "Expands the given term $t$.")],
            rewriter: build_rewriter(binomial_generator),
        }
    }
}

impl BuiltIn for Expand {
    fn category(&self) -> &'static str {
        "Simplification"
    }

    fn title(&self) -> &'static str {
        "Term expansion"
    }

    fn head_symbol(&self) -> &'static str {
        "Expand"
    }

    fn summary(&self) -> &'static str {
        "Expand factors."
    }

    fn pattern_doc(&self) -> Vec<PatternDoc> {
        self.pattern_doc.clone()
    }

    fn examples(&self) -> Vec<(&'static str, &'static str)> {
        vec![("x*(4 + x*(5 - x))", "4*x + 5*x^2 - x^3")]
    }

    fn related(&self) -> Vec<&'static str> {
        vec!["Simplify"]
    }

    fn apply_all(&self, expr: NormExpr) -> NormExpr {
        expr.rewrite_all(&self.rewriter, 100)
    }
}

pub(super) fn build_rewriter(_binomial_gen: Arc<Mutex<BinomialGenerator>>) -> Rewriter {
    let rw = Rewriter::new().with_rule(
        norm_expr!(Expand[Pattern[sum, _ + __] ^ PatternTest[m_, IsPositiveInteger]]),
        move |ctx: &Environment<'_, '_>| {
            expand_multinomial(
                ctx.get_one("sum").unwrap(),
                ctx.get_one("m").unwrap().get_number().unwrap(),
            )
            .normalize()
        },
    );

    let rules = vec![
        (
            norm_expr!(Expand[a_ + b__]),
            raw_expr!(Expand[a] + Expand[Add[b]]),
        ),
        (
            norm_expr!(Expand[a__ * (b_ + c__)]),
            raw_expr!(Expand[Mul[a] * b] + Expand[Mul[a] * Add[c]]),
        ),
        (
            norm_expr!(Expand[a__ * (b_ + c__) ^ PatternTest[m_, IsPositiveInteger]]),
            raw_expr!(Expand[Mul[a] * (b + c) ^ (m - 1) * b + Mul[a] * (b + c) ^ (m - 1) * c]),
        ),
        (norm_expr!(Expand[a_]), raw_expr!(a)),
    ];

    let rw = rw.with_rules(rules.into_iter().map(|(pat, repl)| {
        (pat, move |ctx: &Environment<'_, '_>| {
            ctx.fill(repl.clone()).normalize()
        })
    }));

    rw
}

fn expand_multinomial(sum: &NormExpr, n: &Number) -> RawExpr {
    assert!(sum.args_len() == 2, "Multinomials not implemented yet.");
    let ExprKind::Node { args, .. } = sum.kind() else {
        unreachable!()
    };

    let Number::Integer(n) = n else {
        unimplemented!("Non-integer in multinomial expansion not supported.");
    };

    let Some(n) = n.to_u64() else {
        unimplemented!("Exponent too large in multinomial expansion.");
    };

    let mut bin_gen = BinomialGenerator::default();
    let mut new_args = Vec::new();
    for (c, exp_a, exp_b) in (0..=n).map(|k| (bin_gen.get(n as u64, k as u64).clone(), n - k, k)) {
        new_args.push(RawExpr::new_node(
            MUL_HEAD,
            vec![
                RawExpr::new_atom(Atom::number(Number::Integer(c))),
                RawExpr::new_binary_node(
                    POW_HEAD,
                    args[0].clone().into_raw(),
                    RawExpr::new_number_integer(exp_a as i64),
                ),
                RawExpr::new_binary_node(
                    POW_HEAD,
                    args[1].clone().into_raw(),
                    RawExpr::new_number_integer(exp_b as i64),
                ),
            ],
        ));
    }

    RawExpr::new_unary_node(EXPAND_HEAD, RawExpr::new_node(ADD_HEAD, new_args))
}
