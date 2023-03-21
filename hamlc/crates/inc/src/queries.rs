use std::fs;
use std::path::PathBuf;

use crate::ast::Ast;
use crate::diagnostics::DiagnosticEmitter;
use crate::span::Span;
use crate::syntax::{Lexer, Parser, Token};

#[salsa::input]
pub struct TrackedAst {
    pub ast: Ast,
}

#[salsa::input]
pub struct TrackedSpan {
    pub span: Span,
}

#[salsa::input]
pub struct Path {
    pub path: PathBuf,
}

#[salsa::input]
pub struct SourceFile {
    #[return_ref]
    pub path: PathBuf,
    #[return_ref]
    pub text: String,
}

// Turn a path into a file
#[salsa::tracked]
pub fn read_file(db: &dyn crate::Db, path: Path) -> Option<SourceFile> {
    let path = path.path(db);
    let text = fs::read_to_string(&path).ok()?;
    Some(SourceFile::new(db, path, text))
}

/// Turn a file into an AST
#[salsa::tracked]
pub fn parse_file(db: &dyn crate::Db, file: SourceFile) -> Option<Ast> {
    let diagnostics = DiagnosticEmitter::new(db);
    let mut lexer = Lexer::new(diagnostics, db, file);
    let mut parser = Parser::new(diagnostics, &mut lexer);
    parser.parse()
}

/// Turn a file into a symbol table of declared and imported symbols
#[salsa::tracked]
pub fn build_symbol_table(db: &dyn crate::Db, file: SourceFile) -> Option<Vec<Token>> {
    let _ast = parse_file(db, file)?;
    // Iterate through imports and build their symbol tables. While walking, and
    // combining symbols from all tables, detect duplicate symbols.
    todo!()
}

/// Turn a span into a snippet of code
#[salsa::tracked]
pub fn read_span(db: &dyn crate::Db, path: Path, span: TrackedSpan) -> Option<String> {
    let span = span.span(db);
    let file = read_file(db, path)?;
    let text = file.text(db);
    Some(text[span.start..span.start + span.len].to_string())
}
