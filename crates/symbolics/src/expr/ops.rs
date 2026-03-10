use std::cmp::Ordering;

use crate::expr::{Expr, ExprKind};

fn cmp_expr<S>(lhs: &ExprKind<Expr<S>>, rhs: &ExprKind<Expr<S>>) -> Ordering {
    use ExprKind::*;

    match (lhs, rhs) {
        (Atom { entry: ea }, Atom { entry: eb }) => ea.cmp(eb),
        (Atom { .. }, Node { .. }) => Ordering::Less,
        (Node { .. }, Atom { .. }) => Ordering::Greater,
        (
            Node {
                head: ha, args: aa, ..
            },
            Node {
                head: hb, args: ab, ..
            },
        ) => {
            let ord = cmp_expr(ha.kind(), hb.kind());
            if ord != Ordering::Equal {
                return ord;
            }

            let n = aa.len().min(ab.len());
            for i in 0..n {
                let ord = cmp_expr(aa[i].kind(), ab[i].kind());
                if ord != Ordering::Equal {
                    return ord;
                }
            }
            aa.len().cmp(&ab.len())
        }
    }
}

impl<S> PartialEq for Expr<S> {
    fn eq(&self, other: &Expr<S>) -> bool {
        self.fingerprint() == other.fingerprint() && self.kind == other.kind
    }
}

impl<S> PartialOrd for Expr<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<S> Ord for Expr<S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        cmp_expr(self.kind(), other.kind())
    }
}

impl<S> Eq for Expr<S> {}
