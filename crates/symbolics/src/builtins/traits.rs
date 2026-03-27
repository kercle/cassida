use crate::{
    builtins::BuiltInCategory,
    expr::{Expr, ExprKind, NormExpr, RawExpr},
};

#[derive(Clone, Debug)]
pub struct BuiltInDoc {
    pub category: BuiltInCategory,
    pub title: &'static str,
    pub summary: &'static str,
    pub pattern_doc: Vec<PatternDoc>,
    pub examples: Vec<(&'static str, &'static str)>,
    pub related: Vec<&'static str>,
}

#[derive(Clone, Debug)]
pub struct PatternDoc {
    pub pattern: RawExpr,
    pub summary: String,
}

impl PatternDoc {
    pub fn new<T: ToString>(pattern: RawExpr, summary: T) -> Self {
        Self {
            pattern,
            summary: summary.to_string(),
        }
    }
}

pub enum ApplicationError {
    ExprNotNode,
    HeadMismatch,
    ArityMismatch,
    ExpectedSymbolAt(usize),
    ExpectedTupleAt(usize),
}

pub trait BuiltIn {
    fn doc(&self) -> BuiltInDoc;

    fn apply_all(&self, expr: NormExpr) -> NormExpr {
        expr
    }

    fn head() -> &'static str
    where
        Self: Sized;

    fn head_dyn(&self) -> &'static str;

    fn validate_application_of<S>(
        head: &Expr<S>,
        children: &[Expr<S>],
    ) -> Result<(), ApplicationError>
    where
        Self: Sized;

    fn validate_application<S>(expr: &Expr<S>) -> Result<(), ApplicationError>
    where
        Self: Sized,
    {
        match expr.kind() {
            ExprKind::Atom { .. } => Err(ApplicationError::ExprNotNode),
            ExprKind::Node { head, args } => Self::validate_application_of(head, args),
        }
    }

    fn is_application_of<S>(head: &Expr<S>, children: &[Expr<S>]) -> bool
    where
        Self: Sized,
    {
        Self::validate_application_of(head, children).is_ok()
    }

    fn is_application<S>(expr: &Expr<S>) -> bool
    where
        Self: Sized,
    {
        Self::validate_application(expr).is_ok()
    }
}
