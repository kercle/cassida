use std::fmt::Debug;

use numbers::Number;

pub const ADD_HEAD: &str = "Add";
pub const SUB_HEAD: &str = "Sub";
pub const MUL_HEAD: &str = "Mul";
pub const DIV_HEAD: &str = "Div";
pub const NEG_HEAD: &str = "Neg";
pub const POW_HEAD: &str = "Pow";

#[derive(Debug, Clone, PartialEq)]
pub enum ParserAst {
    Constant {
        value: Number,
    },
    Symbol {
        name: String,
    },
    Add {
        nodes: Vec<ParserAst>,
    },
    Negation {
        arg: Box<ParserAst>,
    },
    Sub {
        lhs: Box<ParserAst>,
        rhs: Box<ParserAst>,
    },
    Mul {
        nodes: Vec<ParserAst>,
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

    pub fn new_symbol<T: ToString>(name: T) -> Self {
        ParserAst::Symbol {
            name: name.to_string(),
        }
    }

    pub fn new_add(nodes: Vec<ParserAst>) -> Self {
        ParserAst::Add { nodes }
    }

    pub fn new_add_pair(lhs: ParserAst, rhs: ParserAst) -> Self {
        Self::new_add(vec![lhs, rhs])
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

    pub fn new_mul_pair(lhs: ParserAst, rhs: ParserAst) -> Self {
        Self::new_mul(vec![lhs, rhs])
    }

    pub fn new_mul(nodes: Vec<ParserAst>) -> Self {
        ParserAst::Mul { nodes }
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

    pub fn map<F>(self, mut f: F) -> Self
    where
        F: FnMut(Self) -> Self,
    {
        self.map_inner(&mut f)
    }

    fn map_inner<F>(self, f: &mut F) -> Self
    where
        F: FnMut(Self) -> Self,
    {
        use ParserAst::*;
        let mapped = match self {
            Add { nodes } => ParserAst::Add {
                nodes: nodes.into_iter().map(|n| n.map_inner(f)).collect(),
            },
            Negation { arg } => Negation {
                arg: Box::new(arg.map_inner(f)),
            },
            Sub { lhs, rhs } => Sub {
                lhs: Box::new(lhs.map_inner(f)),
                rhs: Box::new(rhs.map_inner(f)),
            },
            Mul { nodes } => Mul {
                nodes: nodes.into_iter().map(|n| n.map_inner(f)).collect(),
            },
            Div { lhs, rhs } => Div {
                lhs: Box::new(lhs.map_inner(f)),
                rhs: Box::new(rhs.map_inner(f)),
            },
            Pow { lhs, rhs } => Pow {
                lhs: Box::new(lhs.map_inner(f)),
                rhs: Box::new(rhs.map_inner(f)),
            },
            FunctionCall { name, args } => FunctionCall {
                name,
                args: args.into_iter().map(|a| a.map_inner(f)).collect(),
            },
            Block { nodes } => Block {
                nodes: nodes.into_iter().map(|n| n.map_inner(f)).collect(),
            },
            node @ Constant { .. } | node @ Symbol { .. } => node,
        };

        f(mapped)
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
