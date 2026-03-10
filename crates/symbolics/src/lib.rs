pub mod atom;
pub mod builtin;
pub mod calculus;
pub mod expr;
pub mod format;
pub mod macros;
pub mod pattern;
pub mod rewrite;
pub mod simplify;
pub mod eval;

#[cfg(test)]
mod tests;

// for proc macros: route numbers through parser crate
#[doc(hidden)]
pub use parser_macros as _parser_macros;
