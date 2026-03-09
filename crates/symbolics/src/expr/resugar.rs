use crate::{
    atom::Atom,
    builtin::*,
    expr::{Expr, ExprKind, NormExpr, RawExpr},
};
use numbers::Number;

impl NormExpr {
    pub fn resugar(self) -> RawExpr {
        Self::resugar_inner(self.into_raw())
    }

    fn resugar_number(num: Number) -> RawExpr {
        match num {
            Number::Integer(_) => Atom::number(num).into(),
            Number::Rational(value) => {
                let numerator = Number::Integer(value.numerator().clone());
                let denominator = Number::Integer(value.denominator().clone());

                if denominator.is_one() {
                    Atom::number(numerator).into()
                } else {
                    RawExpr::new_binary_node(
                        DIV_HEAD,
                        Atom::number(numerator).into(),
                        Atom::number(denominator).into(),
                    )
                }
            }
        }
    }

    fn resugar_inner(expr: RawExpr) -> RawExpr {
        match expr.kind {
            ExprKind::Atom {
                entry: Atom::Number(num),
            } => Self::resugar_number(num),
            ExprKind::Node { head, args } if head.matches_symbol(ADD_HEAD) => {
                Self::resugar_add(args)
            }
            ExprKind::Node { head, args } if head.matches_symbol(MUL_HEAD) => {
                Self::resugar_mul(args)
            }
            ExprKind::Node { head, args } if head.matches_symbol(POW_HEAD) && args.len() == 2 => {
                let [lhs, rhs]: [RawExpr; 2] = args.try_into().unwrap();
                Self::resugar_pow(lhs, rhs)
            }
            ExprKind::Node { head, args } => {
                let args: Vec<super::Expr<super::Raw>> =
                    args.into_iter().map(Self::resugar_inner).collect();
                RawExpr::new_node(*head, args)
            }
            _ => expr,
        }
    }

    fn resugar_add(args: Vec<RawExpr>) -> RawExpr {
        assert!(!args.is_empty());

        let mut positives = Vec::new();
        let mut negatives = Vec::new();

        for arg in args {
            let (coeff, term) = NormExpr::split_coefficient(arg);
            let term = Self::resugar_inner(term);

            if coeff.is_negative() {
                let rhs = if coeff.is_minus_one() {
                    term
                } else {
                    RawExpr::new_binary_node(MUL_HEAD, coeff.abs().into(), term)
                };
                negatives.push(rhs);
            } else if coeff.is_one() {
                positives.push(term);
            } else {
                positives.push(RawExpr::new_binary_node(
                    MUL_HEAD,
                    Self::resugar_number(coeff),
                    term,
                ));
            }
        }

        if positives.is_empty() {
            RawExpr::new_unary_node(NEG_HEAD, RawExpr::collapse_add(negatives))
        } else if negatives.is_empty() {
            RawExpr::collapse_add(positives)
        } else {
            RawExpr::new_binary_node(
                SUB_HEAD,
                RawExpr::collapse_add(positives),
                RawExpr::collapse_add(negatives),
            )
        }
    }
    fn resugar_mul(args: Vec<RawExpr>) -> RawExpr {
        let mut has_sign = false;
        let mut numerator = Vec::with_capacity(args.len());
        let mut denominator = Vec::with_capacity(args.len());

        for arg in args {
            if arg.is_number_negative() {
                let num = arg.get_number().unwrap();
                has_sign = !has_sign;

                match num.abs() {
                    num @ Number::Integer(_) => numerator.push(Expr::new_number(num)),
                    Number::Rational(r) => {
                        let num_n = Number::Integer(r.numerator().clone());
                        let num_d = Number::Integer(r.denominator().clone());

                        numerator.push(Expr::new_number(num_n));
                        denominator.push(Expr::new_number(num_d));
                    }
                }

                continue;
            }

            if !arg.matches_head(POW_HEAD) || arg.args_len() != 2 {
                numerator.push(Self::resugar_inner(arg));
                continue;
            }

            let base = arg.get_arg(0).unwrap().clone();
            let exp = arg.get_arg(1).unwrap().clone();

            let (coeff, exp_rest) = NormExpr::split_coefficient(exp);

            let is_in_denominator = coeff.is_negative();
            let coeff = coeff.abs();

            let new_pow_expr = if exp_rest.is_number_one() {
                RawExpr::new_binary_node(POW_HEAD, base, Self::resugar_number(coeff))
            } else if coeff.is_one() {
                RawExpr::new_binary_node(POW_HEAD, base, exp_rest)
            } else {
                let exp = RawExpr::collapse_mul(vec![Self::resugar_number(coeff.abs()), exp_rest]);
                RawExpr::new_binary_node(POW_HEAD, base, exp)
            };

            if is_in_denominator {
                denominator.push(Self::resugar_inner(new_pow_expr));
            } else {
                numerator.push(Self::resugar_inner(new_pow_expr));
            }
        }

        let ret_unsigned = match (numerator.is_empty(), denominator.is_empty()) {
            (_, true) => RawExpr::collapse_mul(numerator),
            (true, _) => RawExpr::new_binary_node(
                DIV_HEAD,
                RawExpr::new_number(Number::one()),
                RawExpr::collapse_mul(denominator),
            ),
            (false, false) => RawExpr::new_binary_node(
                DIV_HEAD,
                RawExpr::collapse_mul(numerator),
                RawExpr::collapse_mul(denominator),
            ),
        };

        if has_sign {
            RawExpr::new_unary_node(NEG_HEAD, ret_unsigned)
        } else {
            ret_unsigned
        }
    }

    fn resugar_pow(lhs: RawExpr, rhs: RawExpr) -> RawExpr {
        let Some(rhs_num) = rhs.get_number() else {
            return RawExpr::new_binary_node(
                POW_HEAD,
                Self::resugar_inner(lhs),
                Self::resugar_inner(rhs),
            );
        };

        let is_neg_exp = rhs_num.is_negative();
        let rhs_num = rhs_num.abs();

        let one_half = Number::new_rational_from_i64(1, 2).unwrap();
        let res = if rhs_num == one_half {
            RawExpr::new_unary_node(CANNONICAL_HEAD_SQRT, Self::resugar_inner(lhs))
        } else {
            RawExpr::new_binary_node(
                POW_HEAD,
                Self::resugar_inner(lhs),
                RawExpr::new_number(rhs_num),
            )
        };

        if is_neg_exp {
            RawExpr::new_binary_node(DIV_HEAD, Expr::from_i64(1), res)
        } else {
            res
        }
    }
}
