use std::{collections::HashMap, fmt::Debug, rc::Rc};

use crate::{
    atom::Atom,
    expr::{Expr, ExprKind, NormExpr, RawExpr},
    pattern::program::{PatternId, Program, VarId},
};

#[derive(Clone)]
pub(super) enum EnvBinding<'s> {
    One(&'s NormExpr),
    Many(Rc<Vec<&'s NormExpr>>),
}

#[derive(Debug, Clone)]
pub struct Environment<'p, 's>
where
    'p: 's,
{
    bindings: HashMap<VarId, EnvBinding<'s>>,
    program: &'p Program,
    pattern_id: PatternId,
}

pub struct ErrorBindCollision;

impl<'p, 's> Environment<'p, 's> {
    pub(super) fn new(program: &'p Program) -> Self {
        Self {
            bindings: HashMap::new(),
            program,
            pattern_id: program.entry_pattern_id,
        }
    }

    pub(super) fn bind_one(
        &mut self,
        bind_var: VarId,
        subject: &'s NormExpr,
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
        subjects: Rc<Vec<&'s NormExpr>>,
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

    pub fn get_one<T: AsRef<str>>(&self, name: T) -> Option<&'s NormExpr> {
        use EnvBinding::*;

        let var_id = self.var_id_from_name(name.as_ref())?;

        match self.bindings.get(&var_id)? {
            One(val) => Some(val),
            Many(_) => None,
        }
    }

    pub fn get_seq<T: AsRef<str>>(&self, name: T) -> Option<&[&'s NormExpr]> {
        use EnvBinding::*;

        let var_id = self.var_id_from_name(name.as_ref())?;

        match self.bindings.get(&var_id)? {
            One(_) => None,
            Many(val) => Some(val.as_slice()),
        }
    }

    pub(super) fn set_pattern_id(&mut self, id: PatternId) {
        self.pattern_id = id;
    }

    pub fn pattern_id(&self) -> PatternId {
        self.pattern_id
    }
}

impl<'p, 's> Environment<'p, 's> {
    pub fn fill<S>(&self, target_expr: Expr<S>) -> RawExpr {
        match target_expr.into_kind() {
            ExprKind::Atom {
                entry: Atom::Symbol(name),
            } => {
                // In case of a symbol -> Replace with blanks
                self.get_one(&name)
                    .cloned()
                    .map(NormExpr::into_raw)
                    .unwrap_or(RawExpr::new_symbol(name).into_raw())
            }
            ExprKind::Node { head, args } => {
                let new_head = self.fill(*head);
                let mut new_args = vec![];

                for arg in args.into_iter() {
                    let Some(name) = arg.get_symbol() else {
                        // Arg is not a symbol -> decend and push to new args
                        new_args.push(self.fill(arg));
                        continue;
                    };

                    if let Some(repl) = self.get_one(name) {
                        new_args.push(repl.clone().into_raw());
                    } else if let Some(repl) = self.get_seq(name) {
                        new_args.extend(repl.iter().map(|&e| e.clone().into_raw()));
                    } else {
                        new_args.push(arg.into_raw());
                    }
                }

                RawExpr::new_node(new_head, new_args)
            }
            other => other.into_raw_expr(),
        }
    }
}

impl<'p, 's> Environment<'p, 's> {
    pub fn dbg_print_bindings(&self) {
        let mut keys: Vec<&VarId> = self.bindings.keys().collect();
        keys.sort();

        for k in keys {
            let v = self.bindings.get(k).unwrap();
            let name = self.program.vars.get(*k as usize).unwrap();
            eprintln!("{name}: {v:?}");
        }
    }
}
