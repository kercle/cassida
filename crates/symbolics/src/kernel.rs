use std::{cell::RefCell, rc::Rc};

use numbers::alg::binomial::BinomialGenerator;
use parser::parse;

use crate::{
    builtins::{self, traits::BuiltIn},
    expr::{NormExpr, RawExpr},
};

pub type Shared<T> = Rc<RefCell<T>>;

fn new_shared<T>(obj: T) -> Shared<T> {
    Rc::new(RefCell::new(obj))
}

#[derive(Debug, Clone)]
pub enum KernelError {
    EvaluationError(String),
    UnknownBuiltIn,
}

pub struct Kernel {
    builtins: Vec<Box<dyn BuiltIn>>,
    auto_apply: Vec<usize>,
    binomial_generator: Shared<BinomialGenerator>,
}

impl Default for Kernel {
    fn default() -> Self {
        let mut result = Self {
            builtins: Vec::new(),
            auto_apply: Vec::new(),
            binomial_generator: new_shared(BinomialGenerator::default()),
        };

        result.register_initial_builtins();
        result
    }
}

impl Kernel {
    fn register_initial_builtins(&mut self) {
        self.register_builtin_default::<builtins::calculus::Integrate>(true);
        self.register_builtin_default::<builtins::calculus::Derivative>(true);
        self.register_builtin_default::<builtins::simplification::Simplify>(true);
        self.register_builtin_default::<builtins::system::Help>(false);

        self.register_builtin(
            Box::new(builtins::simplification::Expand::new(
                self.binomial_generator.clone(),
            )),
            true,
        );
    }

    pub fn register_builtin_default<B: BuiltIn + Default + 'static>(&mut self, auto_apply: bool) {
        self.register_builtin(Box::new(B::default()), auto_apply);
    }

    pub fn register_builtin(&mut self, builtin: Box<dyn BuiltIn>, auto_apply: bool) {
        let id = self.builtins.len();
        self.builtins.push(builtin);

        if auto_apply {
            self.set_auto_apply_by_id(id);
        }
    }

    pub fn get_builtin<T: AsRef<str>>(&self, head_name: T) -> Option<&dyn BuiltIn> {
        self.builtins
            .iter()
            .find(|b| b.head_symbol() == head_name.as_ref())
            .map(|v| &**v)
    }

    fn set_auto_apply_by_id(&mut self, id: usize) {
        if !self.auto_apply.contains(&id) {
            self.auto_apply.push(id);
        }
    }

    pub fn set_auto_apply<T: AsRef<str>>(&mut self, head_name: T) -> Result<(), KernelError> {
        let Some(id) = self.get_builtin_id(head_name) else {
            return Err(KernelError::UnknownBuiltIn);
        };

        self.set_auto_apply_by_id(id);

        Ok(())
    }

    // TODO: This is just a quick & dirty function for
    // table of contents. Should be improved.
    pub fn help_builtins(&self) -> Vec<(String, String, String)> {
        self.builtins
            .iter()
            .map(|b| {
                let doc = b.doc();
                (
                    b.head_symbol().to_string(),
                    doc.summary.to_string(),
                    doc.category.to_string(),
                )
            })
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
