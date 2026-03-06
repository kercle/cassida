use std::str::FromStr;

use numbers::Number;

use crate::{
    ast::ParserAst,
    error::{BoxedError, ParseError},
    lex::{Token, TokenStream},
};

fn parse_identifier_or_call(stream: &mut TokenStream) -> Result<ParserAst, ParseError> {
    // <named_value_or_function_call> ::= <identifier>
    //    | <identifier> "[" "]"
    //    | <identifier> "[" <block> { "," <block> }* ")"

    let identifier = stream.next_if_identifier();

    let Some(identifier) = identifier.map(|i| i.to_string()) else {
        return Err(ParseError {
            message: "Expected an identifier".to_string(),
            at_token: stream.peek().cloned(),
        });
    };

    if stream.next_if_matches_token(&Token::LeftBracket).is_none() {
        return Ok(ParserAst::new_named_value(identifier));
    }

    if stream.next_if_matches_token(&Token::RightBracket).is_some() {
        return Ok(ParserAst::new_function_call(identifier, vec![]));
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

    Ok(ParserAst::new_function_call(identifier, args))
}

fn parse_atom(stream: &mut TokenStream) -> Result<ParserAst, ParseError> {
    // <atom> ::= <number>
    //    | "(" <block> ")"
    //    | <named_value_or_function_call>

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
        result = ParserAst::new_pow(result, parse_power(stream)?);
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
            Some(Token::Asterisk) => ParserAst::new_mul_pair(result, parse_signed_power(stream)?),
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
            Some(Token::Plus) => ParserAst::new_add_pair(result, parse_product(stream)?),
            Some(Token::Minus) => ParserAst::new_sub(result, parse_product(stream)?),
            None => break, // No more operators
            _ => unreachable!(),
        };
    }

    Ok(result)
}

fn parse_expression(stream: &mut TokenStream) -> Result<ParserAst, ParseError> {
    // <expression> ::= <sum>

    parse_sum(stream)
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
                ParserAst::new_add_pair(
                    ParserAst::new_constant(Number::from_str("3").unwrap()),
                    ParserAst::new_mul_pair(
                        ParserAst::new_constant(Number::from_str("4").unwrap()),
                        ParserAst::new_constant(Number::from_str("2").unwrap()),
                    )
                ),
                ParserAst::new_pow(
                    ParserAst::new_add_pair(
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
                ParserAst::new_add_pair(
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
            ParserAst::new_add_pair(
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
            ParserAst::new_add_pair(
                ParserAst::new_constant(Number::from_str("3").unwrap()),
                ParserAst::new_mul_pair(
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
            ParserAst::new_add_pair(
                ParserAst::new_mul_pair(
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
            ParserAst::new_mul_pair(
                ParserAst::new_add_pair(
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
            ParserAst::new_add_pair(
                ParserAst::new_constant(Number::from_str("3").unwrap()),
                ParserAst::new_pow(
                    ParserAst::new_mul_pair(
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
            ParserAst::new_mul_pair(
                ParserAst::new_mul_pair(
                    ParserAst::new_div(
                        ParserAst::new_mul_pair(
                            ParserAst::new_constant(Number::from_str("5").unwrap()),
                            ParserAst::new_pow(
                                ParserAst::new_named_value("pi".to_string()),
                                ParserAst::new_constant(Number::from_str("2").unwrap())
                            )
                        ),
                        ParserAst::new_constant(Number::from_str("4").unwrap())
                    ),
                    ParserAst::new_cos(ParserAst::new_div(
                        ParserAst::new_mul_pair(
                            ParserAst::new_named_value("pi".to_string()),
                            ParserAst::new_named_value("x".to_string())
                        ),
                        ParserAst::new_constant(Number::from_str("2").unwrap())
                    ))
                ),
                ParserAst::new_sin(ParserAst::new_div(
                    ParserAst::new_mul_pair(
                        ParserAst::new_named_value("pi".to_string()),
                        ParserAst::new_named_value("y".to_string()),
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
                ParserAst::new_add_pair(
                    ParserAst::new_constant(Number::from_str("3").unwrap()),
                    ParserAst::new_constant(Number::from_str("4").unwrap()),
                ),
                ParserAst::new_mul_pair(
                    ParserAst::new_constant(Number::from_str("5").unwrap()),
                    ParserAst::new_constant(Number::from_str("6").unwrap())
                )
            ])
        );
    }
}
