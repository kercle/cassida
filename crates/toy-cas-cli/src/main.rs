use clap::Parser;
use symbolics::expr::{Expr, NormalizedExpr};
use symbolics::format::MathDisplay;
use symbolics::parser::ast::ParserAst;
use symbolics::parser::parse;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,

    #[arg(short, long, default_value = "latex")]
    output_type: String,
}

fn print_markdown(input: &str) {
    let ast = parse(input).unwrap_or_else(|err| {
        eprintln!("Error parsing input: {}", err);
        std::process::exit(1);
    });

    let expr = NormalizedExpr::new(Expr::from_parser_ast(&ast));

    if let Ok(ast) = ParserAst::try_from(expr.take_expr()) {
        let latex = ast.to_latex();
        println!("$$\n{}\n$$", latex);
    } else {
        eprintln!("Cannot recover ParserAst from Expr");
    }
}

fn main() {
    let args = Args::parse();

    match args.output_type.as_str() {
        "latex" => {
            print_markdown(&args.input);
        }
        _ => {
            eprintln!("Unsupported output type: {}", args.output_type);
            std::process::exit(1);
        }
    };
}
