import socket
import time

def main():
    # IP = '127.0.0.1'
    IP = '172.20.103.205'
    PORT = 5005
    BUFF_SIZE = 1024
    runtime = 10
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.bind((IP, PORT))
    start = time.time()
    while time.time() - start < runtime:
        print sock.recvfrom(BUFF_SIZE)


if __name__ == '__main__':
    main()