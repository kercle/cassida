use std::{cmp::Ordering, ops};

use numbers::Number;

use crate::{expr::Expr, parser::ast::ADD_HEAD};

fn cmp_expr<A: Clone + PartialEq>(lhs: &Expr<A>, rhs: &Expr<A>) -> Ordering {
    use Expr::*;

    match (lhs, rhs) {
        (Atom { entry: ea, .. }, Atom { entry: eb, .. }) => ea.cmp(eb),
        (Atom { .. }, Compound { .. }) => Ordering::Less,
        (Compound { .. }, Atom { .. }) => Ordering::Greater,
        (
            Compound {
                head: ha, args: aa, ..
            },
            Compound {
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

impl<A> PartialOrd for Expr<A>
where
    A: Clone + PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

impl<A: Clone + PartialEq + Default> ops::Add for Expr<A> {
    type Output = Expr<A>;

    fn add(self, other: Self) -> Self::Output {
        Expr::new_compound(Expr::new_symbol(ADD_HEAD), vec![self, other])
    }
}

impl<A: Clone + PartialEq + Default> ops::Add for &Expr<A> {
    type Output = Expr<A>;

    fn add(self, other: Self) -> Self::Output {
        self.clone() + other.clone()
    }
}

impl<A: Clone + PartialEq + Default> ops::Add<Expr<A>> for &Expr<A> {
    type Output = Expr<A>;

    fn add(self, other: Expr<A>) -> Self::Output {
        self.clone() + other
    }
}

impl<A: Clone + PartialEq + Default> ops::Add<&Expr<A>> for Expr<A> {
    type Output = Expr<A>;

    fn add(self, other: &Expr<A>) -> Self::Output {
        self + other.clone()
    }
}

impl<A: Clone + PartialEq + Default> ops::Add<i32> for Expr<A> {
    type Output = Expr<A>;

    fn add(self, other: i32) -> Self::Output {
        self + Expr::new_number(Number::from_i64(other as i64))
    }
}

impl<A: Clone + PartialEq + Default> ops::Add<Expr<A>> for i32 {
    type Output = Expr<A>;

    fn add(self, other: Expr<A>) -> Self::Output {
        Expr::new_number(Number::from_i64(self as i64)) + other
    }
}

impl<A: Clone + PartialEq + Default> ops::AddAssign for Expr<A> {
    fn add_assign(&mut self, other: Expr<A>) {
        *self = &*self + other;
    }
}

impl<A: Clone + PartialEq + Default> ops::Sub for Expr<A> {
    type Output = Expr<A>;

    fn sub(self, other: Self) -> Self::Output {
        Expr::new_compound(Expr::new_symbol("Add"), vec![self, other * -1])
    }
}

impl<A: Clone + PartialEq + Default> ops::Sub for &Expr<A> {
    type Output = Expr<A>;

    fn sub(self, other: Self) -> Self::Output {
        self.clone() - other.clone()
    }
}

impl<A: Clone + PartialEq + Default> ops::Sub<Expr<A>> for &Expr<A> {
    type Output = Expr<A>;

    fn sub(self, other: Expr<A>) -> Self::Output {
        self.clone() - other
    }
}

impl<A: Clone + PartialEq + Default> ops::Sub<&Expr<A>> for Expr<A> {
    type Output = Expr<A>;

    fn sub(self, other: &Expr<A>) -> Self::Output {
        self - other.clone()
    }
}

impl<A: Clone + PartialEq + Default> ops::Sub<i32> for Expr<A> {
    type Output = Expr<A>;

    fn sub(self, other: i32) -> Self::Output {
        self - Expr::new_number(Number::from_i64(other as i64))
    }
}

impl<A: Clone + PartialEq + Default> ops::Sub<Expr<A>> for i32 {
    type Output = Expr<A>;

    fn sub(self, other: Expr<A>) -> Self::Output {
        Expr::new_number(Number::from_i64(self as i64)) - other
    }
}

impl<A: Clone + PartialEq + Default> ops::Mul for Expr<A> {
    type Output = Expr<A>;

    fn mul(self, other: Self) -> Self::Output {
        Expr::new_compound(Expr::new_symbol("Mul"), vec![self, other])
    }
}

impl<A: Clone + PartialEq + Default> ops::Mul for &Expr<A> {
    type Output = Expr<A>;

    fn mul(self, other: Self) -> Self::Output {
        self.clone() * other.clone()
    }
}

impl<A: Clone + PartialEq + Default> ops::Mul<Expr<A>> for &Expr<A> {
    type Output = Expr<A>;

    fn mul(self, other: Expr<A>) -> Self::Output {
        self.clone() * other
    }
}

impl<A: Clone + PartialEq + Default> ops::Mul<&Expr<A>> for Expr<A> {
    type Output = Expr<A>;

    fn mul(self, other: &Expr<A>) -> Self::Output {
        self * other.clone()
    }
}

impl<A: Clone + PartialEq + Default> ops::Mul<i32> for Expr<A> {
    type Output = Expr<A>;

    fn mul(self, other: i32) -> Self::Output {
        self * Expr::new_number(Number::from_i64(other as i64))
    }
}

impl<A: Clone + PartialEq + Default> ops::Mul<Expr<A>> for i32 {
    type Output = Expr<A>;

    fn mul(self, other: Expr<A>) -> Self::Output {
        Expr::new_number(Number::from_i64(self as i64)) * other
    }
}

impl<A: Clone + PartialEq + Default> ops::Div for Expr<A> {
    type Output = Expr<A>;

    fn div(self, other: Self) -> Self::Output {
        Expr::new_compound(Expr::new_symbol("Div"), vec![self, other])
    }
}

impl<A: Clone + PartialEq + Default> ops::Div for &Expr<A> {
    type Output = Expr<A>;

    fn div(self, other: Self) -> Self::Output {
        self.clone() / other.clone()
    }
}

impl<A: Clone + PartialEq + Default> ops::Div<Expr<A>> for &Expr<A> {
    type Output = Expr<A>;

    fn div(self, other: Expr<A>) -> Self::Output {
        self.clone() / other
    }
}

impl<A: Clone + PartialEq + Default> ops::Div<&Expr<A>> for Expr<A> {
    type Output = Expr<A>;

    fn div(self, other: &Expr<A>) -> Self::Output {
        self / other.clone()
    }
}

impl<A: Clone + PartialEq + Default> ops::Div<i32> for Expr<A> {
    type Output = Expr<A>;

    fn div(self, other: i32) -> Self::Output {
        self / Expr::new_number(Number::from_i64(other as i64))
    }
}

impl<A: Clone + PartialEq + Default> ops::Div<Expr<A>> for i32 {
    type Output = Expr<A>;

    fn div(self, other: Expr<A>) -> Self::Output {
        Expr::new_number(Number::from_i64(self as i64)) / other
    }
}

impl<A: Clone + PartialEq + Default> ops::Neg for Expr<A> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -1 * self
    }
}
