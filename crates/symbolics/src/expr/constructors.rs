use numbers::Number;

use crate::{
    atom::Atom,
    builtin::{ADD_HEAD, CANNONICAL_HEAD_LIST, MUL_HEAD},
    expr::{ExprKind, RawExpr},
    pattern::BLANK_ONE_HEAD,
};

pub static EXPR_PLACEHOLDER: std::sync::LazyLock<RawExpr> = std::sync::LazyLock::new(RawExpr::new_placeholder);

impl RawExpr {
    pub fn new(kind: ExprKind<RawExpr>) -> Self {
        Self::new_unchecked(kind)
    }

    pub fn new_placeholder() -> Self {
        Self::new_symbol("")
    }

    pub fn new_node<T: Into<RawExpr>>(head: T, args: Vec<RawExpr>) -> Self {
        Self::new_unchecked(ExprKind::Node {
            head: Box::new(head.into()),
            args,
        })
    }

    pub fn new_atom(entry: Atom) -> Self {
        Self::new_unchecked(ExprKind::Atom { entry })
    }

    pub fn new_unary_node<T: Into<RawExpr>>(head: T, arg: RawExpr) -> Self {
        Self::new_node(head, vec![arg])
    }

    pub fn new_binary_node<T: Into<RawExpr>>(head: T, lhs: RawExpr, rhs: RawExpr) -> Self {
        Self::new_node(head, vec![lhs, rhs])
    }

    pub fn new_number<T: Into<Number>>(value: T) -> Self {
        Self::new_atom(Atom::Number(value.into()))
    }

    pub fn new_number_integer(value: i64) -> Self {
        Self::new_number(Number::from_i64(value))
    }

    pub fn new_number_rational(numerator: i64, denominator: u64) -> Result<Self, String> {
        Ok(Self::new_number(Number::new_rational_from_i64(
            numerator,
            denominator,
        )?))
    }

    pub fn new_symbol<T: AsRef<str>>(symb: T) -> Self {
        Self::new_atom(Atom::Symbol(symb.as_ref().to_string()))
    }

    pub fn new_blank() -> Self {
        Self::new_node(BLANK_ONE_HEAD, vec![])
    }

    pub fn new_list(args: Vec<RawExpr>) -> Self {
        Self::new_node(CANNONICAL_HEAD_LIST, args)
    }

    pub fn collapse_add(args: Vec<RawExpr>) -> RawExpr {
        match args.len() {
            0 => Number::zero().into(),
            1 => args.into_iter().next().unwrap(),
            _ => RawExpr::new_node(ADD_HEAD, args),
        }
    }

    pub fn collapse_mul(args: Vec<RawExpr>) -> RawExpr {
        match args.len() {
            0 => Number::one().into(),
            1 => args.into_iter().next().unwrap(),
            _ => RawExpr::new_node(MUL_HEAD, args),
        }
    }
}
