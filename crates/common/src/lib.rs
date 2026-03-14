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
        input: String,
        output: ExpressionForms,
    },
    Plot {
        input: String,
        data: Vec<(f64, f64)>,
    },
    HelpTableOfContents {
        input: String,
        builtins: Vec<(String, String, String)>,
    },
    HelpBuiltin {
        input: String,
        title: String,
        patterns: Vec<(String, String)>,
        examples: Vec<(String, String)>,
        related: Vec<String>,
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
