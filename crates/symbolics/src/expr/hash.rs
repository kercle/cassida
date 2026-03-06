use std::hash::{DefaultHasher, Hash, Hasher};

use crate::expr::Expr;

impl<A> Hash for Expr<A> {
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

impl<A> Expr<A> {
    pub fn to_digest(&self) -> u64 {
        let mut state = DefaultHasher::new();
        self.hash(&mut state);
        state.finish()
    }

    pub fn recompute_digest(mut self) -> Self {
        let digest: u64 = self.to_digest();
        match &mut self {
            Expr::Atom {
                digest: digest_ref, ..
            }
            | Expr::Node {
                digest: digest_ref, ..
            } => *digest_ref = digest,
        }
        self
    }
}
