///////////////
//// HEADS ////
///////////////

// Exponentials

use crate::expr::Expr;

pub const CANNONICAL_HEAD_EXP: &str = "Exp";
pub const CANNONICAL_HEAD_LOG: &str = "Log";

pub fn is_application_of_exp<A>(expr: Expr<A>) -> bool {
    expr.is_application_of(CANNONICAL_HEAD_EXP, 1)
}

pub fn is_application_of_log<A>(expr: Expr<A>) -> bool {
    expr.is_application_of(CANNONICAL_HEAD_LOG, 1)
}

// Roots

pub const CANNONICAL_HEAD_SQRT: &str = "Sqrt";

pub fn is_application_of_sqrt<A>(expr: Expr<A>) -> bool {
    expr.is_application_of(CANNONICAL_HEAD_SQRT, 1)
}

// Trigonometry

pub const CANNONICAL_HEAD_COS: &str = "Cos";
pub const CANNONICAL_HEAD_SIN: &str = "Sin";
pub const CANNONICAL_HEAD_TAN: &str = "Tan";

pub fn is_application_of_cos<A>(expr: Expr<A>) -> bool {
    expr.is_application_of(CANNONICAL_HEAD_COS, 1)
}

pub fn is_application_of_sin<A>(expr: Expr<A>) -> bool {
    expr.is_application_of(CANNONICAL_HEAD_SIN, 1)
}

pub fn is_application_of_tan<A>(expr: Expr<A>) -> bool {
    expr.is_application_of(CANNONICAL_HEAD_TAN, 1)
}

// Calculus

pub const CANNONICAL_HEAD_DERIVATIVE: &str = "D";

// Other

pub const CANNONICAL_SYM_INDETERMINATE: &str = "Indeterminate";
