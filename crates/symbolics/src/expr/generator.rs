use crate::expr::Expr;

pub trait ExprBuilder {
    fn build(&self) -> Expr<()>;
}

pub struct SymbolGenerator {
    name: String,
}

impl SymbolGenerator {
    pub fn new<T: AsRef<str>>(name: T) -> SymbolGenerator {
        SymbolGenerator {
            name: name.as_ref().to_string(),
        }
    }
}

impl ExprBuilder for SymbolGenerator {
    fn build(&self) -> Expr {
        Expr::new_symbol(&self.name)
    }
}
