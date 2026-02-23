use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
};
use common::{ClientMessage, KernelMessage};
use futures_util::{sink::SinkExt, stream::StreamExt};
use symbolics::{
    expr::Expr,
    format::MathDisplay,
    parser::{ast::ParserAst, parse},
    simplify::Simplifier,
};
use tracing::Level;
use tracing::{error, info};

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
                Err(err) => serde_json::to_string(&err),
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

fn process_message(inbound_msg: String) -> Result<KernelMessage, KernelMessage> {
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
