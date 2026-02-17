use std::{cmp::Ordering, ops};

use crate::expr::Expr;

fn make_binary_expr<A: Clone + PartialEq + Default>(
    symb: &str,
    lhs: Expr<A>,
    rhs: Expr<A>,
) -> Expr<A> {
    Expr::App {
        head: Box::new(Expr::new_symbol(symb)),
        args: vec![lhs, rhs],
        ann: A::default(),
    }
}

fn cmp_expr<A: Clone + PartialEq>(lhs: &Expr<A>, rhs: &Expr<A>) -> Ordering {
    use Expr::*;

    match (lhs, rhs) {
        (Atom { entry: ea, .. }, Atom { entry: eb, .. }) => ea.cmp(eb),
        (Atom { .. }, App { .. }) => Ordering::Less,
        (App { .. }, Atom { .. }) => Ordering::Greater,
        (
            App {
                head: ha, args: aa, ..
            },
            App {
                head: hb, args: ab, ..
            },
        ) => {
            let ord = cmp_expr(ha, hb);
            if ord != Ordering::Equal {
                return ord;
            }

            let n = aa.len().min(ab.len());
            for i in 0..n {
                let ord = cmp_expr(&aa[i], &ab[i]);
                if ord != Ordering::Equal {
                    return ord;
                }
            }
            aa.len().cmp(&ab.len())
        }
    }
}

impl<A: Clone + PartialEq + Default> ops::Add for Expr<A> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        make_binary_expr("Add", self, other)
    }
}

impl<A: Clone + PartialEq + Default> ops::Add<i64> for Expr<A> {
    type Output = Self;

    fn add(self, other: i64) -> Self::Output {
        make_binary_expr("Add", self, Expr::from_i64(other))
    }
}

impl<A: Clone + PartialEq + Default> ops::Add<Expr<A>> for i64 {
    type Output = Expr<A>;

    fn add(self, other: Expr<A>) -> Self::Output {
        make_binary_expr("Add", Expr::from_i64(self), other)
    }
}

impl<A: Clone + PartialEq + Default> ops::Sub for Expr<A> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        make_binary_expr("Sub", self, other)
    }
}

impl<A: Clone + PartialEq + Default> ops::Mul for Expr<A> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        make_binary_expr("Mul", self, other)
    }
}

impl<A: Clone + PartialEq + Default> ops::Mul<i64> for Expr<A> {
    type Output = Self;

    fn mul(self, other: i64) -> Self::Output {
        make_binary_expr("Mul", self, Expr::from_i64(other))
    }
}

impl<A: Clone + PartialEq + Default> ops::Mul<Expr<A>> for i64 {
    type Output = Expr<A>;

    fn mul(self, other: Expr<A>) -> Self::Output {
        make_binary_expr("Mul", Expr::from_i64(self), other)
    }
}

impl<A: Clone + PartialEq + Default> ops::Div for Expr<A> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        make_binary_expr("Div", self, other)
    }
}

impl<A: Clone + PartialEq + Default> ops::Neg for Expr<A> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -1 * self
    }
}

impl<A> PartialOrd for Expr<A>
where
    A: Clone + PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(cmp_expr(self, other))
    }
}

impl<A> Ord for Expr<A>
where
    A: Clone + PartialEq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        cmp_expr(self, other)
    }
}

impl<A> Eq for Expr<A> where A: Clone + PartialEq {}
