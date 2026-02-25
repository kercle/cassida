use std::fmt::Debug;

use expr_macro::expr;
use numbers::Number;

use crate::{
    atom::Atom,
    builtin::*,
    expr::{Expr, NormalizedExpr},
    matcher::MatchIter,
    parser::ast::{ADD_HEAD, MUL_HEAD, POW_HEAD},
    pattern::Pattern,
};

pub fn resolve_derivatives<A>(expr: Expr<A>) -> NormalizedExpr
where
    A: Default + Clone + PartialEq + Debug,
{
    let expr = expr.drop_annotation();
    let pattern_expr =
        expr! { D[Pattern[f, Blank[]], PatternTest[Pattern[x, Blank[]], IsSymbolQ]] };

    let res = expr.map_bottom_up(&|e| {
        let pattern = Pattern::from_expr(&pattern_expr);

        if let Some(ctx) = MatchIter::new(&e, pattern).next() {
            let f = ctx.get_one("f").unwrap();
            let x = ctx.get_one("x").unwrap().get_symbol().unwrap();

            derivative(NormalizedExpr::new(f.clone()), x)
        } else {
            e
        }
    });

    NormalizedExpr::new(res)
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
        Expr::Node { head, args, .. } if head.matches_symbol(ADD_HEAD) => {
            let args = args.into_iter().map(|a| derivative_inner(a, var)).collect();
            Expr::new_node(*head, args)
        }
        Expr::Node { head, args, .. } if head.matches_symbol(MUL_HEAD) => {
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

                new_args.push(Expr::new_node(MUL_HEAD, prod_args));
            }

            Expr::new_node(ADD_HEAD, new_args)
        }
        Expr::Node { head, mut args, .. }
            if head.matches_symbol(POW_HEAD) && args.len() == 2 =>
        {
            let rhs = args.pop().unwrap();
            let lhs = args.pop().unwrap();

            Expr::new_node(POW_HEAD, vec![lhs.clone(), rhs.clone()])
                * (rhs.clone()
                    * derivative_inner(lhs.clone(), var)
                    * Expr::new_node(POW_HEAD, vec![lhs.clone(), (-1).into()])
                    + Expr::new_node(CANNONICAL_HEAD_LOG, vec![lhs.clone()])
                        * derivative_inner(rhs, var))
        }
        Expr::Node { head, mut args, .. }
            if head.matches_symbol(CANNONICAL_HEAD_EXP) && args.len() == 1 =>
        {
            let x = args.pop().unwrap();
            Expr::new_node(CANNONICAL_HEAD_EXP, vec![x.clone()]) * derivative_inner(x, var)
        }
        Expr::Node { head, mut args, .. }
            if head.matches_symbol(CANNONICAL_HEAD_LOG) && args.len() == 1 =>
        {
            let x = args.pop().unwrap();
            Expr::new_node(POW_HEAD, vec![x.clone(), (-1).into()]) * derivative_inner(x, var)
        }
        Expr::Node { head, mut args, .. }
            if head.matches_symbol(CANNONICAL_HEAD_COS) && args.len() == 1 =>
        {
            let x = args.pop().unwrap();
            Expr::new_node(
                MUL_HEAD,
                vec![
                    (-1).into(),
                    Expr::new_node(CANNONICAL_HEAD_SIN, vec![x.clone()]),
                ],
            ) * derivative_inner(x, var)
        }
        Expr::Node { head, mut args, .. }
            if head.matches_symbol(CANNONICAL_HEAD_SIN) && args.len() == 1 =>
        {
            let x = args.pop().unwrap();
            Expr::new_node(CANNONICAL_HEAD_COS, vec![x.clone()]) * derivative_inner(x, var)
        }
        Expr::Node { head, mut args, .. }
            if head.matches_symbol(CANNONICAL_HEAD_TAN) && args.len() == 1 =>
        {
            let x = args.pop().unwrap();
            Expr::new_node(
                ADD_HEAD,
                vec![
                    1.into(),
                    Expr::new_node(
                        POW_HEAD,
                        vec![
                            Expr::new_node(CANNONICAL_HEAD_TAN, vec![x.clone()]),
                            2.into(),
                        ],
                    ),
                ],
            ) * derivative_inner(x, var)
        }
        Expr::Node { head, mut args, .. }
            if head.matches_symbol(CANNONICAL_HEAD_SQRT) && args.len() == 1 =>
        {
            let x = args.pop().unwrap();

            Expr::new_node(
                POW_HEAD,
                vec![
                    x.clone(),
                    Expr::new_number(Number::new_rational_from_i64(-1, 2).unwrap()),
                ],
            ) * derivative_inner(x, var)
        }
        _ => Expr::new_node(
            CANNONICAL_HEAD_DERIVATIVE,
            vec![expr.annotation_to_default(), var.into()],
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        expr::{NormalizedExpr, generator::*},
        simplify::Simplifier,
        symbol,
    };

    fn f<T: Into<Expr>>(x: T) -> Expr {
        Expr::new_node("f", vec![x.into()])
    }

    fn g<T: Into<Expr>>(x: T) -> Expr {
        Expr::new_node("g", vec![x.into()])
    }

    fn h<T: Into<Expr>>(x: T) -> Expr {
        Expr::new_node("h", vec![x.into()])
    }

    fn dd<T: Into<Expr>>(f: T, x: &str) -> Expr {
        Expr::new_node(CANNONICAL_HEAD_DERIVATIVE, vec![f.into(), x.into()])
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
            Expr::new_node(
                ADD_HEAD,
                vec![
                    Expr::new_node(MUL_HEAD, vec![dd(f(x), "x"), g(x), h(x)]),
                    Expr::new_node(MUL_HEAD, vec![f(x), dd(g(x), "x"), h(x)]),
                    Expr::new_node(MUL_HEAD, vec![f(x), g(x), dd(h(x), "x")])
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

        assert_eq!(result, expr! { Add[5, Mul[2, x, y]] });
    }

    #[test]
    fn test_resolve_nested_derivate() {
        let expr = expr! {
            Exp[1 + D[f[x] + Sin[x] + Pow[x, 2] + 2, x]]
        };

        let expr = Simplifier::new(resolve_derivatives(expr)).finish();

        assert_eq!(
            expr,
            expr! {
                Exp[Add[1, Cos[x], D[f[x], x], Mul[2, x]]]
            }
        );
    }
}
