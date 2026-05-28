lexer grammar AbfahrtenLexer;

HRS : ([0-1]?[0-9]|2[0-3]);
MINS : [0-5][0-9];
TIME : HRS':'MINS;

KW_BUS : ('Bus'|'KAT'|'RE');
DIGIT_BUS : [0-9];
NUM_BUS : DIGIT_BUS(DIGIT_BUS)*;
DAYS : [Mo-Fr] - [Mo-Fr];
ROUTINE : (DAYS|'nicht'? 'täglich');

WS: [ \t\n\r\f]+ -> channel(HIDDEN);
