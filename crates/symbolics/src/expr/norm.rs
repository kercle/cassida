use std::hash::{DefaultHasher, Hash, Hasher};

use numbers::Number;

use crate::{
    expr::{Expr, atom::Atom},
    parser::ast::{ADD_HEAD, MUL_HEAD},
};

fn cannonical_fold_ac_with_neutral_el<A: Default + Clone + PartialEq>(
    head: &Expr<A>,
    value: Number,
    is_neutral_element: bool,
    args_rest: &mut Vec<Expr<A>>,
) -> Expr<A> {
    if args_rest.is_empty() {
        // Only number left: collapse node
        Expr::new_number(value)
    } else if is_neutral_element {
        // Constants fold to neutral
        if args_rest.len() == 1 {
            // Only one arg left: collapse node
            args_rest.pop().unwrap()
        } else {
            // New node contains only remaining args
            Expr::new_compound(head.clone(), args_rest.clone())
        }
    } else {
        // Multiple remaining args and number is not neutral element
        args_rest.push(Expr::new_number(value));
        Expr::new_compound(head.clone(), args_rest.clone())
    }
}

fn cannonical_fold_op<A: Default + Clone + PartialEq>(
    head: &Expr<A>,
    c_iter: &mut dyn Iterator<Item = &Number>,
    args_rest: &mut Vec<Expr<A>>,
) -> Option<Expr<A>> {
    if head.matches_symbol(ADD_HEAD) {
        let value: Number = c_iter.sum();
        let is_neutral_element = value.is_zero();
        Some(cannonical_fold_ac_with_neutral_el(
            head,
            value,
            is_neutral_element,
            args_rest,
        ))
    } else if head.matches_symbol(MUL_HEAD) {
        let value: Number = c_iter.product();

        if value.is_zero() {
            return Some(Number::zero().into());
        }

        let is_neutral_element = value.is_one();
        Some(cannonical_fold_ac_with_neutral_el(
            head,
            value,
            is_neutral_element,
            args_rest,
        ))
    } else {
        None
    }
}

impl<A: Clone + PartialEq + Default> Expr<A> {
    pub fn normalize(self) -> Self {
        let head_predicate = |e: &Expr<A>| e.matches_symbol(ADD_HEAD) || e.matches_symbol(MUL_HEAD);

        let mut last_hash = None;
        let mut current = self;

        loop {
            current = current
                .flatten(&head_predicate)
                .fold_constants(|head, c_iter, args_rest| {
                    cannonical_fold_op(head, c_iter, args_rest)
                })
                .sort_args(&head_predicate);

            let mut state = DefaultHasher::new();
            current.hash(&mut state);
            let current_hash = state.finish();

            if let Some(last_hash) = last_hash {
                if current_hash == last_hash {
                    // if hashes agree, assume fixed point
                    break;
                }
            }

            last_hash = Some(current_hash);
        }

        current
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
        fold_op: impl Fn(
            &Expr<A>,
            &mut dyn Iterator<Item = &Number>,
            &mut Vec<Expr<A>>,
        ) -> Option<Expr<A>>
        + Copy,
    ) -> Self {
        match self {
            Expr::Atom { .. } => self.drop_annotation(),
            Expr::Compound { head, args, .. } => {
                // Split constant and other arguments
                let (c_args, mut args): (Vec<Expr<A>>, Vec<Expr<A>>) = args
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
                if let Some(e) = fold_op(&head, &mut c_iter, &mut args) {
                    e
                } else {
                    args.extend(c_args);
                    Expr::new_compound(*head, args)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{expr::generator::*, symbol};

    fn mul(s: &[Expr<()>]) -> Expr<()> {
        Expr::new_compound(Expr::new_symbol(MUL_HEAD), s.to_vec())
    }

    fn add(s: &[Expr<()>]) -> Expr<()> {
        Expr::new_compound(Expr::new_symbol(ADD_HEAD), s.to_vec())
    }

    #[test]
    fn test_expr_flatten() {
        let (x, y) = symbol!("x", "y");
        let expr: Expr<()> = 2 + x + 3 * (5 + (1 + (1 + y)));

        assert_eq!(
            expr.flatten(|e| e.matches_symbol(ADD_HEAD)),
            add(&[
                2.into(),
                x.build(),
                mul(&[3.into(), add(&[5.into(), 1.into(), 1.into(), y.build()])])
            ])
        );
    }

    #[test]
    fn test_expr_sorting() {
        let (x, y) = symbol!("x", "y");

        let expr1 = add(&[
            x.build(),
            2.into(),
            mul(&[3.into(), add(&[5.into(), y.build(), 1.into(), 1.into()])]),
        ]);

        assert_eq!(
            expr1.sort_args(|e| e.matches_symbol(ADD_HEAD)),
            add(&[
                2.into(),
                x.build(),
                mul(&[3.into(), add(&[1.into(), 1.into(), 5.into(), y.build()])]),
            ])
        );
    }

    #[test]
    fn test_expr_normalizing() {
        let (x, y) = symbol!("x", "y");

        let expr1 = add(&[
            x.build(),
            2.into(),
            mul(&[3.into(), add(&[5.into(), y.build(), 1.into(), 1.into()])]),
        ]);

        assert_eq!(
            expr1.normalize(),
            add(&[
                2.into(),
                x.build(),
                mul(&[3.into(), add(&[7.into(), y.build()])]),
            ])
        );
    }
}
