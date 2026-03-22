use symbolics::{
    eval::{program::EvalProgram, runtime::EvalRuntime},
    expr::NormExpr,
    format::MathDisplay,
    kernel::Kernel,
    norm_expr,
    pattern::{program::Compiler as PatternCompiler, runtime::Runtime as PatternRuntime},
};

use crate::{ClientMessage, ExpressionForms, KernelMessage};

pub fn process_message(inbound_msg: String) -> Result<KernelMessage, KernelMessage> {
    let inbound_msg: ClientMessage =
        serde_json::from_str(&inbound_msg).map_err(|err| KernelMessage::ParseError {
            input: "n/a".to_string(),
            msg: format!("Cannot unpack inbound message: {err}"),
        })?;

    let ClientMessage::Eval(input) = inbound_msg;

    let kernel = Kernel::default();
    let expr = match kernel.eval(input.clone()) {
        Ok(expr) => expr,
        Err(err) => {
            return Err(KernelMessage::ParseError {
                input: "n/a".to_string(),
                msg: format!("Cannot unpack inbound message: {err:?}"),
            });
        }
    };

    if let Some(data) = generate_data_if_plot(&expr) {
        Ok(KernelMessage::Plot {
            input: input.clone(),
            data,
        })
    } else if let Some(msg) = help(input.clone(), &kernel, &expr) {
        Ok(msg)
    } else {
        let expr = expr.resugar();

        Ok(KernelMessage::EvalResult {
            input: input.clone(),
            output: ExpressionForms {
                raw: expr.to_input_form(),
                latex: expr.to_latex_form(),
            },
        })
    }
}

pub fn generate_data_if_plot(expr: &NormExpr) -> Option<Vec<(f64, f64)>> {
    let program = PatternCompiler::new().compile(&norm_expr!(
        Plot[f_, (x_?IsSymbol, x0_?IsNumber, x1_?IsNumber)]
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

pub fn help(input: String, kernel: &Kernel, expr: &NormExpr) -> Option<KernelMessage> {
    if expr == &norm_expr!(Help[]) {
        return Some(KernelMessage::HelpTableOfContents {
            input,
            builtins: kernel.help_builtins(),
        });
    }

    let program = PatternCompiler::new().compile(&norm_expr!(
        Help[name_?IsSymbol]
    ));

    let mut runtime = PatternRuntime::new(&program, expr);
    let env = runtime.first_match()?;

    let builtin = kernel.get_builtin(env.get_one("name")?.get_symbol()?)?;
    let doc = builtin.doc();
    Some(KernelMessage::HelpBuiltin {
        input,
        title: doc.title.to_string(),
        patterns: doc
            .pattern_doc
            .into_iter()
            .map(|e| (e.pattern.to_input_form(), e.summary))
            .collect(),
        examples: doc
            .examples
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect(),
        related: doc.related.iter().map(|a| a.to_string()).collect(),
    })
}
