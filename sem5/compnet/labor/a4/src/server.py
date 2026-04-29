import socket
import struct

socket.setdefaulttimeout(30)

class CalculateServer:
    def build_req(self, req_id: int, op: str, n: int, z: list[int]):
        calc_req = {
                "id": req_id,
                "operation": op,
                "n": n,
                "z": z
                }

        def build_msg(calc_req: dict[str, int | str]):
            msg_id, op, n, z = calc_req
            op_encod = op.encode("utf-8")

            return struct.pack("utf-8", msg_id, op_encod, n, [i for i in z])


    def build_res(self, res_id: int, result: int):
        calc_req = {
                "id": res_id,
                "result": result
        }

        def build_msg(calc_req: dict[str, int]):
            res_id, result = calc_req
            return struct.pack("utf-8",res_id, result)





