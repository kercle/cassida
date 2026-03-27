use crate::{
    expr::NormExpr,
    pattern::{environment::Environment, program::Compiler, runtime::Runtime},
};

pub(super) fn first_match<'p, 's>(
    program: &'p crate::pattern::program::Program,
    subject: &'s NormExpr,
) -> Option<Environment<'p, 's>> {
    Runtime::new(program, subject).next()
}

pub(super) fn count_matches(pattern: &NormExpr, subject: &NormExpr) -> usize {
    let program = Compiler::new()
        .with_multiset_predicate(|e| {
            e.is_head("CommutativeOp") || e.is_head("Add") || e.is_head("Mul")
        })
        .compile(pattern);
    let runtime = Runtime::new(&program, subject);
    runtime.count()
}
