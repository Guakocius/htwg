# Übungsblatt 4

## Aufgabe 4.6

### Teilaufgabe 4.6.1

a) w1 = abc
(q0,abc,\#)
(q0,bc,A\#), a,\#;A\#
(q0,c,ϵ), b,A;ϵ
(q1,c,ϵ), Zustandswechsel ϵ,ϵ;ϵ

Nicht akzeptiert, da keine passende Transition für c zu q1

b) w2 = aab
(q0,aab,\#)
(q0,ab,A\#), a,\#;A\#
(q0,b,AA\#), a,A;AA
(q0,ϵ,A\#), b,A;ϵ
(q1,ϵ,A\#), ϵ,ϵ;ϵ
(q1,ϵ,\#), ϵ,A;ϵ

Nicht akzeptiert, da ϵ als Eingabe, braucht aber c für q2

c) w3 = bbbaac
(q0,bbbaac,\#)
(q0,bbaac,B\#), b,\#;B\#
(q0,baac,BB\#), b,B;BB
(q0,aac,BBB\#), b,B;BB
(q0,ac,BB\#), a,B;ϵ
(q0,c,B\#), a,B;ϵ
(q1,c,B\#), ϵ,ϵ;ϵ
(q1,c,\#), ϵ,B;ϵ
(q2,ϵ,ϵ), c,\#;ϵ

Wird akzeptiert, weil es in den Endzustand kommt

### Teilaufgabe 4.6.2

L(Pabc) = {wc | w ∈ {a,b}\*}

## Aufgabe 4.7

## Aufgabe 4.8

### Teilaufgabe 4.8.1

#### L1

q0,x,\# => q0,X\#
q0,x,X => q0,XX
q0,ϵ,\# => q2,\#
q0,y,X => q1,ϵ
q1,y,X => q1,ϵ
q1,ϵ,X )> q2,X
q1,ϵ,\# => q2,\#

Deterministisch

#### L2

q0,x,\# => q0,X\#
q0,x,X => q0,XX
q0,y,X => q1,ϵ
q1,y,X => q1,ϵ
q1,ϵ,X => q2,ϵ

Deterministisch

### Teilaufgabe 4.8.2

#### L3

q0,x,\# => q0,X\#
q0,x,X => q0,XX
q0,ϵ,\# => q2,\#
q0,y,X => q1,ϵ
q1,y,X => q1,ϵ
q1,ϵ,\# => q2,\#
q2,y,\# => q2,\#

Nicht deterministisch

#### L4

q0,x,\# => q0,X\#
q0,x,X => q0,XX
q0,y,X => q1,ϵ
q1,y,X => q1,ϵ
q1,y,\# => q2,\#
q2,y,\# => q2,\#

Deterministisch

## Aufgabe 4.11

a) n und m können beliebig groß werden, deshalb müsste der reguläre Automat unendlich viele
Zustände besitzen. Nach Pumping-Lemma Symmetrie von a^nb^n verletzt.

b) a^nb^n.
q0,a,\# => q0,A\#
q0,a,A => q0,AA
q0,b,A => q1,ϵ
q1,b,A => q1,ϵ

a^mb^p
q1,a,\# => q2,A\#
q2,a,A => q2,AA
q2,b,A => q3,ϵ
q3,b,A => q3,ϵ
q3,b,\# => q4,\#
q4,b,\# => q4,\#

c) GA = ({a,b},{S,S1,S2,B},P,S) mit LA = LA1LA2

LA1 = {a^nb^n | n >= 1}, LA2 = {a^mb^p | p > m >= 1}

S => S1S2
S1 => aS1b | ab
S2 => aS2b | aBb
B => bB | b
