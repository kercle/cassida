use numbers::{Number, alg::binomial::BinomialGenerator, integer::BigInteger};

use crate::{
    atom::Atom, builtins::{
        self, BuiltInCategory,
        traits::{ApplicationError, BuiltIn, BuiltInDoc, PatternDoc},
    }, ensure, expr::{Expr, ExprKind, NormExpr, RawExpr, constructors::EXPR_PLACEHOLDER}, kernel::Shared, norm_expr, pattern::environment::Environment, raw_expr, rewrite::Rewriter
};

pub struct Expand {
    pattern_doc: Vec<PatternDoc>,
    rewriter: Rewriter,
}

impl Expand {
    pub fn new(binomial_generator: Shared<BinomialGenerator>) -> Self {
        Self {
            pattern_doc: vec![PatternDoc::new(
                raw_expr!(Expand[t_]),
                "Expands the given term $t$.",
            )],
            rewriter: build_rewriter(binomial_generator),
        }
    }
}

impl BuiltIn for Expand {
    #[inline(always)]
    fn head() -> &'static str {
        "Expand"
    }

    fn head_dyn(&self) -> &'static str {
        Self::head()
    }

    fn doc(&self) -> BuiltInDoc {
        BuiltInDoc {
            category: BuiltInCategory::Simplification,
            title: "Term expansion",
            summary: "Expand factors.",
            pattern_doc: self.pattern_doc.clone(),
            examples: vec![("Expand[x*(4 + x*(5 - x))]", "4*x + 5*x^2 - x^3")],
            related: vec!["Simplify"],
        }
    }

    fn apply_all(&self, expr: NormExpr) -> NormExpr {
        expr.rewrite_all(&self.rewriter, 1000)
    }

    fn validate_application<S>(expr: &Expr<S>) -> Result<(), ApplicationError> {
        ensure!(expr.args_len() == 1, ApplicationError::ArityMismatch);
        ensure!(expr.is_head(Self::head()), ApplicationError::HeadMismatch);
        Ok(())
    }
}

pub(super) fn build_rewriter(_binomial_gen: Shared<BinomialGenerator>) -> Rewriter {
    let rw = Rewriter::new().with_native_rule(
        norm_expr!( Expand[a__. * HoldPattern[Pattern[sum, Add[__]]]^m_?IsPositiveInteger] ),
        move |ctx: &Environment<'_, '_>| {
            let sum = ctx.get_one("sum").unwrap();
            let m = ctx.get_one("m").unwrap();

            let Number::Integer(n) = m.get_number().unwrap() else {
                unreachable!("m is guaranteed to be a positive integer.");
            };

            let Some(n) = n.to_u64() else {
                // n is too large to be expandable. For now: silently fall back
                // to input expression.
                // TODO: at some point report this to user, but these large
                // polynomials would not be feasable anyway.

                return RawExpr::new_binary_node(
                    builtins::Pow::head(),
                    sum.clone().into_raw(),
                    m.clone().into_raw(),
                );
            };

            RawExpr::new_unary_node(Expand::head(), expand_multinomial(sum, ctx.get_seq("a"), n))
        },
    );

    let rw = rw.with_native_rule(
        norm_expr!(Expand[Pattern[sum, _ + __]]),
        move |ctx: &Environment<'_, '_>| {
            let ExprKind::Node { mut args, .. } =
                ctx.get_one("sum").unwrap().clone().into_raw().into_kind()
            else {
                unreachable!();
            };

            for arg in args.iter_mut() {
                let old_expr = std::mem::replace(arg, EXPR_PLACEHOLDER.clone());
                let new_expr = RawExpr::new_unary_node(Expand::head(), old_expr);
                *arg = new_expr;
            }

            RawExpr::new_node(builtins::Add::head(), args)
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
        (norm_expr!(Expand[a_]), raw_expr!(a)),
    ];

    rw.with_rules_from_tuples(
        rules
            .into_iter()
            .map(|(pat, repl)| (pat, move |ctx: &Environment<'_, '_>| ctx.fill(repl.clone()))),
    )
}

struct CombinationGenerator {
    state: Vec<u64>,
    k: u64,
    first_step: bool,
}

impl CombinationGenerator {
    fn new(n: u64, k: u64) -> Self {
        let mut state = vec![0; n as usize];
        state[0] = k;
        Self {
            state,
            k,
            first_step: false,
        }
    }

    fn next(&mut self) -> Option<&[u64]> {
        if !self.first_step {
            self.first_step = true;
            return Some(&self.state);
        }

        if self.state.is_empty() || *self.state.last().unwrap() == self.k {
            return None;
        }

        let n = self.state.len();

        // Among the first n-1 elements, find last one that is not
        // zero → we can distribute one further up.
        let i = (0..n - 1).rposition(|i| self.state[i] > 0)?;
        self.state[i] -= 1;

        if i + 1 == n - 1 {
            self.state[i + 1] += 1;
        } else {
            // If i+1 is not n-1 we moved the cummulative value from
            // the last bucket to i+1, so we need to drain the last
            // bucket.
            self.state[i + 1] = self.state[n - 1] + 1;
            self.state[n - 1] = 0;
        }

        Some(&self.state)
    }
}

fn expand_multinomial(sum: &NormExpr, overall_factors: Option<&[&NormExpr]>, n: u64) -> RawExpr {
    let ExprKind::Node { args, .. } = sum.kind() else {
        unreachable!()
    };

    let k = args.len() as u64;

    let mut bin_gen = BinomialGenerator::default();
    let mut new_args = Vec::with_capacity(
        bin_gen
            .get(n + k - 1, n)
            .to_u64()
            .expect("Number of terms exceeds capabilities.") as usize,
    );

    let mut combinations = CombinationGenerator::new(k, n);
    while let Some(comb) = combinations.next() {
        let mut coeff = BigInteger::one();

        let mut n = n;
        for ki in comb {
            coeff *= bin_gen.get(n, *ki);
            n -= *ki;
        }

        let mut factors = if let Some(overall_factors) = overall_factors {
            let mut factors = Vec::with_capacity((k + 2) as usize + overall_factors.len());
            factors.extend(overall_factors.iter().map(|&e| e.clone().into_raw()));
            factors
        } else {
            Vec::with_capacity((k + 2) as usize)
        };
        factors.push(RawExpr::new_atom(Atom::number(Number::Integer(coeff))));

        for (factor, &exponent) in args.iter().zip(comb) {
            factors.push(RawExpr::new_binary_node(
                builtins::Pow::head(),
                factor.clone().into_raw(),
                RawExpr::new_number_integer(exponent as i64),
            ));
        }

        new_args.push(RawExpr::new_node(builtins::Mul::head(), factors));
    }

    RawExpr::new_node(builtins::Add::head(), new_args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_multinomials() {
        let expanded = expand_multinomial(&norm_expr!(a + b + c), None, 4).normalize();
        let expected = norm_expr!(
            a ^ 4 + 4 * a
                ^ 3 * b + 6 * a
                ^ 2 * b
                ^ 2 + 4 * a * b
                ^ 3 + b
                ^ 4 + 4 * a
                ^ 3 * c + 12 * a
                ^ 2 * b * c + 12 * a * b
                ^ 2 * c + 4 * b
                ^ 3 * c + 6 * a
                ^ 2 * c
                ^ 2 + 12 * a * b * c
                ^ 2 + 6 * b
                ^ 2 * c
                ^ 2 + 4 * a * c
                ^ 3 + 4 * b * c
                ^ 3 + c
                ^ 4
        );

        assert_eq!(expanded, expected)
    }
}
