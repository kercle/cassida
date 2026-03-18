use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

use numbers::Number;

use crate::atom::Atom;

#[derive(Clone, Copy)]
pub struct Raw;

#[derive(Clone, Copy)]
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

// -------- ExprPool brainstorming -------------

pub(crate) type ExprId = u32;
pub(crate) type ArgsId = u32;

#[derive(Clone, PartialEq, Eq, Hash)]
pub(super) enum ExprCell {
    Atom(Atom),
    Node { head_id: ExprId, args_id: ArgsId },
}

pub struct ExprPool {
    objs: Vec<ExprCell>,
    args: Vec<Vec<ExprId>>,

    obj_map: HashMap<ExprCell, ExprId>,
    args_map: HashMap<Vec<ExprId>, ArgsId>,
}

impl ExprPool {
    fn new() -> Self {
        ExprPool {
            objs: Vec::new(),
            args: Vec::new(),
            obj_map: HashMap::new(),
            args_map: HashMap::new(),
        }
    }

    pub(super) fn get_obj(&self, id: ExprId) -> &ExprCell {
        &self.objs[id as usize]
    }

    pub(super) fn get_args(&self, id: ArgsId) -> &[ExprId] {
        &self.args[id as usize]
    }

    pub(super) fn insert_args(&mut self, args: Vec<ExprId>) -> ArgsId {
        if let Some(&id) = self.args_map.get(&args) {
            return id;
        }
        let id = self.args.len() as ArgsId;
        self.args_map.insert(args.clone(), id);
        self.args.push(args);
        id
    }

    pub(super) fn insert(&mut self, obj: ExprCell) -> ExprId {
        if let Some(&id) = self.obj_map.get(&obj) {
            return id;
        }
        let id = self.objs.len() as ExprId;
        self.obj_map.insert(obj.clone(), id);
        self.objs.push(obj);
        id
    }

    pub(crate) fn atom(&mut self, a: Atom) -> ExprId {
        self.insert(ExprCell::Atom(a))
    }

    pub(crate) fn number(&mut self, n: Number) -> ExprId {
        self.insert(ExprCell::Atom(Atom::number(n)))
    }

    pub(crate) fn number_from_i64(&mut self, n: i64) -> ExprId {
        self.insert(ExprCell::Atom(Atom::number(Number::from_i64(n))))
    }

    pub(crate) fn rational_from_i64(
        &mut self,
        numerator: i64,
        denominator: u64,
    ) -> Result<ExprId, String> {
        let num = Number::new_rational_from_i64(numerator, denominator)?;
        Ok(self.insert(ExprCell::Atom(Atom::number(num))))
    }

    pub(crate) fn symbol<T: AsRef<str>>(&mut self, s: T) -> ExprId {
        self.insert(ExprCell::Atom(Atom::symbol(s)))
    }

    pub(crate) fn node(&mut self, head: ExprId, args: Vec<ExprId>) -> ExprId {
        let args_id = self.insert_args(args);
        self.insert(ExprCell::Node {
            head_id: head,
            args_id,
        })
    }

    pub(crate) fn binary_node(&mut self, head: ExprId, lhs: ExprId, rhs: ExprId) -> ExprId {
        self.node(head, vec![lhs, rhs])
    }

    pub(crate) fn binary_node_with_head_symbol<T: AsRef<str>>(
        &mut self,
        head: T,
        lhs: ExprId,
        rhs: ExprId,
    ) -> ExprId {
        let head_id = self.symbol(head);
        self.node(head_id, vec![lhs, rhs])
    }

    pub(crate) fn unary_node(&mut self, head: ExprId, child: ExprId) -> ExprId {
        self.node(head, vec![child])
    }

    pub(crate) fn unary_node_with_head_symbol<T: AsRef<str>>(
        &mut self,
        head: T,
        child: ExprId,
    ) -> ExprId {
        let head_id = self.symbol(head);
        self.node(head_id, vec![child])
    }

    pub fn insert_expr<S>(&mut self, expr: &Expr<S>) -> ExprId {
        match expr.kind() {
            ExprKind::Atom { entry } => self.atom(entry.clone()),
            ExprKind::Node { head, args } => {
                let head_id = self.insert_expr(head);
                let arg_ids: Vec<ExprId> = args.iter().map(|arg| self.insert_expr(arg)).collect();
                self.node(head_id, arg_ids)
            }
        }
    }

    pub fn insert_raw(&mut self, expr: &RawExpr) -> RawExprHandle {
        let id = self.insert_expr(expr);
        ExprHandle::new(id)
    }

    pub fn insert_norm(&mut self, expr: &NormExpr) -> NormExprHandle {
        let id = self.insert_expr(expr);
        ExprHandle::new_unchecked(id)
    }
}

#[derive(Copy, Clone)]
pub struct ExprHandle<S> {
    id: ExprId,
    _state: PhantomData<S>,
}

pub type RawExprHandle = ExprHandle<Raw>;
pub type NormExprHandle = ExprHandle<Normalized>;

impl RawExprHandle {
    pub(crate) fn new(id: ExprId) -> Self {
        Self::new_unchecked(id)
    }
}

impl<S> ExprHandle<S> {
    pub(super) fn new_unchecked(id: ExprId) -> Self {
        ExprHandle {
            id,
            _state: PhantomData,
        }
    }

    pub fn id(&self) -> ExprId {
        self.id
    }

    fn materialize(&self, pool: &ExprPool) -> Expr<S> {
        match pool.get_obj(self.id()) {
            ExprCell::Atom(atom) => Expr::new_unchecked(ExprKind::Atom {
                entry: atom.clone(),
            }),
            ExprCell::Node { head_id, args_id } => Expr::new_unchecked(ExprKind::Node {
                head: Box::new(Self::new_unchecked(*head_id).materialize(pool)),
                args: pool
                    .get_args(*args_id)
                    .iter()
                    .map(|a| Self::new_unchecked(*a).materialize(&pool))
                    .collect(),
            }),
        }
    }
}

impl<S: Copy> ExprHandle<S> {
    pub(crate) fn view(self, pool: &ExprPool) -> ExprView<S> {
        match &pool.objs[self.id as usize] {
            ExprCell::Atom(a) => ExprView::Atom(a),
            ExprCell::Node {
                head_id: head,
                args_id: args,
            } => ExprView::Node {
                head: ExprHandle::new_unchecked(*head),
                args: &pool.args[*args as usize],
            },
        }
    }

    fn children(self, pool: &ExprPool) -> impl Iterator<Item = ExprHandle<S>> {
        let args = match &pool.objs[self.id as usize] {
            ExprCell::Node { args_id: args, .. } => &pool.args[*args as usize],
            ExprCell::Atom(_) => &[] as &[ExprId],
        };
        args.iter().map(move |&id| ExprHandle::new_unchecked(id))
    }

    fn eq(self, other: ExprHandle<S>) -> bool {
        self.id == other.id
    }
}

pub enum ExprView<'a, S> {
    Atom(&'a Atom),
    Node {
        head: ExprHandle<S>,
        args: &'a [ExprId],
    },
}

impl<'a, S> ExprView<'a, S> {
    pub fn get_symbol(&self) -> Option<&str> {
        match self {
            Self::Atom(Atom::Symbol(sym)) => Some(sym),
            _ => None,
        }
    }
}
