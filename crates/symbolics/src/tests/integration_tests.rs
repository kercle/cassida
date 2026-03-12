use parser::parse;

use crate::{expr::RawExpr, format::MathDisplay, simplify::Simplifier};

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
        ("D[Cos[Sin[x]],x]", "-Cos[x] * Sin[Sin[x]]"),
        ("D[x, x]", "1"),
        ("D[x^2, x]", "2 * x"),
        ("D[x^3, x]", "3 * x^2"),
        ("D[x^n, x]", "n * x^(n - 1)"),
        ("D[Sin[x], x]", "Cos[x]"),
        ("D[Cos[x], x]", "-Sin[x]"),
        ("D[Tan[x], x]", "1/Cos[x]^2"),
        ("D[Log[x], x]", "1/x"),
        ("D[Exp[x], x]", "Exp[x]"),
        ("D[Exp[2*x], x]", "2 * Exp[2 * x]"),
        ("D[Sin[x^2], x]", "2 * x * Cos[x^2]"),
        ("D[Log[Sin[x]], x]", "Cos[x]/Sin[x]"),
        ("D[x*Sin[x], x]", "x * Cos[x] + Sin[x]"),
        ("D[Sin[x]/x, x]", "Cos[x]/x - Sin[x]/x^2"),
        ("D[5, x]", "0"),
        ("D[f, x]", "0"),
    ];

    for (input, output) in cases {
        let ast = parse(input).expect(&format!("Parsing of `{input}` failed"));

        let expr = RawExpr::from(ast).normalize();
        let result = Simplifier::new(expr).simple();

        assert_eq!(output, result.resugar().to_input_form(), "For expression `{input}`:")
    }
}
