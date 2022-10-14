parser grammar HamlParser;

options {
  tokenVocab = HamlLexer;
}

program: (decl)* EOF ;

decl
  : structDecl
  | enumDecl
  | interfaceDecl
  ;

// This parser rule is required to support identifiers that are the same as a
// keyword. For example, allowing "type" to be both a keyword and a field name.
identifier
  : STRUCT
  | TYPE
  | IDENTIFIER
  ;

type
  : TYPE
  | IDENTIFIER
  ;

// --------------------------------------------------
// Struct declaration
// --------------------------------------------------

// struct Foo { ... }
structDecl: STRUCT identifier OPEN_BRACE (fieldDecl)* CLOSE_BRACE ;

// field myField: string;
fieldDecl: identifier COLON type SEMICOLON ;

// --------------------------------------------------
// Enum declaration
// --------------------------------------------------

enumDecl: ENUM identifier OPEN_BRACE (identifier SEMICOLON)* CLOSE_BRACE ;

// --------------------------------------------------
// Class declaration
// --------------------------------------------------

interfaceDecl: INTERFACE identifier OPEN_BRACE (methodDecl)* CLOSE_BRACE ;

// --------------------------------------------------
// Method declaration
// --------------------------------------------------

methodDecl: identifier OPEN_PAREN (argDecl (COMMA argDecl)*)* CLOSE_PAREN (COLON type)? SEMICOLON;

argDecl: identifier COLON type ;