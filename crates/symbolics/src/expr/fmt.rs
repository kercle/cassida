use std::fmt::{Debug, Error, Formatter};

use crate::expr::{Atom, Expr};

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
            Expr::Compound { head, args, .. } => {
                let head_str = if matches!(**head, Expr::Compound { .. }) {
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
