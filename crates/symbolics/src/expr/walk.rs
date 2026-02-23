use crate::expr::Expr;

pub struct ExprTopDownWalker<'a, A> {
    stack: Vec<&'a Expr<A>>,
}

impl<'a, A> ExprTopDownWalker<'a, A>
where
    A: PartialEq,
{
    pub fn new(root: &'a Expr<A>) -> Self {
        Self { stack: vec![root] }
    }
}

impl<'a, A> Iterator for ExprTopDownWalker<'a, A> {
    type Item = &'a Expr<A>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        if let Expr::Compound { head, args, .. } = node {
            for a in args.iter().rev() {
                self.stack.push(a);
            }
            self.stack.push(head);
        }

        Some(node)
    }
}

enum Visit<'a, A> {
    Enter(&'a Expr<A>),
    Exit(&'a Expr<A>),
}

pub struct ExprBottomUpWalker<'a, A> {
    stack: Vec<Visit<'a, A>>,
}

impl<'a, A> ExprBottomUpWalker<'a, A> {
    pub fn new(root: &'a Expr<A>) -> Self {
        Self {
            stack: vec![Visit::Enter(root)],
        }
    }

    fn visit_enter(&mut self, node: &'a Expr<A>) {
        self.stack.push(Visit::Exit(node));
        if let Expr::Compound { head, args, .. } = node {
            self.stack.push(Visit::Enter(head));
            for a in args.iter().rev() {
                self.stack.push(Visit::Enter(a));
            }
        }
    }
}

impl<'a, A> Iterator for ExprBottomUpWalker<'a, A> {
    type Item = &'a Expr<A>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(visit) = self.stack.pop() {
            match visit {
                Visit::Enter(node) => {
                    self.visit_enter(node);
                }
                Visit::Exit(node) => {
                    return Some(node);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use expr_macro::expr;

    use crate::atom::Atom;

    use super::*;

    #[test]
    fn test_walker() {
        let expr = expr! { 2 + x * Cos[x + D[Exp[Pow[y, 2] + 7 * z], x]] };
        let mut walk_seq = vec![
            expr! { 2 },
            expr! { x },
            expr! { x },
            expr! { y },
            expr! { 2 },
            expr! { Pow },
            expr! { Pow[y, 2] },
            expr! { 7 },
            expr! { z },
            expr! { Mul },
            expr! { Mul[7, z] },
            expr! { Add },
            expr! { Add[Pow[y, 2], Mul[7, z]] },
            expr! { Exp },
            expr! { Exp[Add[Pow[y, 2], Mul[7, z]]] },
            expr! { x },
            expr! { D },
            expr! { D[Exp[Add[Pow[y, 2], Mul[7, z]]], x] },
            expr! { Add },
            expr! { Add[x, D[Exp[Add[Pow[y, 2], Mul[7, z]]], x]] },
            expr! { Cos },
            expr! { Cos[Add[x, D[Exp[Add[Pow[y, 2], Mul[7, z]]], x]]] },
            expr! { Mul },
            expr! { Mul[x, Cos[Add[x, D[Exp[Add[Pow[y, 2], Mul[7, z]]], x]]]] },
            expr! { Add },
            expr! { Add[2, Mul[x, Cos[Add[x, D[Exp[Add[Pow[y, 2], Mul[7, z]]], x]]]]] },
        ];

        walk_seq.reverse();

        for actual in ExprBottomUpWalker::new(&expr) {
            let expected = walk_seq
                .pop()
                .expect("ExprBottomUpWalker emits more tokens than expected.");

            assert_eq!(*actual, expected);
        }

        assert!(walk_seq.is_empty())
    }
}
