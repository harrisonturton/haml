use std::path::Path;

use crate::{
    diagnostics::Emitter,
    queries::SourceFile,
    span::Span,
    syntax::{Lexer, ParseSession},
    Db,
};

pub struct TestContext<'db> {
    sess: ParseSession<'db>,
    emitter: MockEmitter,
    file: SourceFile,
}

impl<'db> TestContext<'db> {
    pub fn new(db: &'db dyn Db, text: &str) -> TestContext<'db> {
        let path = Path::new("file.haml").to_path_buf();
        let file = SourceFile::new(db, path, text.to_string());
        let sess = ParseSession::new(db, file);
        let emitter = MockEmitter;
        TestContext {
            sess,
            emitter,
            file,
        }
    }

    pub fn lexer(&self) -> Lexer {
        Lexer::new(&self.sess, &self.emitter)
    }

    pub fn span(&self, start: usize, end: usize) -> Span {
        Span::new(start, end, self.file)
    }
}

pub struct MockEmitter;

impl Emitter for MockEmitter {
    fn emit_message(&self, message: &str) {
        // pass
    }

    fn emit_unexpected_eof(&self, token: crate::syntax::Token) {
        // pass
    }

    fn emit_duplicate_identifier(&self, token: crate::syntax::Token) {
        // pass
    }

    fn emit_unexpected_token(&self, token: crate::syntax::Token, expected: &str) {
        // pass
    }

    fn emit_unterminated_comment(&self, token: crate::syntax::Token) {
        // pass
    }

    fn emit_unterminated_string(&self, token: crate::syntax::Token) {
        // pass
    }
}
