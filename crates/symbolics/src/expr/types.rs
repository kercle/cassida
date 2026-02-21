use crate::atom::Atom;

#[derive(Clone, PartialEq)]
pub enum Expr<A = ()> {
    Atom {
        entry: Atom,
        annotation: A,
    },
    Compound {
        head: Box<Expr<A>>,
        args: Vec<Expr<A>>,
        annotation: A,
    },
}

#[repr(transparent)]
#[derive(Debug)]
pub struct NormalizedExpr<A = ()>(Expr<A>)
where
    A: Clone + PartialEq;

impl<A: Clone + PartialEq + Default> NormalizedExpr<A> {
    pub fn new(expr: Expr<A>) -> Self {
        NormalizedExpr(expr.normalize())
    }
}

impl<A: Clone + PartialEq> NormalizedExpr<A> {
    pub fn take_expr(self) -> Expr<A> {
        self.0
    }
}

impl<A: Clone + PartialEq> AsRef<Expr<A>> for NormalizedExpr<A> {
    fn as_ref(&self) -> &Expr<A> {
        &self.0
    }
}
