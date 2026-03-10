use std::{collections::VecDeque, str::FromStr};

use crate::error::LexError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenPos {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Quantity {
    One,
    Many(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // Double-character operators
    EqEq,       // '=='
    NotEq,      // '!='
    LesserEq,   // '<='
    GreaterEq,  // '>='
    ColonEq,    // ':='
    ColonGt,    // ':>'
    DoubleExcl, // '!!'

    // Single-character operators
    LeftBrace,    // '{'
    RightBrace,   // '}'
    LeftBracket,  // '['
    RightBracket, // ']'
    LeftParen,    // '('
    RightParen,   // ')'
    Comma,        // ','
    Semicolon,    // ';'
    Colon,        // ':'
    Equals,       // '='
    Plus,         // '+'
    Minus,        // '-'
    Asterisk,     // '*'
    Slash,        // '/'
    Percent,      // '%'
    Ampersand,    // '&'
    Pipe,         // '|'
    Caret,        // '^'
    Tilde,        // '~'
    Exclamation,  // '!'
    Question,     // '?'
    LessThan,     // '<'
    GreaterThan,  // '>'

    // Multi-character tokens
    Number(String),
    Identifier(String),
    StringLiteral(String),
    CodeBlock {
        language: String,
        code: String,
    },
    Pattern {
        quantity: Quantity,
        prefix: Option<String>,
        postfix: Option<String>,
        optional: bool,
    },
}

struct CharIterator<'a> {
    iter: std::str::Chars<'a>,
    line: usize,
    column: usize,
    lookahead_buffer: VecDeque<char>,
}

impl<'a> CharIterator<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            iter: s.chars(),
            line: 1,
            column: 1,
            lookahead_buffer: VecDeque::new(),
        }
    }

    fn next(&mut self) -> Option<char> {
        let next_opt = if self.lookahead_buffer.is_empty() {
            self.iter.next()
        } else {
            self.lookahead_buffer.pop_front()
        };

        if let Some(c) = next_opt {
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }

        next_opt
    }

    fn lookahead(&mut self, n: usize) -> Option<&char> {
        while self.lookahead_buffer.len() <= n {
            if let Some(c) = self.iter.next() {
                self.lookahead_buffer.push_back(c);
            } else {
                return None;
            }
        }
        self.lookahead_buffer.get(n)
    }

    fn peek(&mut self) -> Option<&char> {
        self.lookahead(0)
    }

    fn pos(&self) -> TokenPos {
        TokenPos {
            line: self.line,
            column: self.column,
        }
    }
}

#[derive(Debug)]
pub struct TokenStream {
    tokens: Vec<(Token, TokenPos)>,
    current: usize,
}

impl TokenStream {
    fn comsume_number_chars(
        iter: &mut CharIterator,
        tokens: &mut Vec<(Token, TokenPos)>,
    ) -> Result<(), LexError> {
        let pos = iter.pos();

        let mut base_string = String::new();
        let mut exp_sign_string = String::new();
        let mut exponent_string = String::new();

        while let Some(c) = iter.peek() {
            if !c.is_ascii_digit() {
                break;
            }

            base_string.push(iter.next().unwrap());
        }

        if base_string.is_empty() {
            return Err(LexError {
                message: "Invalid number format: Expected digits in number".to_string(),
                line: iter.line,
                column: iter.column,
            });
        }

        if let Some('.') = iter.peek() {
            base_string.push(iter.next().unwrap());

            while let Some(c) = iter.peek() {
                if !c.is_ascii_digit() {
                    break;
                }

                base_string.push(iter.next().unwrap());
            }

            if base_string.ends_with('.') {
                return Err(LexError {
                    message: "Invalid number format: Expected digits after decimal point"
                        .to_string(),
                    line: iter.line,
                    column: iter.column,
                });
            }
        }

        if let Some('e') | Some('E') = iter.peek() {
            iter.next(); // Consume 'e' or 'E'

            if let Some('+') | Some('-') = iter.peek() {
                exp_sign_string.push(iter.next().unwrap());
            }

            while iter.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                let c = iter.next().unwrap();
                exponent_string.push(c);
            }

            if exponent_string.is_empty() {
                return Err(LexError {
                    message: "Invalid number format: Expected digits in exponent".to_string(),
                    line: iter.line,
                    column: iter.column,
                });
            }
        }

        let number_string = if !exponent_string.is_empty() {
            format!("{}e{}{}", base_string, exp_sign_string, exponent_string)
        } else {
            base_string
        };

        tokens.push((Token::Number(number_string), pos));

        Ok(())
    }

    fn consume_ident_or_pattern_chars(
        iter: &mut CharIterator,
    ) -> Result<(Token, TokenPos), LexError> {
        let pos = iter.pos();

        let prefix = if let Some(c) = iter.peek()
            && c.is_alphabetic()
        {
            Some(Self::consume_identifier_chars(iter)?)
        } else {
            None
        };

        let blank_pos = iter.pos();
        let blank_count = Self::consume_blanks_and_count(iter);

        if blank_count == 0 {
            let Some((prefix, _)) = prefix else {
                return Err(LexError {
                    message: "Expected identifier or pattern.".to_string(),
                    line: iter.line,
                    column: iter.column,
                });
            };

            return Ok((Token::Identifier(prefix), pos));
        }

        let mut optional = false;
        let postfix = if let Some(c) = iter.peek()
            && c.is_alphabetic()
        {
            let (value, _) = Self::consume_identifier_chars(iter)?;
            Some(value)
        } else if matches!(iter.peek(), Some('.')) {
            // Postfix identifiers and the dot denoting optionals are
            // mutually exclusive, hence only check for dot if there is
            // no postfix.

            iter.next(); // consume '.'
            optional = true;

            None
        } else {
            None
        };

        let quantity = match blank_count {
            1 => Quantity::One,
            2 => Quantity::Many(1),
            3 => Quantity::Many(0),
            _ => {
                return Err(LexError {
                    message: "Only _, __ and ___ are supported as pattern variants.".to_string(),
                    line: blank_pos.line,
                    column: blank_pos.column,
                });
            }
        };

        Ok((
            Token::Pattern {
                quantity,
                prefix: prefix.map(|x| x.0),
                postfix: postfix,
                optional,
            },
            pos,
        ))
    }

    fn consume_blanks_and_count(iter: &mut CharIterator) -> u32 {
        let mut count = 0;

        while let Some('_') = iter.peek() {
            iter.next(); // consume '_'
            count += 1;
        }

        count
    }

    fn consume_identifier_chars(iter: &mut CharIterator) -> Result<(String, TokenPos), LexError> {
        let pos = iter.pos();
        let mut identifier_string = String::new();

        if let Some(c) = iter.next() {
            if c.is_alphabetic() {
                identifier_string.push(c);
            } else {
                return Err(LexError {
                    message: "Invalid identifier: Must start with an alphabetic character"
                        .to_string(),
                    line: iter.line,
                    column: iter.column,
                });
            }
        }

        while let Some(c) = iter.peek() {
            if c.is_alphanumeric() {
                identifier_string.push(iter.next().unwrap());
            } else {
                break;
            }
        }

        Ok((identifier_string, pos))
    }

    fn consume_operator_chars(
        iter: &mut CharIterator,
        tokens: &mut Vec<(Token, TokenPos)>,
    ) -> bool {
        let Some(c) = iter.peek().cloned() else {
            return false; // No character to consume
        };

        let pos = iter.pos();
        if c == '=' {
            iter.next(); // Consume '='

            if matches!(iter.peek(), Some('=')) {
                iter.next(); // Consume second '='
                tokens.push((Token::EqEq, pos));
            } else {
                tokens.push((Token::Equals, pos));
            }
        } else if c == '!' {
            iter.next(); // Consume '!'

            if matches!(iter.peek(), Some('=')) {
                iter.next(); // Consume '='
                tokens.push((Token::NotEq, pos));
            } else if matches!(iter.peek(), Some('!')) {
                iter.next(); // Consume '!'
                tokens.push((Token::DoubleExcl, pos));
            } else {
                tokens.push((Token::Exclamation, pos));
            }
        } else if c == '<' {
            iter.next(); // Consume '<'

            if matches!(iter.peek(), Some('=')) {
                iter.next(); // Consume '='
                tokens.push((Token::LesserEq, pos));
            } else {
                tokens.push((Token::LessThan, pos));
            }
        } else if c == '>' {
            iter.next(); // Consume '>'

            if matches!(iter.peek(), Some('=')) {
                iter.next(); // Consume '='
                tokens.push((Token::GreaterEq, pos));
            } else {
                tokens.push((Token::GreaterThan, pos));
            }
        } else if c == ':' {
            iter.next(); // Consume ':'

            if matches!(iter.peek(), Some('=')) {
                iter.next(); // Consume '='
                tokens.push((Token::ColonEq, pos));
            } else if matches!(iter.peek(), Some('>')) {
                iter.next(); // Consume '>'
                tokens.push((Token::ColonGt, pos));
            } else {
                tokens.push((Token::Colon, pos));
            }
        } else {
            let token = match c {
                '{' => Token::LeftBrace,
                '}' => Token::RightBrace,
                '[' => Token::LeftBracket,
                ']' => Token::RightBracket,
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                ',' => Token::Comma,
                ';' => Token::Semicolon,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Asterisk,
                '/' => Token::Slash,
                '%' => Token::Percent,
                '&' => Token::Ampersand,
                '|' => Token::Pipe,
                '^' => Token::Caret,
                '~' => Token::Tilde,
                '?' => Token::Question,
                _ => return false, // Not an operator character
            };

            tokens.push((token, iter.pos()));
            iter.next(); // Consume the operator character
        }

        true
    }

    fn consume_string_literal(
        term_sequence: &str,
        iter: &mut CharIterator,
    ) -> Result<String, LexError> {
        let mut string_literal = String::new();

        while let Some(next_char) = iter.next() {
            if term_sequence.starts_with(next_char) {
                let mut term_sequence_candidate = String::new();
                term_sequence_candidate.push(next_char);

                for _ in 1..term_sequence.len() {
                    if let Some(c) = iter.peek() {
                        term_sequence_candidate.push(*c);
                        iter.next(); // Consume the next character
                    } else {
                        return Err(LexError {
                            message: "Unterminated string literal".to_string(),
                            line: iter.line,
                            column: iter.column,
                        });
                    }
                }

                if term_sequence_candidate == term_sequence {
                    return Ok(string_literal); // End of string literal
                } else {
                    string_literal.push_str(&term_sequence_candidate);
                    continue; // Continue processing the next character
                }
            }
            if next_char == '\\' {
                if let Some(escaped_char) = iter.next() {
                    match escaped_char {
                        'n' => string_literal.push('\n'),
                        't' => string_literal.push('\t'),
                        'r' => string_literal.push('\r'),
                        '"' => string_literal.push('"'),
                        '\\' => string_literal.push('\\'),
                        _ => {
                            return Err(LexError {
                                message: format!("Unknown escape sequence: \\{}", escaped_char),
                                line: iter.line,
                                column: iter.column,
                            });
                        }
                    }
                } else {
                    return Err(LexError {
                        message: "Unterminated string literal".to_string(),
                        line: iter.line,
                        column: iter.column,
                    });
                }
            } else {
                string_literal.push(next_char);
            }
        }

        Err(LexError {
            message: "Unterminated string literal".to_string(),
            line: iter.line,
            column: iter.column,
        })
    }

    pub fn next_token(&mut self) -> Option<&Token> {
        if self.current < self.tokens.len() {
            self.current += 1;
            Some(&self.tokens[self.current - 1].0)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&(Token, TokenPos)> {
        if self.current < self.tokens.len() {
            Some(&self.tokens[self.current])
        } else {
            None
        }
    }

    pub fn peek_token(&self) -> Option<&Token> {
        self.peek().map(|(token, _)| token)
    }

    pub fn next_if_matches<F>(&mut self, matcher: F) -> Option<&Token>
    where
        F: Fn(&Token) -> bool,
    {
        let current_token = self.peek_token()?;

        if matcher(current_token) {
            self.next_token()
        } else {
            None
        }
    }

    pub fn next_if_matches_token(&mut self, token: &Token) -> Option<&Token> {
        self.next_if_matches(|t| t == token)
    }

    pub fn next_if_symbol_or_pattern(&mut self) -> Option<&Token> {
        Some(self.next_if_matches(|t| matches!(t, Token::Identifier(_) | Token::Pattern { .. }))?)
    }

    pub fn next_if_number(&mut self) -> Option<&str> {
        let token = self.next_if_matches(|t| matches!(t, Token::Number(_)))?;

        if let Token::Number(value) = token {
            Some(value)
        } else {
            None
        }
    }

    pub fn end_of_stream(&self) -> bool {
        self.current >= self.tokens.len()
    }
}

impl FromStr for TokenStream {
    type Err = LexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = vec![];

        let mut iter = CharIterator::new(s);
        while let Some(&c) = iter.peek() {
            if c == '"' {
                let pos = iter.pos();
                iter.next();
                let string = Self::consume_string_literal("\"", &mut iter)?;
                tokens.push((Token::StringLiteral(string), pos));
            } else if c == '`' {
                let pos = iter.pos();
                for _ in 0..3 {
                    if iter.next() != Some('`') {
                        return Err(LexError {
                            message: "Unterminated string literal".to_string(),
                            line: iter.line,
                            column: iter.column,
                        });
                    }
                }
                let (language, _) = Self::consume_identifier_chars(&mut iter)?;
                let code = Self::consume_string_literal("```", &mut iter)?;

                tokens.push((Token::CodeBlock { language, code }, pos));
            } else if c.is_whitespace() {
                iter.next(); // Consume whitespace
            } else if Self::consume_operator_chars(&mut iter, &mut tokens) {
                continue;
            } else if c.is_ascii_digit() {
                Self::comsume_number_chars(&mut iter, &mut tokens)?;
            } else if c.is_alphabetic() || c == '_' {
                tokens.push(Self::consume_ident_or_pattern_chars(&mut iter)?);
            } else {
                return Err(LexError {
                    message: format!("Unknown token: {}", c),
                    line: iter.line,
                    column: iter.column,
                });
            }
        }

        Ok(Self { tokens, current: 0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token_streams_eq(actual: &[(Token, TokenPos)], expected: &[Token]) -> bool {
        if actual.len() != expected.len() {
            return false;
        }

        for ((actual_token, _), expected_token) in actual.iter().zip(expected.iter()) {
            if actual_token != expected_token {
                return false;
            }
        }

        true
    }

    #[test]
    fn test_simple_token_stream_from_str() {
        let input = "1+3";
        let token_stream = TokenStream::from_str(input).unwrap();
        dbg!(token_stream);
    }

    #[test]
    fn test_token_stream_from_str() {
        let input = r#"
            c := 42;
            integrate(c * x^2, x);
        "#;

        let token_stream = TokenStream::from_str(input).unwrap();
        assert!(
            !token_stream.tokens.is_empty(),
            "Token stream should not be empty"
        );

        let expected = vec![
            Token::Identifier("c".to_string()),
            Token::ColonEq,
            Token::Number("42".to_string()),
            Token::Semicolon,
            Token::Identifier("integrate".to_string()),
            Token::LeftParen,
            Token::Identifier("c".to_string()),
            Token::Asterisk,
            Token::Identifier("x".to_string()),
            Token::Caret,
            Token::Number("2".to_string()),
            Token::Comma,
            Token::Identifier("x".to_string()),
            Token::RightParen,
            Token::Semicolon,
        ];

        assert_eq!(
            token_stream.tokens.len(),
            expected.len(),
            "Token count mismatch"
        );

        for ((token, _), expected_token) in token_stream.tokens.iter().zip(expected.iter()) {
            assert_eq!(
                token, expected_token,
                "Token mismatch at position {:?}",
                token
            );
        }
    }

    #[test]
    fn test_string_literal() {
        let input = r#"f["Hello, world!"]"#;
        let token_stream = TokenStream::from_str(input).unwrap();
        assert_eq!(token_stream.tokens.len(), 4);

        let expected = vec![
            Token::Identifier("f".to_string()),
            Token::LeftBracket,
            Token::StringLiteral("Hello, world!".to_string()),
            Token::RightBracket,
        ];

        for ((token, _), expected_token) in token_stream.tokens.iter().zip(expected.iter()) {
            assert_eq!(
                token, expected_token,
                "Token mismatch at position {:?}",
                token
            );
        }
    }

    #[test]
    fn test_code_block() {
        let input = r#"```python
            let x = 42;
            print("Hello, world {x}!")
        ```"#;
        let token_stream = TokenStream::from_str(input).unwrap();
        assert_eq!(token_stream.tokens.len(), 1);
        let expected = vec![Token::CodeBlock {
            language: "python".to_string(),
            code: "\n            let x = 42;\n            print(\"Hello, world {x}!\")\n        "
                .to_string(),
        }];
        for ((token, _), expected_token) in token_stream.tokens.iter().zip(expected.iter()) {
            assert_eq!(
                token, expected_token,
                "Token mismatch at position {:?}",
                token
            );
        }
    }

    #[test]
    fn test_pow_minus_one() {
        let input = r#"a^-1"#;
        let token_stream = TokenStream::from_str(input).unwrap();
        assert_eq!(token_stream.tokens.len(), 4);

        let expected = vec![
            Token::Identifier("a".to_string()),
            Token::Caret,
            Token::Minus,
            Token::Number("1".to_string()),
        ];
        for ((token, _), expected_token) in token_stream.tokens.iter().zip(expected.iter()) {
            assert_eq!(
                token, expected_token,
                "Token mismatch at position {:?}",
                token
            );
        }
    }

    #[test]
    fn test_negative_number_at_beginning() {
        let input = r#"-2"#;

        let token_stream = TokenStream::from_str(input).unwrap();
        let expected = vec![Token::Minus, Token::Number("2".to_string())];

        assert!(token_streams_eq(&token_stream.tokens, &expected));
    }

    #[test]
    fn test_minus_sign_at_beginning() {
        let input = r#"-a"#;

        let token_stream = TokenStream::from_str(input).unwrap();
        let expected = vec![Token::Minus, Token::Identifier("a".to_string())];

        assert!(token_streams_eq(&token_stream.tokens, &expected));
    }

    #[test]
    fn test_exponent_does_not_swallow_next_token() {
        let input = "f(1e2)";
        let token_stream = TokenStream::from_str(input).unwrap();

        let expected = vec![
            Token::Identifier("f".to_string()),
            Token::LeftParen,
            Token::Number("1e2".to_string()),
            Token::RightParen,
        ];

        assert!(
            token_streams_eq(&token_stream.tokens, &expected),
            "Expected tokens {:?}, got {:?}",
            expected,
            token_stream
                .tokens
                .iter()
                .map(|(t, _)| t)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_exponent_does_not_swallow_following_operator() {
        let input = "1e2+3";
        let token_stream = TokenStream::from_str(input).unwrap();

        let expected = vec![
            Token::Number("1e2".to_string()),
            Token::Plus,
            Token::Number("3".to_string()),
        ];

        assert!(
            token_streams_eq(&token_stream.tokens, &expected),
            "Expected tokens {:?}, got {:?}",
            expected,
            token_stream
                .tokens
                .iter()
                .map(|(t, _)| t)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_unterminated_string_literal_errors() {
        let input = r#""hello"#;

        let err = TokenStream::from_str(input).unwrap_err();
        assert!(
            err.message.to_lowercase().contains("unterminated"),
            "Expected unterminated error, got: {:?}",
            err
        );
    }

    #[test]
    fn test_unterminated_code_fence_errors() {
        let input = "```python\nprint(1)\n";

        let err = TokenStream::from_str(input).unwrap_err();
        assert!(err.message.to_lowercase().contains("unterminated"),);
    }

    #[test]
    fn test_operator_position_points_to_operator_start_single_char() {
        let input = "a+b";
        let token_stream = TokenStream::from_str(input).unwrap();

        assert_eq!(token_stream.tokens.len(), 3);

        let plus_pos = &token_stream.tokens[1].1;
        assert_eq!((plus_pos.line, plus_pos.column), (1, 2),);
    }

    #[test]
    fn test_operator_position_points_to_operator_start_double_char() {
        let input = "a==b";
        let token_stream = TokenStream::from_str(input).unwrap();

        assert_eq!(token_stream.tokens.len(), 3);

        let eqeq_pos = &token_stream.tokens[1].1;
        assert_eq!((eqeq_pos.line, eqeq_pos.column), (1, 2),);
    }

    #[test]
    fn test_operator_position_star() {
        let ts = TokenStream::from_str("a*b").unwrap();
        let star_pos = &ts.tokens[1].1;
        assert_eq!((star_pos.line, star_pos.column), (1, 2));
    }

    #[test]
    fn test_operator_position_left_paren() {
        let ts = TokenStream::from_str("f(x)").unwrap();
        let lparen_pos = &ts.tokens[1].1;
        assert_eq!((lparen_pos.line, lparen_pos.column), (1, 2));
    }

    #[test]
    fn test_string_literal_position_points_to_opening_quote() {
        let ts = TokenStream::from_str(r#""x""#).unwrap();
        assert_eq!(ts.tokens.len(), 1);

        let (_tok, pos) = &ts.tokens[0];
        assert_eq!((pos.line, pos.column), (1, 1));
    }
}

#[test]
fn test_pattern_tokens() {
    let cases = vec![
        (
            "x_",
            Token::Pattern {
                quantity: Quantity::One,
                prefix: Some("x".into()),
                postfix: None,
                optional: false,
            },
        ),
        (
            "x_Integer",
            Token::Pattern {
                quantity: Quantity::One,
                prefix: Some("x".into()),
                postfix: Some("Integer".into()),
                optional: false,
            },
        ),
        (
            "x_.",
            Token::Pattern {
                quantity: Quantity::One,
                prefix: Some("x".into()),
                postfix: None,
                optional: true,
            },
        ),
        (
            "x__",
            Token::Pattern {
                quantity: Quantity::Many(1),
                prefix: Some("x".into()),
                postfix: None,
                optional: false,
            },
        ),
        (
            "_",
            Token::Pattern {
                quantity: Quantity::One,
                prefix: None,
                postfix: None,
                optional: false,
            },
        ),
    ];

    for (input, expected) in cases {
        let ts = TokenStream::from_str(input).unwrap();
        assert_eq!(ts.tokens.len(), 1);
        assert_eq!(ts.tokens[0].0, expected, "failed on input: {input}");
    }
}
