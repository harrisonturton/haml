use crate::db::Database;
use crate::syntax::{testing::support::TestContext, Token, TokenKind};

#[test]
fn test_lexes_package_statement() {
    let text = "package one.two";

    let db = Database::default();
    let ctx = TestContext::new(&db, text);
    let mut lexer = ctx.lexer();

    let span = ctx.span(0, 7);
    let token = Token::new(TokenKind::Package, span);
    assert_eq!(lexer.advance(), Some(token));

    let span = ctx.span(8, 11);
    let token = Token::new(TokenKind::Ident, span);
    assert_eq!(lexer.advance(), Some(token));

    let span = ctx.span(11, 12);
    let token = Token::new(TokenKind::Period, span);
    assert_eq!(lexer.advance(), Some(token));

    let span = ctx.span(12, 15);
    let token = Token::new(TokenKind::Ident, span);
    assert_eq!(lexer.advance(), Some(token));
}

#[test]
fn test_lexes_import_statement() {
    let text = "import \"testing\"";

    let db = Database::default();
    let ctx = TestContext::new(&db, text);
    let mut lexer = ctx.lexer();

    let span = ctx.span(0, 6);
    let token = Token::new(TokenKind::Import, span);
    assert_eq!(lexer.advance(), Some(token));

    let span = ctx.span(7, 16);
    let token = Token::new(TokenKind::StringLiteral, span);
    assert_eq!(lexer.advance(), Some(token));
}

#[test]
fn test_lexes_string_literals() {
    let cases = vec!["\"foo\"", "\"foo bar\"", "\"foo bar 1.23 ?\""];

    cases.iter().for_each(|text| {
        let db = Database::default();
        let ctx = TestContext::new(&db, text);
        let mut lexer = ctx.lexer();

        let span = ctx.span(0, text.len());
        let token = Token::new(TokenKind::StringLiteral, span);
        assert_eq!(lexer.advance(), Some(token));
    });
}

#[test]
fn test_lexes_identifiers() {
    let cases = vec!["foo", "foo_bar", "FooBar", "_foo", "_Foo9"];

    cases.iter().for_each(|text| {
        let db = Database::default();
        let ctx = TestContext::new(&db, text);
        let mut lexer = ctx.lexer();

        let span = ctx.span(0, text.len());
        let expected = Token::new(TokenKind::Ident, span);

        let token = lexer.advance();
        assert_eq!(token, Some(expected));
    });
}

#[test]
fn test_lexes_numeric_literals() {
    let cases = vec![
        ("1", TokenKind::IntLiteral),
        ("12", TokenKind::IntLiteral),
        ("123", TokenKind::IntLiteral),
        ("123", TokenKind::IntLiteral),
        ("1.2", TokenKind::FloatLiteral),
        ("1.23", TokenKind::FloatLiteral),
        ("12.34", TokenKind::FloatLiteral),
    ];

    cases.iter().for_each(|(text, kind)| {
        let db = Database::default();
        let ctx = TestContext::new(&db, text);
        let mut lexer = ctx.lexer();

        let span = ctx.span(0, text.len());
        let expected = Token::new(*kind, span);

        let token = lexer.advance();
        assert_eq!(token, Some(expected));
    });
}

#[test]
fn test_lexes_reserved_characters() {
    let cases = vec![
        ("(", TokenKind::OpenParen),
        (")", TokenKind::CloseParen),
        ("{", TokenKind::OpenBrace),
        ("}", TokenKind::CloseBrace),
        ("<", TokenKind::OpenChevron),
        (">", TokenKind::CloseChevron),
        (":", TokenKind::Colon),
        (";", TokenKind::Semi),
        ("@", TokenKind::At),
        (".", TokenKind::Period),
        ("?", TokenKind::QuestionMark),
    ];

    cases.iter().for_each(|(text, kind)| {
        let db = Database::default();
        let ctx = TestContext::new(&db, text);
        let mut lexer = ctx.lexer();

        let span = ctx.span(0, 1);
        let expected = Token::new(*kind, span);

        let token = lexer.advance();
        assert_eq!(token, Some(expected));
    });
}
