use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClientMessage {
    Eval(String),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum KernelMessage {
    EvalResult { input: String, output: String },
    ParseError { input: String, msg: String },
}
