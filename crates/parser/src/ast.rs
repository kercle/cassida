use std::fmt::Debug;

use numbers::Number;

use crate::{error::ParseError, utils::nonempty};

#[derive(Debug, Clone, PartialEq)]
pub enum ParserAst {
    Constant {
        value: Number,
    },
    Symbol {
        name: String,
    },
    LesserThan {
        lhs: Box<ParserAst>,
        rhs: Box<ParserAst>,
    },
    LesserEq {
        lhs: Box<ParserAst>,
        rhs: Box<ParserAst>,
    },
    Equals {
        lhs: Box<ParserAst>,
        rhs: Box<ParserAst>,
    },
    GreaterEq {
        lhs: Box<ParserAst>,
        rhs: Box<ParserAst>,
    },
    GreaterThan {
        lhs: Box<ParserAst>,
        rhs: Box<ParserAst>,
    },
    Add {
        lhs: Box<ParserAst>,
        rhs: Box<ParserAst>,
    },
    Negation {
        arg: Box<ParserAst>,
    },
    Sub {
        lhs: Box<ParserAst>,
        rhs: Box<ParserAst>,
    },
    Mul {
        lhs: Box<ParserAst>,
        rhs: Box<ParserAst>,
    },
    Div {
        lhs: Box<ParserAst>,
        rhs: Box<ParserAst>,
    },
    Pow {
        lhs: Box<ParserAst>,
        rhs: Box<ParserAst>,
    },
    FunctionCall {
        name: String,
        args: Vec<ParserAst>,
    },
    Block {
        nodes: Vec<ParserAst>,
    },
    Blank {
        bind_name: Option<String>,
        head_constraint: Option<String>,
    },
    BlankSeq {
        bind_name: Option<String>,
        head_constraint: Option<String>,
    },
    BlankNullSeq {
        bind_name: Option<String>,
        head_constraint: Option<String>,
    },
}

impl ParserAst {
    pub fn new_constant(value: Number) -> Self {
        ParserAst::Constant { value }
    }

    pub fn from_i64(value: i64) -> Self {
        Self::new_constant(Number::from_i64(value))
    }

    pub fn new_constant_from_i64(value: i64) -> Self {
        ParserAst::new_constant(Number::from_i64(value))
    }

    pub fn new_constant_one() -> Self {
        Self::new_constant_from_i64(1)
    }

    pub fn new_constant_zero() -> Self {
        Self::new_constant_from_i64(0)
    }

    pub fn new_symbol<T: AsRef<str>>(name: T) -> Self {
        ParserAst::Symbol {
            name: name.as_ref().to_string(),
        }
    }

    pub fn new_symbol_or_pattern<T: AsRef<str>>(name: T) -> Result<Self, ParseError> {
        debug_assert!(!name.as_ref().is_empty());

        if !name.as_ref().contains('_') {
            return Ok(Self::new_symbol(name));
        }

        let parts: Vec<&str> = name.as_ref().split('_').collect();
        let ret = match parts.as_slice() {
            ["", ""] => {
                // blank _
                Self::new_blank(None, None)
            }
            ["", "", ""] => {
                // double blank __
                Self::new_blank_seq(None, None)
            }
            ["", "", "", ""] => {
                // blank sequence ___
                Self::new_blank_null_seq(None, None)
            }
            [bind_name, ""] => {
                // blank with binding x_
                Self::new_blank(nonempty(bind_name), None)
            }
            ["", head_constr] => {
                // blank with head constraint _x
                Self::new_blank(None, nonempty(head_constr))
            }
            [bind_name, head_constr] => {
                // blank with binding and head constr x_y
                Self::new_blank(nonempty(bind_name), nonempty(head_constr))
            }
            [bind_name, "", ""] => {
                // blank seq with bind name x__
                Self::new_blank_seq(nonempty(bind_name), None)
            }
            ["", "", head_constr] => {
                // blank seq with head constraint __x
                Self::new_blank_seq(None, nonempty(head_constr))
            }
            [bind_name, "", head_constr] => {
                // blank seq with binding and head constr x_y
                Self::new_blank_seq(nonempty(bind_name), nonempty(head_constr))
            }
            [bind_name, "", "", ""] => {
                // blank null seq with bind name x___
                Self::new_blank_null_seq(nonempty(bind_name), None)
            }
            ["", "", "", head_constr] => {
                // blank null seq with head constraint ___x
                Self::new_blank_null_seq(None, nonempty(head_constr))
            }
            [bind_name, "", "", head_constr] => {
                // blank null seq with binding and head constr x_y
                Self::new_blank_null_seq(nonempty(bind_name), nonempty(head_constr))
            }
            _ => {
                return Err(ParseError {
                    message: format!("The pattern `{}` is invalid.", name.as_ref()),
                    at_token: None,
                });
            }
        };

        Ok(ret)
    }

    pub fn new_blank(bind_name: Option<String>, head_constraint: Option<String>) -> Self {
        ParserAst::Blank {
            bind_name,
            head_constraint,
        }
    }

    pub fn new_blank_seq(bind_name: Option<String>, head_constraint: Option<String>) -> Self {
        ParserAst::BlankSeq {
            bind_name,
            head_constraint,
        }
    }

    pub fn new_blank_null_seq(bind_name: Option<String>, head_constraint: Option<String>) -> Self {
        ParserAst::BlankNullSeq {
            bind_name,
            head_constraint,
        }
    }

    pub fn new_lt(lhs: ParserAst, rhs: ParserAst) -> Self {
        ParserAst::LesserThan {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn new_le(lhs: ParserAst, rhs: ParserAst) -> Self {
        ParserAst::LesserEq {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn new_eq(lhs: ParserAst, rhs: ParserAst) -> Self {
        ParserAst::Equals {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn new_ge(lhs: ParserAst, rhs: ParserAst) -> Self {
        ParserAst::GreaterEq {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn new_gt(lhs: ParserAst, rhs: ParserAst) -> Self {
        ParserAst::GreaterThan {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn new_add(lhs: ParserAst, rhs: ParserAst) -> Self {
        ParserAst::Add {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn new_negation(arg: ParserAst) -> Self {
        ParserAst::Negation { arg: Box::new(arg) }
    }

    pub fn new_sub(lhs: ParserAst, rhs: ParserAst) -> Self {
        ParserAst::Sub {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn new_mul(lhs: ParserAst, rhs: ParserAst) -> Self {
        ParserAst::Mul {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn new_div(lhs: ParserAst, rhs: ParserAst) -> Self {
        ParserAst::Div {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn new_pow(lhs: ParserAst, rhs: ParserAst) -> Self {
        ParserAst::Pow {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn new_cos(arg: ParserAst) -> Self {
        Self::new_function_call("cos".to_string(), vec![arg])
    }

    pub fn new_sin(arg: ParserAst) -> Self {
        Self::new_function_call("sin".to_string(), vec![arg])
    }

    pub fn new_tan(arg: ParserAst) -> Self {
        Self::new_function_call("tan".to_string(), vec![arg])
    }

    pub fn new_sqrt(arg: ParserAst) -> Self {
        Self::new_function_call("sqrt".to_string(), vec![arg])
    }

    pub fn new_function_call<T: ToString>(name: T, args: Vec<ParserAst>) -> Self {
        ParserAst::FunctionCall {
            name: name.to_string(),
            args,
        }
    }

    pub fn new_block(nodes: Vec<ParserAst>) -> Self {
        ParserAst::Block { nodes }
    }

    pub fn value_from_constant(&self) -> Option<Number> {
        if let ParserAst::Constant { value, .. } = self {
            Some(value.clone())
        } else {
            None
        }
    }
}

impl ParserAst {
    pub fn is_constant(&self) -> bool {
        matches!(self, ParserAst::Constant { .. })
    }

    pub fn is_one(&self) -> bool {
        if let ParserAst::Constant { value, .. } = self {
            value.is_one()
        } else {
            false
        }
    }

    pub fn is_zero(&self) -> bool {
        if let ParserAst::Constant { value, .. } = self {
            value.is_zero()
        } else {
            false
        }
    }

    pub fn is_symbol(&self) -> bool {
        matches!(self, ParserAst::Symbol { .. })
    }

    pub fn matches_symbol<T: AsRef<str>>(&self, x: T) -> bool {
        if let ParserAst::Symbol { name, .. } = self {
            name == x.as_ref()
        } else {
            false
        }
    }
}
