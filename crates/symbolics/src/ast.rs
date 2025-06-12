use numbers::RealScalar;

pub trait Operator {
    fn precedence(&self) -> u8;
    fn is_left_associative(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
}

impl Operator for UnaryOp {
    fn precedence(&self) -> u8 {
        3
    }

    fn is_left_associative(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl Operator for BinaryOp {
    fn precedence(&self) -> u8 {
        match self {
            BinaryOp::Add | BinaryOp::Sub => 1,
            BinaryOp::Mul | BinaryOp::Div => 2,
            BinaryOp::Pow => 4,
        }
    }

    fn is_left_associative(&self) -> bool {
        match self {
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div => true,
            BinaryOp::Pow => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Number(RealScalar),
    NamedValue(String),
    UnaryNode {
        op: UnaryOp,
        expr: Box<AstNode>,
    },
    BinaryNode {
        op: BinaryOp,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    FunctionCall {
        name: String,
        args: Vec<AstNode>,
    },
    Block(Vec<AstNode>),
}

impl AstNode {
    pub fn is_numeric(&self) -> bool {
        matches!(self, AstNode::Number(_))
    }

    fn fancy_name(name: &str) -> String {
        match name {
            "alpha" => "\\alpha".to_string(),
            "beta" => "\\beta".to_string(),
            "gamma" => "\\gamma".to_string(),
            "delta" => "\\delta".to_string(),
            "epsilon" => "\\epsilon".to_string(),
            "zeta" => "\\zeta".to_string(),
            "eta" => "\\eta".to_string(),
            "theta" => "\\theta".to_string(),
            "iota" => "\\iota".to_string(),
            "kappa" => "\\kappa".to_string(),
            "lambda" => "\\lambda".to_string(),
            "mu" => "\\mu".to_string(),
            "nu" => "\\nu".to_string(),
            "xi" => "\\xi".to_string(),
            "omicron" => "\\omicron".to_string(),
            "pi" => "\\pi".to_string(),
            "rho" => "\\rho".to_string(),
            "sigma" => "\\sigma".to_string(),
            "tau" => "\\tau".to_string(),
            "upsilon" => "\\upsilon".to_string(),
            "phi" => "\\phi".to_string(),
            "chi" => "\\chi".to_string(),
            "psi" => "\\psi".to_string(),
            "omega" => "\\omega".to_string(),
            _ => name.to_string(),
        }
    }

    fn ast_to_latex(ast: &AstNode, parent_precedence: Option<u8>) -> String {
        match ast {
            AstNode::Number(value) => value.to_string(),
            AstNode::NamedValue(name) => Self::fancy_name(name),
            AstNode::UnaryNode { op, expr } => {
                let expr_str = Self::ast_to_latex(expr, Some(op.precedence()));
                match op {
                    UnaryOp::Negate => format!("-{}", expr_str),
                }
            }
            AstNode::BinaryNode { op, lhs, rhs } => {
                let precedence = if *op == BinaryOp::Div {
                    None // For fractions we don't want parentheses
                } else {
                    Some(op.precedence())
                };

                let lhs_str = Self::ast_to_latex(lhs, precedence);
                let rhs_str = Self::ast_to_latex(rhs, precedence);

                let sub_tree_disp = match op {
                    BinaryOp::Add => format!("{} + {}", lhs_str, rhs_str),
                    BinaryOp::Sub => format!("{} - {}", lhs_str, rhs_str),
                    BinaryOp::Mul => {
                        if lhs.is_numeric() && rhs.is_numeric() {
                            format!("{} \\cdot {}", lhs_str, rhs_str)
                        } else {
                            format!("{} {}", lhs_str, rhs_str)
                        }
                    }
                    BinaryOp::Div => format!("\\frac{{{}}}{{{}}}", lhs_str, rhs_str),
                    BinaryOp::Pow => format!("{}^{{{}}}", lhs_str, rhs_str),
                };

                if parent_precedence > precedence && precedence.is_some() {
                    format!("\\left({}\\right)", sub_tree_disp)
                } else {
                    sub_tree_disp
                }
            }
            AstNode::FunctionCall { name, args } => {
                let mut args_disp = Vec::new();

                for arg in args {
                    args_disp.push(Self::ast_to_latex(arg, None));
                }

                let mut lbracket = "\\left(".to_string();
                let mut rbracket = "\\right)".to_string();

                let name = match name.as_str() {
                    "sin" => "\\sin".to_string(),
                    "cos" => "\\cos".to_string(),
                    "tan" => "\\tan".to_string(),
                    "sqrt" => {
                        lbracket = "{".to_string();
                        rbracket = "}".to_string();
                        "\\sqrt".to_string()
                    }
                    _ => format!("\\operatorname{{{}}}", name),
                };

                format!("{name}{lbracket}{}{rbracket}", args_disp.join(", "))
            }
            AstNode::Block(nodes) => {
                let mut block_str = String::new();
                for node in nodes {
                    if !block_str.is_empty() {
                        block_str.push_str(" \\\\\n");
                    }
                    block_str.push_str(&Self::ast_to_latex(node, None));
                }
                block_str
            }
        }
    }

    pub fn to_latex(&self) -> String {
        Self::ast_to_latex(self, None)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;

    #[test]
    fn test_ast_to_latex() {
        let ast = parse("2 + 3").unwrap();
        assert_eq!(ast.to_latex(), "2 + 3");
    }

    #[test]
    fn test_ast_to_latex_with_parenthesis() {
        let ast = parse("(2 + 3) * 6").unwrap();
        assert_eq!(ast.to_latex(), "\\left(2 + 3\\right) 6");
    }

    #[test]
    fn test_ast_to_latex_multiple_adds() {
        let ast = parse("1+2+3+4").unwrap();
        assert_eq!(ast.to_latex(), "1 + 2 + 3 + 4");
    }

    #[test]
    fn test_ast_to_latex_with_unary_op() {
        let ast = parse("-2 + 3").unwrap();
        assert_eq!(ast.to_latex(), "-2 + 3");
    }

    #[test]
    fn test_ast_to_latex_with_function_call() {
        let ast = parse("5*pi^2/4*cos(pi*x/2)*sin(pi*y/2)").unwrap();
        assert_eq!(
            ast.to_latex(),
            "\\frac{5 \\pi^{2}}{4} \\cos\\left(\\frac{\\pi x}{2}\\right) \\sin\\left(\\frac{\\pi y}{2}\\right)"
        );
    }
}
