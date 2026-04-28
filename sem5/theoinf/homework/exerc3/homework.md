# Übungsblatt 3

## Aufgabe 3.1

### Teilaufgabe 3.1.1

#### a)

L2 = {aa, ab, ba, bb}

#### b)

L3 = {ϵ, a, ab, aa, aabbb,...}

#### c)

L4 = {ϵ, aa, bbaa, baabaab, aabaa,...}

#### d)

L5 = {Der, der, Die, die, Das, das}

#### e)

L6 = {+0,+1,...,+9,0,1,...,9,-0,-1,...,-9,001111,...}

#### f)

L7 = {0,1,2,...,A,B,...,F,000A,000B,...,FFFFF...,...}

### Teilaufgabe 3.1.2

#### a)

r8 = M(e|a)(i|y)e?r

#### b)

r9 = 1(0)\*€

#### c)

r10 = (a\*b\*)

#### d)

r11 = (a+b+)

#### e)

r12 = (ab)+

#### f)

r13 = (a|b)\*

#### g)

r14 = (a\*|b\*)

#### h)

r15 = (a+|b+)

### Teilaufgabe 3.1.3

#### a)

G2 = ({S,A},{a,b},P,S) mit der
Regelmenge P:

- S -> aA | bA
- A -> a | b

#### b)

G3 = ({S,A,B},{a,b},P,S) mit
P:

- S -> aA | B
- A -> a | B
- B -> bA | ε

#### c)

G4 = ({S,A,B},{a,b},P,S) mit
P:

- S -> aa | B
- A -> aaB | bB
- B -> A | ε

#### d)

G12 = ({S,A,B},{a,b},P,S) mi
P:

- S -> abA
- A -> abA | B
- B -> A | ε

## Aufgabe 3.3

### a)

La = {n | n ∈ Wörter beginnend mit einem Großbuchstaben, mit einer Größe von mindestens einem
Zeichen, optional mehr, endend mit einem Punkt.}

### b)

Lb = {n | n ∈ Alle positiven sowie optional negativen Ganz- oder optional Dezimalzahlen ohne führende Null.}

### c)

Lc = {n | n ∈ Alle Jahrestage nach dem Schema "YYYY-MM-DD".}

### d)

Ld = {n | n ∈ 4 Zahlen von 0 bis 255 getrennt mit Punkten.} (IPv4)

### e)

Le = {n | n ∈ Strings, welche mindestens ein Sonderzeichen beinhalten.}

## Aufgabe 3.4

### Teilaufgabe 3.4.1

#### a)

```bash
cat StarWars_EpisodeIV_script.txt | grep -E "(\.\.\.)" -c
```

**151**

#### b)

```bash
    cat StarWars_EpisodeIV_script.txt | grep -Eo "[Ss][Tt][Aa][Rr][Ss]?(\.|\s|$)" -c
```

**237**

#### c)

```bash

cat StarWars_EpisodeIV_script.txt | grep -E "(Luke|Leia|Vader)*" -c

```

**7518**

#### d)

```bash

cat StarWars_EpisodeIV_script.txt | grep -E "[Ee][Nn][Dd]*" -c

```

**924**

### Teilaufgabe 3.4.2

#### a)

_Wie viel Zeilen besitzt der Text?_

```bash

cat StarWars_EpisodeIV_script.txt | wc -l

```

7518

_Wie viele Zeilen sind leere Zeilen?_

```bash

cat StarWars_EpisodeIV_script.txt | grep -E "^$" -c

```

2763

#### b)

```bash

cat StarWars_EpisodeIV_script.txt | grep -E "^\s{19}" -c

```

1015

#### c)

```bash

cat StarWars_EpisodeIV_script.txt | grep -Eo "((XP-)[0-9]+|R2(-D2)?|C-3PO?)" -c

```

15

## Aufgabe 3.5

### Teilaufgabe 3.5.1

- A2: DEA => Ein Folgezustand und kein ε-Übergang.
- A3: NEA => ε-Übergang.
- A4: NEA => Mehr als ein Folgezustand von q0 aus.

### Teilaufgabe 3.5.2

#### ω := bbb

- A2: q0 =b> q1 =b> q1 =b> q1; Durchlaufen: q0,q1,q1,q1; Akzeptiert mit Endzustand q1
- A3: (wegen ε-Hülle(q0) = {q0,q1}) -> {q0,q1} =b> {q1} =b> {q1} =b> {q1}; Durchlaufen: {q0,q1},q1,q1,q1; Akzeptiert mit Endzustand q1
- A4: {q0} =b> {q0,q1} =b> {q0,q1} =b> {q0,q1}; Durchlaufen: q0,{q0,q1},{q0,q1},{q0,q1}; Akzeptiert mit Endzustand {q0,q1}

#### ω := aab

- A2: q0 =a> q0 =a> q0 =b> q1; Durchlaufen: q0,q0,q0,q1; Akzeptiert mit Endzustand q1
- A3: {q0,q1} =a> {q2} =a> {q2} =b> {}; Durchlaufen: {q0,q1},q2,q2; Nicht Akzeptiert, da **b** nicht verarbeitet werden kann
- A4: {q0} =a> {q0,q2} =a> {q0,q2} =b> {q0,q1}; Durchlaufen: q0,{q0,q2},{q0,q2},{q0,q1}; Akzeptiert mit Endzustand {q0,q1}

### Teilaufgabe 3.5.3

- L(A2) = {a^nb^m | n,m ∈ N0} ∪ {b^na^m | n,m ∈ N0}
- L(A3) = {c^na^m | n,m ∈ N0} ∪ {b^na^m | n,m ∈ N0}
- L(A4) = {a^nb^m | n,m ∈ N0} ∪ {b^na^m | n,m ∈ N0}

## Aufgabe 3.8

Σ = {a, b}

**Siehe Appendix**
