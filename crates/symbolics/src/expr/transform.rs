use crate::expr::Expr;

impl<A> Expr<A>
where
    A: Default + Clone + PartialEq,
{
    pub fn annotation_to_default(self) -> Self {
        match self {
            Expr::Atom { entry, .. } => Expr::Atom {
                entry,
                annotation: A::default(),
            },
            Expr::Node { head, args, .. } => Expr::Node {
                head,
                args,
                annotation: A::default(),
            },
        }
    }

    pub fn drop_annotation(self) -> Expr {
        self.map_annotations(&|_| ())
    }

    pub fn with_annotation(self, annotation: A) -> Self {
        use Expr::*;
        match self {
            Atom { entry, .. } => Atom { entry, annotation },
            Node { head, args, .. } => Node {
                head,
                args,
                annotation,
            },
        }
    }

    pub fn map_annotations<B, F>(self, f: &F) -> Expr<B>
    where
        F: Fn(A) -> B + Copy,
    {
        match self {
            Expr::Atom { entry, annotation } => Expr::Atom {
                entry,
                annotation: f(annotation),
            },
            Expr::Node {
                head,
                args,
                annotation,
            } => {
                let head = head.map_annotations(f);
                let args = args.into_iter().map(|a| a.map_annotations(f)).collect();
                let annotation = f(annotation);

                Expr::Node {
                    head: Box::new(head),
                    args,
                    annotation,
                }
            }
        }
    }

    pub fn replace<F>(self, f: &F) -> Expr<A>
    where
        F: Fn(&Expr<A>) -> Option<Expr<A>> + Copy,
    {
        if let Some(replacement) = f(&self) {
            return replacement;
        }

        match self {
            Expr::Atom { .. } => f(&self).unwrap_or(self),
            Expr::Node {
                head,
                args,
                annotation,
            } => {
                let head = f(&head).unwrap_or(head.replace(f));
                let args = args
                    .into_iter()
                    .map(|arg| f(&arg).unwrap_or(arg.replace(f)))
                    .collect();

                Expr::new_node(head, args).with_annotation(annotation)
            }
        }
    }

    pub fn map_top_down<F>(self, f: &F) -> Expr<A>
    where
        F: Fn(Expr<A>) -> Expr<A> + Copy,
    {
        let transformed = f(self);

        match transformed {
            Expr::Atom { .. } => transformed,
            Expr::Node { head, args, .. } => {
                let head = head.map_top_down(f);
                let args = args.into_iter().map(|a| a.map_top_down(f)).collect();
                Expr::new_node(head, args)
            }
        }
    }

    pub fn map_bottom_up<F>(self, f: &F) -> Expr<A>
    where
        F: Fn(Expr<A>) -> Expr<A> + Copy,
    {
        match self {
            Expr::Atom { .. } => f(self),
            Expr::Node { head, args, .. } => {
                let head = head.map_bottom_up(f);
                let args = args.into_iter().map(|a| a.map_bottom_up(f)).collect();
                f(Expr::new_node(head, args))
            }
        }
    }
}
