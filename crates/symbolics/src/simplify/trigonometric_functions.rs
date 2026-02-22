use expr_macro::{norm_expr, raw_expr};

use crate::{
    atom::Atom,
    chain_replace_quick_and_dirty,
    expr::{Expr, NormalizedExpr},
    matcher::MatchIter,
    pattern::Pattern,
};

impl Expr {
    fn replace_symbols(self, s: &str, replacement: &Expr) -> Expr {
        self.map_bottom_up(&|e| {
            if e.matches_symbol(s) {
                replacement.clone()
            } else {
                e
            }
        })
    }

    // TODO: implement proper rewrite engine
    fn replace_all_quick_and_dirty(
        self,
        norm_pattern_expr: NormalizedExpr,
        replacement: Expr,
    ) -> Expr {
        let pattern_expr = norm_pattern_expr.take_expr();

        self.map_bottom_up(&|e| {
            let pattern = Pattern::from_expr(&pattern_expr);

            if let Some(ctx) = MatchIter::new(&e, pattern).next() {
                let mut new_term = replacement.clone();
                for (k, v) in ctx.iter() {
                    new_term = new_term.replace_symbols(k, v.get_one().unwrap());
                }
                new_term
            } else {
                e
            }
        })
    }
}

pub fn simplify_trigon(expr: Expr) -> Expr {
    chain_replace_quick_and_dirty!(expr,
        { Sin[pi / 12] } => { Sqrt[2] * (Sqrt[3] - 1) / 4         },
        { Sin[pi / 10] } => { (Sqrt[5] - 1) / 4                   },
        { Sin[pi / 8]  } => { (Sqrt[2 - Sqrt[2]]) / 2             },
        { Sin[pi / 6]  } => { 1/2                                 },
        { Sin[pi / 5]  } => { (Sqrt[2] * (Sqrt[5 - Sqrt[5]])) / 4 },
        { Sin[pi / 4]  } => { 1/Sqrt[2]                           },

        { Cos[pi / 12] } => { Sqrt[2] * (Sqrt[3] + 1) / 4         },
        { Cos[pi / 10] } => { (Sqrt[2] * (Sqrt[5 + Sqrt[5]])) / 4 },
        { Cos[pi / 8]  } => { (Sqrt[2 + Sqrt[2]]) / 2             },
        { Cos[pi / 6]  } => { Sqrt[3] / 2                         },
        { Cos[pi / 5]  } => { (Sqrt[5] + 1) / 4                   },
        { Cos[pi / 4]  } => { 1/Sqrt[2]                           },

        { Sqrt[1 - Cos[Pattern[x, Blank[]]]^2] } => { Sin[x] },
        { Sqrt[1 - Sin[Pattern[x, Blank[]]]^2] } => { Cos[x] },
    )
}
