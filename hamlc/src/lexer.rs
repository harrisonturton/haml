use std::{fmt::Display, str::Chars};

use crate::token::{Token, TokenKind};

pub const EOF_CHAR: char = '\0';

/// Turns strings into tokens.
#[derive(Debug)]
pub struct Lexer<'a> {
    pos: usize,
    len_remaining: usize,
    chars: Chars<'a>,
    input: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            pos: 0,
            len_remaining: input.len(),
            chars: input.chars(),
            input,
        }
    }

    pub fn advance(&mut self) -> TokenResult<Token> {
        loop {
            let ch = match self.bump() {
                Some(ch) => ch,
                None => return Ok(Token::eof()),
            };

            let kind = match ch {
                ch if is_whitespace(ch) => {
                    self.reset_pos_within_token();
                    self.pos = self.pos + 1;
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
                '@' => TokenKind::At,
                '.' => TokenKind::Period,
                '?' => TokenKind::QuestionMark,
                '"' => self.string_literal()?,
                '0'..='9' => self.number_literal(),
                ch if is_id_head(ch) => self.ident_or_keyword(),
                _ => return Err(TokenError::UnknownToken(ch)),
            };

            let token = Token::new(kind, self.pos_within_token());
            self.reset_pos_within_token();
            self.pos = self.pos + token.len;
            return Ok(token);
        }
    }

    /// Consume a series of characters into a string token
    fn string_literal(&mut self) -> TokenResult<TokenKind> {
        while let Some(ch) = self.bump() {
            if ch == '"' {
                return Ok(TokenKind::StringLiteral);
            }
        }
        Err(TokenError::UnterminatedString)
    }

    /// Consume a series of characters into a integer or float token
    fn number_literal(&mut self) -> TokenKind {
        self.eat_while(is_digit);
        match self.first() {
            '.' => {
                self.bump();
                self.eat_while(is_digit);
                TokenKind::FloatLiteral
            }
            EOF_CHAR => TokenKind::Eof,
            _ => TokenKind::IntLiteral,
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

    /// Peek the next value from the input stream without consuming it. If the
    /// requested character doesn't exist, `EOF_CHAR` is returned. However
    /// getting `EOF_CHAR` doesn't always mean it's reached the end of input,
    /// this should be checked with `is_eof`.
    fn first(&mut self) -> char {
        // `next` optimises better than `.nth(0)`
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    /// Checks if there is nothing more to consume.
    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
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
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }
}

/// Check if `ch` is Ascii whitespace or a control character
fn is_whitespace(ch: char) -> bool {
    ch.is_ascii_whitespace() || ch.is_ascii_control()
}

/// Check if `ch` is a number between 0 and 9
fn is_digit(ch: char) -> bool {
    match ch {
        '0'..='9' => true,
        _ => false,
    }
}

/// Check if `ch` is a valid first letter of an identifier
fn is_id_head(ch: char) -> bool {
    ch.is_ascii_alphabetic() && !is_whitespace(ch)
}

// Check if `ch` is a valid nth letter of an identifier
fn is_id_body(ch: char) -> bool {
    ch.is_ascii_alphanumeric() && !is_whitespace(ch)
}

pub type TokenResult<T> = Result<T, TokenError>;

#[derive(Debug)]
pub enum TokenError {
    UnknownToken(char),
    UnterminatedString,
}

impl Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenError::UnknownToken(ch) => write!(f, "Unknown token '{}'", ch),
            TokenError::UnterminatedString => write!(f, "Unterminated string"),
        }
    }
}

impl std::error::Error for TokenError {}
