use numbers::Number;

use crate::{
    builtin::*,
    expr::{Expr, NormalizedExpr, atom::Atom},
    parser::ast::{ADD_HEAD, MUL_HEAD, POW_HEAD},
};

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
            vec![expr.drop_annotation(), var.into()],
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        expr::{NormalizedExpr, generator::*},
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

        let result = NormalizedExpr::new(derivative(e, "x")).collect_like_terms();

        dbg!(&result);
    }
}
