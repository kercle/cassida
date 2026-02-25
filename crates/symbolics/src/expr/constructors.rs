use numbers::Number;

use crate::{atom::Atom, builtin::CANNONICAL_HEAD_LIST, expr::Expr, pattern::BLANK_ONE_HEAD};

impl<A> Expr<A> {
    pub fn new_node_with_annotation(head: Expr<A>, args: Vec<Expr<A>>, ann: A) -> Self {
        Expr::Node {
            head: Box::new(head),
            args,
            annotation: ann,
        }
    }
}

impl<A> Expr<A>
where
    A: Default,
{
    pub fn new_node<T: Into<Expr<A>>>(head: T, args: Vec<Expr<A>>) -> Self {
        Expr::Node {
            head: Box::new(head.into()),
            args,
            annotation: A::default(),
        }
    }

    pub fn new_number<T: Into<Number>>(value: T) -> Self {
        Expr::Atom {
            entry: Atom::Number(value.into()),
            annotation: A::default(),
        }
    }

    pub fn new_number_rational(numerator: i64, denominator: u64) -> Result<Self, String> {
        Ok(Self::new_number(Number::new_rational_from_i64(
            numerator,
            denominator,
        )?))
    }

    pub fn new_symbol<T: AsRef<str>>(symb: T) -> Self {
        Expr::Atom {
            entry: Atom::Symbol(symb.as_ref().to_string()),
            annotation: A::default(),
        }
    }

    pub fn new_blank() -> Self {
        Expr::new_node(BLANK_ONE_HEAD, vec![])
    }

    pub fn new_list(args: Vec<Expr<A>>) -> Self {
        Expr::new_node(CANNONICAL_HEAD_LIST, args)
    }
}
