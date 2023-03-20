use std::fs;
use std::path::PathBuf;

use crate::ast::Ast;
use crate::syntax::{Lexer, Parser};

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

#[salsa::tracked]
pub fn read_file(db: &dyn crate::Db, path: Path) -> Option<SourceFile> {
    let path = path.path(db);
    let text = fs::read_to_string(&path).ok()?;
    Some(SourceFile::new(db, path, text))
}

#[salsa::tracked]
pub fn parse_file(db: &dyn crate::Db, file: SourceFile) -> Option<Ast> {
    let text = file.text(db);
    let mut lexer = Lexer::new(db, text);
    let mut parser = Parser::new(db, &mut lexer);
    parser.parse()
}
