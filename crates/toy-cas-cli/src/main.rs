use clap::Parser;
use symbolics::format::MathDisplay;
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

    let latex = ast.to_latex();
    println!("$$\n{}\n$$", latex);
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
