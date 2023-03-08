#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

impl Token {
    pub fn new(kind: TokenKind, len: usize) -> Token {
        Token { kind, len }
    }

    pub fn eof() -> Token {
        Token {
            kind: TokenKind::Eof,
            len: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
    Semicolon,
    // ,
    Comma,
    // Period
    Period,
    // Signals end of file
    Eof,
}
