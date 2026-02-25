use std::fmt::{Debug, Error, Formatter};

use crate::{atom::Atom, expr::Expr, pattern::Pattern};

impl Debug for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Atom::Number(value) => write!(f, "{value}"),
            Atom::Symbol(name) => write!(f, "{name}"),
            Atom::StringLiteral(value) => write!(f, "{value}"),
        }
    }
}

impl<A: Clone + PartialEq> Debug for Expr<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let s = match self {
            Expr::Atom { entry, .. } => format!("{entry:?}"),
            Expr::Node { head, args, .. } => {
                let head_str = if matches!(**head, Expr::Node { .. }) {
                    format!("({head:?})")
                } else {
                    format!("{head:?}")
                };
                let args_str: Vec<String> = args.iter().map(|a| format!("{a:?}")).collect();

                format!("{head_str}[{}]", args_str.join(", "))
            }
        };

        write!(f, "{s}")
    }
}

impl<'a, A: Clone + PartialEq> Debug for Pattern<'a, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        use Pattern::*;
        match self {
            Literal(e) => write!(f, "Literal{{{e:?}}}"),
            Blank {
                bind_name,
                match_head,
                predicate,
            } => write!(f, "Blank{{{bind_name:?}, {match_head:?}, {predicate:?}}}"),
            BlankSeq {
                bind_name,
                match_head,
                predicate,
            } => write!(
                f,
                "BlankSeq{{{bind_name:?}, {match_head:?}, {predicate:?}}}"
            ),
            BlankNullSeq {
                bind_name,
                match_head,
                predicate,
            } => write!(
                f,
                "BlankNullSeq{{{bind_name:?}, {match_head:?}, {predicate:?}}}"
            ),
            Node {
                head,
                args,
                predicate,
            } => write!(f, "Node{{{head:?}, {args:?}, {predicate:?}}}"),
        }
    }
}
