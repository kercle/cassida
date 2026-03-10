mod util;

use common::hacks::process_message;
use wasm_bindgen::prelude::*;

use crate::util::escape_json;

#[wasm_bindgen]
pub fn eval_input(input: &str) -> String {
    let res = process_message(input.to_string());

    match res {
        Ok(msg) => serde_json::to_string(&msg).unwrap_or_else(|e| {
            format!(
                r#"{{"type":"SerializeError","msg":"{}"}}"#,
                escape_json(&e.to_string())
            )
        }),
        Err(msg) => serde_json::to_string(&msg).unwrap_or_else(|e| {
            format!(
                r#"{{"type":"SerializeError","msg":"{}"}}"#,
                escape_json(&e.to_string())
            )
        }),
    }
}
