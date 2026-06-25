# Übungsblatt 5

## Aufgabe 5.2

w0 = cc => Akzeptiert.

w1 = aca => Nicht akzeptiert.

w2 = acca => Akzeptiert.

w3 = bb => Nicht akzeptiert.

w4 = bacc => Nicht akzeptiert.

w5 = baccba => Akzeptiert.

L(TM) => {w ∈ \[abc\] | w ist ein Palindrom}

## Aufgabe 5.3

### Teilaufgabe 5.3.1

siehe .kra-Datei.

### Teilaufgabe 5.3.2

siehe .kra-Datei.

### Teilaufgabe 5.3.3

Akzeptiert nicht, Pumping-Lemma: keine reguläre Sprache.

## Aufgabe 5.4

### a)

w5 = 11
(q0, 11\[\_\])

### b)

siehe PDF.

### c)

f5(w) = NOT(w) => w wird invertiert. Führende Nullen werden entfernt.

## Aufgabe 5.6

### a)

Diagramm: siehe .kra-Datei.

Konfiguration:

w1 = 1
(q0, \[1\]_)
-> (q1, 1\[_\]) \[q0; 1,1,R\]
-> (q2, 1\[_\]) \[q1: _,_,R\]
-> q1->q3; _,_,R, q3 bei _: \_,x,L ...

w3 = 10
(q0, \[1\]0*)
-> (q1, 1\[0\]*) \[1,1,R\]
-> (q1, 10\[\_\]) \[0,0,R\]
-> ... q2 aktiviert

### b)

siehe PDF.

### c)

fX(w) = x^n => n ist Wert der binären Zahl w und bildet eine Folge von n mal x-Symbolen.

## Aufgabe 5.7

### Teilaufgabe 5.7.1

siehe .kra-Datei.

### Teilaufgabe 5.7.2

m-1 zusätzliche, insgesamt m+2 Zustände, welche 1 löscht und zu nächstem q übergeht.

### Teilaufgabe 5.7.3

Alle Blank-Zeichen sehen gleich aus, wodurch nicht mehr erkennbar ist, welche Blank-Zeichen
Trennzeichen, und welche gelöschte Zeichen sind.

## Aufgabe 5.8
