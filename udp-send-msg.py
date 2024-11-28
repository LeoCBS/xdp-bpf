import socket
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

text = 'client works'
data = text.encode('ascii')
print(sock.sendto(data, ("127.0.0.1",1025)))
