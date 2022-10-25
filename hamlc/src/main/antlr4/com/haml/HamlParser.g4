parser grammar HamlParser;

options {
  tokenVocab = HamlLexer;
}

program
    : (statement)* EOF
    ;

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
    : selectiveImport
    | completeImport
    ;

selectiveImport
    : IMPORT OPEN_BRACE (IDENTIFIER (COMMA IDENTIFIER)*) CLOSE_BRACE FROM STRING SEMICOLON
    ;

completeImport
    : IMPORT STRING SEMICOLON
    ;

ruleDeclaration
    : RULE IDENTIFIER OPEN_BRACE propertyDeclaration* CLOSE_BRACE
    ;

propertyDeclaration
    : IDENTIFIER COLON expression SEMICOLON
    ;

expression
    : type
    | STRING
    | IDENTIFIER
    | object
    ;

type
    : TYPE (OPEN_BRACKET CLOSE_BRACKET)?
    ;

object
    : OPEN_BRACE propertyDeclaration* CLOSE_BRACE
    ;