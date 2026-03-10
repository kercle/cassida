use std::collections::HashMap;

use crate::{
    builtin::*,
    expr::{Expr, ExprKind, NormExpr, RawExpr},
};
use numbers::Number;

use crate::{
    atom::Atom,
    builtin::{CANNONICAL_HEAD_APPLY, CANNONICAL_HEAD_SQRT, CANNONICAL_SYM_INDETERMINATE},
};

fn cannonical_fold_ac_with_neutral_el(
    head: &RawExpr,
    value: Number,
    is_neutral_element: bool,
    args_rest: &mut Vec<RawExpr>,
) -> RawExpr {
    if args_rest.is_empty() {
        // Only number left: collapse node
        RawExpr::new_number(value)
    } else if is_neutral_element {
        // Constants fold to neutral
        if args_rest.len() == 1 {
            // Only one arg left: collapse node
            args_rest.pop().unwrap()
        } else {
            // New node contains only remaining args
            RawExpr::new_node(head.clone(), args_rest.clone())
        }
    } else {
        // Multiple remaining args and number is not neutral element
        args_rest.push(RawExpr::new_number(value));
        RawExpr::new_node(head.clone(), args_rest.clone())
    }
}

fn partition_constants(args: &[RawExpr]) -> (Vec<Number>, Vec<RawExpr>) {
    let (c_args, v_args): (Vec<&RawExpr>, Vec<&RawExpr>) = args.iter().partition(|e| e.is_number());

    let c_args = c_args
        .iter()
        .map(|e| e.get_number().unwrap().clone())
        .collect();

    let v_args: Vec<RawExpr> = v_args.iter().map(|&e| e.clone()).collect();

    (c_args, v_args)
}

fn contains_indeterminate(exprs: &[RawExpr]) -> bool {
    exprs
        .iter()
        .any(|e| e.matches_symbol(CANNONICAL_SYM_INDETERMINATE))
}

fn cannonical_fold_op(head: &RawExpr, args: &[RawExpr]) -> Option<RawExpr> {
    if head.matches_symbol(ADD_HEAD) {
        if contains_indeterminate(args) {
            return Some(RawExpr::new_symbol(CANNONICAL_SYM_INDETERMINATE));
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
            return Some(RawExpr::new_symbol(CANNONICAL_SYM_INDETERMINATE));
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
            return Some(RawExpr::new_symbol(CANNONICAL_SYM_INDETERMINATE));
        }

        let lhs = &args[0];
        let rhs = &args[1];

        if lhs.is_number_zero() {
            if rhs.is_number_zero() || rhs.is_number_negative() {
                Some(RawExpr::new_symbol(CANNONICAL_SYM_INDETERMINATE))
            } else {
                Some(Number::zero().into())
            }
        } else if lhs.is_number_one() || rhs.is_number_zero() {
            Some(Number::one().into())
        } else if rhs.is_number_one() {
            Some(lhs.clone())
        } else if let (Some(lhs), Some(rhs)) = (lhs.get_number(), rhs.get_number()) {
            if rhs.is_integer() {
                lhs.pow(rhs).map(RawExpr::new_number).ok()
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

impl RawExpr {
    fn apply_till_fixed_point(self, f: impl Fn(RawExpr) -> RawExpr) -> Self {
        let mut last_hash = None;
        let mut current = self;

        loop {
            current = f(current);

            if let Some(last_hash) = last_hash
                && last_hash == current.fingerprint()
            {
                // if hashes agree, assume fixed point
                break;
            }

            last_hash = Some(current.fingerprint());
        }

        current
    }

    pub fn normalize(self) -> NormExpr {
        let head_predicate = |e: &RawExpr| e.matches_symbol(ADD_HEAD) || e.matches_symbol(MUL_HEAD);

        let normalized_expr = self.apply_till_fixed_point(|e| {
            e.desugar()
                .reduce_structure()
                .flatten(head_predicate)
                .apply_to_nodes(cannonical_fold_op)
                .sort_args(head_predicate)
        });

        unsafe { std::mem::transmute(normalized_expr) }
    }

    pub fn canonicalize(self) -> Self {
        let head_predicate = |e: &RawExpr| e.matches_symbol(ADD_HEAD) || e.matches_symbol(MUL_HEAD);

        self.apply_till_fixed_point(|e| {
            e.reduce_structure()
                .flatten(head_predicate)
                .apply_to_nodes(cannonical_fold_op)
                .sort_args(head_predicate)
        })
    }

    fn reduce_structure_apply(self) -> Self {
        self.map_bottom_up(&|expr| match expr.kind {
            ExprKind::Node { head, args }
                if head.matches_symbol(CANNONICAL_HEAD_APPLY) && args.len() == 2 =>
            {
                let [lhs, rhs]: [RawExpr; 2] = args.try_into().unwrap();

                match rhs.kind {
                    ExprKind::Atom { .. } => rhs,
                    ExprKind::Node { args: args_rhs, .. } => RawExpr::new_node(lhs, args_rhs),
                }
            }
            _ => expr,
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
    pub fn flatten(self, head_predicate: impl Fn(&RawExpr) -> bool + Copy) -> Self {
        match self.kind {
            ExprKind::Atom { .. } => self,
            ExprKind::Node { head, args } if head_predicate(&head) => {
                let mut new_args = Vec::with_capacity(args.len());

                for arg in args.into_iter() {
                    let arg = arg.flatten(head_predicate);

                    match arg.kind {
                        ExprKind::Node { head: ch, args } if *ch == *head => {
                            new_args.extend(args);
                        }
                        _ => {
                            new_args.push(arg);
                        }
                    }
                }

                Self::new_node(*head, new_args)
            }
            ExprKind::Node { head, args } => Self::new_node(
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
    pub fn sort_args(self, head_predicate: impl Fn(&RawExpr) -> bool + Copy) -> Self {
        match self.kind {
            ExprKind::Atom { .. } => self,
            ExprKind::Node { head, args } => {
                let mut args: Vec<Self> = args
                    .into_iter()
                    .map(|a| a.sort_args(head_predicate))
                    .collect();

                if head_predicate(&head) {
                    args.sort();
                }

                Self::new_node(*head, args)
            }
        }
    }

    pub fn desugar(self) -> Self {
        match self.kind {
            ExprKind::Atom { .. } => self,
            ExprKind::Node { head, args }
                if head.matches_symbol(CANNONICAL_HEAD_HOLD) && args.len() == 1 =>
            {
                Expr::new_node(CANNONICAL_HEAD_HOLD, args)
            }
            ExprKind::Node { head, args } if head.matches_symbol(SUB_HEAD) && args.len() == 2 => {
                let [lhs, rhs]: [RawExpr; 2] = args.try_into().unwrap();

                Self::new_node(
                    ADD_HEAD,
                    vec![lhs, Self::new_node(MUL_HEAD, vec![(-1).into(), rhs])],
                )
            }
            ExprKind::Node { head, mut args }
                if head.matches_symbol(NEG_HEAD) && args.len() == 1 =>
            {
                let arg = args.pop().unwrap().desugar();
                Self::new_node(MUL_HEAD, vec![(-1).into(), arg])
            }
            ExprKind::Node { head, args } if head.matches_symbol(DIV_HEAD) && args.len() == 2 => {
                let [lhs, rhs]: [RawExpr; 2] = args.try_into().unwrap();

                Self::new_node(
                    MUL_HEAD,
                    vec![lhs, Self::new_node(POW_HEAD, vec![rhs, (-1).into()])],
                )
            }
            ExprKind::Node { head, mut args }
                if head.matches_symbol(CANNONICAL_HEAD_SQRT) && args.len() == 1 =>
            {
                let arg = args.pop().unwrap().desugar();
                let one_half = Number::new_rational_from_i64(1, 2).unwrap();
                Self::new_node(POW_HEAD, vec![arg, one_half.into()])
            }
            ExprKind::Node { head, args } => {
                let args: Vec<Self> = args.into_iter().map(|a| a.desugar()).collect();
                Self::new_node(*head, args)
            }
        }
    }

    fn apply_to_nodes(self, f: impl Fn(&RawExpr, &[RawExpr]) -> Option<RawExpr> + Copy) -> Self {
        match self.kind {
            ExprKind::Atom { .. } => self,
            ExprKind::Node { head, args }
                if head.matches_symbol(CANNONICAL_HEAD_HOLD) && args.len() == 1 =>
            {
                Expr::new_node(CANNONICAL_HEAD_HOLD, args)
            }
            ExprKind::Node { head, args } => {
                let args: Vec<Self> = args.into_iter().map(|a| a.apply_to_nodes(f)).collect();

                if let Some(e) = f(&head, &args) {
                    e
                } else {
                    Self::new_node(*head, args)
                }
            }
        }
    }
}

impl NormExpr {
    pub(super) fn split_coefficient(expr: RawExpr) -> (Number, RawExpr) {
        // We are normalized, so if there is a constant, it's the first arg
        // in a commutative operation.

        match expr.kind {
            ExprKind::Atom {
                entry: Atom::Number(val),
                ..
            } => (val, Number::one().into()),
            ExprKind::Node { head, mut args } if head.matches_symbol(MUL_HEAD) => {
                if let Some(coeff) = args.first().and_then(|e| e.get_number()) {
                    let coeff = coeff.clone();
                    let _ = args.swap_remove(0);
                    (coeff.clone(), RawExpr::collapse_mul(args))
                } else {
                    (Number::one(), RawExpr::collapse_mul(args))
                }
            }
            _ => (Number::one(), expr),
        }
    }

    fn collect_like_terms_add_head(args: Vec<RawExpr>) -> RawExpr {
        // Collect like terms preserves normalization
        let coeff_expr_pair_iter = args.into_iter().map(Self::split_coefficient);

        let mut args_map: HashMap<RawExpr, Number> = HashMap::new();
        for (n, e) in coeff_expr_pair_iter {
            if let Some(coeff_current) = args_map.get_mut(&e) {
                *coeff_current += n;
            } else {
                args_map.insert(e, n);
            }
        }

        let new_args = args_map
            .drain()
            .map(|(e, n)| RawExpr::new_binary_node(MUL_HEAD, e, RawExpr::new_number(n)))
            .collect();

        RawExpr::new_node(ADD_HEAD, new_args)
    }

    fn collect_like_exponentials_in_mul(args: Vec<RawExpr>) -> RawExpr {
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

        let mut args_map: HashMap<RawExpr, RawExpr> = HashMap::new();
        for (base, exponent) in coeff_expr_pair_iter {
            let base = Self::collect_like_terms_inner(base);
            let exponent = Self::collect_like_terms_inner(exponent);

            if let Some(current_exponent) = args_map.get_mut(&base) {
                *current_exponent =
                    RawExpr::new_binary_node(ADD_HEAD, current_exponent.clone(), exponent);
            } else {
                args_map.insert(base, exponent);
            }
        }

        let new_args = args_map
            .drain()
            .map(|(base, exponent)| RawExpr::new_binary_node(POW_HEAD, base, exponent))
            .collect();

        RawExpr::new_node(MUL_HEAD, new_args)
    }

    fn collect_like_terms_inner(expr: RawExpr) -> RawExpr {
        // internall we work with RawExpr, because NormExpr does not
        // offer constructors for manipulating the expression.

        match expr.kind {
            ExprKind::Atom { .. } => expr,
            ExprKind::Node { head, args } => {
                if head.matches_symbol(ADD_HEAD) {
                    Self::collect_like_terms_add_head(args)
                } else if head.matches_symbol(MUL_HEAD) {
                    Self::collect_like_exponentials_in_mul(args)
                } else {
                    let args = args
                        .into_iter()
                        .map(Self::collect_like_terms_inner)
                        .collect();
                    RawExpr::new_node(*head, args)
                }
            }
        }
    }

    pub fn collect_like_terms(self) -> NormExpr {
        // internall we work with RawExpr, because NormExpr does not
        // offer constructors for manipulating the expression.
        let expr = self.into_raw();
        Self::collect_like_terms_inner(expr).normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{norm_expr, raw_expr};

    #[test]
    fn test_expr_flatten() {
        let expr = raw_expr!(2 + x + 3 * (5 + (1 + (1 + y))));
        let expected = raw_expr!( Add[2, x, Mul[3, Add[5, 1, 1, y]]] );

        assert_eq!(expr.flatten(|e| e.matches_symbol(ADD_HEAD)), expected);
    }

    #[test]
    fn test_expr_sorting() {
        let expr = raw_expr!( Add[x, 2, Mul[3, Add[5, y, 1, 1]]] );
        let expected = raw_expr!( Add[2, x, Mul[3, Add[1, 1, 5, y]]] );

        assert_eq!(expr.sort_args(|e| e.matches_symbol(ADD_HEAD)), expected);
    }

    #[test]
    fn test_expr_normalizing() {
        let expr = raw_expr!( Add[x, 2, Mul[3, Add[5, y, 1, 1]]] );
        let expected = raw_expr!( Add[2, x, Mul[3, Add[7, y]]] );

        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_divide_by_zero() {
        let expr = raw_expr! { 0 / 0 };

        dbg!(expr.normalize());
    }

    #[test]
    fn test_resugar() {
        let expr = norm_expr! { x - y };

        assert_eq!(expr.resugar(), raw_expr! { Sub[x, y] });
    }

    #[test]
    fn test_hold() {
        let expr = norm_expr!(Hold[1 + 1]);
        let expected = raw_expr!(Hold[1 + 1]);
        dbg!(&expr);
        assert_eq!(expr.into_raw(), expected);
    }
}
