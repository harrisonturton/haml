use std::str::Chars;

use crate::diagnostics::DiagnosticEmitter;
use crate::queries::SourceFile;
use crate::span::Span;
use crate::Db;

use super::token::TokenKind;
use super::Token;

/// Turns strings into tokens.
pub struct Lexer<'i> {
    pos: usize,
    len_remaining: usize,
    chars: Chars<'i>,
    file: SourceFile,
    diagnostics: DiagnosticEmitter<'i>,
    db: &'i dyn Db,
}

impl<'i> Lexer<'i> {
    pub fn new(diagnostics: DiagnosticEmitter<'i>, db: &'i dyn Db, file: SourceFile) -> Lexer<'i> {
        let text = file.text(db);
        Lexer {
            pos: 0,
            len_remaining: text.len(),
            chars: text.chars(),
            file,
            db,
            diagnostics,
        }
    }

    pub fn advance(&mut self) -> Option<Token> {
        loop {
            let ch = match self.bump() {
                Some(ch) => ch,
                None => return None,
            };

            let token = match ch {
                ch if is_whitespace(ch) => {
                    self.reset_pos_within_token();
                    self.pos += 1;
                    continue;
                }
                '(' => self.get_token_and_reset(TokenKind::OpenParen),
                ')' => self.get_token_and_reset(TokenKind::CloseParen),
                '{' => self.get_token_and_reset(TokenKind::OpenBrace),
                '}' => self.get_token_and_reset(TokenKind::CloseBrace),
                '<' => self.get_token_and_reset(TokenKind::OpenChevron),
                '>' => self.get_token_and_reset(TokenKind::CloseChevron),
                ':' => self.get_token_and_reset(TokenKind::Colon),
                ';' => self.get_token_and_reset(TokenKind::Semi),
                ',' => self.get_token_and_reset(TokenKind::Comma),
                '@' => self.get_token_and_reset(TokenKind::At),
                '.' => self.get_token_and_reset(TokenKind::Period),
                '?' => self.get_token_and_reset(TokenKind::QuestionMark),
                '"' => self.string_literal()?,
                '0'..='9' => self.number_literal()?,
                ch if is_id_head(ch) => self.ident_or_keyword(),
                _ => self.get_token_and_reset(TokenKind::Invalid),
            };

            return Some(token);
        }
    }

    /// Consume a series of characters into a string token
    fn string_literal(&mut self) -> Option<Token> {
        while let Some(ch) = self.bump() {
            if ch == '"' {
                let token = self.get_token_and_reset(TokenKind::StringLiteral);
                return Some(token);
            }
        }
        let token = self.get_token_and_reset(TokenKind::StringLiteral);
        self.diagnostics.emit_unterminated_string(token);
        None
    }

    /// Consume a series of characters into a integer or float token
    fn number_literal(&mut self) -> Option<Token> {
        let terminated = self.eat_while(is_digit);
        if terminated {
            let token = self.get_token_and_reset(TokenKind::IntLiteral);
            self.diagnostics.emit_unexpected_eof(token);
            return None;
        }

        let ch = match self.peek() {
            Some(ch) => ch,
            None => {
                let token = self.get_token_and_reset(TokenKind::IntLiteral);
                self.diagnostics.emit_unexpected_eof(token);
                return None;
            }
        };

        if ch == '.' {
            self.bump();
            self.eat_while(is_digit);
            let token = self.get_token_and_reset(TokenKind::FloatLiteral);
            return Some(token);
        }

        let token = self.get_token_and_reset(TokenKind::IntLiteral);
        Some(token)
    }

    /// Consume a series of characters into an identifier token
    fn ident_or_keyword(&mut self) -> Token {
        self.eat_while(is_id_body);

        let start = self.pos;
        let end = start + self.pos_within_token();
        let text = self.file.text(self.db);
        let span = &text[self.pos..end];

        let kind = match span {
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

        self.get_token_and_reset(kind)
    }

    fn get_token_and_reset(&mut self, kind: TokenKind) -> Token {
        let start = self.pos;
        let len = self.pos_within_token();
        self.reset_pos_within_token();
        self.pos += len;
        Token::new(kind, Span::new(start, len, self.file))
    }

    fn peek(&mut self) -> Option<char> {
        // `next` optimises better than `.nth(0)`
        self.chars.clone().next()
    }

    /// Returns the number of already consumed symbols.
    fn pos_within_token(&self) -> usize {
        self.len_remaining - self.chars.as_str().len()
    }

    /// Resets the number of bytes consumed to be `0`.
    fn reset_pos_within_token(&mut self) {
        self.len_remaining = self.chars.as_str().len();
    }

    /// Move to the next character.
    fn bump(&mut self) -> Option<char> {
        self.chars.next()
    }

    /// Eat symbols while the predicate returns true or until end of file is
    /// reached. Returns true if it terminates, false if it reaches EOF.
    fn eat_while(&mut self, predicate: impl Fn(char) -> bool) -> bool {
        loop {
            match self.peek() {
                Some(ch) if predicate(ch) => self.bump(),
                None => return false,
                _ => break,
            };
        }
        true
    }
}

/// Check if `ch` is Ascii whitespace or a control character
fn is_whitespace(ch: char) -> bool {
    ch.is_ascii_whitespace() || ch.is_ascii_control()
}

/// Check if `ch` is a number between 0 and 9
fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

/// Check if `ch` is a valid first letter of an identifier
fn is_id_head(ch: char) -> bool {
    ch.is_ascii_alphabetic() && !is_whitespace(ch)
}

// Check if `ch` is a valid nth letter of an identifier
fn is_id_body(ch: char) -> bool {
    ch.is_ascii_alphanumeric() && !is_whitespace(ch)
}
