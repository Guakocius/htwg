import struct

class ChatServer:

    def __init__(self):
        self.users: dict[str, list[str | int]] = {
                "username": [],
                "ip": [],
                "port": [],
                }

        def get_port(self, username: str) -> int:
            return self.users["port"]

    def handle_users(self):
        calc_req = {
                "username": self.users["username"],
                "ip": self.users["ip"],
                "port": self.users["port"],
                }


        def register(self) -> list[str]:

            return []

        def log_in(self) -> list[str]:
            return []

        def log_out(self) -> list[str]:
            return []

        def update_user_list(self) -> list[str]:
            return []


        def build_msg(calc_req):
            us, op, n, z = calc_req.values()
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
            res_id, op, _, z = calc_req.values()
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





