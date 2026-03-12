use clap::Parser;
use symbolics::format::MathDisplay;
use symbolics::kernel::Kernel;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,

    #[arg(short, long, default_value = "latex")]
    output_type: String,
}

fn print_markdown(input: &str) {
    let result = Kernel::default().eval(input);

    match result {
        Ok(expr) => {
            println!("{}", expr.resugar().to_input_form());
        }
        Err(err) => eprintln!("{err:?}"),
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
