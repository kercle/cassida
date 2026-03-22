use crate::{
    builtin::*,
    builtins::{self, traits::BuiltIn},
    expr::{Expr, ExprKind, RawExpr},
    pattern::{
        BLANK_NULL_SEQ_HEAD, BLANK_ONE_HEAD, BLANK_SEQ_HEAD, PATTERN_HEAD, PATTERN_TEST_HEAD,
    },
};

use numbers::Number;
use parser::ast::ParserAst;

use crate::atom::Atom;

impl<S> ExprKind<S> {
    pub fn into_raw_expr(self) -> RawExpr {
        let raw: ExprKind<RawExpr> = unsafe { std::mem::transmute(self) };
        RawExpr::new(raw)
    }
}

impl<S> Expr<S> {
    pub fn into_raw(self) -> RawExpr {
        unsafe { std::mem::transmute(self) }
    }

    pub fn as_raw_ref(&self) -> &RawExpr {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T: Into<Atom>> From<T> for RawExpr {
    fn from(x: T) -> Self {
        RawExpr::new_atom(x.into())
    }
}

impl From<ExprKind<RawExpr>> for RawExpr {
    fn from(value: ExprKind<RawExpr>) -> Self {
        Self::new_unchecked(value)
    }
}

impl RawExpr {
    pub fn from_i64(value: i64) -> Self {
        Self::new_number(Number::from_i64(value))
    }
}

impl From<ParserAst> for RawExpr {
    fn from(ast: ParserAst) -> Self {
        use ParserAst::*;
        match ast {
            Constant { value } => Self::new_number(value),
            Symbol { name } => Self::new_symbol(name),
            LesserThan { lhs, rhs } => {
                Self::new_binary_node(builtins::Less::head(), Self::from(*lhs), Self::from(*rhs))
            }
            LesserEq { lhs, rhs } => {
                Self::new_binary_node(builtins::LessEqual::head(), Self::from(*lhs), Self::from(*rhs))
            }
            Equals { lhs, rhs } => {
                Self::new_binary_node(builtins::Equal::head(), Self::from(*lhs), Self::from(*rhs))
            }
            GreaterEq { lhs, rhs } => {
                Self::new_binary_node(builtins::GreaterEqual::head(), Self::from(*lhs), Self::from(*rhs))
            }
            GreaterThan { lhs, rhs } => {
                Self::new_binary_node(builtins::Greater::head(), Self::from(*lhs), Self::from(*rhs))
            }
            Add { lhs, rhs } => Self::new_binary_node(ADD_HEAD, Self::from(*lhs), Self::from(*rhs)),
            Sub { lhs, rhs } => Self::new_binary_node(SUB_HEAD, Self::from(*lhs), Self::from(*rhs)),
            Negation { arg } => Self::new_unary_node(NEG_HEAD, Self::from(*arg)),
            Mul { lhs, rhs } => Self::new_binary_node(MUL_HEAD, Self::from(*lhs), Self::from(*rhs)),
            Div { lhs, rhs } => Self::new_binary_node(DIV_HEAD, Self::from(*lhs), Self::from(*rhs)),
            Pow { lhs, rhs } => Self::new_binary_node(POW_HEAD, Self::from(*lhs), Self::from(*rhs)),
            Factorial { arg } => {
                Self::new_unary_node(builtins::Factorial::head(), Self::from(*arg))
            }
            FunctionCall { name, args } => {
                let head = Self::new_symbol(name);
                let args = args.into_iter().map(Self::from).collect();

                Self::new_node(head, args)
            }
            Blank {
                bind_name,
                head_constraint,
                optional,
            } => {
                let inner = make_blank_variant(BLANK_ONE_HEAD, bind_name, head_constraint);

                if optional {
                    RawExpr::new_unary_node(builtins::Optional::head(), inner)
                } else {
                    inner
                }
            }
            BlankSeq {
                bind_name,
                head_constraint,
                optional,
            } => {
                let inner = make_blank_variant(BLANK_SEQ_HEAD, bind_name, head_constraint);

                if optional {
                    RawExpr::new_unary_node(builtins::Optional::head(), inner)
                } else {
                    inner
                }
            }
            BlankNullSeq {
                bind_name,
                head_constraint,
                optional,
            } => {
                let inner = make_blank_variant(BLANK_NULL_SEQ_HEAD, bind_name, head_constraint);

                if optional {
                    RawExpr::new_unary_node(builtins::Optional::head(), inner)
                } else {
                    inner
                }
            }
            PatternTest { pattern, predicate } => RawExpr::new_binary_node(
                PATTERN_TEST_HEAD,
                Self::from(*pattern),
                Self::from(*predicate),
            ),
            Compound { nodes } => {
                let nodes = nodes.into_iter().map(Self::from).collect();
                Self::new_node(builtins::Compound::head(), nodes)
            }
        }
    }
}

fn make_blank_variant(
    head: &str,
    bind_name: Option<String>,
    head_constraint: Option<String>,
) -> RawExpr {
    let args = if let Some(head_constraint) = head_constraint {
        vec![RawExpr::new_symbol(head_constraint)]
    } else {
        Vec::new()
    };
    let ret = RawExpr::new_node(head, args);

    if let Some(bind_name) = bind_name {
        RawExpr::new_node(PATTERN_HEAD, vec![RawExpr::new_symbol(bind_name), ret])
    } else {
        ret
    }
}
