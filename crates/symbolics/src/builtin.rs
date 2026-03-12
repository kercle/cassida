//------------- HEADS -------------

// Operators

use crate::expr::Expr;

pub const LT_HEAD: &str = "Lesser";
pub const LE_HEAD: &str = "LesserEq";
pub const EQ_HEAD: &str = "Equals";
pub const GE_HEAD: &str = "GreaterEq";
pub const GT_HEAD: &str = "Greater";

pub const ADD_HEAD: &str = "Add";
pub const SUB_HEAD: &str = "Sub";
pub const MUL_HEAD: &str = "Mul";
pub const DIV_HEAD: &str = "Div";
pub const NEG_HEAD: &str = "Neg";
pub const POW_HEAD: &str = "Pow";

// Exponentials

pub const CANNONICAL_HEAD_EXP: &str = "Exp";
pub const CANNONICAL_HEAD_LOG: &str = "Log";

pub fn is_application_of_exp<S>(expr: Expr<S>) -> bool {
    expr.is_application_of(CANNONICAL_HEAD_EXP, 1)
}

pub fn is_application_of_log<S>(expr: Expr<S>) -> bool {
    expr.is_application_of(CANNONICAL_HEAD_LOG, 1)
}

// Roots

pub const CANNONICAL_HEAD_SQRT: &str = "Sqrt";

pub fn is_application_of_sqrt<S>(expr: Expr<S>) -> bool {
    expr.is_application_of(CANNONICAL_HEAD_SQRT, 1)
}

// Trigonometry

pub const CANNONICAL_HEAD_COS: &str = "Cos";
pub const CANNONICAL_HEAD_SIN: &str = "Sin";
pub const CANNONICAL_HEAD_TAN: &str = "Tan";

pub fn is_application_of_cos<S>(expr: Expr<S>) -> bool {
    expr.is_application_of(CANNONICAL_HEAD_COS, 1)
}

pub fn is_application_of_sin<S>(expr: Expr<S>) -> bool {
    expr.is_application_of(CANNONICAL_HEAD_SIN, 1)
}

pub fn is_application_of_tan<S>(expr: Expr<S>) -> bool {
    expr.is_application_of(CANNONICAL_HEAD_TAN, 1)
}

// Calculus

pub const CANNONICAL_HEAD_DERIVATIVE: &str = "Diff";
pub const CANNONICAL_HEAD_INTEGRATE: &str = "Integrate";

// Other

pub const CANNONICAL_SYM_INDETERMINATE: &str = "Indeterminate";
pub const CANNONICAL_SYM_PLUS_INFINITY: &str = "Infinity";

// Structure

pub const CANNONICAL_HEAD_APPLY: &str = "Apply";
pub const CANNONICAL_HEAD_LIST: &str = "List";
pub const CANNONICAL_HEAD_HOLD: &str = "Hold";
