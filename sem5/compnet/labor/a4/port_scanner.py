import socket
import sys
import threading
import time

#target = "141.37.122.107"
# NOTE: default localhost target for testing purposes only
target = "127.0.0.1"
open_ports = []

def scan_protocol(target, port, p):
    print("\n scanning on port", port)
    s = socket.socket(socket.AF_INET, p)
    socket.setdefaulttimeout(10)

    result = s.connect_ex((target, port))
    print(result)
    if result == 0:
        open_ports.append(port)
    s.close()


# NOTE: Only works if in HTWG VPN
def scan_tcp():
    print("scanning for TCP ports...")

    #target = "141.37.122.107"

    try:
        for port in range(1, 51):
            open_ports = scan_protocol(target, port, socket.SOCK_STREAM)
            
    except KeyboardInterrupt:
        print("\n exiting program")
        sys.exit()
    except socket.gaierror:
        print("\n hostname could not be resolved")
        sys.exit()
    except socket.error:
        print("\n server not responding")

    return open_ports

def scan_udp():
    print("scanning for UDP ports...")

    #target = "141.37.168.26" 

    try:
        for port in range(1, 51):
            
            open_ports = scan_protocol(target, port, socket.SOCK_DGRAM)

    except KeyboardInterrupt:
        print("\n exiting program")
        sys.exit()
    except socket.gaierror:
        print("\n hostname could not be resolved")
        sys.exit()
    except socket.error:
            print("\n server not responding")

    return open_ports



t = threading.Thread(target=scan_tcp, args=())
t.start()
t.join()

print("open TCP ports:", open_ports)
time.sleep(2)
open_ports = []

t = threading.Thread(target=scan_udp, args=())
t.start()
t.join()
print("open UDP ports:", open_ports)


    
