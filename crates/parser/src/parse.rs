use std::str::FromStr;

use numbers::Number;

use crate::{
    ast::ParserAst,
    error::{BoxedError, ParseError},
    lex::{Quantity, Token, TokenStream},
};

fn parse_identifier_or_call(stream: &mut TokenStream) -> Result<ParserAst, ParseError> {
    // <identifier_or_function_call> ::= <identifier>
    //    | <identifier> "[" "]"
    //    | <identifier> "[" <block> { "," <block> }* ")"

    let Some(symbol_or_pattern) = stream.next_if_symbol_or_pattern().cloned() else {
        return Err(ParseError {
            message: "Expected a symbol or a pattern".to_string(),
            at_token: stream.peek().cloned(),
        });
    };

    let ast_node = match symbol_or_pattern {
        Token::Identifier(value) => ParserAst::new_symbol(value),
        Token::Pattern {
            quantity,
            prefix,
            postfix,
            optional,
        } => match quantity {
            Quantity::One => ParserAst::new_blank(prefix, postfix, optional),
            Quantity::Many(1) => ParserAst::new_blank_seq(prefix, postfix, optional),
            Quantity::Many(0) => ParserAst::new_blank_null_seq(prefix, postfix, optional),
            _ => unreachable!("No other blank varieties are supported"),
        },
        _ => unreachable!("Already exlcuded in next_if_symbol_or_pattern()"),
    };

    if stream.next_if_matches_token(&Token::LeftBracket).is_none() {
        // return ParserAst::new_symbol_or_pattern(symbol_or_pattern).map_err(|err| ParseError {
        //     message: err.message,
        //     at_token: stream.peek().cloned(),
        // });
        return Ok(ast_node);
    }

    if stream.next_if_matches_token(&Token::RightBracket).is_some() {
        let ParserAst::Symbol { name } = ast_node else {
            todo!("Having non-symbol patterns is not supported in parser yet");
        };

        return Ok(ParserAst::new_function_call(name, vec![]));
    }

    let mut args = vec![parse_block(stream)?];
    loop {
        if stream.next_if_matches_token(&Token::Comma).is_none() {
            break; // No more arguments
        }

        args.push(parse_block(stream)?);
    }

    if stream.next_if_matches_token(&Token::RightBracket).is_none() {
        return Err(ParseError {
            message: "Expected closing parenthesis ']' after function arguments".to_string(),
            at_token: stream.peek().cloned(),
        });
    }

    let ParserAst::Symbol { name } = ast_node else {
        todo!("Having non-symbol patterns is not supported in parser yet");
    };

    Ok(ParserAst::new_function_call(name, args))
}

fn parse_atom(stream: &mut TokenStream) -> Result<ParserAst, ParseError> {
    // <atom> ::= <number>
    //    | "(" <block> ")"
    //    | <identifier_or_function_call>

    if stream.next_if_matches_token(&Token::LeftParen).is_some() {
        let expr = parse_block(stream)?;

        if stream.next_if_matches_token(&Token::RightParen).is_some() {
            Ok(expr)
        } else {
            Err(ParseError {
                message: "Expected closing parenthesis ')'".to_string(),
                at_token: stream.peek().cloned(),
            })
        }
    } else if let Some(value) = stream.next_if_number() {
        let value = Number::from_str(value).map_err(|e| ParseError {
            message: e,
            at_token: stream.peek().cloned(),
        })?;
        Ok(ParserAst::new_constant(value))
    } else {
        parse_identifier_or_call(stream)
    }
}

fn parse_power(stream: &mut TokenStream) -> Result<ParserAst, ParseError> {
    // <power> ::= <atom> { "^" <power> }

    let mut result = parse_atom(stream)?;

    if stream.next_if_matches_token(&Token::Caret).is_some() {
        result = ParserAst::new_pow(result, parse_signed_power(stream)?);
    }

    Ok(result)
}

fn parse_signed_power(stream: &mut TokenStream) -> Result<ParserAst, ParseError> {
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
        Ok(ParserAst::new_negation(ast))
    } else {
        Ok(ast)
    }
}

fn parse_product(stream: &mut TokenStream) -> Result<ParserAst, ParseError> {
    // <product> ::= <match_signed_power> { ("*"|"/") <match_signed_power> }*

    let mut result = parse_signed_power(stream)?;

    loop {
        let c = stream.next_if_matches(|token| matches!(token, Token::Asterisk | Token::Slash));

        result = match c {
            Some(Token::Asterisk) => ParserAst::new_mul(result, parse_signed_power(stream)?),
            Some(Token::Slash) => ParserAst::new_div(result, parse_signed_power(stream)?),
            None => break,
            _ => unreachable!(),
        };
    }

    Ok(result)
}

fn parse_sum(stream: &mut TokenStream) -> Result<ParserAst, ParseError> {
    // <sum> ::= <product> { ("+"|"-") <product> }*

    let mut result = parse_product(stream)?;

    loop {
        let op = stream.next_if_matches(|token| matches!(token, Token::Plus | Token::Minus));

        result = match op {
            Some(Token::Plus) => ParserAst::new_add(result, parse_product(stream)?),
            Some(Token::Minus) => ParserAst::new_sub(result, parse_product(stream)?),
            None => break, // No more operators
            _ => unreachable!(),
        };
    }

    Ok(result)
}

fn parse_cmp(stream: &mut TokenStream) -> Result<ParserAst, ParseError> {
    // <cmp> ::= <sum> { ("<" | "<=" | "==" | ">=" | ">") <sum> }*

    let mut result = parse_sum(stream)?;

    loop {
        let Some(op) = stream.next_if_matches(|token| {
            matches!(
                token,
                Token::LessThan
                    | Token::LesserEq
                    | Token::EqEq
                    | Token::GreaterEq
                    | Token::GreaterThan
            )
        }) else {
            break; // No more operators
        };

        result = match op {
            Token::LessThan => ParserAst::new_lt(result, parse_sum(stream)?),
            Token::LesserEq => ParserAst::new_le(result, parse_sum(stream)?),
            Token::EqEq => ParserAst::new_eq(result, parse_sum(stream)?),
            Token::GreaterEq => ParserAst::new_ge(result, parse_sum(stream)?),
            Token::GreaterThan => ParserAst::new_gt(result, parse_sum(stream)?),
            _ => unreachable!(),
        };
    }

    Ok(result)
}

fn parse_expression(stream: &mut TokenStream) -> Result<ParserAst, ParseError> {
    // <expression> ::= <cmp>

    parse_cmp(stream)
}

fn parse_block(stream: &mut TokenStream) -> Result<ParserAst, ParseError> {
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

        if nodes.is_empty() {
            Err(ParseError {
                message: "Expected at least one expression in the block".to_string(),
                at_token: stream.peek().cloned(),
            })
        } else if nodes.len() == 1 {
            Ok(nodes.pop().unwrap())
        } else {
            Ok(ParserAst::new_block(nodes))
        }
    }
}

pub fn parse(input: &str) -> Result<ParserAst, BoxedError> {
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

    #[test]
    fn test_parse_expression_long() {
        let input = "3 + 4 * 2 - (1 + 5) ^ 2 ^ 3";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            ParserAst::new_sub(
                ParserAst::new_add(
                    ParserAst::new_constant(Number::from_str("3").unwrap()),
                    ParserAst::new_mul(
                        ParserAst::new_constant(Number::from_str("4").unwrap()),
                        ParserAst::new_constant(Number::from_str("2").unwrap()),
                    )
                ),
                ParserAst::new_pow(
                    ParserAst::new_add(
                        ParserAst::new_constant(Number::from_str("1").unwrap()),
                        ParserAst::new_constant(Number::from_str("5").unwrap()),
                    ),
                    ParserAst::new_pow(
                        ParserAst::new_constant(Number::from_str("2").unwrap()),
                        ParserAst::new_constant(Number::from_str("3").unwrap()),
                    )
                )
            )
        );
    }

    #[test]
    fn test_parse_expression_short() {
        let input = "3+ 5 -  2";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            ParserAst::new_sub(
                ParserAst::new_add(
                    ParserAst::new_constant(Number::from_str("3").unwrap()),
                    ParserAst::new_constant(Number::from_str("5").unwrap()),
                ),
                ParserAst::new_constant(Number::from_str("2").unwrap())
            )
        );
    }

    #[test]
    fn test_parse_expression_with_float() {
        let input = "3.14 + 2.71";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            ParserAst::new_add(
                ParserAst::new_constant(Number::from_str("3.14").unwrap()),
                ParserAst::new_constant(Number::from_str("2.71").unwrap()),
            )
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
            ParserAst::new_add(
                ParserAst::new_constant(Number::from_str("3").unwrap()),
                ParserAst::new_mul(
                    ParserAst::new_constant(Number::from_str("5").unwrap()),
                    ParserAst::new_constant(Number::from_str("2").unwrap())
                ),
            )
        );
    }

    #[test]
    fn test_parse_expression_with_powers() {
        let input = "7 * 2^3 + 4";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            ParserAst::new_add(
                ParserAst::new_mul(
                    ParserAst::new_constant(Number::from_str("7").unwrap()),
                    ParserAst::new_pow(
                        ParserAst::new_constant(Number::from_str("2").unwrap()),
                        ParserAst::new_constant(Number::from_str("3").unwrap())
                    )
                ),
                ParserAst::new_constant(Number::from_str("4").unwrap()),
            )
        );
    }

    #[test]
    fn test_parse_expression_with_parentheses() {
        let input = "(3 + 5) * 2";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            ParserAst::new_mul(
                ParserAst::new_add(
                    ParserAst::new_constant(Number::from_str("3").unwrap()),
                    ParserAst::new_constant(Number::from_str("5").unwrap()),
                ),
                ParserAst::new_constant(Number::from_str("2").unwrap())
            )
        );
    }

    #[test]
    fn test_parse_expression_with_nested_parentheses() {
        let input = "3 + (2 * (5 - 1))^2";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            ParserAst::new_add(
                ParserAst::new_constant(Number::from_str("3").unwrap()),
                ParserAst::new_pow(
                    ParserAst::new_mul(
                        ParserAst::new_constant(Number::from_str("2").unwrap()),
                        ParserAst::new_sub(
                            ParserAst::new_constant(Number::from_str("5").unwrap()),
                            ParserAst::new_constant(Number::from_str("1").unwrap())
                        )
                    ),
                    ParserAst::new_constant(Number::from_str("2").unwrap())
                ),
            )
        );
    }

    #[test]
    fn test_parse_expression_with_function_call() {
        let input = "5*pi^2/4*cos[pi*x/2]*sin[pi*y/2]";
        let ast = parse(input).expect("Failed to parse expression");

        assert_eq!(
            ast,
            ParserAst::new_mul(
                ParserAst::new_mul(
                    ParserAst::new_div(
                        ParserAst::new_mul(
                            ParserAst::new_constant(Number::from_str("5").unwrap()),
                            ParserAst::new_pow(
                                ParserAst::new_symbol("pi".to_string()),
                                ParserAst::new_constant(Number::from_str("2").unwrap())
                            )
                        ),
                        ParserAst::new_constant(Number::from_str("4").unwrap())
                    ),
                    ParserAst::new_cos(ParserAst::new_div(
                        ParserAst::new_mul(
                            ParserAst::new_symbol("pi".to_string()),
                            ParserAst::new_symbol("x".to_string())
                        ),
                        ParserAst::new_constant(Number::from_str("2").unwrap())
                    ))
                ),
                ParserAst::new_sin(ParserAst::new_div(
                    ParserAst::new_mul(
                        ParserAst::new_symbol("pi".to_string()),
                        ParserAst::new_symbol("y".to_string()),
                    ),
                    ParserAst::new_constant(Number::from_str("2").unwrap())
                ))
            )
        );
    }

    #[test]
    fn test_parsing_block_with_nested_expressions() {
        let input = "{ 3 + 4; 5 * 6 }";
        let ast = parse(input).expect("Failed to parse block with nested expressions");

        assert_eq!(
            ast,
            ParserAst::new_block(vec![
                ParserAst::new_add(
                    ParserAst::new_constant(Number::from_str("3").unwrap()),
                    ParserAst::new_constant(Number::from_str("4").unwrap()),
                ),
                ParserAst::new_mul(
                    ParserAst::new_constant(Number::from_str("5").unwrap()),
                    ParserAst::new_constant(Number::from_str("6").unwrap())
                )
            ])
        );
    }

    #[test]
    fn test_parse_negative_exponent() {
        let input = "a^-1";
        let ast = parse(input).expect("Failed to parse block with nested expressions");

        assert_eq!(
            ast,
            ParserAst::new_pow(
                ParserAst::new_symbol("a"),
                ParserAst::new_negation(ParserAst::from_i64(1))
            ),
        );
    }

    #[test]
    fn test_parse_minus_before_base_has_lower_precidence() {
        let input = "-2^3";
        let ast = parse(input).expect("Failed to parse block with nested expressions");

        assert_eq!(
            ast,
            ParserAst::new_negation(ParserAst::new_pow(
                ParserAst::from_i64(2),
                ParserAst::from_i64(3)
            )),
        );
    }

    #[test]
    fn test_parse_equals() {
        let input = "x+4==-2^3";
        let ast = parse(input).expect("Failed to parse block with nested expressions");

        assert_eq!(
            ast,
            ParserAst::new_eq(
                ParserAst::new_add(ParserAst::new_symbol("x"), ParserAst::from_i64(4)),
                ParserAst::new_negation(ParserAst::new_pow(
                    ParserAst::from_i64(2),
                    ParserAst::from_i64(3)
                ))
            ),
        );
    }

    #[test]
    fn test_parse_pattern() {
        let input = "a+r_^m_.";
        let _ast = parse(input).expect("Failed to parse block with nested expressions");
    }
}
