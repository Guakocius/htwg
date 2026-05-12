class Client:
    def __init__(self, username: str, ip: str, port: int):
        self.username = username
        self.ip = ip
        self.port = port


    def set_username(self, username: str) -> None:

