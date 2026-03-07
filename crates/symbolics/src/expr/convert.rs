use crate::builtin::*;

use numbers::Number;
use parser::ast::ParserAst;

use crate::{atom::Atom, expr::Expr};

impl<A, T: Into<Atom>> From<T> for Expr<A>
where
    A: Default,
{
    fn from(x: T) -> Self {
        Expr::new_atom(x.into())
    }
}

impl<A: Default> From<ParserAst> for Expr<A> {
    fn from(ast: ParserAst) -> Self {
        use ParserAst::*;
        match ast {
            Constant { value } => Self::new_number(value),
            Symbol { name } => Self::new_symbol(name),
            LesserThan { lhs, rhs } => {
                Self::new_binary_node(LT_HEAD, Self::from(*lhs), Self::from(*rhs))
            }
            LesserEq { lhs, rhs } => {
                Self::new_binary_node(LE_HEAD, Self::from(*lhs), Self::from(*rhs))
            }
            Equals { lhs, rhs } => {
                Self::new_binary_node(EQ_HEAD, Self::from(*lhs), Self::from(*rhs))
            }
            GreaterEq { lhs, rhs } => {
                Self::new_binary_node(GE_HEAD, Self::from(*lhs), Self::from(*rhs))
            }
            GreaterThan { lhs, rhs } => {
                Self::new_binary_node(GT_HEAD, Self::from(*lhs), Self::from(*rhs))
            }
            Add { lhs, rhs } => Self::new_binary_node(ADD_HEAD, Self::from(*lhs), Self::from(*rhs)),
            Sub { lhs, rhs } => Self::new_binary_node(SUB_HEAD, Self::from(*lhs), Self::from(*rhs)),
            Negation { arg } => Self::new_unary_node(NEG_HEAD, Self::from(*arg)),
            Mul { lhs, rhs } => Self::new_binary_node(MUL_HEAD, Self::from(*lhs), Self::from(*rhs)),
            Div { lhs, rhs } => Self::new_binary_node(DIV_HEAD, Self::from(*lhs), Self::from(*rhs)),
            Pow { lhs, rhs } => Self::new_binary_node(POW_HEAD, Self::from(*lhs), Self::from(*rhs)),
            FunctionCall { name, args } => {
                let head = Self::new_symbol(name);
                let args = args.into_iter().map(Self::from).collect();

                Self::new_node(head, args)
            }
            Block { .. } => todo!(),
        }
    }
}

impl<A> Expr<A>
where
    A: Default + Clone + PartialEq,
{
    pub fn from_i64(value: i64) -> Self {
        Self::new_number(Number::from_i64(value))
    }
}
