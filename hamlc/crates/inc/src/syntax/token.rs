use crate::span::Span;
use derive_new::new;
use std::fmt::Display;

#[derive(new, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum TokenKind {
    // package
    Package,
    // import
    Import,
    // constructor
    Constructor,
    // annotation
    Annotation,
    // struct
    Struct,
    // map
    Map,
    // unknown
    Unknown,
    // union
    Union,
    // repeatable
    Repeatable,
    // tagged
    Tagged,
    // uint32
    Uint32,
    // uint64
    Uint64,
    // int32
    Int32,
    // int64
    Int64,
    // float32
    Float32,
    // float64
    Float64,
    // string
    String,
    // "the quick brown fox jumped over the lazy dog"
    StringLiteral,
    // 1231
    IntLiteral,
    // 143.92
    FloatLiteral,
    // An identifier like "GetUserRequest"
    Ident,
    // (
    OpenParen,
    // )
    CloseParen,
    // {
    OpenBrace,
    // }
    CloseBrace,
    // <
    OpenChevron,
    // >
    CloseChevron,
    // :
    Colon,
    // ?
    QuestionMark,
    // @
    At,
    // ;
    Semi,
    // ,
    Comma,
    // Period
    Period,
    // Any charactor in the wrong place, or bad character
    Invalid,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TokenKind::Package => "package",
            TokenKind::Import => "import",
            TokenKind::Constructor => "constructor",
            TokenKind::Annotation => "annotation",
            TokenKind::Struct => "struct",
            TokenKind::Map => "map",
            TokenKind::Unknown => "unknown",
            TokenKind::Union => "union",
            TokenKind::Repeatable => "repeatable",
            TokenKind::Tagged => "tagged",
            TokenKind::Uint32 => "uint32",
            TokenKind::Uint64 => "uint64",
            TokenKind::Int32 => "int32",
            TokenKind::Int64 => "int64",
            TokenKind::Float32 => "float32",
            TokenKind::Float64 => "float64",
            TokenKind::String => "string",
            TokenKind::StringLiteral => "string literal",
            TokenKind::IntLiteral => "int literal",
            TokenKind::FloatLiteral => "float literal",
            TokenKind::Ident => "identifier",
            TokenKind::OpenParen => "(",
            TokenKind::CloseParen => ")",
            TokenKind::OpenBrace => "{",
            TokenKind::CloseBrace => "}",
            TokenKind::OpenChevron => "<",
            TokenKind::CloseChevron => ">",
            TokenKind::Colon => ":",
            TokenKind::QuestionMark => "?",
            TokenKind::At => "@",
            TokenKind::Semi => ";",
            TokenKind::Comma => ",",
            TokenKind::Period => ".",
            TokenKind::Invalid => "invalid token",
        };
        write!(f, "{str}")
    }
}
