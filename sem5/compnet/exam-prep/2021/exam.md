# 2021

## Aufgabe 3

### 1)

P = 720B = 5760 Bit
Qtbit = 5760b / 5,76Mbps = 0,001s = 1ms
R1tbit = 5760b / 1,92Mbps = 3ms
R2tbit = 5760b / 2,88Mbps = 2ms
R3tbit = 5760b / 1,92Mbps = 3ms
tbit = 1ms + 3ms + 2ms + 3ms = 9ms

## Aufgabe 5

### 1)

Server: IP, Port = (183.114.145.108, 35193)
Client functions:
(1) Zielsprache Z, Z.default = EN, set_target_lang(l): Z.lang = l
(2) send_message()
(3) exit_session()

(1) LA:\<LANG\>
(2) TR:\<LEN\>\<MSG\>
(3) EX => return

client_req() => UDP => n servers for n in \[...languages\].count()
dict(x,y) = {addr:x,port:y}
Server: resp => Client

Client: TCP => lang => Ü-Portal: UDP => Server_lang
Server resp translation: UDP => Ü-Portal: translation: TCP => Client

Portal resp: \<LEN\>\<MSG\>

LA,TR,EX: msg.encode("uft-8")
\<LANG\>: 2 ASCII-Zeichen
\<MSG\>: max size: 60000 encode("utf-8"), LEN = len(msg) uint

translatorD\[\<LANG\>\] = (\<IP-ADDR\>, \<PORT\>)

### 2)

```python

def start_portal(sock, addr, port):
    sock.bind((addr, port))
    sock.listen(0)

    try:
        conn, addr = sock.accept()
        handle_requests(conn, addr)
    finally:
        conn.close()
        sock.close()

def handle_requests(sock):
    data = sock.recv(1024)


def start_client(sock, addr, port):
    try:
        sock.connect((addr, port))
    finally:
        sock.close()


def main(c):
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM) # TCP
    addr, port = (183.114.145.108, 35193)
    thread.start(start_portal(sock, addr, port))
    thread.start(start_client())


if __name__ == "__main__":
    main()
```

## Aufgabe 7

### 1)

N1: 5 (0101) => 2³ => 3 freie Stellen, N2: 15 (1111), N3: 10 (1010), N4: 6 (0110)
100 (01100100), 171 (10101011)
N1IP = 212.30.175.104-212.30.175.111, Netzwerk: ...104, Broadcast: ...111
255.255.255.255-8 = 255.255.255.248, 8-2 = 6 freie IP-Adressen

N3IP = 212.30.175.112-127
Subnet = 255.255.255.256-16 = 255.255.255.240

N2IP = 212.30.175.128-159
Subnet = 255.255.255.256-32 = 255.255.255.224

N4IP = 212.30.175.160-167
Subnet = 255.255.255.248

### 2)

A: 5 (0101) =>
