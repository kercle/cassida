use parser::parse;

use crate::{
    builtins::{
        calculus::{derivative::Derivative, integrate::Integrate},
        simplify::Simplify,
        traits::BuiltIn,
    },
    expr::{NormExpr, RawExpr},
};

#[derive(Debug, Clone)]
pub enum KernelError {
    EvaluationError(String),
    UnknownBuiltIn,
}

pub struct Kernel {
    builtins: Vec<Box<dyn BuiltIn>>,
    auto_apply: Vec<usize>,
}

impl Default for Kernel {
    fn default() -> Self {
        let mut result = Self {
            builtins: vec![
                Box::new(Integrate::new()),
                Box::new(Derivative::new()),
                Box::new(Simplify::new()),
            ],
            auto_apply: Vec::new(),
        };

        result
            .set_auto_apply("Integrate")
            .expect("Builtin not registerd: Integrate");
        result
            .set_auto_apply("Diff")
            .expect("Builtin not registerd: Diff");
        result
            .set_auto_apply("Simplify")
            .expect("Builtin not registerd: Simplify");

        result
    }
}

impl Kernel {
    pub fn get_builtin<T: AsRef<str>>(&self, head_name: T) -> Option<&dyn BuiltIn> {
        self.builtins
            .iter()
            .find(|b| b.head_symbol() == head_name.as_ref())
            .map(|v| &**v)
    }

    pub fn set_auto_apply<T: AsRef<str>>(&mut self, head_name: T) -> Result<(), KernelError> {
        let Some(id) = self.get_builtin_id(head_name) else {
            return Err(KernelError::UnknownBuiltIn);
        };

        if !self.auto_apply.contains(&id) {
            self.auto_apply.push(id);
        }

        Ok(())
    }

    pub fn help_builtins(&self) -> Vec<(String, String)> {
        self.builtins
            .iter()
            .map(|b| (b.head_symbol().to_string(), b.summary().to_string()))
            .collect()
    }

    fn get_builtin_id<T: AsRef<str>>(&self, head_name: T) -> Option<usize> {
        self.builtins
            .iter()
            .position(|b| b.head_symbol() == head_name.as_ref())
    }

    pub fn eval<T: AsRef<str>>(&self, input: T) -> Result<NormExpr, KernelError> {
        let ast_in = parse(input.as_ref())
            .map_err(|err| KernelError::EvaluationError(format!("Error parsing input: {}", err)))?;

        // TODO: release all holds here is just a workaround until rules are
        // properly integrate into the core of the expression rewriting.
        Ok(self
            .apply_auto_builtins(RawExpr::from(ast_in).normalize())
            .release_all_holds())
    }

    fn apply_auto_builtins(&self, mut expr: NormExpr) -> NormExpr {
        for index in self.auto_apply.iter() {
            let builtin = self.builtins.get(*index).expect("Builtin not registered");
            expr = builtin.apply_all(expr);
        }

        expr
    }
}
