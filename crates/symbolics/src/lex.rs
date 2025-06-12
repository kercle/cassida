use crate::error::LexError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenPos {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    // Triple-character tokens

    // Double-character tokens
    EqEq,      // '=='
    NotEq,     // '!='
    LesserEq,  // '<='
    GreaterEq, // '>='
    ColonEq,   // ':='

    // Single-character tokens
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

struct CharIterator<'a> {
    iter: std::iter::Peekable<std::str::Chars<'a>>,
    line: usize,
    column: usize,
}

impl<'a> CharIterator<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            iter: s.chars().peekable(),
            line: 1,
            column: 1,
        }
    }

    fn next(&mut self) -> Option<char> {
        if let Some(c) = self.iter.next() {
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(c)
        } else {
            None
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    fn pos(&self) -> TokenPos {
        TokenPos {
            line: self.line,
            column: self.column,
        }
    }
}

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

    pub fn from_str(data: &str) -> Result<Self, LexError> {
        let mut tokens = vec![];

        let mut iter = CharIterator::new(&data);
        while let Some(c) = iter.peek().cloned() {
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
            } else if Self::consume_operator_chars(&mut iter, &mut tokens) {
                continue;
            } else if c.is_whitespace() {
                iter.next(); // Consume whitespace
            } else if c == '-' || c == '+' {
                iter.next(); // Consume the sign

                if c == '-' && iter.peek() == Some(&'>') {
                    tokens.push((Token::Minus, iter.pos()));
                    iter.next(); // Consume '>'
                } else if c == '+' && iter.peek() == Some(&'>') {
                    tokens.push((Token::Plus, iter.pos()));
                    iter.next(); // Consume '>'
                }

                if let Some(next_char) = iter.peek() {
                    if next_char.is_ascii_digit() {
                        Self::comsume_number_chars(&mut iter, &mut tokens)?;
                    } else if c == '-' {
                        tokens.push((Token::Minus, iter.pos()));
                    } else if c == '+' {
                        tokens.push((Token::Plus, iter.pos()));
                    }
                }
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

    pub fn next(&mut self) -> Option<&Token> {
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
            self.next()
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

#[cfg(test)]
mod tests {
    use super::*;

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
            code: "\n            let x = 42;\n            print(\"Hello, world {x}!\")\n        ".to_string(),
        }];
        for ((token, _), expected_token) in token_stream.tokens.iter().zip(expected.iter()) {
            assert_eq!(
                token, expected_token,
                "Token mismatch at position {:?}",
                token
            );
        }
    }
}
