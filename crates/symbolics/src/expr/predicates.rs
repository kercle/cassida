use crate::{
    expr::{NormExpr, walk::ExprTopDownWalker},
    pattern::{program::Compiler, runtime::Runtime},
};

impl NormExpr {
    pub fn free_of(&self, pattern: &NormExpr) -> bool {
        let program = Compiler::default().compile(pattern);
        !ExprTopDownWalker::new(self).any(|s| Runtime::new(&program, s).is_match())
    }
}
