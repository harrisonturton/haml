use crate::syntax::Token;
use std::error::Error;
use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SemanticError {
    DuplicateIdentifier { duplicate: Token },
}

impl Display for SemanticError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SemanticError::DuplicateIdentifier { duplicate } => {
                write!(f, "duplicate identifier: {duplicate:?} already defined")
            }
        }
    }
}

impl Error for SemanticError {}
