use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

use crate::atom::Atom;

#[derive(Clone)]
pub struct Raw;

#[derive(Clone)]
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

// -------- Interner brainstorming -------------

type ExprId = u32;
type ArgsId = u32;

#[derive(Clone, PartialEq, Eq, Hash)]
enum ExprObj {
    Atom(Atom),
    Node { head: ExprId, args: ArgsId },
}

struct ExprInterner {
    objs: Vec<ExprObj>,
    args: Vec<Vec<ExprId>>,

    obj_map: HashMap<ExprObj, ExprId>,
    args_map: HashMap<Vec<ExprId>, ArgsId>,
}

impl ExprInterner {
    fn new() -> Self {
        ExprInterner {
            objs: Vec::new(),
            args: Vec::new(),
            obj_map: HashMap::new(),
            args_map: HashMap::new(),
        }
    }

    fn intern_args(&mut self, args: Vec<ExprId>) -> ArgsId {
        if let Some(&id) = self.args_map.get(&args) {
            return id;
        }
        let id = self.args.len() as ArgsId;
        self.args_map.insert(args.clone(), id);
        self.args.push(args);
        id
    }

    fn intern(&mut self, obj: ExprObj) -> ExprId {
        if let Some(&id) = self.obj_map.get(&obj) {
            return id;
        }
        let id = self.objs.len() as ExprId;
        self.obj_map.insert(obj.clone(), id);
        self.objs.push(obj);
        id
    }

    fn atom(&mut self, a: Atom) -> ExprId {
        self.intern(ExprObj::Atom(a))
    }

    fn node(&mut self, head: ExprId, args: Vec<ExprId>) -> ExprId {
        let args_id = self.intern_args(args);
        self.intern(ExprObj::Node {
            head,
            args: args_id,
        })
    }

    pub fn intern_expr<S>(&mut self, expr: &Expr<S>) -> ExprId {
        match expr.kind() {
            ExprKind::Atom { entry } => self.atom(entry.clone()),
            ExprKind::Node { head, args } => {
                let head_id = self.intern_expr(head);
                let arg_ids: Vec<ExprId> = args.iter().map(|arg| self.intern_expr(arg)).collect();
                self.node(head_id, arg_ids)
            }
        }
    }

    pub fn intern_raw(&mut self, expr: &RawExpr) -> RawHandle<'_> {
        let id = self.intern_expr(expr);
        ExprHandle::new(self, id)
    }

    pub fn intern_norm(&mut self, expr: &NormExpr) -> NormHandle<'_> {
        let id = self.intern_expr(expr);
        ExprHandle::new(self, id)
    }
}

#[derive(Copy, Clone)]
struct ExprHandle<'a, S> {
    interner: &'a ExprInterner,
    root: ExprId,
    _state: PhantomData<S>,
}

pub type RawHandle<'a> = ExprHandle<'a, Raw>;
pub type NormHandle<'a> = ExprHandle<'a, Normalized>;

impl<'a, S> ExprHandle<'a, S> {
    fn new(interner: &'a ExprInterner, root: ExprId) -> Self {
        ExprHandle {
            interner,
            root,
            _state: PhantomData,
        }
    }

    fn id(&self) -> ExprId {
        self.root
    }
}

enum ExprView<'a, S> {
    Atom(&'a Atom),
    Node {
        head: ExprHandle<'a, S>,
        args: &'a [ExprId],
    },
}

impl<'a, S: Copy> ExprHandle<'a, S> {
    fn view(self) -> ExprView<'a, S> {
        match &self.interner.objs[self.root as usize] {
            ExprObj::Atom(a) => ExprView::Atom(a),
            ExprObj::Node { head, args } => ExprView::Node {
                head: ExprHandle::new(self.interner, *head),
                args: &self.interner.args[*args as usize],
            },
        }
    }

    fn children(self) -> impl Iterator<Item = ExprHandle<'a, S>> {
        let args = match &self.interner.objs[self.root as usize] {
            ExprObj::Node { args, .. } => &self.interner.args[*args as usize],
            ExprObj::Atom(_) => &[] as &[ExprId],
        };
        args.iter()
            .map(move |&id| ExprHandle::new(self.interner, id))
    }

    fn eq(self, other: ExprHandle<'_, S>) -> bool {
        self.root == other.root
    }
}

impl<'a> ExprHandle<'a, Raw> {
    pub(super) fn mark_normalized(self) -> ExprHandle<'a, Normalized> {
        ExprHandle::new(self.interner, self.root)
    }
}
