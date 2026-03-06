use crate::atom::Atom;

#[derive(Clone, PartialEq)]
pub enum Expr<A = ()> {
    Atom {
        entry: Atom,
        digest: u64,
        annotation: A,
    },
    Node {
        head: Box<Expr<A>>,
        args: Vec<Expr<A>>,
        digest: u64,
        annotation: A,
    },
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq)]
pub struct NormalizedExpr<A = ()>(Expr<A>)
where
    A: Clone + PartialEq;

impl<A: Clone + PartialEq + Default> NormalizedExpr<A> {
    pub fn new(expr: Expr<A>) -> Self {
        NormalizedExpr(expr.normalize())
    }

    pub fn drop_annotations(self) -> Expr {
        self.take_expr().drop_annotation()
    }

    pub fn map_annotations(self) -> Self {
        NormalizedExpr(self.take_expr().map_annotations(&|_| A::default()))
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
