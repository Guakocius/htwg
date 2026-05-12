import socket
import time
from src.server import CalculateServer
from struct import *

Server_IP = '127.0.0.1'
Server_PORT = 50000

calc_serv = CalculateServer(1, "MAX", 2, [5, 10])
ans = CalculateServer.build_req(calc_serv)

unpack_res = unpack(f"<I3sB{calc_serv.n}i", ans)
print(unpack_res)
id, op, n, z1, z2 = unpack_res
op = op.decode("utf-8")
response = f"<{id}><{op}><{n}><{z1}><{z2}>"


#resp = CalculateServer.build_res(calc_serv)

#unpack_res = unpack("<Ii", resp)
#id, result = unpack_res
#response = f"<{id}><{result}>"

for i in range(5):
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.settimeout(10)
    print('Connecting to TCP server with IP ', Server_IP, ' on Port ', Server_PORT)
    sock.connect((Server_IP, Server_PORT))
    print('Sending message', response)
    sock.send(response.encode('utf-8'))
    try:
        msg=sock.recv(1024).decode('utf-8')
        print('Message received; ', msg)
    except socket.timeout:
        print('Socket timed out at',time.asctime())
    
    sock.close()
    Server_PORT = Server_PORT + 1



