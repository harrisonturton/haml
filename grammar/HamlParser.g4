parser grammar HamlParser;

options {
  tokenVocab = HamlLexer;
}

program: (statement)* EOF ;

statement
    : comment
    | specDeclaration
    | declaration
    ;

comment
    : START_SLASH_COMMENT SLASH_COMMENT
    | BLOCK_COMMENT
    ;

declaration
    : specDeclaration
    | ruleDeclaration
    ;

specDeclaration
    : singleSpecDeclaration
    | multipleSpecDeclaration
    ;

singleSpecDeclaration
    : SPEC STRING SEMICOLON;

multipleSpecDeclaration
    : SPEC OPEN_BRACE (STRING SEMICOLON)* CLOSE_BRACE;

ruleDeclaration
    : RULE IDENTIFIER OPEN_BRACE propertyDeclaration* CLOSE_BRACE;

propertyDeclaration
    : IDENTIFIER COLON expression SEMICOLON;

expression
    : type
    | STRING
    | IDENTIFIER
    | object
    ;

type: TYPE (OPEN_BRACE CLOSE_BRACE)?;

object
    : OPEN_BRACE propertyDeclaration* CLOSE_BRACE;