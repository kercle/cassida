use std::hash::{DefaultHasher, Hash, Hasher};

use crate::expr::Expr;

impl<A: Clone + PartialEq> Hash for Expr<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use Expr::*;

        match self {
            Atom { entry, .. } => {
                0u8.hash(state);
                entry.hash(state);
            }
            Node { head, args, .. } => {
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
