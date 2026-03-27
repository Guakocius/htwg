// ExprLexer.g4
lexer grammar ExprLexer;
                    
Number: Digits ('.' Digits)?;

PLUS:   '+';
MINUS:  '-';
MUL:    '*';
DIV:    '/';
LPAREN: '(';
RPAREN: ')';

WS:     [ \t\r\n]+ -> channel(HIDDEN);

InvalidChar: .;

fragment Digits: ([0-9])+;
