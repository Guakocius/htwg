import socket
import threading
import time
from dataclasses import dataclass


SERVER_HOST = "127.0.0.1"
SERVER_PORT = 5001
DEFAULT_IP = "127.0.0.1"


@dataclass
class User:
    nickname: str
    ip_address: str
    udp_port: int


@dataclass
class Chat:
    nickname: str
    conn: socket.socket


nickname = ""
ip_address = DEFAULT_IP
udp_port = 0
chat_tcp_port = 0

server_conn: socket.socket | None = None
udp_socket: socket.socket | None = None
chat_server: socket.socket | None = None

users_by_nickname: dict[str, User] = {}
chats_by_nickname: dict[str, Chat] = {}
pending_chat_nicknames: list[str] = []

users_lock = threading.Lock()
chats_lock = threading.Lock()
pending_lock = threading.Lock()
running = True


def encode(message: str) -> bytes:
    return f"{message}\0".encode("utf-8")


def send_message(conn: socket.socket, message: str) -> bool:
    try:
        conn.sendall(encode(message))
        return True
    except OSError:
        return False


def send_server(message: str) -> None:
    if server_conn is not None:
        send_message(server_conn, message)


def send_udp(ip: str, port: int, message: str) -> None:
    if udp_socket is None:
        return

    try:
        udp_socket.sendto(encode(message), (ip, port))
    except OSError as error:
        print(f"UDP-Fehler: {error}")


def parse_user(nickname_text: str, ip_text: str, port_text: str) -> User | None:
    try:
        port = int(port_text)
    except ValueError:
        return None

    if not nickname_text or not 1 <= port <= 65535:
        return None

    return User(nickname_text, ip_text, port)


def handle_userlist(parts: list[str]) -> None:
    if len(parts) != 2:
        print("Ungueltige USERLIST")
        return

    new_users: dict[str, User] = {}
    if parts[1]:
        for entry in parts[1].split(";"):
            fields = entry.split(",")
            if len(fields) != 3:
                continue

            user = parse_user(fields[0], fields[1], fields[2])
            if user is not None:
                new_users[user.nickname] = user

    with users_lock:
        users_by_nickname.clear()
        users_by_nickname.update(new_users)

    print(f"Registriert. Nutzer online: {len(new_users)}")


def handle_update(parts: list[str]) -> None:
    if len(parts) != 5:
        print("Ungueltiges UPDATE")
        return

    user = parse_user(parts[2], parts[3], parts[4])
    if user is None:
        print("Ungueltiges UPDATE")
        return

    with users_lock:
        if parts[1] == "ADD":
            users_by_nickname[user.nickname] = user
            print(f"{user.nickname} ist online")
        elif parts[1] == "REMOVE":
            users_by_nickname.pop(user.nickname, None)
            close_chat(user.nickname)
            print(f"{user.nickname} ist offline")
        else:
            print("Ungueltiges UPDATE")


def handle_broadcast(parts: list[str]) -> None:
    if len(parts) < 3:
        print("Ungueltiger Broadcast")
        return

    sender = parts[1]
    text = "|".join(parts[2:])
    print(f"[Broadcast] {sender}: {text}")


def handle_server_message(message: str) -> None:
    parts = message.split("|")
    command = parts[0]

    match command:
        case "USERLIST":
            handle_userlist(parts)
        case "UPDATE":
            handle_update(parts)
        case "BROADCAST":
            handle_broadcast(parts)
        case "LOGOUT_SUCCESS":
            print("Logout erfolgreich")
            stop_client()
        case "ERROR":
            print(f"Server-Fehler: {'|'.join(parts[1:])}")
        case _:
            print(f"Unbekannte Servernachricht: {message}")


def receive_one_message(conn: socket.socket) -> str | None:
    buffer = ""

    try:
        while "\0" not in buffer:
            data = conn.recv(4096)
            if not data:
                return None

            buffer += data.decode("utf-8")
            buffer = buffer.replace("\\0", "\0")

    except OSError:
        return None

    message, _ = buffer.split("\0", 1)
    return message


def register_at_server() -> bool:
    global nickname

    if server_conn is None:
        return False

    while running:
        send_server(f"REGISTER|{nickname}|{ip_address}|{udp_port}")
        answer = receive_one_message(server_conn)

        if answer is None:
            print("Keine Antwort vom Server")
            return False

        parts = answer.split("|")

        if parts[0] == "USERLIST":
            handle_userlist(parts)
            return True

        if parts[0] == "ERROR":
            print(f"Server-Fehler: {'|'.join(parts[1:])}")
            nickname = input("Neuer Nickname: ").strip()
            while not nickname:
                nickname = input("Neuer Nickname: ").strip()
        else:
            print(f"Unerwartete Antwort vom Server: {answer}")
            return False

    return False


def receive_server_messages() -> None:
    global running

    if server_conn is None:
        return

    buffer = ""

    try:
        while running:
            data = server_conn.recv(4096)
            if not data:
                break

            buffer += data.decode("utf-8")
            buffer = buffer.replace("\\0", "\0")

            while "\0" in buffer:
                message, buffer = buffer.split("\0", 1)
                if message:
                    handle_server_message(message)

    except OSError as error:
        if running:
            print(f"Server-Verbindung verloren: {error}")

    running = False


def register_chat(nickname_text: str, conn: socket.socket) -> None:
    with chats_lock:
        old_chat = chats_by_nickname.pop(nickname_text, None)
        chats_by_nickname[nickname_text] = Chat(nickname_text, conn)

    if old_chat is not None:
        close_socket(old_chat.conn)

    thread = threading.Thread(
        target=receive_chat_messages,
        args=(nickname_text, conn),
        daemon=True,
    )
    thread.start()


def receive_chat_messages(chat_nickname: str, conn: socket.socket) -> None:
    buffer = ""

    try:
        while running:
            data = conn.recv(4096)
            if not data:
                break

            buffer += data.decode("utf-8")
            buffer = buffer.replace("\\0", "\0")

            while "\0" in buffer:
                message, buffer = buffer.split("\0", 1)
                if message:
                    handle_chat_message(chat_nickname, conn, message)

    except OSError:
        pass

    close_chat(chat_nickname, conn)


def handle_chat_message(chat_nickname: str, conn: socket.socket, message: str) -> None:
    parts = message.split("|", 1)

    if len(parts) == 2 and parts[0] == "MSG" and parts[1]:
        print(f"[{chat_nickname}] {parts[1]}")
        send_message(conn, "SUCCESS|MESSAGE_RECEIVED")
    elif parts[0] == "SUCCESS":
        pass
    elif parts[0] == "ERROR":
        print(f"{chat_nickname} meldet: {message}")
    else:
        send_message(conn, "ERROR|INVALID_MESSAGE_FORMAT")


def next_pending_chat_nickname() -> str | None:
    with pending_lock:
        if not pending_chat_nicknames:
            return None
        return pending_chat_nicknames.pop(0)


def accept_chat_connections() -> None:
    if chat_server is None:
        return

    while running:
        try:
            conn, addr = chat_server.accept()
        except OSError:
            return

        chat_nickname = next_pending_chat_nickname() or f"{addr[0]}:{addr[1]}"
        register_chat(chat_nickname, conn)
        print(f"Chat-Verbindung mit {chat_nickname} hergestellt")


def connect_to_peer(peer_nickname: str, peer_ip: str, peer_port: int) -> None:
    try:
        conn = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        conn.connect((peer_ip, peer_port))
    except OSError as error:
        print(f"Chat-Verbindung fehlgeschlagen: {error}")
        return

    register_chat(peer_nickname, conn)
    print(f"Chat-Verbindung mit {peer_nickname} hergestellt")


def handle_udp_message(message: str, addr: tuple[str, int]) -> None:
    parts = message.split("|")

    if parts[0] == "HANDSHAKE":
        handle_handshake(parts, addr)
    elif parts[0] == "HANDSHAKE_RETURN":
        if len(parts) != 2:
            print("Ungueltige HANDSHAKE_RETURN")
    elif parts[0] == "ERROR":
        print(f"UDP-Fehler: {'|'.join(parts[1:])}")
    else:
        send_udp(addr[0], addr[1], "ERROR|INVALID_FORMAT")


def handle_handshake(parts: list[str], addr: tuple[str, int]) -> None:
    if len(parts) != 3:
        send_udp(addr[0], addr[1], "ERROR|INVALID_HANDSHAKE_FORMAT")
        return

    peer_nickname = parts[1]

    try:
        peer_tcp_port = int(parts[2])
    except ValueError:
        send_udp(addr[0], addr[1], "ERROR|INVALID_HANDSHAKE_FORMAT")
        return

    with users_lock:
        peer_is_known = peer_nickname in users_by_nickname

    if not peer_is_known:
        send_udp(addr[0], addr[1], "ERROR|UNKNOWN_NICKNAME")
        return

    send_udp(addr[0], addr[1], f"HANDSHAKE_RETURN|{chat_tcp_port}")
    connect_to_peer(peer_nickname, addr[0], peer_tcp_port)


def receive_udp_messages() -> None:
    if udp_socket is None:
        return

    while running:
        try:
            data, addr = udp_socket.recvfrom(4096)
        except OSError:
            return

        data_text = data.decode("utf-8")
        data_text = data_text.replace("\\0", "\0")

        for message in data_text.split("\0"):
            if message:
                handle_udp_message(message, addr)


def find_user(nickname_text: str) -> User | None:
    with users_lock:
        return users_by_nickname.get(nickname_text)


def find_chat(nickname_text: str) -> Chat | None:
    with chats_lock:
        return chats_by_nickname.get(nickname_text)


def send_direct_message(peer_nickname: str, text: str) -> None:
    chat = find_chat(peer_nickname)

    if chat is None:
        user = find_user(peer_nickname)
        if user is None:
            print("Nutzer nicht gefunden")
            return

        with pending_lock:
            pending_chat_nicknames.append(peer_nickname)

        send_udp(user.ip_address, user.udp_port, f"HANDSHAKE|{nickname}|{chat_tcp_port}")
        print("Handshake gesendet")
        time.sleep(0.5)
        chat = find_chat(peer_nickname)

    if chat is None:
        print("Noch keine Chat-Verbindung vorhanden. Nachricht nochmal senden.")
        return

    send_message(chat.conn, f"MSG|{text}")


def print_users() -> None:
    with users_lock:
        users = list(users_by_nickname.values())

    if not users:
        print("Keine Nutzer online")
        return

    for user in users:
        marker = " (du)" if user.nickname == nickname else ""
        print(f"{user.nickname}{marker} - {user.ip_address}:{user.udp_port}")


def handle_user_input(command: str) -> None:
    if command == "/help":
        print_help()
    elif command == "/users":
        print_users()
    elif command == "/logout":
        send_server("LOGOUT")
        time.sleep(0.2)
        stop_client()
    elif command.startswith("/broadcast "):
        text = command.removeprefix("/broadcast ").strip()
        send_server(f"BROADCAST|{text}")
    elif command.startswith("/msg "):
        parts = command.split(" ", 2)
        if len(parts) != 3:
            print("Format: /msg nickname nachricht")
        else:
            send_direct_message(parts[1], parts[2])
    else:
        print("Unbekannter Befehl. /help zeigt alle Befehle.")


def print_help() -> None:
    print("Befehle:")
    print("/users")
    print("/broadcast nachricht")
    print("/msg nickname nachricht")
    print("/logout")


def close_chat(nickname_text: str, expected_conn: socket.socket | None = None) -> None:
    with chats_lock:
        chat = chats_by_nickname.get(nickname_text)
        if chat is None:
            return
        if expected_conn is not None and chat.conn is not expected_conn:
            return
        chats_by_nickname.pop(nickname_text, None)

    close_socket(chat.conn)


def close_socket(conn: socket.socket | None) -> None:
    if conn is None:
        return

    try:
        conn.shutdown(socket.SHUT_RDWR)
    except OSError:
        pass

    try:
        conn.close()
    except OSError:
        pass


def stop_client() -> None:
    global running

    running = False
    close_socket(server_conn)
    close_socket(udp_socket)
    close_socket(chat_server)

    with chats_lock:
        chats = list(chats_by_nickname.values())
        chats_by_nickname.clear()

    for chat in chats:
        close_socket(chat.conn)


def ask_port(prompt: str) -> int:
    text = input(prompt).strip()
    if not text:
        return 0
    return int(text)


def start_client() -> None:
    global nickname
    global ip_address
    global udp_port
    global chat_tcp_port
    global server_conn
    global udp_socket
    global chat_server

    nickname = input("Nickname: ").strip()
    ip_address = input(f"Lokale IP [{DEFAULT_IP}]: ").strip() or DEFAULT_IP
    udp_port = ask_port("UDP-Port [0 = automatisch]: ")
    requested_chat_tcp_port = ask_port("Chat-TCP-Port [0 = automatisch]: ")

    udp_socket = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    udp_socket.bind((ip_address, udp_port))
    udp_port = udp_socket.getsockname()[1]

    chat_server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    chat_server.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    chat_server.bind((ip_address, requested_chat_tcp_port))
    chat_server.listen()
    chat_tcp_port = chat_server.getsockname()[1]

    server_conn = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_conn.connect((SERVER_HOST, SERVER_PORT))

    if not register_at_server():
        stop_client()
        return

    threading.Thread(target=receive_server_messages, daemon=True).start()
    threading.Thread(target=receive_udp_messages, daemon=True).start()
    threading.Thread(target=accept_chat_connections, daemon=True).start()

    print(f"UDP-Port: {udp_port}")
    print(f"Chat-TCP-Port: {chat_tcp_port}")
    print_help()


def main() -> None:
    start_client()

    while running:
        try:
            command = input("> ").strip()
        except (EOFError, KeyboardInterrupt):
            print()
            send_server("LOGOUT")
            stop_client()
            return

        if command:
            handle_user_input(command)


if __name__ == "__main__":
    main()
