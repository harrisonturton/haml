pub mod format;
mod semantic;
mod syntax;

pub use format::format;
pub use semantic::SemanticError;
pub use syntax::SyntaxError;

#[derive(Debug)]
pub enum GenericError {
    Syntax(SyntaxError),
    Semantic(SemanticError),
}

impl std::fmt::Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenericError::Syntax(err) => err.fmt(f),
            GenericError::Semantic(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for GenericError {}
