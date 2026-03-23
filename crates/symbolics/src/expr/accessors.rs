use numbers::Number;

use crate::{
    atom::Atom,
    builtins,
    expr::{Expr, ExprKind},
};

impl<S> Expr<S> {
    pub fn as_atom(&self) -> Option<&Atom> {
        match self.kind() {
            ExprKind::Atom { entry } => Some(entry),
            ExprKind::Node { .. } => None,
        }
    }

    pub fn head(&self) -> Option<&Expr<S>> {
        match self.kind() {
            ExprKind::Atom { .. } => None,
            ExprKind::Node { head, .. } => Some(head),
        }
    }

    pub fn args(&self) -> Option<&[Expr<S>]> {
        match self.kind() {
            ExprKind::Atom { .. } => None,
            ExprKind::Node { args, .. } => Some(args.as_slice()),
        }
    }

    pub fn args_len(&self) -> usize {
        match self.kind() {
            ExprKind::Atom { .. } => 0,
            ExprKind::Node { args, .. } => args.len(),
        }
    }

    pub fn get_arg(&self, index: usize) -> Option<&Self> {
        match self.kind() {
            ExprKind::Atom { .. } => None,
            ExprKind::Node { args, .. } => args.get(index),
        }
    }

    pub fn iter_args(&self) -> Option<std::slice::Iter<'_, Expr<S>>> {
        match self.kind() {
            ExprKind::Atom { .. } => None,
            ExprKind::Node { args, .. } => Some(args.iter()),
        }
    }

    pub fn matches_symbol<T: AsRef<str>>(&self, s: T) -> bool {
        matches!(self.kind(), ExprKind::Atom { entry: Atom::Symbol(t), .. } if t == s.as_ref())
    }

    pub fn is_symbol(&self) -> bool {
        matches!(
            self.kind(),
            ExprKind::Atom {
                entry: Atom::Symbol(_),
                ..
            }
        )
    }

    pub fn is_true(&self) -> bool {
        match self.kind() {
            ExprKind::Atom {
                entry: Atom::Boolean(v),
                ..
            } => *v,
            _ => false,
        }
    }

    pub fn is_false(&self) -> bool {
        match self.kind() {
            ExprKind::Atom {
                entry: Atom::Boolean(v),
                ..
            } => !*v,
            _ => false,
        }
    }

    pub fn get_symbol(&self) -> Option<&str> {
        match self.kind() {
            ExprKind::Atom {
                entry: Atom::Symbol(s),
                ..
            } => Some(s),
            _ => None,
        }
    }

    pub fn is_number_zero(&self) -> bool {
        self.get_number().map(|n| n.is_zero()).unwrap_or(false)
    }

    pub fn is_number_one(&self) -> bool {
        self.get_number().map(|n| n.is_one()).unwrap_or(false)
    }

    pub fn is_number_negative(&self) -> bool {
        self.get_number().map(|n| n.is_negative()).unwrap_or(false)
    }

    pub fn is_number_positive(&self) -> bool {
        self.get_number().map(|n| n.is_positive()).unwrap_or(false)
    }

    pub fn is_number(&self) -> bool {
        matches!(
            self.kind(),
            ExprKind::Atom {
                entry: Atom::Number(_),
                ..
            }
        )
    }

    pub fn get_number(&self) -> Option<&Number> {
        match self.kind() {
            ExprKind::Atom {
                entry: Atom::Number(n),
                ..
            } => Some(n),
            _ => None,
        }
    }

    pub fn is_indeterminate(&self) -> bool {
        self.matches_symbol(builtins::symbols::INDETERMINATE)
    }

    pub fn is_application_of<T: AsRef<str>>(&self, head_sym: T, arity: usize) -> bool {
        self.has_head_symbol(head_sym) && self.args_len() == arity
    }

    pub fn has_head_symbol<T: AsRef<str>>(&self, head_sym: T) -> bool {
        match self.kind() {
            ExprKind::Atom { .. } => false,
            ExprKind::Node { head, .. } => head.matches_symbol(head_sym),
        }
    }

    pub fn matches_head<T: Into<Expr<S>>>(&self, test_head: T) -> bool {
        if let Some(head) = self.head() {
            let test_head = test_head.into();
            *head == test_head
        } else {
            false
        }
    }
}
