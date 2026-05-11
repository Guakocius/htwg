import struct

class CalculateServer:

    def __init__(self, req_id, op, n, z):
        self.req_id = req_id
        self.op = op
        self.n = n
        self.z = z

    def build_req(self):
        calc_req = {
                "id": self.req_id,
                "operation": self.op,
                "n": self.n,
                "z": self.z,
                }

        def build_msg(calc_req):
            msg_id, op, n, z = calc_req.values()
            op_encod = op.encode(encoding="utf-8")

            return struct.pack(f"<I3sB{n}i", msg_id, op_encod, n, z[0], z[1])
        return build_msg(calc_req)

    def build_res(self):

        calc_req = {
                "id": self.req_id,
                "operation": self.op,
                "n": self.n,
                "z": self.z
        }

        def build_msg(calc_req):
            res_id, op, n, z = calc_req.values()
            result = 1

            match op:
                case "MAX":
                    result = max(z)
                case "PRO":
                    for i in z:
                        result = result * i
                case "MIN":
                    result = min(z)
                case "SUM":
                    result = sum(z)
                case _:
                    result = 0


            return struct.pack("<Ii",res_id, result)
        return build_msg(calc_req)





