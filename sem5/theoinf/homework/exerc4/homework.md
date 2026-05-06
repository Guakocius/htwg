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
