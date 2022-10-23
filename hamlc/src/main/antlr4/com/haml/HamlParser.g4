parser grammar HamlParser;

options {
  tokenVocab = HamlLexer;
}

program: (statement)* EOF ;

statement
    : comment
    | importStatement
    | declaration
    ;

comment
    : START_SLASH_COMMENT SLASH_COMMENT
    | BLOCK_COMMENT
    ;

declaration
    : importStatement
    | ruleDeclaration
    ;

importStatement
    : singleImportDeclaration
    | multipleImportDeclaration
    ;

singleImportDeclaration
    : IMPORT STRING SEMICOLON;

multipleImportDeclaration
    : IMPORT OPEN_BRACE (STRING SEMICOLON)* CLOSE_BRACE;

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

type: TYPE (OPEN_BRACKET CLOSE_BRACKET)?;

object
    : OPEN_BRACE propertyDeclaration* CLOSE_BRACE;