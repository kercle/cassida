use std::fmt;

use crate::lex::{Token, TokenPos};

pub type BoxedError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Clone)]
pub struct LexError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lexical error at line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for LexError {}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub at_token: Option<(Token, TokenPos)>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let locator = if let Some((_, pos)) = &self.at_token {
            format!(" at line {}, column {}", pos.line, pos.column)
        } else {
            String::new()
        };
        write!(f, "Parse error{locator}: {}", self.message)
    }
}

impl std::error::Error for ParseError {}
