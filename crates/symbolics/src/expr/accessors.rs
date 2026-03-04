use numbers::Number;

use crate::{atom::Atom, expr::Expr};

impl<A> Expr<A> {
    pub fn as_atom(&self) -> Option<&Atom> {
        match self {
            Expr::Atom { entry, .. } => Some(entry),
            Expr::Node { .. } => None,
        }
    }

    pub fn head(&self) -> Option<&Expr<A>> {
        match self {
            Expr::Atom { .. } => None,
            Expr::Node { head, .. } => Some(head),
        }
    }

    pub fn args(&self) -> Option<&[Expr<A>]> {
        match self {
            Expr::Atom { .. } => None,
            Expr::Node { args, .. } => Some(args.as_slice()),
        }
    }

    pub fn args_len(&self) -> usize {
        match self {
            Expr::Atom { .. } => 0,
            Expr::Node { args, .. } => args.len(),
        }
    }

    pub fn get_arg(&self, index: usize) -> Option<&Self> {
        match self {
            Expr::Atom { .. } => None,
            Expr::Node { args, .. } => args.get(index),
        }
    }

    pub fn iter_args(&self) -> Option<std::slice::Iter<'_, Expr<A>>> {
        match self {
            Expr::Atom { .. } => None,
            Expr::Node { args, .. } => Some(args.iter()),
        }
    }

    pub fn matches_symbol<T: AsRef<str>>(&self, s: T) -> bool {
        matches!(self, Expr::Atom { entry: Atom::Symbol(t), .. } if t == s.as_ref())
    }

    pub fn unpack_binary_node<T: AsRef<str>>(&self, s: T) -> Option<(&Self, &Self)> {
        if self.head().map(|e| e.matches_symbol(s)).unwrap_or(false) && self.args_len() == 2 {
            Some((self.get_arg(0).unwrap(), self.get_arg(1).unwrap()))
        } else {
            None
        }
    }

    pub fn is_symbol(&self) -> bool {
        matches!(
            self,
            Expr::Atom {
                entry: Atom::Symbol(_),
                ..
            }
        )
    }

    pub fn get_symbol(&self) -> Option<&str> {
        match self {
            Expr::Atom {
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
            self,
            Expr::Atom {
                entry: Atom::Number(_),
                ..
            }
        )
    }

    pub fn get_number(&self) -> Option<&Number> {
        match self {
            Expr::Atom {
                entry: Atom::Number(n),
                ..
            } => Some(n),
            _ => None,
        }
    }

    pub fn is_application_of<T: AsRef<str>>(&self, head_sym: T, arity: usize) -> bool {
        self.has_head_symbol(head_sym) && self.args_len() == arity
    }

    pub fn has_head_symbol<T: AsRef<str>>(&self, head_sym: T) -> bool {
        match self {
            Expr::Atom { .. } => false,
            Expr::Node { head, .. } => head.matches_symbol(head_sym),
        }
    }
}

impl<A> Expr<A>
where
    A: Default + Clone + PartialEq,
{
    pub fn matches_head<T: Into<Expr<A>>>(&self, test_head: T) -> bool {
        if let Some(head) = self.head() {
            let test_head = test_head.into();
            *head == test_head
        } else {
            false
        }
    }
}
