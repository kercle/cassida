use parser::parse;
use symbolics::{
    eval::{program::EvalProgram, runtime::EvalRuntime},
    expr::{NormExpr, RawExpr},
    format::MathDisplay,
    norm_expr,
    pattern::{program::Compiler as PatternCompiler, runtime::Runtime as PatternRuntime},
    simplify::Simplifier,
};

use crate::{ClientMessage, ExpressionForms, KernelMessage};

pub fn process_message(inbound_msg: String) -> Result<KernelMessage, KernelMessage> {
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

    let input_expr = RawExpr::from(ast_in);

    let input_expr_forms: ExpressionForms = ExpressionForms {
        raw: input_expr.to_input_form(),
        latex: input_expr.to_latex_form(),
    };

    let input_expr = input_expr.normalize();

    let expr = Simplifier::new(input_expr).simple();

    if let Some(data) = generate_data_if_plot(&expr) {
        Ok(KernelMessage::Plot {
            input: input_expr_forms,
            data,
        })
    } else {
        let expr = expr.resugar().canonicalize();

        Ok(KernelMessage::EvalResult {
            input: input_expr_forms,
            output: ExpressionForms {
                raw: expr.to_input_form(),
                latex: expr.to_latex_form(),
            },
        })
    }
}

pub fn generate_data_if_plot(expr: &NormExpr) -> Option<Vec<(f64, f64)>> {
    let program = PatternCompiler::new().compile(&norm_expr!(
        Plot[
            f_,
            PatternTest[x_, IsSymbol],
            PatternTest[x0_, IsNumber],
            PatternTest[x1_, IsNumber]
        ]
    ));

    let mut runtime = PatternRuntime::new(&program, expr);
    let env = runtime.first_match()?;

    let f = env.get_one("f")?;
    let var = env.get_one("x")?.get_symbol()?;
    let x0 = env.get_one("x0")?.get_number()?.to_f64()?;
    let x1 = env.get_one("x1")?.get_number()?.to_f64()?;

    let program = EvalProgram::compile(f).ok()?;
    let mut runtime = EvalRuntime::new(&program);

    const N: u32 = 100;
    let mut data = Vec::new();
    for i in 0..=N {
        let x = x0 + (x1 - x0) * (i as f64) / (N as f64);
        let _ = runtime.bind_var(var, x);

        let y = runtime.execute();
        data.push((x, y.ok()?));
    }

    Some(data)
}
