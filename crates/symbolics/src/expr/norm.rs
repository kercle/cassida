use crate::expr::Expr;

impl<A: Clone + PartialEq + Default> Expr<A> {
    pub fn normalize(self) -> Self {
        todo!()
    }

    /// Flattens nested applications whenever `head_predicate`
    /// returns true.
    ///
    /// # Behavior
    ///
    /// - Atoms are returned unchanged.
    /// - Applications with a head not flagged by `head_predicate` are
    ///   reconstructed with their arguments recursively flattened.
    /// - Applications whose head is flagged by `head_predicate` have
    ///   their nested arguments merged into the parent argument list,
    ///   for all nested arguments that have the same head as their
    ///   parent.
    /// - Annotations in new expression are reset to Default::default().
    pub fn flatten(self, head_predicate: impl Fn(&Expr<A>) -> bool + Copy) -> Self {
        match self {
            Expr::Atom { .. } => self.drop_annotation(),
            Expr::App { head, args, .. } if head_predicate(&*head) => {
                let mut new_args = Vec::with_capacity(args.len());

                for arg in args.into_iter() {
                    let arg = arg.flatten(head_predicate);

                    match arg {
                        Expr::App { head: ch, args, .. } if *ch == *head => {
                            new_args.extend(args);
                        }
                        _ => {
                            new_args.push(arg.drop_annotation());
                        }
                    }
                }

                Expr::new_app(*head, new_args)
            }
            Expr::App { head, args, .. } => Expr::new_app(
                *head,
                args.into_iter()
                    .map(|a| a.flatten(head_predicate))
                    .collect(),
            ),
        }
    }

    /// Sort nested applications whenever `head_predicate` returns
    /// true.
    ///
    /// # Behavior
    ///
    /// - Atoms are returned unchanged.
    /// - Applications with a head not flagged by `head_predicate` are
    ///   reconstructed and sort_args is propagated to args.
    /// - Applications whose head is flagged by `head_predicate` have
    ///   their nested arguments sorted.
    /// - Annotations in new expression are reset to Default::default().
    pub fn sort_args(self, head_predicate: impl Fn(&Expr<A>) -> bool + Copy) -> Self {
        match self {
            Expr::Atom { .. } => self.drop_annotation(),
            Expr::App { head, args, .. } => {
                let mut args: Vec<Expr<A>> = args
                    .into_iter()
                    .map(|a| a.sort_args(head_predicate))
                    .collect();

                if head_predicate(&*head) {
                    args.sort();
                }

                Expr::new_app(*head, args)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mul(s: &[Expr<()>]) -> Expr<()> {
        Expr::new_app(Expr::new_symbol("Mul"), s.to_vec())
    }

    fn add(s: &[Expr<()>]) -> Expr<()> {
        Expr::new_app(Expr::new_symbol("Add"), s.to_vec())
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
}
