use numbers::Number;

use crate::{atom::Atom, builtin::CANNONICAL_HEAD_LIST, expr::Expr, pattern::BLANK_ONE_HEAD};

impl<A> Expr<A> {
    pub fn new_node_with_annotation(head: Expr<A>, args: Vec<Expr<A>>, annotation: A) -> Self {
        Expr::Node {
            head: Box::new(head),
            args,
            digest: 0,
            annotation,
        }
        .recompute_digest()
    }

    pub fn new_atom_with_annotation(entry: Atom, annotation: A) -> Self {
        Expr::Atom {
            entry,
            digest: 0,
            annotation,
        }
        .recompute_digest()
    }
}

impl<A> Expr<A>
where
    A: Default,
{
    pub fn new_node<T: Into<Expr<A>>>(head: T, args: Vec<Expr<A>>) -> Self {
        Expr::new_node_with_annotation(head.into(), args, A::default())
    }

    pub fn new_atom(entry: Atom) -> Self {
        Expr::new_atom_with_annotation(entry, A::default())
    }

    pub fn new_number<T: Into<Number>>(value: T) -> Self {
        Expr::new_atom_with_annotation(Atom::Number(value.into()), A::default())
    }

    pub fn new_number_rational(numerator: i64, denominator: u64) -> Result<Self, String> {
        Ok(Self::new_number(Number::new_rational_from_i64(
            numerator,
            denominator,
        )?))
    }

    pub fn new_symbol<T: AsRef<str>>(symb: T) -> Self {
        Expr::new_atom_with_annotation(Atom::Symbol(symb.as_ref().to_string()), A::default())
    }

    pub fn new_blank() -> Self {
        Expr::new_node(BLANK_ONE_HEAD, vec![])
    }

    pub fn new_list(args: Vec<Expr<A>>) -> Self {
        Expr::new_node(CANNONICAL_HEAD_LIST, args)
    }
}
