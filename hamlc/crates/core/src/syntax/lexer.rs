use std::str::Chars;

use super::token::TokenKind;
use super::Token;
use crate::error::SyntaxError;

/// Turns strings into tokens.
#[derive(Debug)]
pub struct Lexer<'i> {
    pos: usize,
    len_remaining: usize,
    chars: Chars<'i>,
    input: &'i str,
}

impl<'i> Lexer<'i> {
    pub fn new(input: &'i str) -> Lexer<'i> {
        Lexer {
            pos: 0,
            len_remaining: input.len(),
            chars: input.chars(),
            input,
        }
    }

    pub fn advance(&mut self) -> Result<Option<Token>, SyntaxError> {
        loop {
            let ch = match self.bump() {
                Some(ch) => ch,
                None => return Ok(None),
            };

            let kind = match ch {
                ch if is_whitespace(ch) => {
                    self.reset_pos_within_token();
                    self.pos += 1;
                    continue;
                }
                '(' => TokenKind::OpenParen,
                ')' => TokenKind::CloseParen,
                '{' => TokenKind::OpenBrace,
                '}' => TokenKind::CloseBrace,
                '<' => TokenKind::OpenChevron,
                '>' => TokenKind::CloseChevron,
                ':' => TokenKind::Colon,
                ';' => TokenKind::Semi,
                ',' => TokenKind::Comma,
                '/' => self.comment()?,
                '@' => TokenKind::At,
                '.' => TokenKind::Period,
                '?' => TokenKind::QuestionMark,
                '"' => self.string_literal()?,
                '0'..='9' => self.number_literal()?,
                ch if is_id_head(ch) => self.ident_or_keyword(),
                _ => return Err(SyntaxError::UnknownToken(ch)),
            };

            let start = self.pos;
            let len = self.pos_within_token();
            let token = Token::new(kind, start, len);

            self.reset_pos_within_token();
            self.pos += len;

            return Ok(Some(token));
        }
    }

    fn comment(&mut self) -> Result<TokenKind, SyntaxError> {
        let ch = self.peek();
        match ch {
            Some('/') => self.single_line_comment(),
            Some('*') => self.multi_line_comment(),
            Some(ch) => Err(SyntaxError::UnknownToken(ch)),
            None => Err(SyntaxError::UnexpectedEof),
        }
    }

    fn single_line_comment(&mut self) -> Result<TokenKind, SyntaxError> {
        self.bump();
        self.eat_while(is_not_newline);
        Ok(TokenKind::Comment)
    }

    fn multi_line_comment(&mut self) -> Result<TokenKind, SyntaxError> {
        self.bump();
        loop {
            match self.peek() {
                Some('*') => {
                    self.bump();
                    if self.peek() == Some('/') {
                        self.bump();
                        return Ok(TokenKind::Comment);
                    }
                }
                None => return Err(SyntaxError::UnterminatedComment),
                _ => {
                    self.bump();
                }
            }
        }
    }

    /// Consume a series of characters into a string token
    fn string_literal(&mut self) -> Result<TokenKind, SyntaxError> {
        while let Some(ch) = self.bump() {
            if ch == '"' {
                return Ok(TokenKind::StringLiteral);
            }
        }
        Err(SyntaxError::UnterminatedString)
    }

    /// Consume a series of characters into a integer or float token
    fn number_literal(&mut self) -> Result<TokenKind, SyntaxError> {
        self.eat_while(is_digit);
        let ch = self.peek().ok_or(SyntaxError::UnexpectedEof)?;
        match ch {
            '.' => {
                self.bump();
                self.eat_while(is_digit);
                Ok(TokenKind::FloatLiteral)
            }
            _ => Ok(TokenKind::IntLiteral),
        }
    }

    /// Consume a series of characters into an identifier token
    fn ident_or_keyword(&mut self) -> TokenKind {
        self.eat_while(is_id_body);

        let start = self.pos;
        let end = start + self.pos_within_token();
        let span = &self.input[self.pos..end];

        match span {
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
        }
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
    /// reached.
    fn eat_while(&mut self, predicate: impl Fn(char) -> bool) {
        while let Some(ch) = self.peek() {
            if predicate(ch) {
                self.bump();
            } else {
                break;
            }
        }
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

fn is_not_newline(ch: char) -> bool {
    ch != '\n'
}
