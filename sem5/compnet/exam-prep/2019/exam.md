# WS 2019/20

## Aufgabe 3

### 1)

Qx-R1 = 20ms \* 3 = 60ms, R1-R2 = 6ms, R2-R3 = 10ms, R3-Z = 20ms; 6ms + 10ms + 20ms = 36ms
P = 1500B = 12000 Bits
Q1tbit = 12kb / 600kbps = 0.02s = 20ms
Q2tbit = 12kb / 400kbps = 0.03s = 30ms
Q3tbit = 12kb / 300kbps = 0.04s = 40ms
R1tbit = 12kb / 500kbps = 0.024s = 24ms
R2tbit = 12kb / 400kbps = 30ms
R3tbit = 12kb / 300kbps = 40ms

36ms + 24ms + 30ms + 40ms = 130ms

T(Q1,Q2,Q3) = 130 + 20ms + (20ms,30ms,40ms) = (170ms,180ms,190ms)

### 2)

Q1R = 700kbps
jedes vierte Paket landet in den Puffer von R3, das erste Paket geht durch, das erste Paket, was
in den Puffer gelangt, ist Paket 2.
Max Puffer = 15kB / 1500B = 15000B / 1500B = 10 Pakete
Vier Puffer => erstes verlorenes Paket = 2 + 4\*10 = 42, 2. 46, 3. 50.
Pakete gehen zwischen R3-Z verloren.

### 3)

Q2R = 600kbps, Q3R = 3Mbps

## Aufgabe 5

### 1)

c empfängt einzelne Bytes von sock (Chars), wenn c eine Newline ist, soll n zurückgegeben, ansonsten
inkrementiert werden. Wenn c keine Daten empfangen konnte, wird ein leerer String zurückgegeben.

### 2)

C1: Hello, I'm Beta (15) S1: Hello, I'm Alpha (16)
C2: 0.5+0(3595 mal)\n (3600) S2: 0(59 mal)\n (60)
C3: 1.0+0(2495 mal)\n (2500) S3: 0(2499 mal)\n (2500)
C4: 2.0+0(65 mal)\n (70) S4: 0(4899 mal)\n (4900)

## Aufgabe 8

### 1)

N1: 3 addr., IP 100.100.100.80-100.100.100.82, Subnet 255.255.255.248
N2: 6 addr., IP 100.100.100.83-100.100.100.88, Subnet 255.255.255.248
N3: 14 addr., IP 100.100.100.89-100.100.100.102, Subnet 255.255.255.240
N4: 31 addr., IP 100.100.100.103-100.100.100.133, Subnet 255.255.255.192

### 2)

A: 4, Subnet 255.255.255.2
