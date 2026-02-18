use crate::expr::Expr;

const BLANK: &'static str = "Blank";
const BLANK_SEQ: &'static str = "BlankSeq";
const PATTERN: &'static str = "Pattern";

impl<A: Clone + PartialEq + Default> Expr<A> {
    pub fn matches(&self, other: &Self) -> bool {
        // use Expr::*;

        // match self {
        //     Atom { entry: e, .. } => {
        //         if let Atom { entry: oe, .. } = other {
        //             e == oe
        //         } else {
        //             false
        //         }
        //     }
        // }
        todo!()
    }
}
