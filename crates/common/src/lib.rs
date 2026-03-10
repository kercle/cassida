pub mod hacks;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClientMessage {
    Eval(String),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum KernelMessage {
    EvalResult {
        input: ExpressionForms,
        output: ExpressionForms,
    },
    Plot {
        input: ExpressionForms,
        data: Vec<(f64, f64)>
    },
    ParseError {
        input: String,
        msg: String,
    },
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionForms {
    pub raw: String,
    pub latex: String,
}
