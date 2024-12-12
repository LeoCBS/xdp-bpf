import socket
import time
import logging

logging.basicConfig(format='%(asctime)s %(message)s', datefmt='%d/%m/%Y %I:%M:%S %p', level=logging.DEBUG)

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

for i in range(1000):  # infinite loop
    text = f'produce msg count {i}'
    data = text.encode('ascii')
    sock.sendto(data, ("udpredirect",1025))
    logging.info(f"{text}")
    sleep_seconds = 1
    time.sleep(sleep_seconds)

