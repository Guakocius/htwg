import ipaddress
import socket
import threading
from dataclasses import dataclass


HOST = "127.0.0.1"
TCP_PORT = 5001


@dataclass
class User:
    nickname: str
    ip_address: str
    udp_port: int
    conn: socket.socket


users_by_nickname: dict[str, User] = {}
users_by_conn: dict[socket.socket, User] = {}
users_lock = threading.Lock()


def encode(message: str) -> bytes:
    return f"{message}\0".encode("utf-8")


def send_message(conn: socket.socket, message: str) -> bool:
    try:
        conn.sendall(encode(message))
        return True
    except OSError:
        return False


def userlist_payload() -> str:
    return ";".join(
        f"{user.nickname},{user.ip_address},{user.udp_port}"
        for user in users_by_nickname.values()
    )


def broadcast_to_registered(message: str, exclude: socket.socket | None = None) -> None:
    with users_lock:
        targets = [
            user.conn
            for user in users_by_nickname.values()
            if exclude is None or user.conn is not exclude
        ]

    for conn in targets:
        send_message(conn, message)


def remove_user(conn: socket.socket, notify: bool = True) -> User | None:
    with users_lock:
        user = users_by_conn.pop(conn, None)
        if user is None:
            return None

        users_by_nickname.pop(user.nickname, None)

    if notify:
        broadcast_to_registered(
            f"UPDATE|REMOVE|{user.nickname}|{user.ip_address}|{user.udp_port}"
        )

    return user


def handle_register(parts: list[str], conn: socket.socket) -> None:
    if len(parts) != 4:
        send_message(conn, "ERROR|REGISTER expects nickname, IP address and UDP port")
        return

    nickname = parts[1].strip()
    ip_address = parts[2].strip()

    if not nickname:
        send_message(conn, "ERROR|Nickname must not be empty")
        return

    try:
        ipaddress.ip_address(ip_address)
        udp_port = int(parts[3])
    except ValueError:
        send_message(conn, "ERROR|Invalid IP address or UDP port")
        return

    if not 1 <= udp_port <= 65535:
        send_message(conn, "ERROR|UDP port out of range")
        return

    with users_lock:
        if nickname in users_by_nickname:
            send_message(conn, "ERROR|Nickname already registered")
            return

        old_user = users_by_conn.get(conn)
        if old_user is not None:
            send_message(conn, "ERROR|Connection is already registered")
            return

        user = User(nickname, ip_address, udp_port, conn)
        users_by_nickname[nickname] = user
        users_by_conn[conn] = user
        current_userlist = userlist_payload()

    send_message(conn, f"USERLIST|{current_userlist}")
    broadcast_to_registered(
        f"UPDATE|ADD|{nickname}|{ip_address}|{udp_port}",
        exclude=conn,
    )


def handle_logout(conn: socket.socket) -> bool:
    user = remove_user(conn)
    if user is None:
        send_message(conn, "ERROR|Connection is not registered")
    else:
        send_message(conn, "LOGOUT_SUCCESS")

    return False


def handle_broadcast(parts: list[str], conn: socket.socket) -> None:
    with users_lock:
        user = users_by_conn.get(conn)

    if user is None:
        send_message(conn, "ERROR|Register before broadcasting")
        return

    if len(parts) < 2 or not parts[1]:
        send_message(conn, "ERROR|BROADCAST expects a message")
        return

    message = "|".join(parts[1:])
    broadcast_to_registered(f"BROADCAST|{user.nickname}|{message}")


def parse_request(request: str, conn: socket.socket) -> bool:
    parts = request.split("|")
    header = parts[0]

    match header:
        case "REGISTER":
            handle_register(parts, conn)
        case "LOGOUT":
            return handle_logout(conn)
        case "BROADCAST":
            handle_broadcast(parts, conn)
        case _:
            send_message(conn, f"ERROR|Unknown command {header}")

    return True


def handle_tcp_client(conn, addr):
    print(f"TCP-Verbindung von Client: {addr}")
    buffer = ""

    try:
        while True:
            data = conn.recv(4096)
            if not data:
                break

            buffer += data.decode("utf-8")
            buffer = buffer.replace("\\0", "\0")

            while "\0" in buffer:
                request, buffer = buffer.split("\0", 1)
                if request and not parse_request(request, conn):
                    return

    except OSError as error:
        print(f"TCP-Fehler bei {addr}: {error}")

    finally:
        remove_user(conn)
        conn.close()
        print(f"TCP-Verbindung zu {addr} geschlossen")


def start_tcp_server() -> None:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as server:
        server.bind((HOST, TCP_PORT))
        server.listen()

        print(f"Server laeuft auf {HOST}:{TCP_PORT}")

        while True:
            conn, addr = server.accept()
            thread = threading.Thread(
                target=handle_tcp_client,
                args=(conn, addr),
                daemon=True,
            )
            thread.start()


def main() -> None:
    start_tcp_server()


if __name__ == "__main__":
    main()
