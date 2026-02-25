use numbers::Number;

use crate::{
    builtin::CANNONICAL_HEAD_SQRT,
    expr::{Expr, NormalizedExpr},
    parser::ast::{ADD_HEAD, DIV_HEAD, MUL_HEAD, NEG_HEAD, POW_HEAD, SUB_HEAD},
};

impl<A: Clone + PartialEq + Default> NormalizedExpr<A> {
    fn resugar_add(mut args: Vec<Expr<A>>, annotation: A) -> Expr<A> {
        args.reverse();

        let mut new_args = Vec::with_capacity(args.len());

        let (coeff, term) = NormalizedExpr::new(args.pop().unwrap()).split_coefficient();
        if coeff.is_one() {
            new_args.push(term);
        } else if coeff.is_minus_one() {
            new_args.push(Expr::new_node(NEG_HEAD, vec![term]));
        } else if coeff.is_zero() {
            // In normalized expression, this should not happen
            unreachable!()
        } else {
            new_args.push(Expr::new_node(MUL_HEAD, vec![coeff.into(), term]));
        }

        while let Some(arg) = args.pop() {
            let (coeff, term) = NormalizedExpr::new(arg).split_coefficient();

            if coeff.is_one() {
                new_args.push(term);
            } else if coeff.is_minus_one() {
                let lhs = new_args.pop().unwrap();
                new_args.push(Expr::new_node(SUB_HEAD, vec![lhs, term]));
            } else if coeff.is_negative() {
                let lhs = new_args.pop().unwrap();
                new_args.push(Expr::new_node(
                    SUB_HEAD,
                    vec![
                        lhs,
                        Expr::new_node(MUL_HEAD, vec![coeff.abs().into(), term]),
                    ],
                ));
            } else if coeff.is_zero() {
                // In normalized expression, this should not happen
                unreachable!()
            } else {
                new_args.push(Expr::new_node(MUL_HEAD, vec![coeff.into(), term]));
            }
        }

        if new_args.len() == 1 {
            new_args.pop().unwrap().with_annotation(annotation)
        } else {
            Expr::new_node(ADD_HEAD, new_args).with_annotation(annotation)
        }
    }

    fn resugar_mul(args: Vec<Expr<A>>, annotation: A) -> Expr<A> {
        let mut numerator = Vec::with_capacity(args.len());
        let mut denominator = Vec::with_capacity(args.len());

        for a in args.into_iter() {
            if let Some((lhs, rhs)) = a.unpack_binary_node(POW_HEAD) {
                let (mut coeff, rhs_rest) = NormalizedExpr::new(rhs.clone()).split_coefficient();
                if coeff.is_negative() {
                    coeff.flip_sign();
                    denominator.push(Expr::new_node(
                        POW_HEAD,
                        vec![
                            lhs.clone(),
                            Expr::new_node(MUL_HEAD, vec![coeff.into(), rhs_rest]),
                        ],
                    ));
                } else {
                    numerator.push(a);
                }
            } else if let Some((lhs, rhs)) = a.unpack_binary_node(DIV_HEAD) {
                numerator.push(lhs.clone());
                denominator.push(rhs.clone());
            } else {
                numerator.push(a);
            }
        }

        if denominator.is_empty() {
            Expr::new_node(MUL_HEAD, numerator).with_annotation(annotation)
        } else if numerator.is_empty() {
            Expr::new_node(
                DIV_HEAD,
                vec![
                    Expr::new_number(Number::one()),
                    Expr::new_node(MUL_HEAD, denominator),
                ],
            )
            .with_annotation(annotation)
        } else {
            let lhs = if numerator.len() >= 2 {
                Expr::new_node(MUL_HEAD, numerator)
            } else {
                numerator.pop().unwrap()
            };

            let rhs = if denominator.len() >= 2 {
                Expr::new_node(MUL_HEAD, denominator)
            } else {
                denominator.pop().unwrap()
            };

            Expr::new_node(DIV_HEAD, vec![lhs, rhs]).with_annotation(annotation)
        }
    }

    pub fn resugar(self) -> Expr<A> {
        let expr = self.take_expr();
        match expr {
            Expr::Node {
                head,
                args,
                annotation,
            } if head.matches_symbol(ADD_HEAD) && !args.is_empty() => {
                let args = args
                    .into_iter()
                    .map(|e| NormalizedExpr::new(e).resugar())
                    .collect();
                Self::resugar_add(args, annotation)
            }
            Expr::Node {
                head,
                args,
                annotation,
            } if head.matches_symbol(MUL_HEAD) && !args.is_empty() => {
                let args = args
                    .into_iter()
                    .map(|e| NormalizedExpr::new(e).resugar())
                    .collect();

                Self::resugar_mul(args, annotation)
            }
            Expr::Node {
                head,
                args,
                annotation,
            } if head.matches_symbol(POW_HEAD) && args.len() == 2 => {
                let one_half = Number::new_rational_from_i64(1, 2).unwrap();
                if args
                    .last()
                    .unwrap()
                    .get_number()
                    .map(|e| e == &one_half)
                    .unwrap_or(false)
                {
                    return Expr::new_node(
                        CANNONICAL_HEAD_SQRT,
                        vec![args.first().unwrap().clone()],
                    );
                }

                let args = args
                    .into_iter()
                    .map(|e| NormalizedExpr::new(e).resugar())
                    .collect();

                Self::resugar_mul(
                    vec![Expr::Node {
                        head,
                        args,
                        annotation: A::default(),
                    }],
                    annotation,
                )
            }
            _ => expr,
        }
    }
}
