use std::{collections::HashMap, rc::Rc};

use crate::{
    atom::Atom,
    expr::pool::{ExprHandle, ExprPool, ExprView, NormExprHandle, RawExprHandle},
    pattern::program::{Program, VarId},
};

#[derive(Clone)]
pub(super) enum EnvBinding {
    One(NormExprHandle),
    Many(Rc<Vec<NormExprHandle>>),
}

#[derive(Clone)]
pub struct Environment<'p> {
    bindings: HashMap<VarId, EnvBinding>,
    program: &'p Program,
}

pub struct ErrorBindCollision;

impl<'p> Environment<'p> {
    pub(super) fn new(program: &'p Program) -> Self {
        Self {
            bindings: HashMap::new(),
            program,
        }
    }

    pub(super) fn bind_one(
        &mut self,
        bind_var: VarId,
        subject: NormExprHandle,
    ) -> Result<bool, ErrorBindCollision> {
        match self.bindings.get(&bind_var) {
            Some(EnvBinding::One(bound_subject)) => {
                if subject == *bound_subject {
                    Ok(false)
                } else {
                    Err(ErrorBindCollision)
                }
            }
            None => {
                self.bindings.insert(bind_var, EnvBinding::One(subject));
                Ok(true)
            }
            _ => Err(ErrorBindCollision),
        }
    }

    pub(super) fn bind_seq(
        &mut self,
        bind_var: VarId,
        subjects: Rc<Vec<NormExprHandle>>,
    ) -> Result<bool, ErrorBindCollision> {
        match self.bindings.get(&bind_var) {
            Some(EnvBinding::Many(bound_subjects)) => {
                if bound_subjects.len() != subjects.len() {
                    return Err(ErrorBindCollision);
                }

                let all_equal = bound_subjects
                    .iter()
                    .zip(subjects.iter())
                    .all(|(a, b)| *a == *b);

                if all_equal {
                    Ok(false)
                } else {
                    Err(ErrorBindCollision)
                }
            }
            None => {
                self.bindings.insert(bind_var, EnvBinding::Many(subjects));
                Ok(true)
            }
            _ => Err(ErrorBindCollision),
        }
    }

    pub(super) fn unbind(&mut self, bind_var: VarId) {
        self.bindings.remove(&bind_var);
    }

    fn var_id_from_name<T: AsRef<str>>(&self, name: T) -> Option<VarId> {
        self.program.var_ids.get(name.as_ref()).cloned()
    }

    pub fn get_one<T: AsRef<str>>(&self, name: T) -> Option<NormExprHandle> {
        use EnvBinding::*;

        let var_id = self.var_id_from_name(name.as_ref())?;

        match self.bindings.get(&var_id)? {
            One(val) => Some(*val),
            Many(_) => None,
        }
    }

    pub fn get_seq<T: AsRef<str>>(&self, name: T) -> Option<&[NormExprHandle]> {
        use EnvBinding::*;

        let var_id = self.var_id_from_name(name.as_ref())?;

        match self.bindings.get(&var_id)? {
            One(_) => None,
            Many(val) => Some(val.as_slice()),
        }
    }
}

impl<'p> Environment<'p> {
    pub fn fill<S: Copy + 'static>(
        &self,
        pool: &mut ExprPool,
        target_expr: ExprHandle<S>,
    ) -> RawExprHandle {
        match target_expr.view(pool) {
            ExprView::Atom(Atom::Symbol(name)) => {
                // In case of a symbol -> Replace with blanks
                self.get_one(&name)
                    .map(|e| e.as_raw())
                    .unwrap_or(target_expr.as_raw())
            }
            ExprView::Node { head, args } => {
                let new_head = self.fill(pool, head);
                let mut new_args = vec![];

                let args_collected: Vec<ExprHandle<S>> = args.iter(pool).collect();
                for arg in args_collected {
                    let arg_view = arg.view(pool);
                    let Some(name) = arg_view.get_symbol() else {
                        // Arg is not a symbol -> decend and push to new args
                        new_args.push(self.fill(pool, arg));
                        continue;
                    };

                    if let Some(repl) = self.get_one(name) {
                        new_args.push(repl.as_raw());
                    } else if let Some(repl) = self.get_seq(name) {
                        new_args.extend(repl.iter().map(|&e| e.as_raw()));
                    } else {
                        new_args.push(arg.as_raw());
                    }
                }

                pool.variadic_node(new_head, new_args)
            }
            _ => target_expr.as_raw(),
        }
    }
}
