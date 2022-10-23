lexer grammar HamlLexer;

OPEN_BRACE: '{';
CLOSE_BRACE: '}';
OPEN_PAREN: '(';
CLOSE_PAREN: ')';
OPEN_BRACKET: '[';
CLOSE_BRACKET: ']';
FORWARD_SLASH: '/';
STAR: '*';
EQ: '=';
QUESTION: '?';
SEMICOLON: ';';
COLON: ':';
STRUCT: 'struct';
RULE: 'rule';
SPEC: 'spec';
STRING: '"' ~('"')*? '"';
ENUM: 'enum';
COMMA: ',';
IMPORT: 'import';
TYPE: 'string' | 'number';
INTERFACE: 'interface';
IDENTIFIER: [a-zA-Z]+;
KEYWORD: [a-z]+[a-zA-Z0-9_]*;

WS: [ \n\t\r]+ -> skip;

BLOCK_COMMENT: '/*' .*? '*/';

// Must have a different mode for the slash comment since it is sensitive to a
// newline character, which is skipped in the normal lexer mode. It is skipped
// because it makes all the parser rules much simpler.
START_SLASH_COMMENT: '//' -> pushMode(SlashComment);

mode SlashComment;
SLASH_COMMENT: ~('\n')+? ('\n' | EOF) -> popMode;