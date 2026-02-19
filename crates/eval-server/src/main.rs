use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use symbolics::{
    expr::{Expr, NormalizedExpr},
    format::MathDisplay,
    parser::{ast::ParserAst, parse},
};
use tracing::Level;
use tracing::{error, info};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
enum InboundMessage {
    Eval(String),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
enum OutboundMessage {
    EvalResult { result: String, format: String },
    ParseError(String),
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .finish();

    let app = Router::new().route("/ws", get(handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    info!("Kernel listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    info!("Client connected to compute kernel!");

    let (mut sender, mut receiver) = socket.split();

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            let response = match process_message(text.to_string()) {
                Ok(ret) => serde_json::to_string(&ret),
                Err(err) => serde_json::to_string(&OutboundMessage::ParseError(err)),
            };

            if response.is_err() {
                error!("Creating response message failed.");
                break;
            }

            if sender
                .send(Message::Text(response.unwrap().into()))
                .await
                .is_err()
            {
                break;
            }
        }
    }
    info!("Client disconnected.");
}

fn process_message(inbound_msg: String) -> Result<OutboundMessage, String> {
    let inbound_msg: InboundMessage = serde_json::from_str(&inbound_msg)
        .map_err(|err| format!("Cannot unpack inbound message: {err}"))?;

    let InboundMessage::Eval(input) = inbound_msg;

    let ast = parse(&input).map_err(|err| format!("Error parsing input: {}", err))?;
    let expr = NormalizedExpr::new(Expr::from_parser_ast(&ast));

    if let Ok(ast) = ParserAst::try_from(expr) {
        Ok(OutboundMessage::EvalResult {
            result: ast.to_latex(),
            format: "latex".to_string(),
        })
    } else {
        Err("Cannot recover AST from transformed expression.".to_string())
    }
}
