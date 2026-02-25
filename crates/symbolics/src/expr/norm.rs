use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

use numbers::Number;

use crate::{
    atom::Atom,
    builtin::{CANNONICAL_HEAD_APPLY, CANNONICAL_HEAD_SQRT, CANNONICAL_SYM_INDETERMINATE},
    expr::{Expr, NormalizedExpr, generator::pow},
    parser::ast::{ADD_HEAD, DIV_HEAD, MUL_HEAD, NEG_HEAD, POW_HEAD, SUB_HEAD},
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
            Expr::new_node(head.clone(), args_rest.clone())
        }
    } else {
        // Multiple remaining args and number is not neutral element
        args_rest.push(Expr::new_number(value));
        Expr::new_node(head.clone(), args_rest.clone())
    }
}

fn partition_constants<A: Default + Clone + PartialEq>(
    args: &[Expr<A>],
) -> (Vec<Number>, Vec<Expr<A>>) {
    let (c_args, v_args): (Vec<&Expr<A>>, Vec<&Expr<A>>) = args.iter().partition(|e| e.is_number());

    let c_args = c_args
        .iter()
        .map(|e| e.get_number().unwrap().clone())
        .collect();

    let v_args: Vec<Expr<A>> = v_args.iter().map(|&e| e.clone()).collect();

    (c_args, v_args)
}

fn contains_indeterminate<A: Default + Clone + PartialEq>(exprs: &[Expr<A>]) -> bool {
    exprs
        .iter()
        .any(|e| e.matches_symbol(CANNONICAL_SYM_INDETERMINATE))
}

fn cannonical_fold_op<A: Default + Clone + PartialEq>(
    head: &Expr<A>,
    args: &[Expr<A>],
) -> Option<Expr<A>> {
    if head.matches_symbol(ADD_HEAD) {
        if contains_indeterminate(args) {
            return Some(Expr::new_symbol(CANNONICAL_SYM_INDETERMINATE));
        }

        let (c_args, mut args) = partition_constants(args);
        let constant: Number = c_args.iter().sum();

        let is_neutral_element = constant.is_zero();
        Some(cannonical_fold_ac_with_neutral_el(
            head,
            constant,
            is_neutral_element,
            &mut args,
        ))
    } else if head.matches_symbol(MUL_HEAD) {
        if contains_indeterminate(args) {
            return Some(Expr::new_symbol(CANNONICAL_SYM_INDETERMINATE));
        }

        let (c_args, mut args) = partition_constants(args);
        let constant: Number = c_args.iter().product();

        if constant.is_zero() {
            return Some(Number::zero().into());
        }

        let is_neutral_element = constant.is_one();
        Some(cannonical_fold_ac_with_neutral_el(
            head,
            constant,
            is_neutral_element,
            &mut args,
        ))
    } else if head.matches_symbol(POW_HEAD) && args.len() == 2 {
        if contains_indeterminate(args) {
            return Some(Expr::new_symbol(CANNONICAL_SYM_INDETERMINATE));
        }

        let lhs = &args[0];
        let rhs = &args[1];

        if lhs.is_number_zero() {
            if rhs.is_number_zero() || rhs.is_number_negative() {
                Some(Expr::new_symbol(CANNONICAL_SYM_INDETERMINATE))
            } else {
                Some(Number::zero().into())
            }
        } else if lhs.is_number_one() || rhs.is_number_zero() {
            Some(Number::one().into())
        } else if rhs.is_number_one() {
            Some(lhs.clone())
        } else if let (Some(lhs), Some(rhs)) = (lhs.get_number(), rhs.get_number()) {
            if rhs.is_integer() {
                lhs.pow(rhs).map(|n| Expr::new_number(n)).ok()
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

impl<A: Clone + PartialEq + Default> Expr<A> {
    fn apply_till_fixed_point(self, f: impl Fn(Expr<A>) -> Expr<A>) -> Expr<A> {
        let mut last_hash = None;
        let mut current = self;

        loop {
            current = f(current);

            let mut state = DefaultHasher::new();
            current.hash(&mut state);
            let current_hash = state.finish();

            if let Some(last_hash) = last_hash
                && current_hash == last_hash
            {
                // if hashes agree, assume fixed point
                break;
            }

            last_hash = Some(current_hash);
        }

        current
    }

    pub fn normalize(self) -> Self {
        let head_predicate = |e: &Expr<A>| e.matches_symbol(ADD_HEAD) || e.matches_symbol(MUL_HEAD);

        self.apply_till_fixed_point(|e| {
            e.desugar()
                .reduce_structure()
                .flatten(head_predicate)
                .apply_to_nodes(|head, args| cannonical_fold_op(head, args))
                .sort_args(head_predicate)
        })
    }

    pub fn canonicalize(self) -> Self {
        let head_predicate = |e: &Expr<A>| e.matches_symbol(ADD_HEAD) || e.matches_symbol(MUL_HEAD);

        self.apply_till_fixed_point(|e| {
            e.reduce_structure()
                .flatten(head_predicate)
                .apply_to_nodes(|head, args| cannonical_fold_op(head, args))
                .sort_args(head_predicate)
        })
    }

    fn reduce_structure_apply(self) -> Self {
        self.map_bottom_up(&|expr| {
            use Expr::*;
            match expr {
                Node { head, mut args, .. }
                    if head.matches_symbol(CANNONICAL_HEAD_APPLY) && args.len() == 2 =>
                {
                    let rhs = args.pop().unwrap();
                    let lhs = args.pop().unwrap();

                    match rhs {
                        Atom { .. } => rhs,
                        Node { args: args_rhs, .. } => Expr::new_node(lhs, args_rhs),
                    }
                }
                _ => expr,
            }
        })
    }

    pub fn reduce_structure(self) -> Self {
        self.reduce_structure_apply()
    }

    /// Flattens nested nodes whenever `head_predicate`
    /// returns true.
    ///
    /// # Behavior
    ///
    /// - Atoms are returned unchanged.
    /// - Nodes with a head not flagged by `head_predicate` are
    ///   reconstructed with their arguments recursively flattened.
    /// - Nodes whose head is flagged by `head_predicate` have
    ///   their nested arguments merged into the parent argument list,
    ///   for all nested arguments that have the same head as their
    ///   parent.
    /// - Annotations in new expression are reset to Default::default().
    pub fn flatten(self, head_predicate: impl Fn(&Expr<A>) -> bool + Copy) -> Self {
        match self {
            Expr::Atom { .. } => self.annotation_to_default(),
            Expr::Node { head, args, .. } if head_predicate(&*head) => {
                let mut new_args = Vec::with_capacity(args.len());

                for arg in args.into_iter() {
                    let arg = arg.flatten(head_predicate);

                    match arg {
                        Expr::Node { head: ch, args, .. } if *ch == *head => {
                            new_args.extend(args);
                        }
                        _ => {
                            new_args.push(arg.annotation_to_default());
                        }
                    }
                }

                Expr::new_node(*head, new_args)
            }
            Expr::Node { head, args, .. } => Expr::new_node(
                *head,
                args.into_iter()
                    .map(|a| a.flatten(head_predicate))
                    .collect(),
            ),
        }
    }

    /// Sort nested Nodes whenever `head_predicate` returns
    /// true.
    ///
    /// # Behavior
    ///
    /// - Atoms are returned unchanged.
    /// - Nodes with a head not flagged by `head_predicate` are
    ///   reconstructed and sort_args is propagated to args.
    /// - Nodes whose head is flagged by `head_predicate` have
    ///   their nested arguments sorted.
    /// - Annotations in new expression are reset to Default::default().
    pub fn sort_args(self, head_predicate: impl Fn(&Expr<A>) -> bool + Copy) -> Self {
        match self {
            Expr::Atom { .. } => self.annotation_to_default(),
            Expr::Node { head, args, .. } => {
                let mut args: Vec<Expr<A>> = args
                    .into_iter()
                    .map(|a| a.sort_args(head_predicate))
                    .collect();

                if head_predicate(&*head) {
                    args.sort();
                }

                Expr::new_node(*head, args)
            }
        }
    }

    pub fn desugar(self) -> Self {
        match self {
            Expr::Atom { .. } => self,
            Expr::Node { head, mut args, .. }
                if head.matches_symbol(SUB_HEAD) && args.len() == 2 =>
            {
                let rhs = args.pop().unwrap().desugar();
                let lhs = args.pop().unwrap().desugar();

                Expr::new_node(
                    ADD_HEAD,
                    vec![lhs, Expr::new_node(MUL_HEAD, vec![(-1).into(), rhs])],
                )
            }
            Expr::Node { head, mut args, .. }
                if head.matches_symbol(NEG_HEAD) && args.len() == 1 =>
            {
                let arg = args.pop().unwrap().desugar();
                Expr::new_node(MUL_HEAD, vec![(-1).into(), arg])
            }
            Expr::Node { head, mut args, .. }
                if head.matches_symbol(DIV_HEAD) && args.len() == 2 =>
            {
                let rhs = args.pop().unwrap().desugar();
                let lhs = args.pop().unwrap().desugar();

                Expr::new_node(
                    MUL_HEAD,
                    vec![lhs, Expr::new_node(POW_HEAD, vec![rhs, (-1).into()])],
                )
            }
            Expr::Node { head, mut args, .. }
                if head.matches_symbol(CANNONICAL_HEAD_SQRT) && args.len() == 1 =>
            {
                let arg = args.pop().unwrap().desugar();
                let one_half = Number::new_rational_from_i64(1, 2).unwrap();
                Expr::new_node(POW_HEAD, vec![arg, one_half.into()])
            }
            Expr::Node { head, args, .. } => {
                let args: Vec<Expr<A>> = args.into_iter().map(|a| a.desugar()).collect();
                Expr::new_node(*head, args)
            }
        }
    }

    fn apply_to_nodes(
        self,
        f: impl Fn(&Expr<A>, &[Expr<A>]) -> Option<Expr<A>> + Copy,
    ) -> Self {
        match self {
            Expr::Atom { .. } => self.annotation_to_default(),
            Expr::Node { head, args, .. } => {
                let args: Vec<Expr<A>> =
                    args.into_iter().map(|a| a.apply_to_nodes(f)).collect();

                if let Some(e) = f(&head, &args) {
                    e
                } else {
                    Expr::new_node(*head, args)
                }
            }
        }
    }
}

impl<A: Clone + PartialEq + Default> NormalizedExpr<A> {
    pub fn split_coefficient(self) -> (Number, Expr<A>) {
        // We are normalized, so if there is a constant, it's the first arg
        // in a commutative operation.

        let expr = self.take_expr();
        match expr {
            Expr::Atom {
                entry: Atom::Number(val),
                ..
            } => (val, Number::one().into()),
            Expr::Node { head, mut args, .. } if head.matches_symbol(MUL_HEAD) => {
                if let Some(coeff) = args.first().and_then(|e| e.get_number()) {
                    let coeff = coeff.clone();
                    let _ = args.swap_remove(0);
                    (coeff.clone(), Expr::new_node(*head, args).normalize())
                } else {
                    (Number::one(), Expr::new_node(*head, args))
                }
            }
            _ => (Number::one(), expr),
        }
    }

    fn collect_like_terms_add_head(args: Vec<Expr<A>>) -> NormalizedExpr<A> {
        // Collect like terms preserves normalization
        let coeff_expr_pair_iter = args
            .into_iter()
            .map(|e| NormalizedExpr::new(e).split_coefficient());

        let mut args_map: HashMap<Expr<A>, Number> = HashMap::new();
        for (n, e) in coeff_expr_pair_iter {
            if let Some(coeff_current) = args_map.get_mut(&e) {
                *coeff_current += n;
            } else {
                args_map.insert(e, n);
            }
        }

        let new_args = args_map
            .drain()
            .map(|(e, n)| e * Expr::new_number(n))
            .collect();

        NormalizedExpr::new(Expr::new_node(ADD_HEAD, new_args))
    }

    fn collect_like_exponentials_in_mul(args: Vec<Expr<A>>) -> NormalizedExpr<A> {
        // Collect like terms preserves normalization
        let coeff_expr_pair_iter = args.into_iter().map(|e| {
            if e.matches_head(POW_HEAD) && e.args_len() == 2 {
                let rhs = e.get_arg(1).unwrap();
                let lhs = e.get_arg(0).unwrap();

                (lhs.clone(), rhs.clone())
            } else {
                (e, Number::one().into())
            }
        });

        let mut args_map: HashMap<Expr<A>, Expr<A>> = HashMap::new();
        for (base, exponent) in coeff_expr_pair_iter {
            if let Some(current_exponent) = args_map.get_mut(&base) {
                *current_exponent += exponent;
            } else {
                args_map.insert(base, exponent);
            }
        }

        let new_args = args_map
            .drain()
            .map(|(base, exponent)| pow(base, exponent))
            .collect();

        NormalizedExpr::new(Expr::new_node(MUL_HEAD, new_args))
    }

    pub fn collect_like_terms(self) -> NormalizedExpr<A> {
        let expr = self.take_expr();

        match expr {
            Expr::Atom { .. } => NormalizedExpr::new(expr.annotation_to_default()),
            Expr::Node { head, args, .. } => {
                // We get a normalized tree, so we can initialize NormalizedExpr
                // without normalization.
                let args: Vec<Expr<A>> = args
                    .into_iter()
                    .map(|a| NormalizedExpr::new(a).collect_like_terms().take_expr())
                    .collect();

                if head.matches_symbol(ADD_HEAD) {
                    Self::collect_like_terms_add_head(args)
                } else if head.matches_symbol(MUL_HEAD) {
                    Self::collect_like_exponentials_in_mul(args)
                } else {
                    NormalizedExpr::new(Expr::new_node(*head, args))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use expr_macro::{expr, norm_expr};

    use super::*;
    use crate::{expr::generator::*, symbol};

    fn mul(s: &[Expr<()>]) -> Expr<()> {
        Expr::new_node(Expr::new_symbol(MUL_HEAD), s.to_vec())
    }

    fn add(s: &[Expr<()>]) -> Expr<()> {
        Expr::new_node(Expr::new_symbol(ADD_HEAD), s.to_vec())
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

    #[test]
    fn test_gather_like_terms_in_add() {
        let (x, y) = symbol!("x", "y");

        let e = NormalizedExpr::new(exp(1 + x + 2 * x - 7 - 8 * x + y * x));

        dbg!(e.collect_like_terms().take_expr());
    }

    #[test]
    fn test_divide_by_zero() {
        let expr = expr! { 0 / 0 };

        dbg!(expr.normalize());
    }

    #[test]
    fn test_resugar() {
        let expr = norm_expr! { x - y };

        assert_eq!(expr.resugar(), expr! { Sub[x, y] });
    }
}
