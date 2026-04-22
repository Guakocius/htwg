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

## Aufgabe 3.8

Σ = {a, b}

**Siehe Appendix**
