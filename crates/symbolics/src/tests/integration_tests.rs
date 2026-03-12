use crate::{format::MathDisplay, kernel::Kernel};

#[test]
fn test_full_processing_chain() {
    let cases = vec![
        ("r/r^2", "1/r"),
        ("r/r", "1"),
        ("2/4", "1/2"),
        ("6/3", "2"),
        ("x^(-1)", "1/x"),
        ("x^(-2)", "1/x^2"),
        ("x*y/x", "y"),
        ("x^2/x", "x"),
        ("x/x^3", "1/x^2"),
        ("1/2 + 1/3", "5/6"),
        ("1/2 * 2/3", "1/3"),
        ("(x^2)^3", "x^6"),
        ("(x^3)/x^2", "x"),
        ("x*1", "x"),
        ("x*0", "0"),
        ("x+0", "x"),
        ("--x", "x"),
        ("(-1)*(-1)", "1"),
        ("2*r/r^2", "2/r"),
        ("r^3/r^2", "r"),
        ("(-2*x)/(-3*y)", "2 * x/(3 * y)"),
        ("Integrate[f+r,x]", "x * (f + r)"),
        ("Integrate[1, x]", "x"),
        ("Integrate[x, x]", "x^2/2"),
        ("Integrate[x^2, x]", "x^3/3"),
        ("Integrate[Sin[x], x]", "-Cos[x]"),
        ("Integrate[Cos[x], x]", "Sin[x]"),
        ("Integrate[Exp[x], x]", "Exp[x]"),
        ("Integrate[1/x, x]", "Log[Abs[x]]"),
        ("Integrate[2*x, x]", "x^2"),
        ("Integrate[f, x]", "f * x"),
        ("Diff[Cos[Sin[x]],x]", "-Cos[x] * Sin[Sin[x]]"),
        ("Diff[x, x]", "1"),
        ("Diff[x^2, x]", "2 * x"),
        ("Diff[x^3, x]", "3 * x^2"),
        ("Diff[x^n, x]", "n * x^(n - 1)"),
        ("Diff[Sin[x], x]", "Cos[x]"),
        ("Diff[Cos[x], x]", "-Sin[x]"),
        ("Diff[Tan[x], x]", "1/Cos[x]^2"),
        ("Diff[Log[x], x]", "1/x"),
        ("Diff[Exp[x], x]", "Exp[x]"),
        ("Diff[Exp[2*x], x]", "2 * Exp[2 * x]"),
        ("Diff[Sin[x^2], x]", "2 * x * Cos[x^2]"),
        ("Diff[Log[Sin[x]], x]", "Cos[x]/Sin[x]"),
        ("Diff[x*Sin[x], x]", "x * Cos[x] + Sin[x]"),
        ("Diff[Sin[x]/x, x]", "Cos[x]/x - Sin[x]/x^2"),
        ("Diff[5, x]", "0"),
        ("Diff[f, x]", "0"),
    ];

    for (input, output) in cases {
        // let ast = parse(input).expect(&format!("Parsing of `{input}` failed"));

        // let expr = RawExpr::from(ast).normalize();
        // let result = Simplifier::new(expr).simple();
        let kernel = Kernel::default();
        let result = kernel
            .eval(input)
            .expect(&format!("Input fails to evaluate: {input}"));

        let result = kernel
            .get_builtin("Simplify")
            .expect("Simplify not a valid builtin")
            .apply_all(result);

        assert_eq!(
            output,
            result.resugar().to_input_form(),
            "For expression `{input}`:"
        )
    }
}
