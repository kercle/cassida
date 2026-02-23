mod util;

use symbolics::expr::Expr;
use symbolics::parser::ast::ParserAst;
use symbolics::simplify::Simplifier;
use wasm_bindgen::prelude::*;

use common::{ClientMessage, KernelMessage};
use symbolics::format::MathDisplay;
use symbolics::parser::parse;

use crate::util::escape_json;

#[wasm_bindgen]
pub fn process_message_wasm(inbound_msg_json: &str) -> String {
    let res = eval(inbound_msg_json.to_string());

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

fn eval(inbound_msg: String) -> Result<KernelMessage, KernelMessage> {
    let inbound_msg: ClientMessage =
        serde_json::from_str(&inbound_msg).map_err(|err| KernelMessage::ParseError {
            input: "n/a".to_string(),
            msg: format!("Cannot unpack inbound message: {err}"),
        })?;

    let ClientMessage::Eval(input) = inbound_msg;

    let ast_in = parse(&input).map_err(|err| KernelMessage::ParseError {
        input: input.clone(),
        msg: format!("Error parsing input: {}", err),
    })?;

    let input_latex = ast_in.to_latex();
    let input_expr = Expr::from_parser_ast(ast_in);

    let result_expr = Simplifier::new(input_expr)
        .basic_normalized()
        .resugar()
        .canonicalize();

    if let Ok(ast_out) = ParserAst::try_from(result_expr) {
        Ok(KernelMessage::EvalResult {
            input: input_latex,
            output: ast_out.to_latex(),
        })
    } else {
        Err(KernelMessage::ParseError {
            input: input_latex,
            msg: "Cannot recover AST from transformed expression.".to_string(),
        })
    }
}
