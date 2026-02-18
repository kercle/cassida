use numbers::Number;

use crate::expr::{Expr, atom::Atom};

impl<A: Clone + PartialEq + Default> Expr<A> {
    pub fn normalize(self) -> Self {
        self.flatten(|e: &Expr<A>| e.is_symbol("Add") || e.is_symbol("Mul"))
            .fold_constants(|h: &Expr<A>, c_iter: &mut dyn Iterator<Item = &Number>| {
                if h.is_symbol("Add") {
                    Some(c_iter.sum())
                } else if h.is_symbol("Mul") {
                    Some(c_iter.product())
                } else {
                    None
                }
            })
            .sort_args(|e: &Expr<A>| e.is_symbol("Add") || e.is_symbol("Mul"))
    }

    /// Flattens nested compounds whenever `head_predicate`
    /// returns true.
    ///
    /// # Behavior
    ///
    /// - Atoms are returned unchanged.
    /// - Compounds with a head not flagged by `head_predicate` are
    ///   reconstructed with their arguments recursively flattened.
    /// - Compounds whose head is flagged by `head_predicate` have
    ///   their nested arguments merged into the parent argument list,
    ///   for all nested arguments that have the same head as their
    ///   parent.
    /// - Annotations in new expression are reset to Default::default().
    pub fn flatten(self, head_predicate: impl Fn(&Expr<A>) -> bool + Copy) -> Self {
        match self {
            Expr::Atom { .. } => self.drop_annotation(),
            Expr::Compound { head, args, .. } if head_predicate(&*head) => {
                let mut new_args = Vec::with_capacity(args.len());

                for arg in args.into_iter() {
                    let arg = arg.flatten(head_predicate);

                    match arg {
                        Expr::Compound { head: ch, args, .. } if *ch == *head => {
                            new_args.extend(args);
                        }
                        _ => {
                            new_args.push(arg.drop_annotation());
                        }
                    }
                }

                Expr::new_compound(*head, new_args)
            }
            Expr::Compound { head, args, .. } => Expr::new_compound(
                *head,
                args.into_iter()
                    .map(|a| a.flatten(head_predicate))
                    .collect(),
            ),
        }
    }

    /// Sort nested Compounds whenever `head_predicate` returns
    /// true.
    ///
    /// # Behavior
    ///
    /// - Atoms are returned unchanged.
    /// - Compounds with a head not flagged by `head_predicate` are
    ///   reconstructed and sort_args is propagated to args.
    /// - Compounds whose head is flagged by `head_predicate` have
    ///   their nested arguments sorted.
    /// - Annotations in new expression are reset to Default::default().
    pub fn sort_args(self, head_predicate: impl Fn(&Expr<A>) -> bool + Copy) -> Self {
        match self {
            Expr::Atom { .. } => self.drop_annotation(),
            Expr::Compound { head, args, .. } => {
                let mut args: Vec<Expr<A>> = args
                    .into_iter()
                    .map(|a| a.sort_args(head_predicate))
                    .collect();

                if head_predicate(&*head) {
                    args.sort();
                }

                Expr::new_compound(*head, args)
            }
        }
    }

    pub fn fold_constants(
        self,
        fold_op: impl Fn(&Expr<A>, &mut dyn Iterator<Item = &Number>) -> Option<Number> + Copy,
    ) -> Self {
        match self {
            Expr::Atom { .. } => self.drop_annotation(),
            Expr::Compound { head, args, .. } => {
                // Split constant and other arguments
                let (mut c_args, mut args): (Vec<Expr<A>>, Vec<Expr<A>>) = args
                    .into_iter()
                    .map(|a| a.fold_constants(fold_op))
                    .partition(|e| e.is_number());

                // Extract a reference to the underlying numbers
                let mut c_iter = c_args.iter().map(|c| match c {
                    Expr::Atom {
                        entry: Atom::Number(v),
                        ..
                    } => v,
                    _ => unreachable!(),
                });

                // If constants in compound can be folded, do so. Otherwise reconstruct initial expression.
                if let Some(value) = fold_op(&head, &mut c_iter) {
                    args.push(Expr::new_number(value));
                    Expr::new_compound(*head, args)
                } else {
                    args.append(&mut c_args);
                    Expr::new_compound(*head, args)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mul(s: &[Expr<()>]) -> Expr<()> {
        Expr::new_compound(Expr::new_symbol("Mul"), s.to_vec())
    }

    fn add(s: &[Expr<()>]) -> Expr<()> {
        Expr::new_compound(Expr::new_symbol("Add"), s.to_vec())
    }

    fn x() -> Expr<()> {
        Expr::new_symbol("x")
    }

    fn y() -> Expr<()> {
        Expr::new_symbol("y")
    }

    #[test]
    fn test_expr_flatten() {
        let expr = 2 + x() + 3 * (5 + (1 + (1 + y())));

        assert_eq!(
            expr.flatten(|e| e.is_symbol("Add")),
            add(&[
                2.into(),
                x(),
                mul(&[3.into(), add(&[5.into(), 1.into(), 1.into(), y()])])
            ])
        );
    }

    #[test]
    fn test_expr_sorting() {
        let expr1 = add(&[
            x(),
            2.into(),
            mul(&[3.into(), add(&[5.into(), y(), 1.into(), 1.into()])]),
        ]);

        assert_eq!(
            expr1.sort_args(|e| e.is_symbol("Add")),
            add(&[
                2.into(),
                x(),
                mul(&[3.into(), add(&[1.into(), 1.into(), 5.into(), y()])]),
            ])
        );
    }

    #[test]
    fn test_expr_normalizing() {
        let expr1 = add(&[
            x(),
            2.into(),
            mul(&[3.into(), add(&[5.into(), y(), 1.into(), 1.into()])]),
        ]);

        assert_eq!(
            expr1.normalize(),
            add(&[2.into(), x(), mul(&[3.into(), add(&[7.into(), y()])]),])
        );
    }
}
