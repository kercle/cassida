use numbers::RealScalar;

use crate::{
    ast::{AstNode, BinaryOp, UnaryOp},
    error::{BoxedError, ParseError},
    lex::{Token, TokenStream},
};

fn parse_identifier_or_call(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <named_value_or_function_call> ::= <identifier>
    //    | <identifier> "(" ")"
    //    | <identifier> "(" <block> { "," <block> }* ")"

    let identifier = stream.next_if_identifier();

    if identifier.is_none() {
        return Err(ParseError {
            message: "Expected an identifier".to_string(),
            at_token: stream.peek().cloned(),
        });
    }

    let identifier = identifier.unwrap().to_owned();

    if stream.next_if_matches_token(&Token::LeftParen).is_none() {
        return Ok(AstNode::NamedValue(identifier.to_string()));
    }

    if stream.next_if_matches_token(&Token::RightParen).is_some() {
        return Ok(AstNode::FunctionCall {
            name: identifier.to_string(),
            args: Vec::new(),
        });
    }

    let mut args = vec![parse_block(stream)?];
    loop {
        if stream.next_if_matches_token(&Token::Comma).is_none() {
            break; // No more arguments
        }

        args.push(parse_block(stream)?);
    }

    if stream.next_if_matches_token(&Token::RightParen).is_none() {
        return Err(ParseError {
            message: "Expected closing parenthesis ')' after function arguments".to_string(),
            at_token: stream.peek().cloned(),
        });
    }

    Ok(AstNode::FunctionCall {
        name: identifier.to_string(),
        args,
    })
}

fn parse_atom(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <atom> ::= <number>
    //    | "(" <block> ")"
    //    | <named_value_or_function_call>

    if stream.next_if_matches_token(&Token::LeftParen).is_some() {
        let expr = parse_block(stream)?;

        if stream.next_if_matches_token(&Token::RightParen).is_some() {
            return Ok(expr);
        } else {
            return Err(ParseError {
                message: "Expected closing parenthesis ')'".to_string(),
                at_token: stream.peek().cloned(),
            });
        }
    } else if let Some(value) = stream.next_if_number() {
        let scalar = RealScalar::from_str(value).map_err(|e| ParseError {
            message: e,
            at_token: stream.peek().cloned(),
        })?;
        Ok(AstNode::Number(scalar))
    } else {
        parse_identifier_or_call(stream)
    }
}

fn parse_power(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <power> ::= <atom> { "^" <power> }

    let mut result = parse_atom(stream)?;

    if stream.next_if_matches_token(&Token::Caret).is_some() {
        result = AstNode::BinaryNode {
            op: BinaryOp::Pow,
            lhs: Box::new(result),
            rhs: Box::new(parse_power(stream)?),
        };
    }

    Ok(result)
}

fn parse_signed_power(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <signed_power> ::= { "+" | "-" }* power

    let mut negate_count = 0;

    while let Some(sign) =
        stream.next_if_matches(|token| matches!(token, Token::Plus | Token::Minus))
    {
        if *sign == Token::Minus {
            negate_count += 1;
        }
    }

    let ast = parse_power(stream)?;

    if negate_count % 2 == 1 {
        Ok(AstNode::UnaryNode {
            op: UnaryOp::Negate,
            expr: Box::new(ast),
        })
    } else {
        Ok(ast)
    }
}

fn parse_product(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <product> ::= <match_signed_power> { ("*"|"/") <match_signed_power> }*

    let mut result = parse_signed_power(stream)?;

    loop {
        let c = stream.next_if_matches(|token| matches!(token, Token::Asterisk | Token::Slash));

        if c.is_none() {
            break; // No more operators
        }

        result = AstNode::BinaryNode {
            op: match c {
                Some(Token::Asterisk) => BinaryOp::Mul,
                Some(Token::Slash) => BinaryOp::Div,
                _ => unreachable!(),
            },
            lhs: Box::new(result),
            rhs: Box::new(parse_signed_power(stream)?),
        };
    }

    Ok(result)
}

fn parse_sum(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <sum> ::= <product> { ("+"|"-") <product> }*

    let mut result = parse_product(stream)?;

    loop {
        let op = stream.next_if_matches(|token| matches!(token, Token::Plus | Token::Minus));
        if op.is_none() {
            break; // No more operators
        }

        result = AstNode::BinaryNode {
            op: match op {
                Some(Token::Plus) => BinaryOp::Add,
                Some(Token::Minus) => BinaryOp::Sub,
                _ => unreachable!(),
            },
            lhs: Box::new(result),
            rhs: Box::new(parse_product(stream)?),
        };
    }

    Ok(result)
}

fn parse_expression(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <expression> ::= <sum>

    parse_sum(stream)
}

fn parse_block(stream: &mut TokenStream) -> Result<AstNode, ParseError> {
    // <block> ::= <expression> { ";" <expression> }*
    //    | "{" <block> "}"

    let mut nodes = Vec::new();

    if stream.next_if_matches_token(&Token::LeftBrace).is_some() {
        let block = parse_block(stream)?;
        if stream.next_if_matches_token(&Token::RightBrace).is_none() {
            return Err(ParseError {
                message: "Expected closing brace '}'".to_string(),
                at_token: stream.peek().cloned(),
            });
        }

        Ok(block)
    } else {
        loop {
            let expr = parse_expression(stream)?;
            nodes.push(expr);

            if stream.next_if_matches_token(&Token::Semicolon).is_none() {
                break; // No more expressions
            }
        }

        Ok(AstNode::Block(nodes))
    }
}

pub fn parse(input: &str) -> Result<AstNode, BoxedError> {
    let mut stream = TokenStream::from_str(input)?;

    let ast = parse_block(&mut stream);

    if ast.is_ok() && !stream.end_of_stream() {
        return Err(ParseError {
            message: "Unexpected tokens at the end of input".to_string(),
            at_token: stream.peek().cloned(),
        }
        .into());
    }

    match ast {
        Ok(node) => Ok(node),
        Err(err) => Err(err.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use AstNode::*;
    use BinaryOp::*;

    #[test]
    fn test_parse_expression_long() {
        let input = "3 + 4 * 2 - (1 + 5) ^ 2 ^ 3";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            Block(vec![BinaryNode {
                op: Sub,
                lhs: BinaryNode {
                    op: Add,
                    lhs: Number(RealScalar::from_str("3").unwrap()).into(),
                    rhs: BinaryNode {
                        op: Mul,
                        lhs: Number(RealScalar::from_str("4").unwrap()).into(),
                        rhs: Number(RealScalar::from_str("2").unwrap()).into()
                    }
                    .into()
                }
                .into(),
                rhs: BinaryNode {
                    op: Pow,
                    lhs: Block(vec![
                        BinaryNode {
                            op: Add,
                            lhs: Number(RealScalar::from_str("1").unwrap()).into(),
                            rhs: Number(RealScalar::from_str("5").unwrap()).into()
                        }
                        .into()
                    ])
                    .into(),
                    rhs: BinaryNode {
                        op: Pow,
                        lhs: Number(RealScalar::from_str("2").unwrap()).into(),
                        rhs: Number(RealScalar::from_str("3").unwrap()).into()
                    }
                    .into()
                }
                .into()
            }])
        );
    }

    #[test]
    fn test_parse_expression_short() {
        let input = "3+ 5 -  2";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            Block(vec![BinaryNode {
                op: Sub,
                lhs: BinaryNode {
                    op: Add,
                    lhs: Number(RealScalar::from_str("3").unwrap()).into(),
                    rhs: Number(RealScalar::from_str("5").unwrap()).into()
                }
                .into(),
                rhs: Number(RealScalar::from_str("2").unwrap()).into()
            }])
        );
    }

    #[test]
    fn test_parse_expression_with_float() {
        let input = "3.14 + 2.71";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            Block(vec![BinaryNode {
                op: Add,
                lhs: Number(RealScalar::from_str("3.14").unwrap()).into(),
                rhs: Number(RealScalar::from_str("2.71").unwrap()).into()
            }])
        );
    }

    #[test]
    fn test_parse_expression_with_invalid_input() {
        let input = "3 + * 5";
        let ast = parse(input);
        assert!(ast.is_err(), "Expected parse to fail with invalid input");
    }

    #[test]
    fn test_parse_expression_with_products() {
        let input = "3 + 5 * 2";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            Block(vec![BinaryNode {
                op: Add,
                lhs: Number(RealScalar::from_str("3").unwrap()).into(),
                rhs: BinaryNode {
                    op: Mul,
                    lhs: Number(RealScalar::from_str("5").unwrap()).into(),
                    rhs: Number(RealScalar::from_str("2").unwrap()).into()
                }
                .into()
            }])
        );
    }

    #[test]
    fn test_parse_expression_with_powers() {
        let input = "7 * 2^3 + 4";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            Block(vec![BinaryNode {
                op: Add,
                lhs: BinaryNode {
                    op: Mul,
                    lhs: Number(RealScalar::from_str("7").unwrap()).into(),
                    rhs: BinaryNode {
                        op: Pow,
                        lhs: Number(RealScalar::from_str("2").unwrap()).into(),
                        rhs: Number(RealScalar::from_str("3").unwrap()).into()
                    }
                    .into()
                }
                .into(),
                rhs: Number(RealScalar::from_str("4").unwrap()).into()
            }])
        );
    }

    #[test]
    fn test_parse_expression_with_parentheses() {
        let input = "(3 + 5) * 2";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            Block(vec![BinaryNode {
                op: Mul,
                lhs: Block(vec![BinaryNode {
                    op: Add,
                    lhs: Number(RealScalar::from_str("3").unwrap()).into(),
                    rhs: Number(RealScalar::from_str("5").unwrap()).into()
                }])
                .into(),
                rhs: Number(RealScalar::from_str("2").unwrap()).into()
            }])
        );
    }

    #[test]
    fn test_parse_expression_with_nested_parentheses() {
        let input = "3 + (2 * (5 - 1))^2";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            Block(vec![BinaryNode {
                op: Add,
                lhs: Number(RealScalar::from_str("3").unwrap()).into(),
                rhs: BinaryNode {
                    op: Pow,
                    lhs: Block(vec![BinaryNode {
                        op: Mul,
                        lhs: Number(RealScalar::from_str("2").unwrap()).into(),
                        rhs: Block(vec![BinaryNode {
                            op: Sub,
                            lhs: Number(RealScalar::from_str("5").unwrap()).into(),
                            rhs: Number(RealScalar::from_str("1").unwrap()).into()
                        }])
                        .into()
                    }])
                    .into(),
                    rhs: Number(RealScalar::from_str("2").unwrap()).into()
                }
                .into()
            }])
        );
    }

    #[test]
    fn test_parse_expression_with_function_call() {
        let input = "5*pi^2/4*cos(pi*x/2)*sin(pi*y/2)";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            Block(vec![BinaryNode {
                op: Mul,
                lhs: BinaryNode {
                    op: Mul,
                    lhs: BinaryNode {
                        op: Div,
                        lhs: BinaryNode {
                            op: Mul,
                            lhs: Number(RealScalar::from_str("5").unwrap()).into(),
                            rhs: BinaryNode {
                                op: Pow,
                                lhs: AstNode::NamedValue("pi".to_string()).into(),
                                rhs: Number(RealScalar::from_str("2").unwrap()).into()
                            }
                            .into()
                        }
                        .into(),
                        rhs: Number(RealScalar::from_str("4").unwrap()).into()
                    }
                    .into(),
                    rhs: AstNode::FunctionCall {
                        name: "cos".to_string(),
                        args: vec![Block(vec![BinaryNode {
                            op: Div,
                            lhs: BinaryNode {
                                op: Mul,
                                lhs: AstNode::NamedValue("pi".to_string()).into(),
                                rhs: AstNode::NamedValue("x".to_string()).into()
                            }
                            .into(),
                            rhs: Number(RealScalar::from_str("2").unwrap()).into()
                        }])]
                    }.into()
                }
                .into(),
                rhs: AstNode::FunctionCall {
                    name: "sin".to_string(),
                    args: vec![Block(vec![BinaryNode {
                        op: Div,
                        lhs: BinaryNode {
                            op: Mul,
                            lhs: AstNode::NamedValue("pi".to_string()).into(),
                            rhs: AstNode::NamedValue("y".to_string()).into()
                        }
                        .into(),
                        rhs: Number(RealScalar::from_str("2").unwrap()).into()
                    }])]
                }
                .into()
            }])
        );
    }

    #[test]
    fn test_parsing_block_with_nested_expressions() {
        let input = "{ 3 + 4; 5 * 6 }";
        let ast = parse(input).expect("Failed to parse block with nested expressions");

        assert_eq!(
            ast,
            Block(vec![
                BinaryNode {
                    op: Add,
                    lhs: Number(RealScalar::from_str("3").unwrap()).into(),
                    rhs: Number(RealScalar::from_str("4").unwrap()).into()
                }
                .into(),
                BinaryNode {
                    op: Mul,
                    lhs: Number(RealScalar::from_str("5").unwrap()).into(),
                    rhs: Number(RealScalar::from_str("6").unwrap()).into()
                }
                .into()
            ])
        );
    }
}
