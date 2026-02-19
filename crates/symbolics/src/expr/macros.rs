#[macro_export]
macro_rules! symbol {
    ( $($x:expr),* $(,)? ) => {
        ( $(SymbolGenerator::new($x)),* )
    };
}
