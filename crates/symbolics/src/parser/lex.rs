use std::{collections::VecDeque, str::FromStr};

use crate::parser::error::LexError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenPos {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // Double-character operators
    EqEq,      // '=='
    NotEq,     // '!='
    LesserEq,  // '<='
    GreaterEq, // '>='
    ColonEq,   // ':='

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
    LessThan,     // '<'
    GreaterThan,  // '>'

    // Multi-character tokens
    Number(String),
    Identifier(String),
    StringLiteral(String),
    CodeBlock { language: String, code: String },
}

impl Token {
    fn is_operator(&self) -> bool {
        use Token::*;
        match self {
            Number(_) | Identifier(_) | StringLiteral(_) | CodeBlock { .. } => false,
            _ => true,
        }
    }
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

    fn skip(&mut self, n: usize) {
        for _ in 0..n {
            self.next();
        }
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

        let mut sign_string = String::new();
        let mut base_string = String::new();
        let mut exp_sign_string = String::new();
        let mut exponent_string = String::new();

        if let Some('+') | Some('-') = iter.peek() {
            sign_string.push(iter.next().unwrap());
        }

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

            while let Some(c) = iter.next() {
                if !c.is_ascii_digit() {
                    break;
                }

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

        let number_string = if !exp_sign_string.is_empty() {
            format!(
                "{}{}e{}{}",
                sign_string, base_string, exp_sign_string, exponent_string
            )
        } else {
            format!("{}{}", sign_string, base_string)
        };

        tokens.push((Token::Number(number_string), pos));

        Ok(())
    }

    fn consume_identifier_chars(iter: &mut CharIterator) -> Result<(String, TokenPos), LexError> {
        let pos = iter.pos();
        let mut identifier_string = String::new();

        if let Some(c) = iter.next() {
            if c.is_alphabetic() || c == '_' {
                identifier_string.push(c);
            } else {
                return Err(LexError {
                    message: "Invalid identifier: Must start with an alphabetic character or '_'"
                        .to_string(),
                    line: iter.line,
                    column: iter.column,
                });
            }
        }

        while let Some(c) = iter.peek() {
            if c.is_alphanumeric() || *c == '_' {
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
        let c = iter.peek().cloned();

        if c.is_none() {
            return false; // No character to consume
        }

        if c == Some('=') {
            iter.next(); // Consume '='

            if iter.peek().cloned() == Some('=') {
                iter.next(); // Consume second '='
                tokens.push((Token::EqEq, iter.pos()));
            } else {
                tokens.push((Token::Equals, iter.pos()));
            }
        } else if c == Some('!') {
            iter.next(); // Consume '!'

            if iter.peek().cloned() == Some('=') {
                iter.next(); // Consume '='
                tokens.push((Token::NotEq, iter.pos()));
            } else {
                tokens.push((Token::Exclamation, iter.pos()));
            }
        } else if c == Some('<') {
            iter.next(); // Consume '<'

            if iter.peek().cloned() == Some('=') {
                iter.next(); // Consume '='
                tokens.push((Token::LesserEq, iter.pos()));
            } else {
                tokens.push((Token::LessThan, iter.pos()));
            }
        } else if c == Some('>') {
            iter.next(); // Consume '>'

            if iter.peek().cloned() == Some('=') {
                iter.next(); // Consume '='
                tokens.push((Token::GreaterEq, iter.pos()));
            } else {
                tokens.push((Token::GreaterThan, iter.pos()));
            }
        } else if c == Some(':') {
            iter.next(); // Consume ':'

            if iter.peek().cloned() == Some('=') {
                iter.next(); // Consume '='
                tokens.push((Token::ColonEq, iter.pos()));
            } else {
                tokens.push((Token::Colon, iter.pos()));
            }
        } else {
            match c {
                Some('{') => tokens.push((Token::LeftBrace, iter.pos())),
                Some('}') => tokens.push((Token::RightBrace, iter.pos())),
                Some('[') => tokens.push((Token::LeftBracket, iter.pos())),
                Some(']') => tokens.push((Token::RightBracket, iter.pos())),
                Some('(') => tokens.push((Token::LeftParen, iter.pos())),
                Some(')') => tokens.push((Token::RightParen, iter.pos())),
                Some(',') => tokens.push((Token::Comma, iter.pos())),
                Some(';') => tokens.push((Token::Semicolon, iter.pos())),
                Some('+') => tokens.push((Token::Plus, iter.pos())),
                Some('-') => tokens.push((Token::Minus, iter.pos())),
                Some('*') => tokens.push((Token::Asterisk, iter.pos())),
                Some('/') => tokens.push((Token::Slash, iter.pos())),
                Some('%') => tokens.push((Token::Percent, iter.pos())),
                Some('&') => tokens.push((Token::Ampersand, iter.pos())),
                Some('|') => tokens.push((Token::Pipe, iter.pos())),
                Some('^') => tokens.push((Token::Caret, iter.pos())),
                Some('~') => tokens.push((Token::Tilde, iter.pos())),
                _ => return false, // Not an operator character
            }

            iter.next(); // Consume the operator character
        }

        true
    }

    fn consume_string_literal(
        term_sequence: &str,
        iter: &mut CharIterator,
    ) -> Result<(String, TokenPos), LexError> {
        let mut string_literal = String::new();
        let start_pos = iter.pos();

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
                    break; // End of string literal
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

        Ok((string_literal, start_pos))
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

    pub fn next_if_identifier(&mut self) -> Option<&str> {
        let token = self.next_if_matches(|t| matches!(t, Token::Identifier(_)))?;

        if let Token::Identifier(name) = token {
            Some(name.as_str())
        } else {
            None
        }
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
                iter.next();
                let (string, pos) = Self::consume_string_literal("\"", &mut iter)?;
                tokens.push((Token::StringLiteral(string), pos));
            } else if c == '`' {
                for _ in 0..3 {
                    if iter.next() != Some('`') {
                        return Err(LexError {
                            message: "Unterminated string literal".to_string(),
                            line: iter.line,
                            column: iter.column,
                        });
                    }
                }
                let (language, pos) = Self::consume_identifier_chars(&mut iter)?;
                let (code, _) = Self::consume_string_literal("```", &mut iter)?;

                tokens.push((Token::CodeBlock { language, code }, pos));
            } else if c.is_whitespace() {
                iter.next(); // Consume whitespace
            } else if c == '-' || c == '+' {
                if c == '-' && iter.peek() == Some(&'>') {
                    tokens.push((Token::Minus, iter.pos()));
                    iter.skip(2); // Consume '-' and '>'
                } else if c == '+' && iter.peek() == Some(&'>') {
                    tokens.push((Token::Plus, iter.pos()));
                    iter.skip(2); // Consume '-' and '>'
                }

                if let Some(char_after_minus) = iter.lookahead(1) {
                    let prev_token_is_operator_or_start =
                        tokens.last().map(|(t, _)| t.is_operator()).unwrap_or(true);

                    if char_after_minus.is_ascii_digit() && prev_token_is_operator_or_start {
                        Self::comsume_number_chars(&mut iter, &mut tokens)?;
                    } else if c == '-' {
                        iter.next(); // Consume '-'
                        tokens.push((Token::Minus, iter.pos()));
                    } else if c == '+' {
                        iter.next(); // Consume '+'
                        tokens.push((Token::Plus, iter.pos()));
                    } else {
                        unreachable!(
                            "Sign should have been consumed at least by previous two branches."
                        )
                    }
                }
            } else if Self::consume_operator_chars(&mut iter, &mut tokens) {
                continue;
            } else if c.is_ascii_digit() {
                Self::comsume_number_chars(&mut iter, &mut tokens)?;
            } else if c.is_alphabetic() || c == '_' {
                let (identifer, pos) = Self::consume_identifier_chars(&mut iter)?;
                tokens.push((Token::Identifier(identifer), pos));
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
        assert_eq!(token_stream.tokens.len(), 3);

        let expected = vec![
            Token::Identifier("a".to_string()),
            Token::Caret,
            Token::Number("-1".to_string()),
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
        let expected = vec![Token::Number("-2".to_string())];

        assert!(token_streams_eq(&token_stream.tokens, &expected));
    }

    #[test]
    fn test_minus_sign_at_beginning() {
        let input = r#"-a"#;

        let token_stream = TokenStream::from_str(input).unwrap();
        let expected = vec![Token::Minus, Token::Identifier("a".to_string())];

        assert!(token_streams_eq(&token_stream.tokens, &expected));
    }
}
