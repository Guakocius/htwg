from src.server import CalculateServer
from struct import *
import socket
import time

ip = '127.0.0.1'
port = 50000

def main():
    calc_serv = CalculateServer(1, "MAX", 2, [5, 10])
    ans = CalculateServer.build_req(calc_serv)

    unpack_res = unpack(f"<I3sB{calc_serv.n}i", ans)
    id, op, n, z1, z2 = unpack_res
    op = op.decode("utf-8")
    response = f"<{id}><{op}><{n}><{z1}><{z2}>"

    print(f"Request to the client: {response}")

    resp = CalculateServer.build_res(calc_serv)

    unpack_res = unpack("<Ii", resp)
    id, result = unpack_res
    response = f"<{id}><{result}>"
    #response_ = client_stream(ip, port, response)

    print(f"Client response: {response}")
    

if __name__ == "__main__":
    main()
