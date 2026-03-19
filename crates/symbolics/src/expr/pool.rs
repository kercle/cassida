use std::{
    collections::HashMap,
    marker::PhantomData,
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
};

use numbers::Number;

use crate::{
    atom::Atom,
    expr::{Expr, ExprKind, NormExpr, Normalized, Raw, RawExpr},
};

static POOL_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

type PoolId = u64;
type ExprId = u32;
type ArgsId = u32;

#[derive(Clone, PartialEq, Eq, Hash)]
pub(super) enum ExprCell {
    Atom(Atom),
    Node { head_id: ExprId, args_id: ArgsId },
}

pub struct ExprPool {
    pool_id: u64,

    objs: Vec<Rc<ExprCell>>,
    args: Vec<Rc<Vec<ExprId>>>,

    obj_map: HashMap<Rc<ExprCell>, ExprId>,
    args_map: HashMap<Rc<Vec<ExprId>>, ArgsId>,
}

impl ExprPool {
    pub(crate) fn new() -> Self {
        ExprPool {
            pool_id: POOL_ID_COUNTER.fetch_add(1, Ordering::AcqRel),
            objs: Vec::new(),
            args: Vec::new(),
            obj_map: HashMap::new(),
            args_map: HashMap::new(),
        }
    }

    fn get_obj(&self, id: ExprId) -> &ExprCell {
        &self.objs[id as usize]
    }

    fn get_args(&self, id: ArgsId) -> &[ExprId] {
        &self.args[id as usize]
    }

    pub(super) fn insert_args(&mut self, args: Vec<ExprId>) -> ArgsId {
        if let Some(&id) = self.args_map.get(&args) {
            return id;
        }
        let id = self.args.len() as ArgsId;

        let rc = Rc::new(args);
        self.args_map.insert(rc.clone(), id);
        self.args.push(rc);
        id
    }

    fn insert(&mut self, obj: ExprCell) -> ExprId {
        if let Some(&id) = self.obj_map.get(&obj) {
            return id;
        }
        let id = self.objs.len() as ExprId;

        let obj = Rc::new(obj);
        self.obj_map.insert(obj.clone(), id);
        self.objs.push(obj);
        id
    }

    pub(crate) fn atom(&mut self, a: Atom) -> RawExprHandle {
        RawExprHandle::new(self.pool_id, self.insert(ExprCell::Atom(a)))
    }

    pub(crate) fn number(&mut self, n: Number) -> RawExprHandle {
        RawExprHandle::new(self.pool_id, self.insert(ExprCell::Atom(Atom::number(n))))
    }

    pub(crate) fn integer_from_i64(&mut self, n: i64) -> RawExprHandle {
        RawExprHandle::new(
            self.pool_id,
            self.insert(ExprCell::Atom(Atom::number(Number::from_i64(n)))),
        )
    }

    pub(crate) fn rational_from_i64(
        &mut self,
        numerator: i64,
        denominator: u64,
    ) -> Result<RawExprHandle, String> {
        let num = Number::new_rational_from_i64(numerator, denominator)?;
        Ok(RawExprHandle::new(
            self.pool_id,
            self.insert(ExprCell::Atom(Atom::number(num))),
        ))
    }

    pub(crate) fn symbol<T: AsRef<str>>(&mut self, s: T) -> RawExprHandle {
        RawExprHandle::new(self.pool_id, self.insert(ExprCell::Atom(Atom::symbol(s))))
    }

    pub(crate) fn node(&mut self, head: RawExprHandle, args: RawArgsHandle) -> RawExprHandle {
        debug_assert!(head.pool_id == self.pool_id);
        debug_assert!(args.pool_id == self.pool_id);

        RawExprHandle::new(
            self.pool_id,
            self.insert(ExprCell::Node {
                head_id: head.id,
                args_id: args.id,
            }),
        )
    }

    pub(crate) fn binary_node(
        &mut self,
        head: RawExprHandle,
        lhs: RawExprHandle,
        rhs: RawExprHandle,
    ) -> RawExprHandle {
        self.variadic_node(head, vec![lhs, rhs])
    }

    pub(crate) fn binary_node_with_head_symbol<T: AsRef<str>>(
        &mut self,
        head: T,
        lhs: RawExprHandle,
        rhs: RawExprHandle,
    ) -> RawExprHandle {
        let head = self.symbol(head);
        self.binary_node(head, lhs, rhs)
    }

    pub(crate) fn unary_node(
        &mut self,
        head: RawExprHandle,
        child: RawExprHandle,
    ) -> RawExprHandle {
        self.variadic_node(head, vec![child])
    }

    pub(crate) fn _unary_node_with_head_symbol<T: AsRef<str>>(
        &mut self,
        head: T,
        child: RawExprHandle,
    ) -> RawExprHandle {
        let head = self.symbol(head);
        self.unary_node(head, child)
    }

    pub(crate) fn variadic_node(
        &mut self,
        head: RawExprHandle,
        args: Vec<RawExprHandle>,
    ) -> RawExprHandle {
        debug_assert!(head.pool_id == self.pool_id);
        for a in args.iter() {
            debug_assert!(a.pool_id == self.pool_id);
        }

        let args_id = self.insert_args(args.into_iter().map(|a| a.id).collect());
        self.node(head, ArgsHandle::new(self.pool_id, args_id))
    }

    pub(crate) fn variadic_node_with_head_symbol<T: AsRef<str>>(
        &mut self,
        head: T,
        args: Vec<RawExprHandle>,
    ) -> RawExprHandle {
        let head_id = self.symbol(head);
        self.variadic_node(head_id, args)
    }

    pub fn insert_expr<S>(&mut self, expr: Expr<S>) -> RawExprHandle {
        match expr.into_kind() {
            ExprKind::Atom { entry } => self.atom(entry),
            ExprKind::Node { head, args } => {
                let head = self.insert_expr(*head);
                let arg_ids: Vec<ExprId> = args
                    .into_iter()
                    .map(|arg| self.insert_expr(arg).id)
                    .collect();

                let args = self.insert_args(arg_ids);
                self.node(head, RawArgsHandle::new(self.pool_id, args))
            }
        }
    }

    pub fn insert_raw(&mut self, expr: RawExpr) -> RawExprHandle {
        self.insert_expr(expr)
    }

    pub fn insert_norm(&mut self, expr: NormExpr) -> NormExprHandle {
        let handle = self.insert_expr(expr);
        ExprHandle::new_unchecked(self.pool_id, handle.id)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ExprHandle<S> {
    id: ExprId,
    pool_id: PoolId,
    _state: PhantomData<S>,
}

pub type RawExprHandle = ExprHandle<Raw>;
pub type NormExprHandle = ExprHandle<Normalized>;

impl RawExprHandle {
    fn new(pool_id: PoolId, id: ExprId) -> Self {
        Self::new_unchecked(pool_id, id)
    }

    pub(super) fn into_normexpr_unchecked(self) -> NormExprHandle {
        NormExprHandle::new_unchecked(self.pool_id, self.id)
    }
}

impl<S> ExprHandle<S> {
    fn new_unchecked(pool_id: PoolId, id: ExprId) -> Self {
        ExprHandle {
            id,
            pool_id,
            _state: PhantomData,
        }
    }

    pub(crate) fn as_raw(&self) -> RawExprHandle {
        RawExprHandle::new(self.pool_id, self.id)
    }

    pub fn materialize(&self, pool: &ExprPool) -> Expr<S> {
        debug_assert!(pool.pool_id == self.pool_id);

        match pool.get_obj(self.id) {
            ExprCell::Atom(atom) => Expr::new_unchecked(ExprKind::Atom {
                entry: atom.clone(),
            }),
            ExprCell::Node { head_id, args_id } => Expr::new_unchecked(ExprKind::Node {
                head: Box::new(Self::new_unchecked(self.pool_id, *head_id).materialize(pool)),
                args: pool
                    .get_args(*args_id)
                    .iter()
                    .map(|a| Self::new_unchecked(self.pool_id, *a).materialize(pool))
                    .collect(),
            }),
        }
    }
}

impl<S: Copy> ExprHandle<S> {
    pub(crate) fn view(self, pool: &ExprPool) -> ExprView<'_, S> {
        debug_assert!(pool.pool_id == self.pool_id);

        match &pool.objs[self.id as usize].as_ref() {
            ExprCell::Atom(a) => ExprView::Atom(a),
            ExprCell::Node {
                head_id: head,
                args_id: args,
            } => ExprView::Node {
                head: ExprHandle::new_unchecked(self.pool_id, *head),
                args: ArgsHandle::new_unchecked(self.pool_id, *args),
            },
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ArgsHandle<S> {
    id: ArgsId,
    pool_id: PoolId,
    _state: PhantomData<S>,
}

pub type RawArgsHandle = ArgsHandle<Raw>;
pub type NormArgsHandle = ArgsHandle<Normalized>;

impl RawArgsHandle {
    fn new(pool_id: PoolId, id: ExprId) -> Self {
        Self::new_unchecked(pool_id, id)
    }
}

impl<S> ArgsHandle<S> {
    fn new_unchecked(pool_id: PoolId, id: ArgsId) -> ArgsHandle<S> {
        ArgsHandle {
            id,
            pool_id,
            _state: PhantomData,
        }
    }

    pub(crate) fn as_raw(&self) -> RawArgsHandle {
        RawArgsHandle::new(self.pool_id, self.id)
    }

    pub fn get(&self, pool: &ExprPool, index: usize) -> Option<ExprHandle<S>> {
        debug_assert!(pool.pool_id == self.pool_id);

        let pool_id = self.pool_id;

        pool.get_args(self.id)
            .get(index)
            .copied()
            .map(move |s| ExprHandle::<S>::new_unchecked(pool_id, s))
    }

    pub fn len(&self, pool: &ExprPool) -> usize {
        pool.get_args(self.id).len()
    }
}

impl<S: Copy + 'static> ArgsHandle<S> {
    pub(crate) fn iter(
        self,
        pool: &ExprPool,
    ) -> impl ExactSizeIterator<Item = ExprHandle<S>> + DoubleEndedIterator<Item = ExprHandle<S>> + '_
    {
        debug_assert!(pool.pool_id == self.pool_id);

        let pool_id = self.pool_id;

        pool.get_args(self.id)
            .iter()
            .copied()
            .map(move |s| ExprHandle::<S>::new_unchecked(pool_id, s))
    }

    pub(crate) fn to_vec(self, pool: &ExprPool) -> Vec<ExprHandle<S>> {
        debug_assert!(pool.pool_id == self.pool_id);

        self.iter(pool).collect()
    }
}

pub enum ExprView<'a, S> {
    Atom(&'a Atom),
    Node {
        head: ExprHandle<S>,
        args: ArgsHandle<S>,
    },
}

impl<'a, S> ExprView<'a, S>
where
    S: Copy + 'static,
{
    pub fn get_head(&self) -> Option<ExprHandle<S>> {
        match self {
            ExprView::Atom(_) => None,
            ExprView::Node { head, .. } => Some(*head),
        }
    }

    pub fn get_args(&self) -> Option<ArgsHandle<S>> {
        match self {
            ExprView::Atom(_) => None,
            ExprView::Node { args, .. } => Some(*args),
        }
    }

    pub fn get_symbol(&self) -> Option<&str> {
        match self {
            Self::Atom(Atom::Symbol(sym)) => Some(sym),
            _ => None,
        }
    }

    pub fn get_number(&self) -> Option<&Number> {
        match self {
            Self::Atom(Atom::Number(num)) => Some(num),
            _ => None,
        }
    }

    pub fn is_symbol<T: AsRef<str>>(&self, sym: T) -> bool {
        self.get_symbol()
            .map(|s| s == sym.as_ref())
            .unwrap_or(false)
    }

    pub fn is_node<T: AsRef<str>>(
        &self,
        pool: &ExprPool,
        head_sym: T,
        arity: Option<usize>,
    ) -> bool {
        match self {
            Self::Node { head, args } => {
                head.view(pool).is_symbol(head_sym)
                    && arity.map(|arity| args.len(pool) == arity).unwrap_or(true)
            }
            _ => false,
        }
    }

    pub fn is_number(&self, n: i64) -> bool {
        let ExprView::Atom(Atom::Number(num)) = self else {
            return false;
        };

        num.try_to_i64().map(|num| num == n).unwrap_or(false)
    }
}
