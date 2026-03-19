use std::cmp::Ordering;

use crate::expr::{
    Expr, ExprKind,
    pool::{ExprHandle, ExprPool, ExprView},
};

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

pub(crate) fn cmp_expr_handle<S: Copy>(
    pool: &ExprPool,
    lhs: &ExprHandle<S>,
    rhs: &ExprHandle<S>,
) -> Ordering {
    use ExprView::*;

    let lhs = lhs.view(pool);
    let rhs = rhs.view(pool);

    match (lhs, rhs) {
        (Atom(ea), Atom(eb)) => ea.cmp(eb),
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
            let ord = cmp_expr_handle(pool, &ha, &hb);
            if ord != Ordering::Equal {
                return ord;
            }

            let n = aa.len(pool).min(ab.len(pool));
            for i in 0..n {
                let aa_i = aa.get(pool, i).unwrap();
                let ab_i = ab.get(pool, i).unwrap();

                let ord = cmp_expr_handle(pool, &aa_i, &ab_i);
                if ord != Ordering::Equal {
                    return ord;
                }
            }

            aa.len(pool).cmp(&ab.len(pool))
        }
    }
}
