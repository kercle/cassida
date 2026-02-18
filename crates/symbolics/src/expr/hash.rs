use std::hash::{DefaultHasher, Hash, Hasher};

use crate::expr::{Expr, atom::Atom};

impl Hash for Atom {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use Atom::*;

        match self {
            Number(n) => {
                0u8.hash(state);
                n.hash(state);
            }
            Symbol(s) => {
                1u8.hash(state);
                s.hash(state);
            }
            StringLiteral(v) => {
                2u8.hash(state);
                v.hash(state);
            }
        }
    }
}

impl<A: Clone + PartialEq> Hash for Expr<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use Expr::*;

        match self {
            Atom { entry, .. } => {
                0u8.hash(state);
                entry.hash(state);
            }
            Compound { head, args, .. } => {
                1u8.hash(state);
                head.hash(state);
                args.len().hash(state);
                for a in args {
                    a.hash(state);
                }
            }
        }
    }
}

impl<A: Clone + PartialEq> Expr<A> {
    pub fn to_hash(&self) -> u64 {
        let mut state = DefaultHasher::new();
        self.hash(&mut state);
        state.finish()
    }
}
