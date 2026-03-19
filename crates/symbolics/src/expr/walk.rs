use crate::expr::{
    Expr, ExprKind,
    pool::{ExprHandle, ExprPool, ExprView},
};

pub struct ExprHandleTopDownWalker<'a, S> {
    stack: Vec<ExprHandle<S>>,
    pool: &'a ExprPool,
}

impl<'a, S> ExprHandleTopDownWalker<'a, S> {
    pub fn new(pool: &'a ExprPool, root: ExprHandle<S>) -> Self {
        Self {
            stack: vec![root],
            pool,
        }
    }
}

impl<'a, S: Copy + 'static> Iterator for ExprHandleTopDownWalker<'a, S> {
    type Item = ExprHandle<S>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        if let ExprView::Node { head, args } = node.view(self.pool) {
            for a in args.iter(self.pool).rev() {
                self.stack.push(a);
            }
            self.stack.push(head);
        }

        Some(node)
    }
}

pub struct ExprTopDownWalker<'a, S> {
    stack: Vec<&'a Expr<S>>,
}

impl<'a, S> ExprTopDownWalker<'a, S> {
    pub fn new(root: &'a Expr<S>) -> Self {
        Self { stack: vec![root] }
    }
}

impl<'a, S> Iterator for ExprTopDownWalker<'a, S> {
    type Item = &'a Expr<S>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        if let ExprKind::Node { head, args } = node.kind() {
            for a in args.iter().rev() {
                self.stack.push(a);
            }
            self.stack.push(head);
        }

        Some(node)
    }
}

enum Visit<'a, S> {
    Enter(&'a Expr<S>),
    Exit(&'a Expr<S>),
}

pub struct ExprBottomUpWalker<'a, S> {
    stack: Vec<Visit<'a, S>>,
}

impl<'a, S> ExprBottomUpWalker<'a, S> {
    pub fn new(root: &'a Expr<S>) -> Self {
        Self {
            stack: vec![Visit::Enter(root)],
        }
    }

    fn visit_enter(&mut self, node: &'a Expr<S>) {
        self.stack.push(Visit::Exit(node));
        if let ExprKind::Node { head, args } = node.kind() {
            self.stack.push(Visit::Enter(head));
            for a in args.iter().rev() {
                self.stack.push(Visit::Enter(a));
            }
        }
    }
}

impl<'a, S> Iterator for ExprBottomUpWalker<'a, S> {
    type Item = &'a Expr<S>;

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
    use crate::raw_expr;

    use super::*;

    #[test]
    fn test_walker() {
        let expr = raw_expr! { 2 + x * Cos[x + D[Exp[Pow[y, 2] + 7 * z], x]] };
        let mut walk_seq = vec![
            raw_expr! { 2 },
            raw_expr! { x },
            raw_expr! { x },
            raw_expr! { y },
            raw_expr! { 2 },
            raw_expr! { Pow },
            raw_expr! { Pow[y, 2] },
            raw_expr! { 7 },
            raw_expr! { z },
            raw_expr! { Mul },
            raw_expr! { Mul[7, z] },
            raw_expr! { Add },
            raw_expr! { Add[Pow[y, 2], Mul[7, z]] },
            raw_expr! { Exp },
            raw_expr! { Exp[Add[Pow[y, 2], Mul[7, z]]] },
            raw_expr! { x },
            raw_expr! { D },
            raw_expr! { D[Exp[Add[Pow[y, 2], Mul[7, z]]], x] },
            raw_expr! { Add },
            raw_expr! { Add[x, D[Exp[Add[Pow[y, 2], Mul[7, z]]], x]] },
            raw_expr! { Cos },
            raw_expr! { Cos[Add[x, D[Exp[Add[Pow[y, 2], Mul[7, z]]], x]]] },
            raw_expr! { Mul },
            raw_expr! { Mul[x, Cos[Add[x, D[Exp[Add[Pow[y, 2], Mul[7, z]]], x]]]] },
            raw_expr! { Add },
            raw_expr! { Add[2, Mul[x, Cos[Add[x, D[Exp[Add[Pow[y, 2], Mul[7, z]]], x]]]]] },
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
