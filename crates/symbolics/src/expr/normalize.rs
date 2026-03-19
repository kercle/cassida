use std::collections::{BTreeMap, HashMap};

use numbers::Number;

use crate::{
    atom::Atom,
    builtin::{
        ADD_HEAD, CANNONICAL_HEAD_HOLD, CANNONICAL_HEAD_SQRT, CANNONICAL_SYM_INDETERMINATE,
        DIV_HEAD, MUL_HEAD, NEG_HEAD, POW_HEAD, SUB_HEAD,
    },
    expr::{
        ExprKind, ExprPool, ExprView, NormExpr, NormExprHandle, RawArgsHandle, RawExpr,
        RawExprHandle, ops::cmp_expr_handle,
    },
};

impl RawExpr {
    pub fn normalize(self) -> NormExpr {
        let mut pool = ExprPool::new();
        let handle = pool.insert_expr(self);
        handle.normalize(&mut pool).materialize(&pool)
    }

    fn into_normexpr_unsafe(self) -> NormExpr {
        unsafe { std::mem::transmute(self) }
    }
}

impl RawExprHandle {
    pub fn normalize(self, pool: &mut ExprPool) -> NormExprHandle {
        match self.view(pool) {
            ExprView::Atom(_) => self.into_normexpr_unchecked(),
            ExprView::Node { head, args } => normalize_raw_node_handle(pool, head, args),
        }
    }

    fn into_normexpr_unchecked(self) -> NormExprHandle {
        NormExprHandle::new_unchecked(self.id())
    }
}

impl NormExpr {
    pub fn normalize(self) -> NormExpr {
        self
    }

    fn new_simple_node_unchecked<T: AsRef<str>>(head: T, args: Vec<Self>) -> Self {
        let head_kind = ExprKind::Atom {
            entry: Atom::Symbol(head.as_ref().to_string()),
        };

        let head = Self::new_unchecked(head_kind);

        Self::new_unchecked(ExprKind::Node {
            head: Box::new(head),
            args,
        })
    }
}

impl NormExprHandle {
    pub fn normalize(self) -> NormExprHandle {
        self
    }
}

fn normalize_raw_node(head_expr: RawExpr, args: Vec<RawExpr>) -> NormExpr {
    match head_expr.get_symbol() {
        Some(ADD_HEAD) => normalize_raw_add(args),
        Some(SUB_HEAD) if args.len() == 2 => {
            let [lhs, rhs]: [RawExpr; 2] = args.try_into().unwrap();
            RawExpr::new_binary_node(
                ADD_HEAD,
                lhs,
                RawExpr::new_binary_node(MUL_HEAD, Number::minus_one().into(), rhs),
            )
            .normalize()
        }
        Some(NEG_HEAD) if args.len() == 1 => {
            let [arg]: [RawExpr; 1] = args.try_into().unwrap();
            RawExpr::new_binary_node(MUL_HEAD, Number::minus_one().into(), arg).normalize()
        }
        Some(MUL_HEAD) => normalize_raw_mul(args),
        Some(DIV_HEAD) if args.len() == 2 => {
            let [lhs, rhs]: [RawExpr; 2] = args.try_into().unwrap();

            if rhs.is_number_zero() {
                return RawExpr::new_symbol(CANNONICAL_SYM_INDETERMINATE).normalize();
            }

            RawExpr::new_binary_node(
                MUL_HEAD,
                lhs,
                RawExpr::new_binary_node(POW_HEAD, rhs, Number::minus_one().into()),
            )
            .normalize()
        }
        Some(POW_HEAD) if args.len() == 2 => {
            let [base, exponent]: [RawExpr; 2] = args.try_into().unwrap();
            normalize_raw_pow(base, exponent)
        }
        Some(CANNONICAL_HEAD_SQRT) if args.len() == 1 => {
            let [arg]: [RawExpr; 1] = args.try_into().unwrap();
            let one_half = Number::new_rational_from_i64(1, 2).unwrap();
            RawExpr::new_binary_node(POW_HEAD, arg, one_half.into()).normalize()
        }
        Some(CANNONICAL_HEAD_HOLD) if args.len() == 1 => NormExpr::new_unchecked(ExprKind::Node {
            head: Box::new(head_expr.into_normexpr_unsafe()),
            args: args.into_iter().map(|a| a.into_normexpr_unsafe()).collect(),
        }),
        _ => NormExpr::new_unchecked(ExprKind::Node {
            head: Box::new(head_expr.normalize()),
            args: args.into_iter().map(|a| a.normalize()).collect(),
        }),
    }
}

fn normalize_raw_node_handle(
    pool: &mut ExprPool,
    head: RawExprHandle,
    args: RawArgsHandle,
) -> NormExprHandle {
    match head.view(pool).get_symbol() {
        Some(ADD_HEAD) => normalize_raw_add_handle(pool, args),
        Some(SUB_HEAD) if args.len(pool) == 2 => {
            // Takes Sub[a, b] and produces Add[a, Mul[-1, b]]

            let lhs = args.get(pool, 0).unwrap();
            let rhs = args.get(pool, 1).unwrap();

            let coeff = pool.integer_from_i64(-1);
            let mul_node = pool.binary_node_with_head_symbol(MUL_HEAD, coeff, rhs);
            pool.binary_node_with_head_symbol(ADD_HEAD, lhs, mul_node)
                .normalize(pool)
        }
        Some(MUL_HEAD) => normalize_raw_mul_handle(pool, args),
        Some(DIV_HEAD) if args.len(pool) == 2 => {
            // Takes Div[a, b] and produces Mul[a, Pow[b, -1]] if
            // b != 0, otherwise Indeterminate

            let lhs = args.get(pool, 0).unwrap();
            let rhs = args.get(pool, 1).unwrap();

            if let Some(num) = rhs.view(pool).get_number()
                && num.is_zero()
            {
                pool.symbol(CANNONICAL_SYM_INDETERMINATE)
                    .into_normexpr_unchecked()
            } else {
                let coeff = pool.integer_from_i64(-1);
                let pow_node = pool.binary_node_with_head_symbol(POW_HEAD, rhs, coeff);
                pool.binary_node_with_head_symbol(MUL_HEAD, lhs, pow_node)
                    .normalize(pool)
            }
        }
        Some(NEG_HEAD) if args.len(pool) == 1 => {
            // Takes Neg[a] and produces Mul[-1, a]

            let child = args.get(pool, 0).unwrap();

            let coeff = pool.integer_from_i64(-1);
            pool.binary_node_with_head_symbol(MUL_HEAD, coeff, child)
                .normalize(pool)
        }
        Some(POW_HEAD) if args.len(pool) == 2 => {
            let base = args.get(pool, 0).unwrap();
            let exponent = args.get(pool, 1).unwrap();

            normalize_raw_pow_handle(pool, base, exponent)
        }
        Some(CANNONICAL_HEAD_SQRT) if args.len(pool) == 1 => {
            // Takes Sqrt[a] and produces Pow[a, 1/2]

            let child = args.get(pool, 0).unwrap();

            let exponent = pool.rational_from_i64(1, 2).unwrap();
            pool.binary_node_with_head_symbol(POW_HEAD, child, exponent)
                .normalize(pool)
        }
        Some(CANNONICAL_HEAD_HOLD) if args.len(pool) == 1 => {
            // Takes Hold[a] and produces Hold[a] without
            // normalize further.

            let child = args.get(pool, 0).unwrap();

            pool.unary_node(head, child).into_normexpr_unchecked()
        }
        _ => {
            let head = head.normalize(pool).as_raw();
            let mut args: Vec<RawExprHandle> = args.iter(pool).collect();

            for a in args.iter_mut() {
                *a = a.normalize(pool).as_raw()
            }

            // NormExprHandle::new_unchecked(pool.node(head.id(), args))
            pool.variadic_node(head, args).into_normexpr_unchecked()
        }
    }
}

fn flatten(head_symbol: &str, args: Vec<RawExpr>) -> Vec<NormExpr> {
    let mut flattened_args = Vec::new();
    for arg in args {
        let norm_arg = arg.normalize();

        if norm_arg.has_head_symbol(head_symbol) {
            let ExprKind::Node { args, .. } = norm_arg.into_kind() else {
                unreachable!("We know at this point Expr has head symbol");
            };

            flattened_args.extend(args.into_iter());
        } else {
            flattened_args.push(norm_arg);
        }
    }

    flattened_args
}

fn flatten_node_handle(
    pool: &mut ExprPool,
    head_symbol: &str,
    args: RawArgsHandle,
    flattened: &mut Vec<NormExprHandle>,
) {
    // TODO: can we get rid of this clone?
    let args = args.to_vec(pool);

    for arg in args {
        let norm_arg = arg.normalize(pool);

        if norm_arg.view(pool).is_node(pool, head_symbol, None) {
            let ExprView::Node { args, .. } = norm_arg.view(pool) else {
                unreachable!("We know at this point Expr has head symbol");
            };

            flattened.extend(args.iter(pool));
        } else {
            flattened.push(norm_arg);
        }
    }
}

fn normalize_raw_add_handle(pool: &mut ExprPool, args: RawArgsHandle) -> NormExprHandle {
    // We first flatten the node:
    // Add[...,Add[a,...,z],...] -> Add[...,a,...,z,...]

    let mut constant_term = Number::zero();
    let mut terms = HashMap::new();

    let mut flattened_args = Vec::with_capacity(args.len(pool));
    flatten_node_handle(pool, ADD_HEAD, args, &mut flattened_args);

    for arg in flattened_args {
        if arg.view(pool).is_symbol(CANNONICAL_SYM_INDETERMINATE) {
            // Early exit when we encounter indeterminate
            return arg;
        }

        let (coeff, term) = split_coefficient_handle(pool, arg);

        let Some(term) = term else {
            // argument is just a numeric constant
            constant_term += coeff;
            continue;
        };

        if let Some(cummulated_coeff) = terms.get_mut(&term) {
            *cummulated_coeff = &*cummulated_coeff + coeff;
        } else {
            terms.insert(term, coeff);
        }
    }

    let mut new_args = vec![];

    if !constant_term.is_zero() {
        new_args.push(pool.number(constant_term));
    }

    for (term, coeff) in terms.into_iter() {
        if coeff.is_zero() {
            continue;
        }

        let coeff = pool.number(coeff);
        let node = if term.view(pool).is_symbol(MUL_HEAD) {
            let ExprView::Node { head, args } = term.view(pool) else {
                unreachable!("Coefficients should already by isolated");
            };

            // in sort order, numbers are guaranteed to come first
            let mut new_args = vec![coeff];
            new_args.extend(args.iter(pool).map(|a| a.as_raw()));

            debug_assert!(
                new_args
                    .windows(2)
                    .all(|w| cmp_expr_handle(pool, &w[0], &w[1]).is_le()),
                "MUL args are not sorted after prepending coefficient"
            );

            pool.variadic_node(head.as_raw(), new_args)
                .into_normexpr_unchecked()
        } else if let Some(n) = coeff.view(pool).get_number()
            && n.is_one()
        {
            term
        } else {
            // NormExpr::new_simple_node_unchecked(MUL_HEAD, vec![coeff, term])
            pool.binary_node_with_head_symbol(MUL_HEAD, coeff, term.as_raw())
                .into_normexpr_unchecked()
        };

        new_args.push(node.as_raw());
    }

    new_args.sort_by(|a, b| cmp_expr_handle(pool, a, b));

    if new_args.is_empty() {
        pool.integer_from_i64(0).into_normexpr_unchecked()
    } else if new_args.len() == 1 {
        // Note that we already normalized new_args. We just store them
        // as raw, because we can only assemble raw expressions through
        // the pool object.
        new_args.pop().unwrap().into_normexpr_unchecked()
    } else {
        pool.variadic_node_with_head_symbol(ADD_HEAD, new_args)
            .into_normexpr_unchecked()
    }
}

fn normalize_raw_add(args: Vec<RawExpr>) -> NormExpr {
    let mut constant_term = Number::zero();
    let mut terms = BTreeMap::new();

    for arg in flatten(ADD_HEAD, args) {
        if arg.is_indeterminate() {
            // Early exit when we encounter indeterminate
            return arg;
        }

        let (coeff, term) = split_coefficient(arg);

        let Some(term) = term else {
            // argument is just a numeric constant
            constant_term += coeff;
            continue;
        };

        if let Some(cummulated_coeff) = terms.get_mut(&term) {
            *cummulated_coeff = &*cummulated_coeff + coeff;
        } else {
            terms.insert(term, coeff);
        }
    }

    let mut new_args = vec![];

    if !constant_term.is_zero() {
        new_args.push(RawExpr::from(constant_term).into_normexpr_unsafe());
    }

    for (term, coeff) in terms.into_iter() {
        if coeff.is_zero() {
            continue;
        }

        let coeff = RawExpr::new_number(coeff).normalize();
        let node = if term.has_head_symbol(MUL_HEAD) {
            let ExprKind::Node { head, args } = term.into_kind() else {
                unreachable!("Coefficients should already by isolated");
            };

            // in sort order, numbers are guaranteed to come first
            let mut new_args = vec![coeff];
            new_args.extend(args.into_iter());

            debug_assert!(
                new_args.windows(2).all(|w| w[0] <= w[1]),
                "MUL args are not sorted after prepending coefficient"
            );

            NormExpr::new_unchecked(ExprKind::Node {
                head,
                args: new_args,
            })
        } else if coeff.is_number_one() {
            term
        } else {
            NormExpr::new_simple_node_unchecked(MUL_HEAD, vec![coeff, term])
        };

        new_args.push(node);
    }

    if new_args.is_empty() {
        RawExpr::new_number_integer(0).into_normexpr_unsafe()
    } else if new_args.len() == 1 {
        new_args.pop().unwrap()
    } else {
        NormExpr::new_simple_node_unchecked(ADD_HEAD, new_args)
    }
}

pub(super) fn split_coefficient_handle(
    pool: &mut ExprPool,
    expr: NormExprHandle,
) -> (Number, Option<NormExprHandle>) {
    match expr.view(pool) {
        ExprView::Atom(Atom::Number(val)) => (val.clone(), None),
        ExprView::Node { head, args } if head.view(pool).is_symbol(MUL_HEAD) => {
            let Some(first) = args.get(pool, 0) else {
                let node = pool.node(head.as_raw(), args.as_raw());
                return (Number::one(), Some(node.into_normexpr_unchecked()));
            };

            if let Some(coeff) = first.view(pool).get_number().cloned() {
                if args.len(pool) == 1 {
                    (coeff, None)
                } else if args.len(pool) == 2 {
                    (coeff, args.get(pool, 1))
                } else {
                    let rest = args.iter(pool).skip(1).map(|e| e.as_raw()).collect();
                    let node = pool.variadic_node_with_head_symbol(MUL_HEAD, rest);
                    (coeff, Some(node.into_normexpr_unchecked()))
                }
            } else {
                (Number::one(), Some(expr))
            }
        }
        _ => (Number::one(), Some(expr)),
    }
}

pub(super) fn split_coefficient(expr: NormExpr) -> (Number, Option<NormExpr>) {
    match expr.kind {
        ExprKind::Atom {
            entry: Atom::Number(val),
            ..
        } => (val, None),
        ExprKind::Node { head, mut args } if head.matches_symbol(MUL_HEAD) => {
            if let Some(coeff) = args.first().and_then(|e| e.get_number()) {
                let coeff = coeff.clone();
                let _ = args.remove(0);

                if args.is_empty() {
                    (coeff, None)
                } else if args.len() == 1 {
                    (coeff, Some(args.pop().unwrap()))
                } else {
                    (
                        coeff,
                        Some(NormExpr::new_simple_node_unchecked(MUL_HEAD, args)),
                    )
                }
            } else {
                (
                    Number::one(),
                    Some(NormExpr::new_simple_node_unchecked(MUL_HEAD, args)),
                )
            }
        }
        _ => (Number::one(), Some(expr)),
    }
}

fn normalize_raw_mul(args: Vec<RawExpr>) -> NormExpr {
    let mut constant_term = Number::one();
    let mut terms: BTreeMap<NormExpr, Vec<RawExpr>> = BTreeMap::new();

    for arg in flatten(MUL_HEAD, args) {
        if arg.is_number_zero() || arg.is_indeterminate() {
            // In these cases we can exit early.
            return arg;
        }

        if arg.is_number_one() {
            continue;
        }

        if let Some(num) = arg.get_number() {
            constant_term = constant_term * num;
            continue;
        }

        let [base, exponent]: [NormExpr; 2] = if arg.is_application_of(POW_HEAD, 2) {
            let ExprKind::Node { args, .. } = arg.kind else {
                // we've already made sure that we have a pow node.
                unreachable!()
            };
            args.try_into().unwrap()
        } else {
            [arg, RawExpr::new_number_integer(1).normalize()]
        };

        if base.is_number_zero() {
            if exponent.is_number_negative() || exponent.is_number_zero() {
                return RawExpr::new_symbol(CANNONICAL_SYM_INDETERMINATE).normalize();
            } else {
                // return zero
                return base;
            }
        } else if base.is_number_one() {
            continue;
        }

        if let Some(exponents) = terms.get_mut(&base) {
            exponents.push(exponent.into_raw());
        } else {
            terms.insert(base, vec![exponent.into_raw()]);
        }
    }

    let mut new_args = Vec::new();

    if !constant_term.is_one() {
        new_args.push(RawExpr::from(constant_term).into_normexpr_unsafe());
    }

    for (base, exponents) in terms.into_iter() {
        let assembled_exp = RawExpr::new_node(ADD_HEAD, exponents).normalize();

        if assembled_exp.is_indeterminate() {
            return assembled_exp;
        }

        // Note: base cannot be zero as we filter this case
        // before adding expressions to the hashmap.

        if assembled_exp.is_number_one() {
            new_args.push(base);
        } else if !assembled_exp.is_number_zero() {
            new_args.push(NormExpr::new_simple_node_unchecked(
                POW_HEAD,
                vec![base, assembled_exp],
            ));
        }
    }

    new_args.sort();

    if new_args.is_empty() {
        RawExpr::new_number_integer(1).into_normexpr_unsafe()
    } else if new_args.len() == 1 {
        new_args.pop().unwrap()
    } else {
        NormExpr::new_simple_node_unchecked(MUL_HEAD, new_args)
    }
}

fn normalize_raw_mul_handle(pool: &mut ExprPool, args: RawArgsHandle) -> NormExprHandle {
    let mut constant_term = Number::one();
    let mut terms: HashMap<NormExprHandle, Vec<RawExprHandle>> = HashMap::new();

    let mut flattened_args = Vec::with_capacity(args.len(pool));
    flatten_node_handle(pool, MUL_HEAD, args, &mut flattened_args);

    for arg in flattened_args {
        let arg_view = arg.view(pool);
        let arg_num = arg_view.get_number();

        if arg_view.is_symbol(CANNONICAL_SYM_INDETERMINATE) {
            // In these cases we can exit early.
            return arg;
        }

        if let Some(num) = arg_num {
            // Collect numeric factors.

            if num.is_zero() {
                return arg;
            } else if !num.is_one() {
                constant_term = constant_term * num;
            }

            continue;
        }

        let (base, exponent) = if arg_view.is_node(pool, POW_HEAD, Some(2)) {
            let a = arg_view.get_args().unwrap();
            (a.get(pool, 0).unwrap(), a.get(pool, 1).unwrap().as_raw())
        } else {
            (arg, pool.integer_from_i64(1))
        };

        if base.view(pool).is_number(0) {
            let ExprView::Atom(Atom::Number(exp_num)) = exponent.view(pool) else {
                return base;
            };

            if exp_num.is_zero() || exp_num.is_negative() {
                return pool
                    .symbol(CANNONICAL_SYM_INDETERMINATE)
                    .into_normexpr_unchecked();
            }
        } else if base.view(pool).is_number(1) {
            continue;
        }

        if let Some(exponents) = terms.get_mut(&base) {
            exponents.push(exponent);
        } else {
            terms.insert(base, vec![exponent]);
        }
    }

    let mut new_args = Vec::new();

    if !constant_term.is_one() {
        new_args.push(pool.number(constant_term));
    }

    for (base, exponents) in terms.into_iter() {
        let assembled_exp = pool
            .variadic_node_with_head_symbol(ADD_HEAD, exponents)
            .normalize(pool);

        if assembled_exp
            .view(pool)
            .is_symbol(CANNONICAL_SYM_INDETERMINATE)
        {
            return assembled_exp;
        }

        // Note: base cannot be zero as we filter this case
        // before adding expressions to the hashmap.

        if assembled_exp.view(pool).is_number(1) {
            new_args.push(base.as_raw());
        } else if !assembled_exp.view(pool).is_number(0) {
            new_args.push(pool.binary_node_with_head_symbol(
                POW_HEAD,
                base.as_raw(),
                assembled_exp.as_raw(),
            ));
        }
    }

    new_args.sort_by(|a, b| cmp_expr_handle(pool, a, b));

    if new_args.is_empty() {
        pool.integer_from_i64(1).into_normexpr_unchecked()
    } else if new_args.len() == 1 {
        new_args.pop().unwrap().into_normexpr_unchecked()
    } else {
        pool.variadic_node_with_head_symbol(MUL_HEAD, new_args)
            .into_normexpr_unchecked()
    }
}

fn normalize_raw_pow(base: RawExpr, exponent: RawExpr) -> NormExpr {
    let norm_base = base.normalize();
    let norm_exponent = exponent.normalize();

    if norm_base.is_number_zero() {
        if norm_exponent.is_number_zero() || norm_exponent.is_number_negative() {
            return RawExpr::new_symbol(CANNONICAL_SYM_INDETERMINATE).normalize();
        } else {
            // return zero
            return norm_base;
        }
    } else if norm_base.is_number_one() {
        return norm_base;
    }

    if norm_exponent.is_number_one() {
        norm_base
    } else if norm_exponent.is_number_zero() {
        RawExpr::new_number(1).normalize()
    } else if let Some(exp_num) = norm_exponent.get_number()
        && exp_num.is_integer()
    {
        if let Some(base_num) = norm_base.get_number() {
            if let Ok(num) = base_num.pow(exp_num) {
                RawExpr::new_number(num).normalize()
            } else {
                NormExpr::new_simple_node_unchecked(POW_HEAD, vec![norm_base, norm_exponent])
            }
        } else if norm_base.is_application_of(POW_HEAD, 2) {
            let ExprKind::Node { args, .. } = norm_base.kind else {
                // we've already made sure that we have a pow node.
                unreachable!()
            };

            let [lhs, rhs]: [NormExpr; 2] = args.try_into().unwrap();

            NormExpr::new_simple_node_unchecked(
                POW_HEAD,
                vec![
                    lhs,
                    RawExpr::new_binary_node(
                        MUL_HEAD,
                        rhs.into_raw(),
                        RawExpr::new_number(exp_num.clone()),
                    )
                    .normalize(),
                ],
            )
        } else {
            NormExpr::new_simple_node_unchecked(POW_HEAD, vec![norm_base, norm_exponent])
        }
    } else {
        NormExpr::new_simple_node_unchecked(POW_HEAD, vec![norm_base, norm_exponent])
    }
}

fn normalize_raw_pow_handle(
    pool: &mut ExprPool,
    base: RawExprHandle,
    exponent: RawExprHandle,
) -> NormExprHandle {
    let norm_base = base.normalize(pool);
    let norm_exponent = exponent.normalize(pool);

    if norm_base.view(pool).is_number(0) {
        let norm_exp_view = norm_exponent.view(pool);
        let Some(exponent_num) = norm_exp_view.get_number() else {
            // return zero
            return norm_base;
        };

        if exponent_num.is_zero() || exponent_num.is_negative() {
            return pool
                .symbol(CANNONICAL_SYM_INDETERMINATE)
                .into_normexpr_unchecked();
        } else {
            // return zero
            return norm_base;
        }
    } else if norm_base.view(pool).is_number(1) {
        return norm_base;
    }

    if norm_exponent.view(pool).is_number(1) {
        norm_base
    } else if norm_exponent.view(pool).is_number(0) {
        // we've taken care of the base=0 case already
        pool.integer_from_i64(1).into_normexpr_unchecked()
    } else if let Some(exp_num) = norm_exponent.view(pool).get_number()
        && exp_num.is_integer()
    {
        if let Some(base_num) = norm_base.view(pool).get_number() {
            if let Ok(pow_result) = base_num.pow(exp_num) {
                pool.number(pow_result).into_normexpr_unchecked()
            } else {
                pool.binary_node_with_head_symbol(
                    POW_HEAD,
                    norm_base.as_raw(),
                    norm_exponent.as_raw(),
                )
                .into_normexpr_unchecked()
            }
        } else if norm_base.view(pool).is_node(pool, POW_HEAD, Some(2)) {
            let base_args = norm_base.view(pool).get_args().unwrap();

            let lhs = base_args.get(pool, 0).unwrap();

            let rhs = base_args.get(pool, 1).unwrap();

            let exp_num = pool.number(exp_num.clone());
            let new_exponent = pool
                .binary_node_with_head_symbol(MUL_HEAD, rhs.as_raw(), exp_num)
                .normalize(pool)
                .as_raw();

            pool.binary_node_with_head_symbol(POW_HEAD, lhs.as_raw(), new_exponent)
                .into_normexpr_unchecked()
        } else {
            pool.binary_node_with_head_symbol(POW_HEAD, norm_base.as_raw(), norm_exponent.as_raw())
                .into_normexpr_unchecked()
        }
    } else {
        pool.binary_node_with_head_symbol(POW_HEAD, norm_base.as_raw(), norm_exponent.as_raw())
            .into_normexpr_unchecked()
    }
}

#[cfg(test)]
mod normalize_comprehensive_tests {
    use crate::{
        builtin::{ADD_HEAD, MUL_HEAD, POW_HEAD},
        expr::{Expr, RawExpr},
        raw_expr,
    };

    #[test]
    fn test_expr_normalizing_original() {
        let expr = raw_expr!(Add[x, 2, Mul[3, Add[5, y, 1, 1]]]);
        let expected = raw_expr!(Add[2, x, Mul[3, Add[7, y]]]);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_hold_does_not_evaluate() {
        let expr = raw_expr!(Hold[1 + 1]).normalize();
        let expected = raw_expr!(Hold[1 + 1]);
        assert_eq!(expr.into_raw(), expected);
    }

    #[test]
    fn test_like_terms_explicit_plus_implicit_coefficient() {
        let expr = raw_expr!(Add[Mul[2, x], x]);
        let expected = raw_expr!(Mul[3, x]);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_like_terms_implicit_plus_explicit_coefficient() {
        let expr = raw_expr!(Add[x, Mul[2, x]]);
        let expected = raw_expr!(Mul[3, x]);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_like_terms_both_implicit_coefficients() {
        let expr = raw_expr!(Add[x, x]);
        let expected = raw_expr!(Mul[2, x]);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_like_terms_both_explicit_coefficients() {
        let expr = raw_expr!(Add[Mul[3, x], Mul[2, x]]);
        let expected = raw_expr!(Mul[5, x]);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_like_terms_cancel_to_zero() {
        let expr = raw_expr!(Add[Mul[-1, x], x]);
        let expected = raw_expr!(0);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_sub_cancels_to_zero() {
        let expr = raw_expr!(x - x);
        let expected = raw_expr!(0);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_constants_cancel_to_zero() {
        let expr = raw_expr!(Add[5, -5]);
        let expected = raw_expr!(0);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_mixed_full_cancellation() {
        let expr = raw_expr!(Add[x, 3, Mul[-1, x], -3]);
        let expected = raw_expr!(0);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_zero_pow_zero_is_indeterminate() {
        let expr = raw_expr!(Pow[0, 0]);
        let expected = raw_expr!(Indeterminate);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_zero_pow_negative_is_indeterminate() {
        let expr = raw_expr!(Pow[0, -1]);
        let expected = raw_expr!(Indeterminate);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_zero_pow_positive_is_zero() {
        let expr = raw_expr!(Pow[0, 2]);
        let expected = raw_expr!(0);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_zero_pow_positive_rational_is_zero() {
        let expr = raw_expr!(Sqrt[0]);
        let expected = raw_expr!(0);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_zero_pow_negative_rational_is_indeterminate() {
        let expr = raw_expr!(Pow[0, Div[-1, 2]]);
        let expected = raw_expr!(Indeterminate);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_zero_divided_by_zero_is_indeterminate() {
        let expr = raw_expr!(0 / 0);
        let expected = raw_expr!(Indeterminate);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_nonzero_divided_by_zero_is_indeterminate() {
        let expr = raw_expr!(5 / 0);
        let expected = raw_expr!(Indeterminate);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_symbol_divided_by_zero_is_indeterminate() {
        let expr = raw_expr!(x / 0);
        let expected = raw_expr!(Indeterminate);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_add_identity_left() {
        let expr = raw_expr!(Add[0, x]);
        let expected = raw_expr!(x);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_add_identity_right() {
        let expr = raw_expr!(Add[x, 0]);
        let expected = raw_expr!(x);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_mul_identity_left() {
        let expr = raw_expr!(Mul[1, x]);
        let expected = raw_expr!(x);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_mul_identity_right() {
        let expr = raw_expr!(Mul[x, 1]);
        let expected = raw_expr!(x);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_mul_by_zero_left() {
        let expr = raw_expr!(Mul[0, x]);
        let expected = raw_expr!(0);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_mul_by_zero_right() {
        let expr = raw_expr!(Mul[x, 0]);
        let expected = raw_expr!(0);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_pow_base_one() {
        let expr = raw_expr!(Pow[1, x]);
        let expected = raw_expr!(1);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_pow_exponent_one() {
        let expr = raw_expr!(Pow[x, 1]);
        let expected = raw_expr!(x);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_pow_exponent_zero() {
        let expr = raw_expr!(Pow[x, 0]);
        let expected = raw_expr!(1);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_neg_number() {
        let expr = raw_expr!(Neg[5]);
        let expected = Expr::new_number(-5);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_neg_symbol() {
        let expr = raw_expr!(Neg[x]);
        let expected =
            RawExpr::new_binary_node(MUL_HEAD, RawExpr::new_number(-1), RawExpr::new_symbol("x"));
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_double_neg() {
        let expr = raw_expr!(Neg[Neg[x]]);
        let expected = raw_expr!(x);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_sub_basic() {
        let expr = raw_expr!(x - 3);
        let expected =
            RawExpr::new_binary_node(ADD_HEAD, RawExpr::new_number(-3), RawExpr::new_symbol("x"));
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_sqrt_desugars_to_pow_half() {
        let expr = raw_expr!(Sqrt[x]);
        let expected = RawExpr::new_binary_node(
            POW_HEAD,
            RawExpr::new_symbol("x"),
            RawExpr::new_number_rational(1, 2).unwrap(),
        );
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_add_flattens_nested() {
        let expr = raw_expr!(Add[Add[x, y], z]);
        let expected = raw_expr!(Add[x, y, z]);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_mul_flattens_nested() {
        let expr = raw_expr!(Mul[Mul[x, y], z]);
        let expected = raw_expr!(Mul[x, y, z]);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_pow_same_base_exponent_combination() {
        let expr = raw_expr!(Mul[Pow[x, 2], Pow[x, 3]]);
        let expected = raw_expr!(Pow[x, 5]);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_pow_base_times_self() {
        let expr = raw_expr!(Mul[x, x]);
        let expected = raw_expr!(Pow[x, 2]);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_pow_exponents_cancel() {
        let expr = raw_expr!(Mul[Pow[x, 2], Pow[x, -2]]);
        let expected = raw_expr!(1);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_indeterminate_in_add_short_circuits() {
        let expr = raw_expr!(Add[x, Indeterminate, y]);
        let expected = raw_expr!(Indeterminate);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_indeterminate_in_mul_short_circuits() {
        let expr = raw_expr!(Mul[x, Indeterminate, y]);
        let expected = raw_expr!(Indeterminate);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_add_constants_fold() {
        let expr = raw_expr!(Add[2, 3]);
        let expected = raw_expr!(5);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_mul_constants_fold() {
        let expr = raw_expr!(Mul[4, 5]);
        let expected = raw_expr!(20);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_nested_constant_folding() {
        let expr = raw_expr!(Add[Mul[2, 3], 4]);
        let expected = raw_expr!(10);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_normalize_idempotent_add() {
        let expr = raw_expr!(Add[x, 2, Mul[3, y]]);
        let once = expr.clone().normalize();
        let twice = once.clone().normalize();
        assert_eq!(once.into_raw(), twice.into_raw());
    }

    #[test]
    fn test_normalize_idempotent_mul() {
        let expr = raw_expr!(Mul[x, Pow[y, 2], 3]);
        let once = expr.clone().normalize();
        let twice = once.clone().normalize();
        assert_eq!(once.into_raw(), twice.into_raw());
    }

    #[test]
    fn test_normalize_idempotent_pow() {
        let expr = raw_expr!(Pow[Mul[x, y], 3]);
        let once = expr.clone().normalize();
        let twice = once.clone().normalize();
        assert_eq!(once.into_raw(), twice.into_raw());
    }

    #[test]
    fn test_add_commutative_sort() {
        let expr_a = raw_expr!(Add[x, y, 1]);
        let expr_b = raw_expr!(Add[y, 1, x]);
        assert_eq!(expr_a.normalize().into_raw(), expr_b.normalize().into_raw());
    }

    #[test]
    fn test_mul_commutative_sort() {
        let expr_a = raw_expr!(Mul[x, y, 2]);
        let expr_b = raw_expr!(Mul[y, 2, x]);
        assert_eq!(expr_a.normalize().into_raw(), expr_b.normalize().into_raw());
    }

    #[test]
    fn test_normalize_propagates_to_args_of_arbitrary_functions() {
        let expr = raw_expr!(f[2 + 4 + 8, x * x]);
        let expected = raw_expr!(f[14, Pow[x, 2]]);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_add_numbers_sort_before_symbols() {
        let expr = raw_expr!(Add[x, 2]);
        let expected = raw_expr!(Add[2, x]);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_add_symbols_sort_lexicographically() {
        let expr = raw_expr!(Add[z, x, y]);
        let expected = raw_expr!(Add[x, y, z]);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_mul_numbers_sort_before_symbols() {
        let expr = raw_expr!(Mul[x, 3]);
        let expected = raw_expr!(Mul[3, x]);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_add_atoms_sort_before_nodes() {
        let expr = raw_expr!(Add[Pow[y, 2], x]);
        let expected = raw_expr!(Add[x, Pow[y, 2]]);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_mul_atoms_sort_before_nodes() {
        let expr = raw_expr!(Mul[Pow[y, 2], x]);
        let expected = raw_expr!(Mul[x, Pow[y, 2]]);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_add_nodes_sort_by_head_then_args() {
        let expr = raw_expr!(Add[g[x], f[x]]);
        let expected = raw_expr!(Add[f[x], g[x]]);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_add_nodes_same_head_sort_by_args() {
        let expr = raw_expr!(Add[f[y], f[x]]);
        let expected = raw_expr!(Add[f[x], f[y]]);
        assert_eq!(expr.normalize().into_raw(), expected);
    }

    #[test]
    fn test_add_nodes_same_head_same_args_sort_by_arity() {
        let expr = raw_expr!(Add[f[x, y], f[x]]);
        let expected = raw_expr!(Add[f[x], f[x, y]]);
        assert_eq!(expr.normalize().into_raw(), expected);
    }
}
