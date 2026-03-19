use std::{collections::HashMap, marker::PhantomData, rc::Rc};

use numbers::Number;

use crate::atom::Atom;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Raw;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Normalized;

#[derive(Clone, PartialEq)]
pub enum ExprKind<E> {
    Atom { entry: Atom },
    Node { head: Box<E>, args: Vec<E> },
}

#[derive(Clone)]
pub struct Expr<S> {
    pub(super) kind: ExprKind<Expr<S>>,
    fingerprint: u64,
    _state: PhantomData<S>,
}

pub type RawExpr = Expr<Raw>;
pub type NormExpr = Expr<Normalized>;

impl<S> Expr<S> {
    pub(super) fn new_unchecked(kind: ExprKind<Expr<S>>) -> Self {
        let fingerprint = kind.digest();
        Self {
            kind,
            fingerprint,
            _state: PhantomData,
        }
    }

    pub fn fingerprint(&self) -> u64 {
        self.fingerprint
    }

    pub fn kind(&self) -> &ExprKind<Self> {
        &self.kind
    }

    pub fn into_kind(self) -> ExprKind<Self> {
        self.kind
    }
}
