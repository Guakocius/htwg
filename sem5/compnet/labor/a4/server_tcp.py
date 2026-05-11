import socket
import time
import threading

My_IP = '127.0.0.1'
My_PORT = 50000
server_activity_period=30;

t_end=time.time()+server_activity_period # Ende der Aktivitätsperiode

def listen(sock):
    sock.bind((My_IP, My_PORT))
    print('Listening on Port ', My_PORT, ' for incoming TCP connections');

    sock.listen(1)

    print('Listening...')

    while time.time() < t_end:
        try:
            conn, addr = sock.accept()
            print('Incoming connection accepted: ', addr) 
            
            threading.Thread(target=receive, args=(conn,)).start()
            break
        except socket.timeout:
            print('Socket timed out listening', time.asctime())
    sock.close()

def receive(conn):
    while time.time()<t_end:
        try:
            data = conn.recv(1024)
            if not data: # receiving empty messages means that the socket other side closed the socket
                print('Connection closed from other side');
                print('Closing...');
                conn.close()
                break
            print('received message: ', data.decode('utf-8'), 'from ', threading.current_thread())
            conn.send(data[::-1])
        except socket.timeout:
            print('Socket timed out at', time.asctime())

    if conn:
        conn.close()

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
listen(sock)

