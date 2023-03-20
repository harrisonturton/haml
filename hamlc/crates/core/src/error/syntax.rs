use crate::syntax::Token;
use std::error::Error;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SyntaxError {
    UnexpectedToken(Token, String),
    UnexpectedEof,
    UnknownToken(char),
    UnterminatedString,
    UnterminatedComment,
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxError::UnexpectedToken(token, msg) => write!(
                f,
                "Unexpected token: expected {msg} but found {:?}",
                token.kind
            ),
            SyntaxError::UnexpectedEof => write!(f, "Unexpected EOF"),
            SyntaxError::UnknownToken(ch) => write!(f, "Unknown token '{ch}'"),
            SyntaxError::UnterminatedString => write!(f, "Unterminated string"),
            SyntaxError::UnterminatedComment => write!(f, "Unterminated comment"),
        }
    }
}

impl Error for SyntaxError {}
