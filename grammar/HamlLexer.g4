lexer grammar HamlLexer;

OPEN_BRACE: '{';
CLOSE_BRACE: '}';
OPEN_PAREN: '(';
CLOSE_PAREN: ')';
OPEN_BRACKET: '[';
CLOSE_BRACKET: ']';
EQ: '=';
NEWLINE: '\n';
QUESTION: '?';
SEMICOLON: ';';
COLON: ':';
STRUCT: 'struct';
RULE: 'rule';
STRING: '"' ~('"')*? '"';
ENUM: 'enum';
COMMA: ',';
TYPE: 'string' | 'number';
INTERFACE: 'interface';
IDENTIFIER: [a-z]+;
KEYWORD: [a-z]+[a-zA-Z0-9_]*;

// Skip whitespace when lexing.
// This means that the grammar must not be whitespace sensitive.
WS: [ \t\r]+ -> skip ;