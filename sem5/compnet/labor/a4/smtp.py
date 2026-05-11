from socket import *
import base64
import time
import ssl

msg = "\r\n Hello from SMTP!"
endmsg = "\r\n.\r\n"

mailserver = ("asmtp.htwg-konstanz.de", 587)
client_sock = socket(AF_INET, SOCK_STREAM)
client_sock.connect(mailserver)

recv = client_sock.recv(1024)
recv = recv.decode("utf-8")

print("Message after connection request:", recv)

if recv[:3] != "220":
    print("220 not received from server.")

helo_cmd = "EHLO Alex\r\n"
client_sock.send(helo_cmd.encode("utf-8"))
time.sleep(1)

recv1 = client_sock.recv(1024)
recv1 = recv1.decode("utf-8")
time.sleep(1)
print("Message after EHLO command:", recv1)

if recv1[:3] != "250":
    print("250 not received from server.")

starttls = "STARTTLS\r\n"

client_sock.send(starttls.encode("utf-8"))
time.sleep(1)

recv2 = client_sock.recv(1024)
recv2 = recv2.decode("utf-8")
time.sleep(1)
print("Message after STARTTLS command:", recv2)

username = "rnetin03"
password = "jooNaicu5cheiV"

context = ssl.create_default_context()
sock = context.wrap_socket(client_sock, server_hostname="asmtp.htwg-konstanz.de")
print("SOCK:",sock)

base64_str = ("\x00" + username + "\x00" + password).encode()
time.sleep(1)
base64_str = base64.b64encode(base64_str)
print(base64_str)
auth_msg = "AUTH PLAIN ".encode("utf-8") + base64_str + "\r\n".encode("utf-8")
time.sleep(1)
print("auth msg:",auth_msg)
sock.send(auth_msg)
recv_auth = sock.recv(1024)
print(recv_auth.decode("utf-8"))
time.sleep(1)

mail_from = "MAIL FROM: test123@test.test\r\n"
sock.send(mail_from.encode("utf-8"))
time.sleep(1)

recv3 = sock.recv(1024)
recv3 = recv3.decode("utf-8")
time.sleep(1)

print("After MAIL FROM command:", recv3)

rcpt_to = "RCPT TO: alexander.engelhardt@htwg-konstanz.de\r\n"

sock.send(rcpt_to.encode("utf-8"))
time.sleep(1)

recv4 = sock.recv(1024)
recv4 = recv4.decode("utf-8")
time.sleep(1)

print("After RCPT TO command:", recv4)

data = "DATA\r\n"

sock.send(data.encode("utf-8"))
time.sleep(1)

recv5 = sock.recv(1024)
recv5 = recv5.decode("utf-8")
time.sleep(1)

print("After DATA command:", recv5)

subject = "Subject: Testing SMTP Client\r\n\r\n"

sock.send(subject.encode("utf-8"))
time.sleep(1)

date = time.strftime("%a, %d %b %Y %H:%M:%S +0000", time.gmtime())
date = date + "\r\n\r\n"

sock.send(date.encode("utf-8"))
time.sleep(1)
sock.send(msg.encode("utf-8"))
time.sleep(1)
sock.send(endmsg.encode("utf-8"))
time.sleep(1)
recv_msg = sock.recv(1024)

print("Response after sending message body:", recv_msg.decode("utf-8"))
time.sleep(1)

quit = "QUIT\r\n"
sock.send(quit.encode("utf-8"))
time.sleep(1)
recv5 = sock.recv(1024)
print(recv5.decode("utf-8"))
time.sleep(1)
sock.close()
client_sock.close()


