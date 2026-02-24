use std::{
    collections::{HashMap, hash_map::Iter as HashMapIter},
    fmt::{Debug, Formatter},
};

use crate::expr::Expr;

#[derive(Clone)]
pub enum Bound<'a, A> {
    One(&'a Expr<A>),
    Seq(Vec<&'a Expr<A>>),
}

#[derive(Clone)]
pub struct Binding<'a, A> {
    content: Bound<'a, A>,
    rc: u32,
}

#[derive(Clone, Debug)]
pub struct BindingDecError;

impl<'a, A: Clone + Debug + PartialEq> Debug for Binding<'a, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Bound::*;
        match &self.content {
            One(expr) => write!(f, "{:?}", expr),
            Seq(seq) => write!(f, "{:?}", seq),
        }
    }
}

impl<'a, A> Binding<'a, A> {
    pub fn new_one(expr: &'a Expr<A>) -> Self {
        Self {
            content: Bound::One(expr),
            rc: 1,
        }
    }

    pub fn new_seq(expr_arr: Vec<&'a Expr<A>>) -> Self {
        Self {
            content: Bound::Seq(expr_arr),
            rc: 1,
        }
    }

    pub fn inc_bindings(&mut self) {
        self.rc += 1;
    }

    pub fn dec_bindings(&mut self) -> Result<(), BindingDecError> {
        if self.rc > 0 {
            self.rc -= 1;
            Ok(())
        } else {
            Err(BindingDecError)
        }
    }

    pub fn has_no_bindings(&self) -> bool {
        self.rc == 0
    }

    pub fn get_one(&self) -> Option<&'a Expr<A>> {
        use Bound::*;
        match self.content {
            One(expr) => Some(expr),
            Seq(_) => None,
        }
    }

    pub fn get_seq(&self) -> Option<Vec<&'a Expr<A>>> {
        use Bound::*;
        match &self.content {
            One(_) => None,
            Seq(seq) => Some(seq.clone()),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct MatchContext<'a, A = ()>
where
    A: PartialEq + Clone,
{
    bindings: HashMap<String, Binding<'a, A>>,
}

#[derive(Clone, Debug)]
pub enum MatchContextError {
    Bind,
    Missing,
}

impl<'a, A> MatchContext<'a, A>
where
    A: PartialEq + Clone,
{
    pub fn bind_one<T: AsRef<str>>(
        &mut self,
        name: T,
        expr: &'a Expr<A>,
    ) -> Result<(), MatchContextError> {
        if let Some(b) = self.bindings.get_mut(name.as_ref()) {
            let e = b.get_one().ok_or(MatchContextError::Bind)?;

            if e == expr {
                b.inc_bindings();
                Ok(())
            } else {
                Err(MatchContextError::Bind)
            }
        } else {
            self.bindings
                .insert(name.as_ref().to_string(), Binding::new_one(expr));
            Ok(())
        }
    }

    pub fn get_one<T: AsRef<str>>(&self, name: T) -> Option<&'a Expr<A>> {
        if let Some(e) = self.bindings.get(name.as_ref()) {
            e.get_one()
        } else {
            None
        }
    }

    pub fn take_one<T: AsRef<str>>(&mut self, name: T) -> Result<&'a Expr<A>, MatchContextError> {
        if let Some(e) = self.bindings.remove(name.as_ref()) {
            Ok(e.get_one().unwrap())
        } else {
            Err(MatchContextError::Missing)
        }
    }

    pub fn bind_seq<T: AsRef<str>>(
        &mut self,
        name: T,
        expr_arr: Vec<&'a Expr<A>>,
    ) -> Result<(), MatchContextError> {
        if let Some(b) = self.bindings.get_mut(name.as_ref()) {
            let ea = b.get_seq().ok_or(MatchContextError::Bind)?;

            if ea.len() == expr_arr.len()
                && ea.iter().zip(expr_arr.iter()).all(|(&e1, &e2)| e1 == e2)
            {
                b.inc_bindings();
                Ok(())
            } else {
                Err(MatchContextError::Bind)
            }
        } else {
            self.bindings
                .insert(name.as_ref().to_string(), Binding::new_seq(expr_arr));
            Ok(())
        }
    }

    pub fn get_seq<T: AsRef<str>>(&self, name: T) -> Option<Vec<&'a Expr<A>>> {
        if let Some(e) = self.bindings.get(name.as_ref()) {
            e.get_seq()
        } else {
            None
        }
    }

    pub fn take_seq<T: AsRef<str>>(
        &mut self,
        name: T,
    ) -> Result<Vec<&'a Expr<A>>, MatchContextError> {
        if let Some(e) = self.bindings.remove(name.as_ref()) {
            Ok(e.get_seq().unwrap())
        } else {
            Err(MatchContextError::Missing)
        }
    }

    pub fn unbind<T: AsRef<str>>(&mut self, name: T) {
        if let Some(b) = self.bindings.get_mut(name.as_ref()) {
            let _ = b.dec_bindings();
            if b.has_no_bindings() {
                self.bindings.remove(name.as_ref());
            }
        }
    }

    pub fn iter(&self) -> HashMapIter<'_, String, Binding<'_, A>> {
        self.bindings.iter()
    }
}

impl<'a, A> MatchContext<'a, A>
where
    A: PartialEq + Clone + Default,
{
    pub fn fill(&self, target_expr: Expr<A>) -> Expr<A> {
        target_expr.replace(&|expr| {
            let Some(name) = expr.get_symbol() else {
                return None;
            };

            if let Some(e) = self.get_one(name) {
                Some(e.clone())
            } else if let Some(e) = self.get_seq(name) {
                Some(Expr::new_list(e.into_iter().cloned().collect()))
            } else {
                None
            }
        })
    }
}
