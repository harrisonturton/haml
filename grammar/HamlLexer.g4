lexer grammar HamlLexer;

OPEN_BRACE: '{';
CLOSE_BRACE: '}';
OPEN_PAREN: '(';
CLOSE_PAREN: ')';
EQ: '=';
SEMICOLON: ';';
COLON: ':';
STRUCT: 'struct';
ENUM: 'enum';
COMMA: ',';
TYPE: 'string' | 'number';
INTERFACE: 'interface';
IDENTIFIER: [a-zA-Z]+;

// Skip whitespace when lexing.
// This means that the grammar must not be whitespace sensitive.
WS: [ \t\r\n]+ -> skip ;