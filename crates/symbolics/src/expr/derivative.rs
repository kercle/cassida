use std::fmt::Debug;

use expr_macro::raw_expr;
use numbers::Number;

use crate::{
    builtin::*,
    expr::{Expr, NormalizedExpr, atom::Atom, matcher::MatchIter, pattern::Pattern},
    parser::ast::{ADD_HEAD, MUL_HEAD, POW_HEAD},
};

pub fn resolve_derivatives<A>(expr: Expr<A>) -> Expr
where
    A: Default + Clone + PartialEq + Debug,
{
    let expr = expr.drop_annotation();
    let pattern_expr = raw_expr! { D[Pattern[f, Blank[]], Pattern[x, Blank[]]] };
    let pattern = Pattern::from_expr(&pattern_expr);

    expr.map_bottom_up(&|e| {
        if let Some(ctx) = MatchIter::new(&e, &pattern).next() {
            let f = ctx.get("f");
            let x = ctx.get("x");

            if let (
                Some(f),
                Some(Expr::Atom {
                    entry: Atom::Symbol(x),
                    ..
                }),
            ) = (f, x)
            {
                derivative(NormalizedExpr::new(f.clone()), x)
            } else {
                todo!(
                    "Implement type checking for patterns, s.t. I can guaranteed that x is symbol."
                );
            }
        } else {
            e
        }
    })
}

pub fn derivative<A: Default + Clone + PartialEq>(expr: NormalizedExpr<A>, var: &str) -> Expr<A> {
    derivative_inner(expr.take_expr(), var)
}

fn derivative_inner<A: Default + Clone + PartialEq>(expr: Expr<A>, var: &str) -> Expr<A> {
    match expr {
        Expr::Atom {
            entry: Atom::Number(_),
            ..
        } => Number::zero().into(),
        Expr::Atom {
            entry: Atom::Symbol(x),
            ..
        } if x == var => Number::one().into(),
        Expr::Atom {
            entry: Atom::Symbol(_),
            ..
        } => Number::zero().into(),
        Expr::Compound { head, args, .. } if head.matches_symbol(ADD_HEAD) => {
            let args = args.into_iter().map(|a| derivative_inner(a, var)).collect();
            Expr::new_compound(*head, args)
        }
        Expr::Compound { head, args, .. } if head.matches_symbol(MUL_HEAD) => {
            let mut new_args = Vec::with_capacity(args.len());

            for i in 0..args.len() {
                let prod_args = args
                    .iter()
                    .enumerate()
                    .map(|(j, e)| {
                        if j == i {
                            derivative_inner(e.clone(), var)
                        } else {
                            e.clone()
                        }
                    })
                    .collect();

                new_args.push(Expr::new_compound(MUL_HEAD, prod_args));
            }

            Expr::new_compound(ADD_HEAD, new_args)
        }
        Expr::Compound { head, mut args, .. }
            if head.matches_symbol(POW_HEAD) && args.len() == 2 =>
        {
            let rhs = args.pop().unwrap();
            let lhs = args.pop().unwrap();

            Expr::new_compound(POW_HEAD, vec![lhs.clone(), rhs.clone()])
                * (rhs.clone()
                    * derivative_inner(lhs.clone(), var)
                    * Expr::new_compound(POW_HEAD, vec![lhs.clone(), (-1).into()])
                    + Expr::new_compound(CANNONICAL_HEAD_LOG, vec![lhs.clone()])
                        * derivative_inner(rhs, var))
        }
        _ => Expr::new_compound(
            CANNONICAL_HEAD_DERIVATIVE,
            vec![expr.annotation_to_default(), var.into()],
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        expr::{NormalizedExpr, generator::*, simplify::Simplifier},
        symbol,
    };

    fn f<T: Into<Expr>>(x: T) -> Expr {
        Expr::new_compound("f", vec![x.into()])
    }

    fn g<T: Into<Expr>>(x: T) -> Expr {
        Expr::new_compound("g", vec![x.into()])
    }

    fn h<T: Into<Expr>>(x: T) -> Expr {
        Expr::new_compound("h", vec![x.into()])
    }

    fn dd<T: Into<Expr>>(f: T, x: &str) -> Expr {
        Expr::new_compound(CANNONICAL_HEAD_DERIVATIVE, vec![f.into(), x.into()])
    }

    #[test]
    fn test_summation_rule() {
        let x = symbol!("x");
        let e = NormalizedExpr::new(f(x) + g(x));

        let result = derivative(e, "x");
        assert_eq!(result, dd(f(x), "x") + dd(g(x), "x"));
    }

    #[test]
    fn test_product_rule() {
        let x = symbol!("x");
        let e = NormalizedExpr::new(f(x) * g(x) * h(x));

        let result = derivative(e, "x");
        assert_eq!(
            result,
            Expr::new_compound(
                ADD_HEAD,
                vec![
                    Expr::new_compound(MUL_HEAD, vec![dd(f(x), "x"), g(x), h(x)]),
                    Expr::new_compound(MUL_HEAD, vec![f(x), dd(g(x), "x"), h(x)]),
                    Expr::new_compound(MUL_HEAD, vec![f(x), g(x), dd(h(x), "x")])
                ]
            )
        );
    }

    #[test]
    fn test_power_rule() {
        let x = symbol!("x");
        let e = NormalizedExpr::new(pow(f(x), g(x)));

        let result = derivative(e, "x");
        assert_eq!(
            result,
            pow(f(x), g(x)) * ((g(x) * dd(f(x), "x")) * pow(f(x), -1) + log(f(x)) * dd(g(x), "x"))
        );
    }

    #[test]
    fn test_polynomials() {
        let (x, y) = symbol!("x", "y");
        let e = NormalizedExpr::new(1 + 5 * x + y * pow(x, 2));

        let result = NormalizedExpr::new(derivative(e, "x"))
            .collect_like_terms()
            .take_expr();

        assert_eq!(result, raw_expr! { Add[5, Mul[2, x, y]] });
    }

    #[test]
    fn test_resolve_nested_derivate() {
        let expr = raw_expr! {
            Exp[1 + D[f[x] + Sin[x] + Pow[x, 2] + 2, x]]
        };

        let expr = Simplifier::new(resolve_derivatives(expr)).finish();

        assert_eq!(
            expr,
            raw_expr! {
                Exp[Add[1, D[Sin[x], x], D[f[x], x], Mul[2, x]]]
            }
        );
    }
}
