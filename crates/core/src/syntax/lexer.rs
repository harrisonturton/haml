use std::str::Chars;

use super::{parser::ParseSession, Token, TokenKind};
use crate::{diagnostics::Emitter, span::Span};

pub struct Lexer<'db> {
    sess: &'db ParseSession<'db>,
    emitter: &'db dyn Emitter,
    chars: Chars<'db>,
    len_remaining: usize,
}

impl<'db> Lexer<'db> {
    pub fn new(sess: &'db ParseSession<'db>, emitter: &'db dyn Emitter) -> Lexer<'db> {
        Lexer {
            sess,
            emitter,
            chars: sess.text().chars(),
            len_remaining: sess.text().len(),
        }
    }

    pub fn advance(&mut self) -> Option<Token> {
        match self.bump_ignoring_whitespace()? {
            '"' => self.string_literal(),
            '0'..='9' => self.numeric_literal(),
            ch if is_id_head(ch) => self.ident_or_keyword(),
            _ => self.reserved_char(),
        }
    }

    fn string_literal(&mut self) -> Option<Token> {
        self.bump_while(|ch| ch != '"');

        if let None = self.peek() {
            let token = self.eat_and_advance(TokenKind::StringLiteral);
            self.emitter.emit_unterminated_string(token);
            return None;
        }

        self.bump();
        let token = self.eat_and_advance(TokenKind::StringLiteral);
        Some(token)
    }

    fn ident_or_keyword(&mut self) -> Option<Token> {
        self.bump_while(is_id_body);

        let span = self.span();
        let text = self.sess.text();
        let span_text = &text[span.start..span.end];

        let kind = match span_text {
            "import" => TokenKind::Import,
            "package" => TokenKind::Package,
            "constructor" => TokenKind::Constructor,
            "annotation" => TokenKind::Annotation,
            "struct" => TokenKind::Struct,
            "map" => TokenKind::Map,
            "unknown" => TokenKind::Unknown,
            "union" => TokenKind::Union,
            "repeatable" => TokenKind::Repeatable,
            "tagged" => TokenKind::Tagged,
            "uint32" => TokenKind::Uint32,
            "uint64" => TokenKind::Uint64,
            "int32" => TokenKind::Int32,
            "int64" => TokenKind::Int64,
            "float32" => TokenKind::Float32,
            "float64" => TokenKind::Float64,
            "string" => TokenKind::String,
            _ => TokenKind::Ident,
        };

        let token = self.eat_and_advance(kind);
        Some(token)
    }

    fn numeric_literal(&mut self) -> Option<Token> {
        self.bump_while(is_digit);

        if let Some('.') = self.peek() {
            self.bump();
            self.bump_while(is_digit);
            let token = self.eat_and_advance(TokenKind::FloatLiteral);
            return Some(token);
        }

        let token = self.eat_and_advance(TokenKind::IntLiteral);
        Some(token)
    }

    fn reserved_char(&mut self) -> Option<Token> {
        let span = self.span();
        let text = self.sess.text();
        let span_text = &text[span.start..span.end];

        let kind = match span_text {
            "(" => TokenKind::OpenParen,
            ")" => TokenKind::CloseParen,
            "{" => TokenKind::OpenBrace,
            "}" => TokenKind::CloseBrace,
            "<" => TokenKind::OpenChevron,
            ">" => TokenKind::CloseChevron,
            ":" => TokenKind::Colon,
            ";" => TokenKind::Semi,
            "," => TokenKind::Comma,
            "@" => TokenKind::At,
            "." => TokenKind::Period,
            "?" => TokenKind::QuestionMark,
            _ => TokenKind::Invalid,
        };

        Some(self.eat_and_advance(kind))
    }

    fn eat_and_advance(&mut self, kind: TokenKind) -> Token {
        let span = self.span();
        self.eat();
        Token::new(kind, span)
    }

    fn eat(&mut self) {
        self.len_remaining = self.chars.as_str().len();
    }

    fn span(&self) -> Span {
        let text_len = self.sess.text().len();
        let start = text_len - self.len_remaining;
        let end = text_len - self.chars.as_str().len();
        Span::new(start, end, self.sess.file)
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn bump_ignoring_whitespace(&mut self) -> Option<char> {
        while let Some(ch) = self.bump() {
            if is_whitespace(ch) {
                self.eat();
                continue;
            }
            return Some(ch);
        }
        None
    }

    fn bump_while(&mut self, predicate: impl Fn(char) -> bool) {
        while self.peek().is_some_and(&predicate) {
            self.bump();
        }
    }
}

// Check if `ch` is a number between 0 and 9
fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

// Check if `ch` is a valid first letter of an identifier
fn is_id_head(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

// Check if `ch` is a valid nth letter of an identifier
fn is_id_body(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

fn is_whitespace(ch: char) -> bool {
    ch.is_ascii_whitespace()
}
