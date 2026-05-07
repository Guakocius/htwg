# Hausaufgabe 4

## Übungsblatt 3

### Aufgabe 3.6

#### Teilaufgabe 3.6.1

A1(w1) => start q0 ->(x) q1 ->(y) q2 ->(z) q0 => F := q0 <=> akzeptiert
A2(w1) => start q0 ->(x) q1 ->(y) q2 ->(z) q3 => F := q3 <=> akzeptiert
N3(w1) => start q0 ->(x) {q0,q1} ->(y) {q0,q2} ->(z) q0 => F := q0 <=> akzeptiert

A1(w2) => start q0 ->(x) q1 ->(y) q2 ->(x) ! => nicht akzeptiert
A2(w2) => start q0 ->(x) q1 ->(y) q2 ->(x) ! => nicht akzeptiert
N3(w2) => start q0 ->(x) {q0,q1} ->(y) {q0,q2} ->(x) {q0,q1} ->(y) {q0,q2} ->(z) q0 => F := q0 <=> akzeptiert

#### Teilaufgabe 3.6.2

L(A1) := {(X+Y+Z+)+}
L(A2) := {(x+y+z+)+x\*}
L(N3) := {x\*y+z\*}

### Aufgabe 3.7

#### Teilaufgabe 3.7.1

Ap = (Q,Σ,δ,qs,F), Σ = {0,1}, Q = {qs,q0,q1}, q0: gerade, q1: ungerade, F = {q0}

start qs ->(0) q0 | ->(1) q1
q0 ->(0) q0 | ->(1) q1
q1 ->(0) q1 | ->(1) q0

#### Teilaufgabe 3.7.2

1 verfälschtes Bit: Zustand der Parität ändert sich (gleich=>ungleich, ungleich=>gleich)
2 verfälschte Bits: Parität hat selben Zustand (gleich=>gleich, ungleich=>ungleich)

#### Teilaufgabe 3.7.3

## Übungsblatt 30

### Aufgabe 30.1

#### Teilaufgabe 30.1.1

##### a)

L(N1) = {w | w := [abc]\*a}

##### b)

#### Teilaufgabe 30.1.2

##### a)

L(N2) = {w | w := [xyz]\*x}

##### b)

### Aufgabe 30.2

### Aufgabe 30.3

#### Teilaufgabe 30.3.1

rx = 01[01]\*01

#### Teilaufgabe 30.3.4

Gx​ = (N,Σ,P,S), N = {S,A,B,C}, Σ = {0,1}
S => 0A
A => 1B
B => 0B | 1B | 0C
x => 1

### Aufgabe 30.5

#### Teilaufgabe 30.5.1

NEA(N1) => start q0 ->(a,g,k,w) q0 ->(i) q1